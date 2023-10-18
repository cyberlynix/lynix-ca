<script lang="ts">
	import FurImage from "$lib/components/FurImage.svelte";
</script>

<svelte:head>
	<title>Lynix</title>
</svelte:head>

<!--<div class="relative h-full">
    <div class="flex items-center h-full justify-center bg-black bg-opacity-40 absolute z-20 w-full">
        <div class="max-w-4xl mx-auto text-center">
            <div class="flex justify-center">
                <FurImage src="/images/lynix-mulli-256.webp" alt="Lynix" class="h-32 w-32 mr-5 ml-5 mb-5 rounded-full float-animation" loading="lazy"/>
            </div>
            <h1 class="mb-4 text-4xl text-cyan-500 md:text-5xl lg:text-6xl" style="text-shadow: 5px 5px #075985;">
                Hello, {" "}
                <span class="px-3">I'm <span class="">Lynix</span>!</span>
            </h1>
            <p class="text-lg px-5 md:px-0 md:text-2xl text-zinc-300">
                <span class="wave">👋</span> Greetings! I'm your cybernetic lynx enthusiast from the Canadian Maritimes, deeply immersed in the world of Cybersecurity. Delighted to have you join me on my website!
            </p>
        </div>
    </div>
    <FurImage src="/images/pre-resonite.webp" alt="Background image" class="h-full w-full object-cover" loading="lazy"/>
</div>-->

<div class="relative h-full" style="background:url(/test-v3.jpg);background-size:cover;background-repeat:none;background-position:center;">
    <div class="flex items-center h-full justify-center bg-black bg-opacity-70 absolute z-20 w-full">
        <div class="max-w-4xl mx-auto text-center">
            <div class="flex justify-center">
                <FurImage src="/images/lynix-mulli-256.webp" alt="Lynix" class="h-32 w-32 mr-5 ml-5 mb-5 rounded-full float-animation" loading="lazy"/>
            </div>
            <h1 class="mb-4 text-4xl text-cyan-500 md:text-5xl lg:text-6xl" style="text-shadow: 5px 5px #075985;">
                <span class="px-3">Lynix, the IT Specialist</span>
            </h1>
            <p class="text-lg px-5 md:px-0 md:text-2xl text-zinc-300">
                <span class="hidden">👋</span>VR enthusiast, Developer, Cybersecurity Student, and Furry, I'm seeking opportunities to change the world for better for everyone with IT.
            </p>
            <div class="flex justify-center space-x-4 mt-5">
                <a href="/projects" class="font-cyber px-4 py-2 bg-cyan-700 text-white hover:bg-cyan-800 transition-colors hover:text-white text-xl">Projects</a>
                <a href="/contact" class="font-cyber px-4 py-2 bg-cyan-700 text-white hover:bg-cyan-800 transition-colors hover:text-white text-xl">Contact Me</a>
            </div>
        </div>
    </div>
</div>