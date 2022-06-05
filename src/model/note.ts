export class Note {
    constructor(
        public word?: string,
        public reading?: string,
        public definition?: string,
        public transcription?: string,
        public useReading: boolean = false,
        public audioState?: string,
        public id?: number
    ) { }
}