<script lang="ts">
    import Checkbox from './Checkbox.svelte'
    import FileUploader from './FileUploader.svelte'
    import FormInput from './FormInput.svelte'
    import FormTextArea from './FormTextArea.svelte'
    import {
        noteForm,
        noteField,
        addNote,
        noteValue,
        deck,
        deckPath,
        jsonDeck,
        words,
        nextNoteId,
        curNoteId,
        deckValue,
    } from '../stores'
    import { Note } from '../models'
    import { showSuccessToast } from '../toasts'
    import { showConfirmModal, showConfirmModalPromise, showErrorModal } from '../modals'
    import { invoke } from '@tauri-apps/api/tauri'
    import { prettifyFormErrors, setField } from '../util/form'
    import { formatRuby, isRubyString, sanitizeTranscription } from '../util/string'
    import { field } from 'svelte-forms'
    import { rubify, rubifyText } from '../util/kanji'
    import PreviewInput from './PreviewInput.svelte'
    import { onMount } from 'svelte'
    import PreviewTextArea from './PreviewTextArea.svelte'

    const wordPreview = field('wordPreview', '')
    const trPreview = field('trPreview', '')
    
    onMount(() => onWordChange())

    async function onAdd() {
        if (!$deck) {
            showErrorModal('Select a deck first')
            return
        }

        await noteForm.validate()

        if (words.includes(noteValue('word'))) {
            const ok = await showConfirmModalPromise(`${noteValue('word')} already exists, continue?`)
            if (!ok) return
        }

        if ($noteForm.errors.length > 0) {
            showErrorModal('Missing required fields', prettifyFormErrors(noteForm))
            return
        }

        if (noteValue('word') === noteValue('reading')) {
            showErrorModal('Word and Reading are identical',
                'Only add Reading when the word is written in kanji')
            return
        }
        
        if (noteValue('useReading') && !noteValue('reading')) {
            showErrorModal('Missing required fields', 'Use Reading checkbox is marked, but the Reading field is empty')
            return
        }

        if (noteValue('media')) {
            try {
                await invoke('move_media', {
                    destDir: deckPath(),
                    srcFile: noteValue('media'),
                    deckId: deckValue('id'),
                    noteId: curNoteId(),
                })
            } catch (err) {
                showErrorModal('Failed to move media file', err)
                return
            }
        } else {
            if (!noteValue('transcription')) {
                showErrorModal('Missing important fields', 'Add either a transcription or a media file')
                return
            } else {
                if (!await showConfirmModalPromise('No media selected. Continue?'))
                    return
            }
        }

        let transcription: string
        try {
            transcription = rubifyText(sanitizeTranscription(noteValue('transcription')))
        } catch (err) {
            showErrorModal('Error while parsing transcription', err)
            return
        }

        addNote(
            new Note(
                noteValue<string>('word'),
                noteValue<string>('reading') || null,
                noteValue<string>('definition'),
                transcription,
                !!noteValue('useReading'),
                null,
                curNoteId()
            )
        )

        try {
            await invoke('write_backup', { dir: deckPath(), json: jsonDeck() })
        } catch (err) {
            showErrorModal('Backup error', err)
            return
        }

        noteForm.clear()

        nextNoteId()
        clearPreviews()
        showSuccessToast('Note added')
    }

    async function onClear() {
        showConfirmModal('Reset all fields?', () => {
            noteForm.clear()
            clearPreviews()
        })
    }

    function onWordChange() {
        const word: string = noteValue('word')
        const reading: string = noteValue('reading')
        const isRuby = isRubyString(word)
        if (word && reading && !isRuby) {
            try {
                $wordPreview.value = rubify(word, reading)
            } catch (err) {
                $wordPreview.value = '???'
            }
        } else {
            $wordPreview.value = isRuby ? formatRuby(word) : word
        }
    }

    function onTrChange() {
        try {
            $trPreview.value = sanitizeTranscription(rubifyText(noteValue('transcription')))
        } catch (err) {
            $trPreview.value = '???'
        }
    }

    function onInsertRubySeparators(ev: CustomEvent) {
        const start = ev.detail.domRef.selectionStart
        const end = ev.detail.domRef.selectionEnd

        let tr: string = noteValue('transcription')
        let trl = tr.slice(0, start) + '|'
        let trm = tr.slice(start, end)
        let trr = '|' + tr.slice(end)

        setField(noteForm, 'transcription', trl + trm + trr)

        setTimeout(() => ev.detail.domRef.setSelectionRange(end + 1, end + 1), 0)
    }

    function onNextSeparator(ev: CustomEvent) {
        const i = ev.detail.domRef.selectionStart
        const tr: string = noteValue('transcription')
        const sep = tr.indexOf('|', i)
        if (sep >= 0)
            setTimeout(() =>  ev.detail.domRef.setSelectionRange(sep + 1, sep + 1), 0)
    }

    function clearPreviews() {
        $trPreview.value = null
        $wordPreview.value = null
    }
</script>

<section class="form">
    <FormInput id="word" value={noteField('word')} on:change={onWordChange} />
    <label for="reading">Reading</label>
    <div id="reading" class="combined-input">
        <FormInput value={noteField('reading')} on:change={onWordChange} />
        <Checkbox value={noteField('useReading')} />
    </div>
    <PreviewInput id="word-preview" value={wordPreview} />
    <FormInput id="definition" value={noteField('definition')} />
    <FormTextArea
        id="transcription"
        value={noteField('transcription')}
        on:change={onTrChange}
        on:ctrlSpace={onInsertRubySeparators}
        on:ctrlEnter={onNextSeparator}
        changeDelay={700} />
    <PreviewTextArea id="transcription-preview" value={trPreview} />
    <label for="media">Media</label>
    <FileUploader id="media" file={noteField('media')} />
    <div class="button-bar">
        <button class="form-button" on:click={onClear}>Clear</button>
        <button class="form-button" on:click={onAdd}>Add</button>
    </div>
</section>