<script lang="ts">
    import type { Field } from 'svelte-forms/types'
    import type { Writable } from 'svelte/store'
    import { capitalize } from '../util/string'

    export let id = '__input'
    export let showLabel: boolean = true
    export let label: string = null
    export let value: Writable<Field<string>>
</script>

{#if showLabel && id !== '__input'}
    <label class="form-label" for={id}>{label || capitalize(id)}</label>
{/if}
<input {id} class="form-input" type="text" bind:value={$value.value} autocomplete="new-password"/>