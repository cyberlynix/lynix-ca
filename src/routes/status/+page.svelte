<script>
	import FurImage from "$lib/components/FurImage.svelte";
</script>

<svelte:head>
	<title>Lynx Network Status</title>
</svelte:head>

<div class="min-h-screen p-5">
    <div class="mx-[10em] py-[2em]">
        <h1 class="text-4xl text-gray-300">Lynx Network Status</h1>
        <p class="mt-2 text-orange-400">Some services are experiencing issues | Uptime: 99.99%</p>

        <div class="mt-5">
            <h1 class="text-2xl font-bold text-gray-300">Chicago, IL (Primary Host)</h1>
            <div class="space-y-3 my-5 px-5">
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">LynixCA</h1>
                        <span class="bg-green-100 text-green-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-green-900 dark:text-green-300">Operational</span>
                    </div>
                    
                </div>
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">Lynix API</h1>
                        <span class="bg-red-100 text-red-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300">Down</span>
                    </div>
                </div>
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">Lynix WS</h1>
                        <span class="bg-red-100 text-red-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300">Down</span>
                    </div>
                </div>
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">LynxVR Module</h1>
                        <span class="bg-red-100 text-red-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300">Down</span>
                    </div>
                </div>
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">Lynix FC Module</h1>
                        <span class="bg-red-100 text-red-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300">Down</span>
                    </div>
                </div>
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">Database</h1>
                        <span class="bg-red-100 text-red-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300">Down</span>
                    </div>

                </div>
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">Matrix</h1>
                        <span class="bg-yellow-100 text-yellow-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-yellow-900 dark:text-yellow-300">Degraded Performance</span>
                    </div>
                </div>
                <div class="">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-2xl ">Lynx VPN & Tunnel</h1>
                        <span class="bg-red-100 text-red-800 text-sm font-medium mr-2 px-2.5 py-0.5 rounded dark:bg-red-900 dark:text-red-300">Down</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="mt-5">
            <h1 class="text-2xl font-bold text-gray-300">Montreal, QC</h1>
            <p class="text-zinc-300 mt-1">No services are online in this location!</p>
        </div>


        <h1 class="text-4xl text-gray-300 my-10">Downtime History</h1>
        <div class="h-40 w-full border-2 border-red-600">
            <div class="bg-red-600 h-10 py-2 px-5 font-bold text-zinc-300">
                <p>Server Migration in Progress</p>
            </div>
        </div>

    </div>
</div>