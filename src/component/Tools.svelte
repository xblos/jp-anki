<script lang="ts">
    import { deck, deckPath, jsonDeck } from '../stores'
    import { sanitizeTranscription } from '../util/string'
    import { showSuccessToast } from '../toasts'
    import { showConfirmModal, showConfirmModalPromise, showErrorModal, showLoadingModal } from '../modals'
    import { invoke } from '@tauri-apps/api'

    async function onSanitize() {
        if (!$deck) {
            showErrorModal('Select a deck first')
            return
        }
        try {
            for (const note of $deck.notes) {
                note.transcription = sanitizeTranscription(note.transcription)
            }
            await invoke('write_backup', { dir: deckPath(), json: jsonDeck() })
            showSuccessToast('Deck sanitized')
        } catch (err) {
            showErrorModal(null, err)
        }
    }

    async function onUpgradeMediaNaming() {
        if (!$deck) {
            showErrorModal('Select a deck first')
            return
        }
        try {
            await invoke('upgrade_media_naming', { dir: deckPath(), json: jsonDeck() })
            showSuccessToast('Media renamed')
        } catch (err) {
            showErrorModal(null, err)
        }
    }

    async function onGenerateTemplate() {
        if (!$deck) {
            showErrorModal('Select a deck first')
            return
        }
        try {
            const ok = await showConfirmModalPromise('This will overwrite any existing template. Continue?')
            if (!ok) return
            await invoke('write_template', { dir: deckPath() })
            showSuccessToast('Template generated')
        } catch (err) {
            showErrorModal(null, err)
        }
    }
</script>

<section>
    <h2 class="title">Tools</h2>
    <button class="form-button" on:click={onSanitize}>Sanitize Deck</button>
    <button class="form-button" on:click={onUpgradeMediaNaming}>Upgrade Media Naming</button>
    <button class="form-button" on:click={onGenerateTemplate}>Generate Template</button>
</section>

<style>
    button {
        width: 60%;
    }
</style>