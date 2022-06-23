<script lang='ts'>
    import { open } from '@tauri-apps/api/dialog'
    import { listen } from '@tauri-apps/api/event'
    import { createEventDispatcher, onDestroy } from 'svelte'

    export let id = 'file-uploader'
    export let name = 'Browse...'
    export let file: any
    export let dragAndDrop = true
    export let options = {}

    const dispatch = createEventDispatcher()

    let unlisten: any

    if (dragAndDrop) {
        unlisten = listen('tauri://file-drop', async ev => {
            $file.value = ev.payload[0]
            onChange()
        })
    }

    function onChange() {
        dispatch('change')
    }

    async function onOpen() {
        let path = await open(options)
        if (path) {
            $file.value = path
            onChange()
        }
    }

    function onClear() {
        $file.value = null
        onChange()
    }

    onDestroy(async () => unlisten && (await unlisten).call(null))
</script>

<section>
    <div {id}>
        <div id='drop-area'>
            <button class="file-uploader" on:click={onOpen}>{name}</button>
            <!-- <label id='file' for='file-upl' class='file-uploader'>{name}</label>
            <input id='file-upl' type='file' bind:value={$file.value} on:change={onChange}/> -->
            <p id='filename'>
                {$file.value?.replace(/.*[\/\\]/, '') || 'No file selected'}
            </p>
            <button class="file-uploader" on:click="{onClear}">Clear</button>
        </div>
    </div>
</section>

<style>
    section {
        padding: 1em 0;
        display: inline-block;
        width: 100%;
    }

    div {
        display: flex;
        flex-direction: row;
        vertical-align: middle;
        align-items: center;
    }
    
    p {
        height: 1.3em;
        align-self: baseline;
    }

    #filename {
        width: 100%;
        margin-left: 0.5em;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        text-align: left;
        filter: brightness(50%);
    }

    #drop-area {
        overflow: hidden;
        background-image: repeating-linear-gradient(
                -60deg,
                var(--border-color),
                var(--border-color) 12px,
                transparent 12px,
                transparent 17px,
                var(--border-color) 17px
            ),
            repeating-linear-gradient(
                30deg,
                var(--border-color),
                var(--border-color) 12px,
                transparent 12px,
                transparent 17px,
                var(--border-color) 17px
            ),
            repeating-linear-gradient(
                120deg,
                var(--border-color),
                var(--border-color) 12px,
                transparent 12px,
                transparent 17px,
                var(--border-color) 17px
            ),
            repeating-linear-gradient(
                210deg,
                var(--border-color),
                var(--border-color) 12px,
                transparent 12px,
                transparent 17px,
                var(--border-color) 17px
            );
        background-size: 2px 100%, 100% 2px, 2px 100%, 100% 2px;
        background-position: 0 0, 0 0, 100% 0, 0 100%;
        background-repeat: no-repeat;
        width: 100%;
        padding: 1em;
        border-radius: 0.3em;
    }
</style>
