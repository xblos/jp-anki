<script lang="ts">
    import Checkbox from './Checkbox.svelte'
    import FileUploader from './FileUploader.svelte'
    import FormInput from './FormInput.svelte'
    import FormTextArea from './FormTextArea.svelte'
    import { noteForm, noteField, addNote, noteValue, deck, deckPath, jsonDeck, words, nextNoteId, curNoteId } from '../stores'
    import { Note } from '../model/note'
    import { showSuccessToast } from '../toasts';
    import { showConfirmModal, showConfirmModalPromise, showErrorModal } from '../modals'
    import { invoke } from '@tauri-apps/api/tauri'
    import { prettifyFormErrors } from '../util/form'
    import { sanitizeTranscription } from '../util/string'

    async function onAdd() {
        if (!$deck) {
            showErrorModal('Select a deck first')
            return
        }

        await noteForm.validate()

        if (words.includes(noteValue('word'))) {
            const ok = await showConfirmModalPromise(`${noteValue('word')} already exists, continue?`)
            if (!ok) return;
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

        if (noteValue('media')) {
            try {
                await invoke('move_media', {
                    destDir: deckPath(),
                    srcFile: noteValue('media'),
                    id: curNoteId()
                })
            } catch (err) {
                showErrorModal('Failed to move media file', err)
                return
            }
        }

        addNote(new Note(
            noteValue<string>('word'),
            noteValue<string>('reading') || null,
            noteValue<string>('definition'),
            sanitizeTranscription(noteValue('transcription')),
            !!noteValue('useReading'),
            null,
            curNoteId()
        ))

        try {
            await invoke('write_backup', { dir: deckPath(), json: jsonDeck() })
        } catch (err) {
            showErrorModal('Backup error', err)
            return
        }

        noteForm.clear()

        nextNoteId()
        showSuccessToast('Note added')
    }

    async function onClear() {
        showConfirmModal('Reset all fields?', () => noteForm.clear())
    }
</script>

<section class="form">
    <FormInput id="word" value={noteField('word')}/>
    <label for="reading">Reading</label>
    <div id="reading" class="combined-input">
        <FormInput value={noteField('reading')}/>
        <Checkbox value={noteField('useReading')}/>
    </div>
    <FormInput id="definition" value={noteField('definition')}/>
    <FormTextArea id="transcription" value={noteField('transcription')}/>
    <label for="media">Media</label>
    <FileUploader id="media" file={noteField('media')}/>
    <div class="button-bar">
        <button class="form-button" on:click={onClear}>Clear</button>
        <button class="form-button" on:click={onAdd}>Add</button>
    </div>
</section>