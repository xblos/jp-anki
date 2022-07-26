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
    import { prettifyFormErrors } from '../util/form'
    import { formatRuby, isRubyString, sanitizeTranscription } from '../util/string'
    import { field } from 'svelte-forms'
    import { rubify } from '../util/kanji'
    import PreviewInput from './PreviewInput.svelte'
    import { onMount } from 'svelte'

    const notePreview = field('preview', '')
    
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

        addNote(
            new Note(
                noteValue<string>('word'),
                noteValue<string>('reading') || null,
                noteValue<string>('definition'),
                sanitizeTranscription(noteValue('transcription')),
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
        $notePreview.value = null
        showSuccessToast('Note added')
    }

    async function onClear() {
        showConfirmModal('Reset all fields?', () => {
            noteForm.clear()
            $notePreview.value = null
        })
    }

    function onWordChange() {
        const word: string = noteValue('word')
        const reading: string = noteValue('reading')
        const isRuby = isRubyString(word)
        if (word && reading && !isRuby) {
            try {
                $notePreview.value = rubify(word, reading)
            } catch (err) {
                $notePreview.value = '???'
            }
        } else {
            $notePreview.value = isRuby ? formatRuby(word) : word
        }
    }
</script>

<section class="form">
    <FormInput id="word" value={noteField('word')} on:change={onWordChange} />
    <label for="reading">Reading</label>
    <div id="reading" class="combined-input">
        <FormInput value={noteField('reading')} on:change={onWordChange} />
        <Checkbox value={noteField('useReading')} />
    </div>
    <PreviewInput id="preview" value={notePreview} />
    <FormInput id="definition" value={noteField('definition')} />
    <FormTextArea id="transcription" value={noteField('transcription')} />
    <label for="media">Media</label>
    <FileUploader id="media" file={noteField('media')} />
    <div class="button-bar">
        <button class="form-button" on:click={onClear}>Clear</button>
        <button class="form-button" on:click={onAdd}>Add</button>
    </div>
</section>
