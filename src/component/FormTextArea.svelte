<script lang="ts">
    import type { Field } from 'svelte-forms/types';
    import type { Writable } from 'svelte/store'
    import { capitalize } from '../util/string'

    export let id = '__input'
    export let value: Writable<Field<string>>
</script>

{#if id !== '__input'}
    <label for={id}>{capitalize(id)}</label>
{/if}
<textarea {id} class="form-textarea" type="text" bind:value={$value.value} autocomplete="off"/>