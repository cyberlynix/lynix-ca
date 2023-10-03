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
    });
</script>

{#if isStreamURL(currentPath)}
<div>
    <slot></slot>
</div>
{:else}
<Navbar/>
<Alert link="https://resonite.com" linkName="Learn More" message="Resonite is launching October 6th! Get ready with the new Resonlynx API for Resonite add it to your avatar today!"/>

<slot></slot>

<footer class="w-full text-gray-700 bg-white dark:text-gray-300 dark:bg-dark py-3 px-5 flex justify-center items-center font-cyber">
    <div class="w-3/4 flex">
        <p>
            © 2023 <span class="text-cyan-500">Lynix</span>. All Rights
            Reserved.
        </p>
        <div class="flex ml-5">
            <p class="mr-3">Protected by </p>
            <a href="https://path.net/"><img src="/path.png" alt="Path Network" class="h-6"/></a>
        </div>
    </div>
</footer>
{/if}
