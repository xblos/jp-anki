<script lang="ts">
    import { Router, link, Route } from 'svelte-routing'
    import { Icon } from 'svelte-awesome'
    import { folderOpen, plus, wrench } from 'svelte-awesome/icons'
    import DeckManager from './component/DeckManager.svelte'
    import NoteEditor from './component/NoteEditor.svelte'
    import Tools from './component/Tools.svelte'
    import { SvelteToast } from '@zerodevx/svelte-toast'
    import Modal from 'svelte-simple-modal'
    import {
        confirmModal,
        errorModal,
        loadingModal,
        newDeckModal,
        showErrorModal,
    } from './modals'
    import { onMount } from 'svelte'
    import { loadSettings } from './stores'

    export let url = '/'

    const iconScale = 1.3
    
    onMount(async () => {
        try {
            await loadSettings()
        } catch (err) {
            showErrorModal('Error while reading settings file', err)
        }
    })
</script>

<main>
    <Router {url}>
        <div class="sidebar">
            <nav>
                <a href="/" use:link>
                    <button>
                        <Icon data={folderOpen} scale={iconScale} />
                    </button>
                </a>
                <a href="note" use:link>
                    <button>
                        <Icon data={plus} scale={iconScale} />
                    </button>
                </a>
                <a href="tools" use:link>
                    <button>
                        <Icon data={wrench} scale={iconScale} />
                    </button>
                </a>
            </nav>
        </div>

        <div>
            <Route path="/" component={DeckManager} />
            <Route path="note" component={NoteEditor} />
            <Route path="tools" component={Tools} />
            <SvelteToast />
        </div>
    </Router>

    <Modal
        classWindow="modal"
        closeButton={false}
        closeOnOuterClick={false}
        closeOnEsc={false}
        show={$confirmModal}
    />
    <Modal
        classWindow="modal"
        closeButton={false}
        closeOnOuterClick={false}
        closeOnEsc={false}
        show={$loadingModal}
    />
    <Modal
        classWindow="modal"
        closeButton={false}
        closeOnOuterClick={false}
        closeOnEsc={false}
        show={$newDeckModal}
    />
    <Modal classWindow="modal" closeButton={false} show={$errorModal} />
</main>

<style>
    :global(:root) {
        --color: white;
        --accent-color: rgb(255, 62, 0);
        --accent-color-bg: rgb(92, 23, 0);
        --success-color: rgb(76, 129, 76);
        --border-focus-color: rgb(134, 134, 134);
        --accent-color-secondary: rgb(127, 122, 192);
        --background-color: rgb(33, 33, 33);
        --background-color-darker: rgb(29, 29, 29);
        --border-color: rgb(63, 63, 63);
        --button-color: rgb(41, 41, 41);
        --sidebar-width: 3em;
    }

    :global(body) {
        color: var(--color);
        background-color: var(--background-color);
    }

    :global(button, input:not(.checkbox-input), textarea, .file-uploader) {
        color: var(--color);
        background-color: var(--button-color);
        border: 1px solid var(--border-color);
    }

    :global(button:hover, .file-uploader:hover) {
        filter: brightness(150%);
    }

    :global(button:active, .file-uploader:active) {
        background-color: var(--button-color) !important;
        filter: brightness(120%);
    }

    :global(button:disabled) {
        filter: brightness(70%);
    }

    :global(h1, h2, h3, p) {
        color: var(--color);
        font-weight: 300;
    }

    :global(h1) {
        font-size: 3em;
    }

    :global(h2) {
        font-size: 2em;
    }

    :global(.error-text) {
        color: var(--accent-color);
        font-size: 3em;
        font-weight: 300;
    }

    :global(.success-text) {
        color: var(--success-color);
    }

    :global(label:not(.file-uploader, .checkbox-label)) {
        color: var(--color);
        float: left;
        filter: brightness(80%);
        font-weight: lighter;
    }

    :global(.file-uploader) {
        padding: 0.7em 1.5em;
        margin: 0;
    }

    :global(.form) {
        margin: 0 auto;
    }

    :global(.form-input, .form-textarea, .form-button, .preview-input, .preview-text-area) {
        display: inline-block;
        width: 100%;
        margin: 1em 0;
    }

    :global(.preview-input, .preview-text-area) {
        filter: brightness(90%);
    }

    :global(.form-textarea, .preview-text-area) {
        height: 5.5em;
        resize: none;
    }

    :global(.button-bar) {
        display: flex;
        flex-direction: row;
    }

    :global(.button-bar :not(:last-child)) {
        margin-right: 1em;
    }

    :global(input[type='file']) {
        display: none;
    }

    :global(input[type='text']:focus, textarea:focus, input[type='checkbox']:focus) {
        outline: 2px solid var(--border-focus-color);
        outline-offset: -1px;
    }

    :global(.checkbox) {
        display: flex;
        flex-direction: row;
        align-items: baseline;
        padding: 0 1em;
    }

    :global(.checkbox-label) {
        white-space: nowrap;
        margin-left: 0.5em;
        font-weight: lighter;
    }

    :global(.combined-input) {
        display: flex;
        flex-direction: row;
        flex-wrap: nowrap;
        align-items: baseline;
        margin-top: 1.5em;
    }

    :global(.combined-input > *:not(:last-child)) {
        margin-right: 1em;
    }

    :global(input[type='checkbox']) {
        cursor: pointer;
        appearance: none;
        font: inherit;
        color: var(--border-color);
        width: 1.15em;
        height: 1.15em;
        border: 0.15em solid currentColor;
        border-radius: 0.15em;
        transform: translateY(-0.075em);
        display: grid;
        place-content: center;
    }

    :global(input[type='checkbox']::before) {
        content: '';
        width: 0.65em;
        height: 0.65em;
        transform: scale(0);
        transition: 120ms transform ease-in-out;
        box-shadow: inset 1em 1em var(--accent-color);
        transform-origin: bottom left;
        clip-path: polygon(14% 44%, 0 65%, 50% 100%, 100% 16%, 80% 0%, 43% 62%);
    }

    :global(input[type='checkbox']:checked::before) {
        transform: scale(1);
    }

    :global(.modal) {
        border-radius: 0 !important;
        background-color: var(--background-color) !important;
    }

    :global(.title) {
        margin-top: 0.2em !important;
        margin-bottom: 0.5em !important;
    }

    main {
        text-align: center;
        padding: 1em;
        margin: 0 auto;
        margin-left: var(--sidebar-width);
    }

    a {
        color: var(--accent-color-secondary);
    }

    .sidebar {
        height: 100%;
        width: var(--sidebar-width);
        position: fixed;
        left: 0;
        top: 0;
        background-color: var(--background-color-darker);
    }

    .sidebar button {
        width: 100%;
        height: 3em;
        border: none;
        background-color: var(--background-color-darker);
    }
</style>
