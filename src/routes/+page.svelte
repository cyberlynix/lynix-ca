<script lang="ts">
	import FurImage from "$lib/components/FurImage.svelte";
</script>

<svelte:head>
	<title>Lynix</title>
</svelte:head>

<div
    class="bg-emerald-600 font-cyber text-2xl text-black py-1 px-3"
>
    Welcome to lynix.ca my personal website! Thanks for scanning my badge!
</div>
<div class="relative h-full">
    <div class="flex items-center h-full justify-center bg-black bg-opacity-40 absolute z-20 w-full">
        <div class="max-w-4xl mx-auto text-center">
            <div class="flex justify-center">
                <FurImage src="/images/lynix-mulli-256.webp" alt="Lynix" class="h-32 w-32 mr-5 ml-5 mb-5 rounded-full float-animation" loading="lazy"/>
            </div>
            <h1 class="mb-4 text-3xl text-cyan-500 md:text-5xl lg:text-6xl" style="text-shadow: 5px 5px #075985;">
                Hello, {" "}
                <span class="px-3">I'm <span class="">Lynix</span>!</span>
            </h1>
            <p class="text-2xl text-gray-500 dark:text-zinc-300">
                <span class="wave">👋</span> Greetings! I'm your cybernetic lynx enthusiast from the Canadian Maritimes, deeply immersed in the world of Cybersecurity. Delighted to have you join me on my website!
            </p>
        </div>
    </div>
    <FurImage src="/images/pre-resonite.webp" alt="Background image" class="h-[700px] w-full object-cover" loading="lazy"/>
</div>
<!-- Container -->
<div class="flex-col items-center py-20 hidden"> <!-- REMOVED THIS SECTION -->
    <!-- Content-->
    <section class="p-6 md:w-1/2 w-full">
        <FurImage src="/images/lynix-mulli-256.webp" alt="Lynix" class="float-right h-32 w-32 rotate-6 mr-5 ml-5 mb-5 rounded-2xl float-animation" loading="lazy"/>
        <h1 class="text-6xl font-cyber">
            👋 Hello <span class="text-cyan-600">There</span>!
        </h1>
        <p class="text-gray-300 text-lg font-cyber mt-3">
            As a Cybersecurity student, I'm actively seeking opportunities to
            enhance my knowledge and gain practical experience in the field. In
            addition to my studies, I have a passion for artistic expression and
            enjoy drawing in my free time. Playing video games is another interest
            of mine, which allows me to unwind and explore different virtual worlds.
            I'm proudly based in Canada, as indicated by my domain name being a .ca.
        </p>
    </section>
    <section class="p-6 md:w-1/2 w-full">
        <!--<CyberImage
            alt="Background"
            src="/bg.png"
            containerclass="w-[300px] h-[200px] float-left -rotate-6 mb-4 mr-6"
            imageclass="rounded-2xl"
        />-->
        <h1 class="text-6xl font-cyber">
            <span class="text-cyan-600">About</span> Me
        </h1>
        <p class="text-gray-300 text-lg font-cyber mt-3">
            I began my journey in IT back in 2013, where I started by learning basic
            HTML and C++ from university books. Since then, I've relied on resources
            like Google and YouTube to expand my knowledge and learn new programming
            languages. In 2022, I transitioned into the fields of System
            Administration and Cybersecurity, which have been incredibly enriching
            experiences. I've been constantly learning and I'm eager to continue
            growing in these areas. Currently, I'm in my second year of studying
            Cybersecurity in college. Aside from my IT pursuits, I also find
            enjoyment in VR gaming, and I proudly identify as a full-time furry.
        </p>
    </section>
    <section class="p-6 md:w-1/2 w-full">
        <h1 class="text-6xl font-cyber">
            Why <span class="text-cyan-600">Cybersecurity</span>?
        </h1>
        <p class="text-gray-300 text-lg font-cyber  mt-3">
            In a constantly advancing technological world, ensuring the safety and
            security of our data has become increasingly crucial. I am passionate
            about contributing to the efforts that protect our valuable information.
            Each day, we come across news about cybersecurity exploits being
            discovered, and I aspire to be part of the community that proactively
            prevents such incidents. This is particularly critical for businesses,
            organizations, and individuals who rely on robust mitigation processes
            to safeguard their systems.
        </p>
        <br />
        <br />
        <p class="text-gray-300 text-lg font-cyber">
            For instance, as the owner of lynix.ca, I understand the importance of
            regularly updating and patching my website to fend off daily attacks.
        </p>
    </section>
</div>