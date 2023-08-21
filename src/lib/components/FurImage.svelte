<!--
  FurImage is the implementation of a wrapper for svelte-image for lynix.ca
-->

<script lang="ts">
	/** REQUIRED */

	/** @type {Picture} */
	export let src: string;

	export let alt: string = '';

	/** OPTIONAL */

	/** @type {boolean} */
	export let draggable: boolean = false;

	/** @type {'async' | 'sync' | 'auto'} */
	export let decoding: 'async' | 'sync' | 'auto' = 'async';

	/** @type {'lazy' | 'eager'} */
	export let loading: 'lazy' | 'eager' = 'lazy';

	let classes: string = '';
	export { classes as class };

	/** @type {number} */
	//export let width: number | undefined;
</script>

<img
	src={src}
	{alt}
	class={classes}
	{loading}
	{decoding}
	{draggable}
/>
