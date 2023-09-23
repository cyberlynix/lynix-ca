<script lang="ts">
	import SvelteMarkdown from 'svelte-markdown';
	import type Post from '$lib/Post';

	// we get it from +page.server.ts
	export let data: Post;

	// we want something like "monthname day, year"
	let dateObj = new Date(data.date);
	let date = dateObj.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
</script>

<svelte:head>
	<title>Lynix Blog -</title>
</svelte:head>

<div class="min-h-screen p-5 text-center flex justify-center w-full">
	<div class="prose dark:prose-invert">
		<a class="text-zinc-400 w-full text-left flex pb-5 pt-8" href="/">&larr; Back to home</a>
		<h1 class="text-cyan-600">{data.title}</h1>
		<h2 class="text-zinc-400">Written by <span class="text-white">{data.author}</span> on <span class="text-white">{date}</span></h2>
		<div class="text-justify"><SvelteMarkdown source={data.content} /></div>
	</div>
</div>
