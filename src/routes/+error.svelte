<script>
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>Lynix - Error</title>
</svelte:head>

<div class="text-center p-5 h-screen flex justify-center items-center">
	<div>
		<div class="flex justify-center">
			<img src="/images/lynix-cute.webp" class="w-[200px] mb-5" alt="Lynix Cute" />
		</div>
		<h1 class="text-4xl font-cyber text-cyan-600 font-bold">
			Error [<span class="text-green-600">{$page.status}</span>]
		</h1>
		{#if $page.status === 404}
			<p class="text-md mt-2 font-cyber text-gray-300">
				The page you requested was not found or is currently unavailable.
			</p>
		{:else}
			<p class="text-md mt-2 font-cyber text-gray-300">
				{$page.error?.message}
			</p>
		{/if}
		<div class="mt-5">
			<a href="/" class="text-cyan-600 hover:text-cyan-500 font-cyber text-md">
				Back to Home
			</a>
			</div>
	</div>
</div>
