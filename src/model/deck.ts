import type { Note } from "./note";

export class Deck {
    constructor(
        public name?: string,
        public id?: string,
        public notes: Note[] = []
    ) { }
}