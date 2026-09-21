<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

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

<main class="mx-auto flex w-full max-w-7xl flex-1 flex-col justify-center px-4 py-2 sm:px-6">
	{@render children()}
</main>
