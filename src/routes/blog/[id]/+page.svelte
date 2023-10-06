<script lang="ts">
	import SvelteMarkdown from 'svelte-markdown';
</script>

<svelte:head>
	<title>Lynix Blog -</title>
</svelte:head>

<div class="min-h-screen p-5 text-center flex justify-center w-full">
	<div class="prose dark:prose-invert">
		<a class="text-zinc-400 w-full text-left flex pb-5 pt-8" href="/">&larr; Back to home</a>
		<h1 class="text-cyan-600">title</h1>
		<h2 class="text-zinc-400">Written by <span class="text-white">{''}</span> on <span class="text-white">{''}</span></h2>
		<div class="text-justify"><SvelteMarkdown source={''} /></div>
	</div>
</div>
