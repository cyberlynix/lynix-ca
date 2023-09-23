<script lang="ts">
    import { onMount } from 'svelte';

    import '../app.css';
    import Navbar from "$lib/components/Navbar.svelte";
    import Alert from "$lib/components/Alert.svelte";

    let currentPath = "";

    function isStreamURL(url: string) {
        return url.startsWith('/stream/');
    }

    function updateCurrentPath() {
        currentPath = window.location.pathname;
    }

    onMount(() => {
        updateCurrentPath(); // Initialize currentPath on component mount
        window.addEventListener('popstate', updateCurrentPath);

        document.documentElement.setAttribute('data-theme', 'dark');
    });
</script>

{#if isStreamURL(currentPath)}
<div>
    <slot></slot>
</div>
{:else}
<slot></slot>
{/if}
