<script lang="ts">
	import FurImage from "./FurImage.svelte";
    import { fade } from 'svelte/transition';


    let showReferencesDropdown = false;
    let showAboutDropdown = false;

    function toggleReferenceDropdown() {
        showReferencesDropdown = !showReferencesDropdown;
    }

    function toggleAboutDropdown() {
        showAboutDropdown = !showAboutDropdown;
    }
</script>


<!--<div class="w-full text-gray-700 bg-white dark:text-gray-200 dark:bg-dark md:h-20 min-h-20 items-center">
    <div class="flex flex-col max-w-screen-xl h-full px-4 mx-auto md:items-center md:justify-between md:flex-row md:px-6 lg:px-8">
        <div class="p-4 flex flex-row items-center justify-between">
            <a href="/" style="text-shadow: 5px 5px #075985;" class="flex items-center text-3xl font-semibold font-cyber tracking-widest text-cyan-400 uppercase rounded-lg focus:outline-none focus:shadow-outline">
                <FurImage src="/images/lynix-mulli-64.webp" alt="Lynix" class="h-12 w-12 mr-3 rounded-full" loading="lazy"/>
                Lynix
            </a>
            <button class="md:hidden rounded-lg focus:outline-none focus:shadow-outline" aria-label="Toggle Nav">
                <svg fill="currentColor" viewBox="0 0 20 20" class="w-6 h-6" id="toggle-icon">
                </svg>
            </button>
        </div>
        <nav id="nav" class="hidden flex-col flex-grow pb-4 md:pb-0 md:flex md:justify-end md:flex-row h-full">
            <a class="nav-link" href="#information">
                Information
            </a>
            <a class="nav-link" href="#projects">
                Projects
            </a>
            <a class="nav-link" href="/blog">
                Blog
            </a>
            <a class="nav-link" href="/about">
                About
            </a>
            <a class="nav-link" href="/contact">
                Contact
            </a>
        </nav>
    </div>
</div>-->

<div class="w-full flex items-center justify-center py-5">
    <a href="/" style="text-shadow: 5px 5px #075985;" class="flex items-center text-4xl font-semibold font-cyber tracking-widest text-cyan-400 uppercase rounded-lg focus:outline-none focus:shadow-outline">
        <FurImage src="/images/lynix-mulli-64.webp" alt="Lynix" class="h-12 w-12 mr-3 rounded-full" loading="lazy"/>
        Lynix
    </a>
</div>
<div class="flex justify-center items-center mb-5">
    <nav id="nav" class="hidden flex-col pb-4 md:pb-0 md:flex md:justify-end md:flex-row h-full">
        <a class="nav-link" href="#projects">
            Projects
        </a>
        <a class="nav-link" href="#projects">
            Gallery
        </a>
        <!--<a class="nav-link" href="#projects">
            Information
        </a>-->
        <button id="auth-dropdown" class="relative" on:mouseenter={toggleReferenceDropdown} on:mouseleave={toggleReferenceDropdown}>
            <span class="nav-link h-full flex items-center">
                References
            </span>
            {#if showReferencesDropdown}
            <div class="absolute left-0 w-full mt-0 origin-top-right shadow-lg md:w-48 z-30" id="dropdown-menu" transition:fade={{ delay: 0, duration: 200 }}>
                <div class="bg-white shadow dark:bg-dark">
                    <a class="dropdown-link text-left" href="#test">
                        Notes
                    </a>
                    <a class="dropdown-link text-left" href="#test">
                        Writeups
                    </a>
                </div>
            </div>
            {/if}
        </button>
        <a class="nav-link" href="/blog">
            Blog
        </a>
        <!--<a class="nav-link" href="/about">
            About
        </a>-->
        <button id="auth-dropdown" class="relative" on:mouseenter={toggleAboutDropdown} on:mouseleave={toggleAboutDropdown}>
            <span class="nav-link h-full flex items-center">
                About
            </span>
            {#if showAboutDropdown}
            <div class="absolute left-0 w-full mt-0 origin-top-right shadow-lg md:w-48 z-30" id="dropdown-menu" transition:fade={{ delay: 0, duration: 200 }}>
                <div class="bg-white shadow dark:bg-dark">
                    <a class="dropdown-link text-left" href="#test">
                        About Me
                    </a>
                    <a class="dropdown-link text-left" href="#test">
                        Resume
                    </a>
                    <a class="dropdown-link text-left" href="#test">
                        Certifications
                    </a>
                    <a class="dropdown-link text-left" href="#test1">
                        Fursona
                    </a>
                </div>
            </div>
            {/if}
        </button>
        <a class="nav-link" href="/contact">
            Contact
        </a>
    </nav>
</div>