<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	interface Props extends HTMLButtonAttributes {
		type?: 'submit' | 'button';
		disabled?: boolean;
		children: Snippet;
	}

	let {
		type = 'submit',
		disabled = false,
		class: className = '',
		children,
		...restProps
	}: Props = $props();
</script>

<button
	{...restProps}
	{disabled}
	{type}
	class="cursor-pointer rounded-lg bg-emerald-500 px-4 py-2 text-xs font-semibold tracking-wider text-zinc-950 uppercase transition-all duration-150 enabled:hover:bg-emerald-400 enabled:active:scale-95 disabled:opacity-50 {className}"
>
	{@render children()}
</button>
