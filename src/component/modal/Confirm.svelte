<script lang="ts">
    import { getContext } from 'svelte'

    export let msg = 'Confirm'
    export let handler: (ok: boolean) => void

    const { _, close } = getContext('simple-modal')

    function onConfirm() {
        handler?.call(null, true)
        close()
    }

    function onCancel() {
        handler?.call(null, false)
        close()
    }
</script>

<section class="modal">
    <h2>{msg}</h2>
    <div class="button-bar">
        <button class="form-button" on:click={onCancel}>Cancel</button>
        <button class="form-button" on:click={onConfirm}>Confirm</button>
    </div>
</section>