<script lang="ts">
	import { invoke } from '@tauri-apps/api'
    import FileUploader from './FileUploader.svelte'
    import { deck, deckPathField, deckPath, jsonDeck, deckValue } from '../stores'
    import {
        hideLoadingModal,
        showConfirmModal,
        showConfirmModalPromise,
        showErrorModal,
        showLoadingModal,
        showNewDeckModal
    } from '../modals'
    import { showSuccessToast } from '../toasts'
    import { save } from '@tauri-apps/api/dialog'
import { Deck } from '../model/deck';

    async function onDeckSelected() {
        if (!deckPath()) {
            $deck = null
            return
        }
        try {
            $deck = JSON.parse(await invoke('open_deck', { dir: deckPath() }))
        } catch (err) {
            if (err.includes("does not exist")) {
                showConfirmModal(
                    'Deck not found, start a new one?',
                    () => onStartNewDeck()
                )
            } else {
                showErrorModal(null, err)
                deckPathField.clear()
            }
        }
    }

    async function onStartNewDeck() {
        try {
            const ok = await showNewDeckModal()
            if (ok) {
                $deck = new Deck(deckValue('name'), deckValue('id'))
                await invoke('write_deck', { dir: deckPath(), json: jsonDeck() })
                showSuccessToast('New deck created')
            }
        } catch (err) {
            showErrorModal(null, err)
        }
    }

    async function onOpenCurDir() {
        $deckPathField.value = "./"
        onDeckSelected()
    }

    async function onWrite() {
        try {
            await invoke('write_deck', { dir: deckPath(), json: jsonDeck() })
            showSuccessToast('Deck written successfully')
        } catch (err) {
            showErrorModal(null, err)
        }
    }

    async function onWriteApkg() {
        try {
            let dest = await save({
                defaultPath: 'out',
                filters: [{ name: 'Anki Package', extensions: ['apkg'] }]
            })
            if (!dest) return

            const template_exists = await invoke('check_template', { dir: deckPath() })
            
            if (!template_exists) {
                const ok = await showConfirmModalPromise('No template found, generate one?')
                if (!ok) {
                    showErrorModal('A template is required')
                    return
                }
                await invoke('write_template', { dir: deckPath() })
            }

            showLoadingModal()

            const json = await invoke('fetch_audio', { dir: deckPath(), json: jsonDeck() })
            await invoke('write_deck', { dir: deckPath(), json })
            await invoke('write_apkg', { dest, dir: deckPath(), json })

            showSuccessToast('Package written successfully')
        } catch (err) {
            showErrorModal(null, err)
        } finally {
            hideLoadingModal()
        }
    }

    async function onRestoreBackup() {
        try {
            $deck = JSON.parse(await invoke('restore_backup', { dir: deckPath() }))
            showSuccessToast('Backup restored')
        } catch (err) {
            if (err.includes('file not found')) {
                showErrorModal('Backup file not found')                
            } else {
                showErrorModal(null, err)
            }
        }
    }
</script>

<section>
    <h2 class="title">Choose Deck</h2>
    <FileUploader id="deck" file={deckPathField} options={{ directory: true }} on:change={onDeckSelected}/>
    {#if $deck}
        <h2 class="success-text">Deck loaded successfully</h2>
        <div class="deck-info">
            <h3>Note Count:&nbsp;&nbsp;{$deck.notes.length}</h3>
            {#if $deck.notes.length > 0}
                <h3>Last added word:&nbsp;&nbsp;{$deck.notes.at(-1).word}</h3>
            {/if}
        </div>
        <button class="form-button" on:click={onWrite}>Write</button>
        <button class="form-button" on:click={onWriteApkg}>Write .apkg</button>
        <button class="form-button" on:click={onRestoreBackup}>Restore Backup</button>
    {:else}
        <button class="form-button" on:click={onOpenCurDir}>Open current directory</button>
    {/if}
</section>

<style>
    .deck-info {
        display: flex;
        flex-direction: column;
    }

    .success-text {
        margin-top: .3em;
        margin-bottom: .2em;
    }

    .deck-info > *:not(:last-child) {
        margin-bottom: .2em;
    }

    .form-button {
        width: 70%;
        height: 3em;
    }
</style>