<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';
	import { DEV } from '$lib';
	import '$lib/init';

	import NavBar from '$lib/components/NavBar.svelte';

	onMount(() => {
		if (DEV) {
			import('eruda').then((eruda) => eruda.default.init());
		}
	});

	let { children } = $props();
</script>

<NavBar />

<main class="mx-auto w-full max-w-7xl flex-1 px-4 py-6 sm:px-6">
	{@render children()}
</main>
