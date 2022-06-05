import { field, form } from 'svelte-forms'
import { pattern, required } from 'svelte-forms/validators'
import { get, writable } from 'svelte/store'
import type { Deck } from './model/deck'
import type { Note } from './model/note'
import { fieldValue, setField, WritableField, writableField } from './util/form'

export const deck = writable<Deck>(null)
export let words: string[] = []

deck.subscribe(deck => {
    words = deck ? deck.notes.map(it => it.word) : []
})

const word = field('word', '', [required()])
const reading = field('reading', '')
const useReading = field('useReading', false)
const definition = field('definition', '', [required()])
const media = field('media', '')
const transcription = field('transcription', '', [required()])

const deckName = field('name', '', [required()])
const deckId = field('id', '', [required(), pattern(/\d+/)])

export const noteForm = form(word, reading, useReading, definition, transcription, media)
export const deckForm = form(deckName, deckId)
export const deckPathField = field('deckPath', '', [required()])

let maxNoteId = 0

export function deckPath(): string {
    return get(deckPathField).value
}

export function jsonDeck(): string {
    return JSON.stringify(get(deck))
}

export function deckField(name: string): WritableField {
    return writableField(deckForm, name)
}

export function deckValue<T>(name: string): T {
    return fieldValue(deckForm, name)
}

export function generateDeckId() {
    setField(deckForm, 'id', Math.round(Math.random() * Math.pow(10, 16)))
}

export function parseDeckId() {
    setField(deckForm, 'id', parseInt(deckValue('id')))
}

export function noteField(name: string): WritableField {
    return writableField(noteForm, name)
}

export function noteValue<T>(name: string): T {
    return fieldValue(noteForm, name)
}

export function nextNoteId(): number {
    if (maxNoteId < 1) {
        for (const note of get(deck).notes) {
            if (note.id === undefined || note.id === null) {
                throw Error("Note ID error")
            }
            if (note.id > maxNoteId) {
                maxNoteId = note.id
            }
        }
    }

    maxNoteId += 1
    return maxNoteId
}

export function curNoteId(): number {
    return maxNoteId < 1 ? nextNoteId() : maxNoteId
}

export function addNote(note: Note) {
    deck.update(() => {
        let value = get(deck)
        value.notes.push(note)
        return value
    })
}