<script lang="ts">
    export let msg: string = null
</script>

<section class="modal">
    <h2>{msg || 'Processing...'}</h2>
    <div class="progress-bar">
        <div class="progress-bar-value" />
    </div>
</section>

<style>
    .progress-bar {
        height: 2px;
        background-color: var(--accent-color-bg);
        width: 100%;
        overflow: hidden;
    }

    .progress-bar-value {
        width: 100%;
        height: 100%;
        background-color: var(--accent-color);
        animation: indeterminateAnimation 2s infinite linear;
        transform-origin: 0% 50%;
    }

    @keyframes indeterminateAnimation {
        0% {
            transform: translateX(0) scaleX(0);
        }
        40% {
            transform: translateX(0) scaleX(0.4);
        }
        100% {
            transform: translateX(100%) scaleX(0.5);
        }
    }
</style>
