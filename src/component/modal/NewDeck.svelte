<script lang="ts">
    import { deckField, deckForm, deckValue, generateDeckId, parseDeckId } from '../../stores'
    import FormInput from '../FormInput.svelte'
    import { getContext } from 'svelte'
    import { showErrorModal } from '../../modals';
    import Icon from 'svelte-awesome';
import { refresh } from 'svelte-awesome/icons';

    export let handler: (ok: boolean) => void

    const { _, close } = getContext('simple-modal')

    async function onConfirm() {
        await deckForm.validate()

        if ($deckForm.hasError('id.pattern') && !$deckForm.hasError('id.required')) {
            showErrorModal('The ID must be a valid number')
            return
        }

        if (deckValue('id') < 0) {
            showErrorModal('The ID cannot be negative')
            return
        }

        if ($deckForm.errors.length > 0) {
            showErrorModal('Missing required fields')
            return
        }

        try {
            parseDeckId()
        } catch (err) {
            showErrorModal('The ID must be a valid number')
            return
        }

        handler?.call(null, true)
        close()
    }

    function onCancel() {
        handler?.call(null, false)
        close()
    }

    function onGenerateId() {
        generateDeckId()
    }
</script>

<section class="modal form">
    <h2>New Deck</h2>
    <FormInput id="name" value={deckField('name')}/>
    <label for="id">ID</label>
    <div id="id" class="combined-input">
        <FormInput value={deckField('id')}/>
        <button id="gen" class="form-button" on:click={onGenerateId}>
            <Icon data={refresh}/>
            Generate
        </button>
    </div>
    <div class="button-bar">
        <button class="form-button" on:click={onCancel}>Cancel</button>
        <button class="form-button" on:click={onConfirm}>Confirm</button>
    </div>
</section>

<style>
    #gen {
        max-width: 7em;
    }
</style>