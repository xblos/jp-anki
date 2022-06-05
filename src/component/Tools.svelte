<script lang="ts">
    import { deck, deckPath, jsonDeck, saveSettings, setSetting, settingsField } from '../stores'
    import { sanitizeTranscription } from '../util/string'
    import { showSuccessToast } from '../toasts'
    import { showConfirmModalPromise, showErrorModal } from '../modals'
    import { invoke } from '@tauri-apps/api'
    import FormInput from './FormInput.svelte'
    import { open } from '@tauri-apps/api/dialog'

    async function onChangeDefaultDir() {
        try {
            const path = await open({ directory: true })
            if (!path) return
            setSetting('defaultDir', path)
            await saveSettings()
        } catch (err) {
            showErrorModal(null, err)
        }
    }

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
            await invoke('upgrade_media_naming', {
                dir: deckPath(),
                json: jsonDeck(),
            })
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
            const ok = await showConfirmModalPromise(
                'This will overwrite any existing template. Continue?')
            if (!ok) return
            await invoke('write_template', { dir: deckPath() })
            showSuccessToast('Template generated')
        } catch (err) {
            showErrorModal(null, err)
        }
    }
</script>

<section class="form">
    <h2 class="title">Config</h2>
    <label for="default-dir">Default Directory</label>
    <div id="default-dir" class="combined-input">
        <FormInput value={settingsField('defaultDir')} disabled={true}/>
        <button class="form-button conf-button" on:click={onChangeDefaultDir}>Change</button>
    </div>
    <h2 class="title">Tools</h2>
    <button class="form-button" on:click={onSanitize}>Sanitize Deck</button>
    <button class="form-button hide" on:click={onUpgradeMediaNaming}>Upgrade Media Naming</button>
    <button class="form-button" on:click={onGenerateTemplate}>Generate Template</button>
</section>

<style>
    div {
        width: 80%;
        margin: 0 auto;
    }

    button {
        width: 60%;
    }

    .conf-button {
        width: 10em;
        min-width: 7em;
    }

    .hide {
        display: none;
    }

    label {
        float: none;
    }
</style>
