<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	interface Props extends HTMLInputAttributes {
		id: string;
		type?: 'text' | 'number' | 'date' | 'time' | 'email' | 'password' | 'url';
		placeholder?: string;
		value: any;
		prefix?: string;
		transform?: (value: string) => string;
		element?: HTMLInputElement | null;
	}

	let {
		id,
		type = 'text',
		placeholder = '',
		value = $bindable(),
		prefix,
		transform,
		element = $bindable(null),
		class: className = '',
		oninput,
		...restProps
	}: Props = $props();

	function handleInput(e: Event & { currentTarget: HTMLInputElement }) {
		let newValue = e.currentTarget.value;

		if (transform) {
			newValue = transform(newValue);
			e.currentTarget.value = newValue;
		}

		value = newValue;

		if (typeof oninput === 'function') {
			oninput(e);
		}
	}
</script>

<div class="relative flex items-center">
	<input
		{...restProps}
		{id}
		{type}
		{placeholder}
		bind:this={element}
		bind:value
		oninput={handleInput}
		class="w-full rounded-lg border border-zinc-800 bg-zinc-950/60 py-1.5 pr-10 pl-3 font-mono text-sm text-zinc-100 scheme-dark transition-colors duration-150 outline-none focus:border-emerald-500/50 focus:ring-1 focus:ring-emerald-500/50 sm:w-32 {className}"
	/>
	{#if prefix}
		<span class="pointer-events-none absolute right-3 text-xs font-semibold text-zinc-500">
			{prefix}
		</span>
	{/if}
</div>
