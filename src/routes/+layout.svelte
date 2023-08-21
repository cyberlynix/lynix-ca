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
<Alert link="https://lynix.ca/blog/why-we-switched-to-svelte" linkName="Learn More" message="Welcome to the new Lynix.ca, built in SvelteKit!"/>

<slot></slot>

<footer class="w-full text-gray-700 bg-white dark:text-gray-300 dark:bg-dark py-3 px-5 flex justify-center items-center font-cyber">
    <div class="w-1/2">
        <p>
            © 2023 <span class="text-green-600">Cyberlynix</span>. All Rights
            Reserved.
        </p>
    </div>
</footer>
{/if}
