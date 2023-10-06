<script lang="ts">
    import '../app.css';
    import Navbar from "$lib/components/Navbar.svelte";
    import Alert from "$lib/components/Alert.svelte";
</script>

<Navbar/>
<Alert link="https://twitch.tv/lynixthelynx" linkName="Learn More" message="Resonite is launching October 6th! Watch the extended stream of Resonite Launch now!"/>

<slot></slot>

<footer class="w-full text-gray-700 bg-white dark:text-gray-300 dark:bg-dark py-3 px-5 flex justify-center items-center font-cyber">
    <div class="w-3/4 flex items-center">
        <p>
            © 2023 <span class="text-cyan-500">Lynix</span>. All Rights
            Reserved.
        </p>
        <div class="flex ml-5">
            <a href="https://www.cloudflare.com/"><img src="/cf.png" alt="Cloudflare" class="h-10 rounded"/></a>
        </div>
    </div>
</footer>