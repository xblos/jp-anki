<script lang="ts">
    import { CollapsibleCard } from 'svelte-collapsible'
    import { writable } from 'svelte/store'

    export let msg = null
    export let desc: any = null

    if (desc && typeof(desc) !== 'string' && desc.toString) {
        desc = desc.toString()
    }

    let open = writable(false)
</script>

<section class="modal">
    <h1 class="error-text">{msg || 'An error occurred'}</h1>
    {#if desc}
        <CollapsibleCard bind:open={$open}>
            <h3 slot="header">{$open ? "Hide" : "Show"} details</h3>
            <p slot="body">{desc}</p>
        </CollapsibleCard>
    {/if}
</section>

<style>
    h3 {
        font-weight: lighter;
    }
</style>