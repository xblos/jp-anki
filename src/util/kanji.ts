const kanjiReg = /[\u3000-\u303F\u4E00-\u9FEF]/
const symbolsReg = /[、。！？・～「」ー＝＋]/
const hiraganaReg = /[ぁ-ん]/
const sep = '|'

export function isKanji(c: string): boolean {
    return !symbolsReg.test(c) && kanjiReg.test(c)
}

export function isHiragana(c: string) {
    return hiraganaReg.test(c)
}

function isSymbol(c: string) {
    return symbolsReg.test(c)
}

function isSpace(c: string) {
    return c === ' ' || c === '　'
}

function isComma(c: string) {
    return c === ',' || c === ';'
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
                    throw Error(`Kanji and Kana sentences do not match (${kanjiStr} =/= ${kanaStr})`)
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

export function rubifyText(t: string): string {
    const buf: string[] = []
    
    for (let i = 0; i < t.length; i++) {
        if (t[i] === sep) {
            i += 1

            let j = findSpace(t, i)
            const kanjiStr = t.substring(i, j)

            i = j + 1
            j = findSep(t, i)
            const otherStr = t.substring(i, j)

            const kanaStr = extractHiragana(otherStr)
            const trStr = extractTr(otherStr)

            let rubified: string
            if (kanaStr) {
                try {
                    rubified = rubify(kanjiStr, kanaStr)
                } catch (err) {
                    throw Error(`Failed to rubify string. Result: ${buf.join('')}\n${err}`)
                }
            } else {
                rubified = null
            }

            if (buf.length > 0 && !isSpace(buf[buf.length - 1]))
                buf.push(' ')

            buf.push(rubified || kanjiStr)

            if (trStr) {
                buf.push(trStr)
                if (!isSymbol(t[j + 1]) && !isSpace(t[j + 1]))
                    buf.push(' ')
            }
            
            i = j
        } else {
            buf.push(t[i])
        }
    }

    return buf.join('')
}

function findSpace(t: string, i: number): number {
    for (; i < t.length; i++) {
        if (isSpace(t[i]))
            break
    }

    if (i === t.length)
        throw Error('Malformed text')

    return i
}

function findSep(t: string, i: number): number {
    for (; i < t.length; i++) {
        if (t[i] === sep)
            break
    }

    if (i === t.length)
        throw Error('Malformed text')

    return i
}

function extractHiragana(t: string): string {
    const buf: string[] = []
    let i = 0

    while (i < t.length && !isHiragana(t[i]))
        i += 1

    while (i < t.length && isHiragana(t[i]))
        buf.push(t[i++])

    return buf.join('')
}

function extractTr(t: string): string {
    const buf: string[] = []
    for (let i = 0; i < t.length; i++) {
        let j = i
        while (j < t.length && !isHiragana(t[j]) && !isComma(t[j]))
            j += 1
        if (j !== i)
            buf.push(t.substring(i, j))
        i = j
    }
    const str = buf.join('').trim()
    return str ? ` (${str})` : null
}