<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import type { Field } from 'svelte-forms/types'
    import type { Writable } from 'svelte/store'
    import { capitalize } from '../util/string'

    export let id = '__input'
    export let value: Writable<Field<string>>
    export let changeDelay = 400

    let dispatch = createEventDispatcher()
    let changeId: any = null
    let domRef: any

    async function onChange() {
        if (changeId != null) {
            clearTimeout(changeId)
        }
        changeId = setTimeout(() => dispatch('change'), changeDelay)
    }

    function onKeyPress(ev: KeyboardEvent) {
        if (ev.ctrlKey && ev.key === 'Enter')
            dispatch('ctrlEnter', { domRef })
        else if (ev.ctrlKey && ev.key === ' ')
            dispatch('ctrlSpace', { domRef })
    }
</script>

{#if id !== '__input'}
    <label for={id}>{capitalize(id)}</label>
{/if}
<textarea {id} class="form-textarea" type="text" bind:value={$value.value} autocomplete="off" on:input={onChange} on:keydown={onKeyPress} bind:this={domRef} />
