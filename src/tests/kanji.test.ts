import { rubifyTranscription } from '../util/kanji'

test('rubifyTranscription', () => {
    const test = 'あ、電気＊でんき light, electricity＊を照らして＊てらして illuminate＊、僕を導いてるよね、これ絶対にー'
    const expected = 'あ、 電気[でんき] (light, electricity) を 照[て]らして (illuminate)、僕を導いてるよね、これ絶対にー'
    const actual = rubifyTranscription(test)
    expect(actual).toBe(expected)
})