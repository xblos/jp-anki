const kanjiReg = /[\u3000-\u303F\u4E00-\u9FEF]/

export function isKanji(c: string): boolean {
    return kanjiReg.test(c)
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

export function generateRubyString(kanjiStr: string, kanaStr: string): string {
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