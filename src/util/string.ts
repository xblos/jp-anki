import { isKanji } from './kanji'

export function capitalize(str: string): string {
    if (!str) return str
    return str.split(/[- ]/)
        .map(w => w[0].toUpperCase() + w.substring(1))
        .join(' ')
}

export function isRubyString(str: string): boolean {
    return /[「[]/.test(str)
}

export function ignoreRuby(str: string): string {
    const buf = []
    let open = false
    for (const c of str) {
        if (c === '「' || c === '[') {
            open = true
        } else if (c === '」' || c === ']') {
            open = false
        } else if (!open) {
            buf.push(c)
        }
    }
    return buf.join('')
}

export function formatRuby(str: string): string {
    const buf: string[] = []
    let separate = true

    for (const c of str) {
        if (separate && isKanji(c)) {
            buf.push(' ')
            separate = false
        } else if (c === '」' || c === ']') {
            separate = true
        }
        switch (c) {
            case '「':
                buf.push('[')
                break
            case '」':
                buf.push(']')
                break
            default:
                buf.push(c)
        }
    }

    if (buf[0] === ' ') {
        buf.shift()
    }

    return buf.join('')
}

export function sanitizeTranscription(tr: string) {
    const buf = []
    for (const c of tr) {
        switch (c) {
            case '!':
            case '！':
                buf.push('!')
                buf.push(' ')
                break
            case '? ':
            case '？':
                buf.push('?')
                buf.push(' ')
                break
            case '　':
                buf.push(' ')
                break
            case '…':
                buf.push('...')
                buf.push(' ')
                break
            case '(':
            case '（':
                buf.push(' ')
                buf.push('(')
                break
            case '）':
            case ')':
                trim(buf)
                buf.push(')')
                buf.push(' ')
                break
            case '\n':
                buf.push('<br>')
                break
            case '」':
                trim(buf)
                buf.push('」')
                buf.push(' ')
                break
            default:
                buf.push(c)
        }
    }
    return buf.join('').replace(/  +/g, ' ').trim()
}

function trim(buf: string[]) {
    while (buf.at(-1) === ' ') {
        buf.pop()
    }
}