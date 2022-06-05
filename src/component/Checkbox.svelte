<script lang="ts">
    import type { Field } from 'svelte-forms/types'
    import type { Writable } from 'svelte/store'

    export let value: Writable<Field<boolean>>
</script>

<div class="checkbox">
    <input id="use-reading" type="checkbox" class="checkbox-input" bind:checked={$value.value}/>
    <label for="use-reading" class="checkbox-label">Use Reading</label>
</div>