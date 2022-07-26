const kanjiReg = /[\u3000-\u303F\u4E00-\u9FEF]/
const symbolsReg = /[、。！？・～「」ー＝＋]/
const hiraganaReg = /[ぁ-ん]/

export function isKanji(c: string): boolean {
    return !symbolsReg.test(c) && kanjiReg.test(c)
}

export function isHiragana(c: string) {
    return hiraganaReg.test(c)
}

export function isSymbol(c: string) {
    return symbolsReg.test(c)
}

/* Adapted from: https://github.com/zacharied/autofurigana */
export function generateFurigana(kanjiStr: string, kanaStr: string): string[][] {
    let pairs: string[][] = []
    let kanaBuf: string[] = []
    let kanjiBuf: string[] = []
        
    function addPair() {
        pairs.push([kanjiBuf.join(''), kanaBuf.join('')])
        kanaBuf = []
        kanjiBuf = []
    }

    let j = 0

    for (let kp = 0; kp < kanjiStr.length; kp++) {
        kanjiBuf.push(kanjiStr[kp])

        if (kp + 1 == kanjiStr.length) {
            if (isKanji(kanjiStr[kp])) {
                while (j < kanaStr.length) {
                    kanaBuf.push(kanaStr[j])
                    j++
                }
            } else {
                kanaBuf = []
            }
            addPair()
        } else if (isKanji(kanjiStr[kp]) && !isKanji(kanjiStr[kp + 1])) {
            while (kanjiStr[kp + 1] !== kanaStr[j] || kanaBuf.length < kanjiBuf.length) {
                if (kanaStr[j] === undefined) {
                    throw Error('Kanji and Kana sentences do not match')
                }
                kanaBuf.push(kanaStr[j])
                j++
            }
            addPair()
        } else if (!isKanji(kanjiStr[kp]) && isKanji(kanjiStr[kp + 1])) {
            kanaBuf = []
            j += kanjiBuf.length
            addPair()
        }
    }

    return pairs
}

export function rubify(kanjiStr: string, kanaStr: string): string {
    let pairs = generateFurigana(kanjiStr, kanaStr)
    let str: string[] = []

    for (let i = 0; i < pairs.length; i++) {
        if (pairs[i][1]) {
            if (i != 0) {
                str.push(' ')
            }
            str.push(`${pairs[i][0]}[${pairs[i][1]}]`)
        } else {
            str.push(pairs[i][0])
        }
    }

    return str.join('')
}

export function rubifyTranscription(t: string): string {
    const sep = '＊'
    let kanjiPos = -1
    let buf: string[] = []
    let fgBuf: string[] = []
    let trBuf: string[] = []

    for (let i = 0; i < t.length; i++) {
        if (kanjiPos < 0) {
            if (isKanji(t[i]))
                kanjiPos = i
            else
                buf.push(t[i])
        } else if (t[i] === sep) {
            let j = i + 1
            for (; j < t.length; j++) {
                if (t[j] === sep)
                    break
                if (isHiragana(t[j])) {
                    fgBuf.push(t[j])
                } else if (t[j] === ' ' || t[j] === '　') {
                    if (trBuf.length > 0 && trBuf[trBuf.length - 1] !== ' ')
                        trBuf.push(' ')
                } else {
                    trBuf.push(t[j])
                }
            }
            if (j === t.length || j === i + 1)
                throw Error('Malformed transcription')

            let kanjiStr = t.substring(kanjiPos, i)
            let rubified = rubify(kanjiStr, fgBuf.join(''))

            if (buf.length > 0) buf.push(' ')
            buf.push(rubified)
            buf.push(` (${trBuf.join('')})`)
            if (!isSymbol(t[j + 1]))
                buf.push(' ')

            i = j
            kanjiPos = -1
            fgBuf = []
            trBuf = []
        }
    }

    if (kanjiPos >= 0)
        buf.push(t.substring(kanjiPos))

    return buf.join('')
}