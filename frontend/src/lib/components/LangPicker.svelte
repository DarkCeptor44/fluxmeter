<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { i18n, type Language } from '$lib/i18n/index.svelte';

	let isOpen = $state(false);

	const languages = i18n.availableLanguages.sort((a, b) => a.label.localeCompare(b.label));

	function selectLanguage(code: Language) {
		i18n.setLanguage(code);
		isOpen = false;
	}

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function handleBlur(event: FocusEvent) {
		const currentTarget = event.currentTarget as HTMLElement;
		if (!currentTarget.contains(event.relatedTarget as Node)) {
			isOpen = false;
		}
	}
</script>

<div class="relative inline-block text-left" onfocusout={handleBlur}>
	<button
		type="button"
		onclick={toggleDropdown}
		class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-lg border border-transparent px-2.5 text-xs font-medium text-zinc-400 transition-colors duration-150 select-none hover:bg-zinc-800/60 hover:text-zinc-100 focus-visible:ring-2 focus-visible:ring-emerald-400 focus-visible:outline-none active:scale-95 {isOpen
			? 'border-zinc-700/50 bg-zinc-800/80 text-zinc-100'
			: ''}"
		aria-expanded={isOpen}
		aria-haspopup="true"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke="currentColor"
			class="h-4 w-4"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Zm0 0c2.5 0 4.5-4.03 4.5-9S14.5 3 12 3m0 18c-2.5 0-4.5-4.03-4.5-9S9.5 3 12 3m-9 9h18"
			/>
		</svg>

		<span class="tracking-wider uppercase">
			{i18n.currentLanguage}
		</span>

		<svg
			class="h-3.5 w-3.5 transition-transform duration-150 {isOpen ? 'rotate-180' : ''}"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 20 20"
			fill="currentColor"
		>
			<path
				fill-rule="evenodd"
				d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
				clip-rule="evenodd"
			/>
		</svg>
	</button>

	{#if isOpen}
		<div
			class="absolute right-0 z-50 mt-2 w-36 origin-top-right rounded-lg border border-zinc-800 bg-zinc-950 py-1 shadow-xl select-none focus-visible:outline-none sm:bg-zinc-950/90 sm:backdrop-blur-md"
			role="menu"
		>
			{#each languages as lang}
				{@const isActive = i18n.currentLanguage === lang.code}

				<button
					type="button"
					onclick={() => selectLanguage(lang.code)}
					class="flex w-full cursor-pointer items-center justify-between px-3 py-1.5 text-left text-xs transition-colors duration-150 hover:bg-zinc-800/60 hover:text-zinc-100 {isActive
						? 'font-medium text-emerald-400'
						: 'text-zinc-300'}"
					role="menuitem"
				>
					<span>{lang.label}</span>
					{#if isActive}
						<span class="h-1.5 w-1.5 rounded-full bg-emerald-400"></span>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>
