<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import type { Field } from 'svelte-forms/types'
    import type { Writable } from 'svelte/store'
    import { capitalize } from '../util/string'

    export let id = '__input'
    export let showLabel: boolean = true
    export let label: string = null
    export let value: Writable<Field<string>>
    export let placeholder = null
    export let changeDelay = 400

    let dispatch = createEventDispatcher()
    let changeId: any = null

    async function onChange() {
        if (changeId != null) {
            clearTimeout(changeId)
        }
        changeId = setTimeout(() => dispatch('change'), changeDelay)
    }
</script>

{#if showLabel && id !== '__input'}
    <label class="form-label" for={id}>{label || capitalize(id)}</label>
{/if}
<textarea {id} class="preview-text-area" type="text" bind:value={$value.value} {placeholder} on:input={onChange} disabled={true} />