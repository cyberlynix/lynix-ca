<script lang="ts">
	import FurImage from "./FurImage.svelte";
</script>

<div class="w-full text-gray-700 bg-white dark:text-gray-200 dark:bg-dark md:h-20 min-h-20 items-center">
    <div class="flex flex-col max-w-screen-xl h-full px-4 mx-auto md:items-center md:justify-between md:flex-row md:px-6 lg:px-8">
        <div class="p-4 flex flex-row items-center justify-between">
            <a href="/" style="text-shadow: 5px 5px #075985;" class="flex items-center text-3xl font-semibold font-cyber tracking-widest text-cyan-400 uppercase rounded-lg focus:outline-none focus:shadow-outline">
                <FurImage src="/images/Lynix_Mulli.png" alt="Lynix" class="h-12 w-12 mr-3 rounded-full" loading="lazy"/>
                Lynix
            </a>
            <button class="md:hidden rounded-lg focus:outline-none focus:shadow-outline" aria-label="Toggle Nav">
                <svg fill="currentColor" viewBox="0 0 20 20" class="w-6 h-6" id="toggle-icon">
                    <!-- SVG paths for collapsed and expanded states -->
                </svg>
            </button>
        </div>
        <nav id="nav" class="hidden flex-col flex-grow pb-4 md:pb-0 md:flex md:justify-end md:flex-row h-full">
            <a class="nav-link" href="/blog">
                Information
            </a>
            <a class="nav-link" href="/blog">
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
            <div class="py-4">
                <button class="auth-btn-2 h-full flex items-center bg-cyan-600 font-semibold ml-5">
                    <span>Login</span>
                </button>
            </div>
            <!-- Check if the user is authenticated 
            <div id="auth-dropdown" class="relative hidden">
                <button class="auth-btn h-full flex items-center">
                    <img src="/logo.png" alt="Lynix" class="h-6 w-6 mr-2 rounded-full">
                    <span>
                        {"{"}{"{"} $username {"}"}{"}"}
                    </span>
                    <svg fill="currentColor" viewBox="0 0 20 20" class="w-4 h-4 transition-transform duration-200 transform ml-1">
                        <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                    </svg>
                </button>
                <div class="absolute right-0 w-full mt-0 origin-top-right shadow-lg md:w-48 z-30" id="dropdown-menu">
                    <div class="bg-white shadow dark:bg-dark">
                        <a class="dropdown-link" href="#test">
                            Account Settings
                        </a>
                        <a class="dropdown-link" href="#test1">
                            API Keys
                        </a>
                        <a class="dropdown-link" href="#test2">
                            Logout
                        </a>
                    </div>
                </div>
            </div>-->
        </nav>
    </div>
</div>