<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { page } from '$app/state';

	function navClass(path: string) {
		const isActive = page.url.pathname === path;
		const base =
			'flex h-9 w-9 items-center justify-center rounded-lg border border-transparent text-zinc-400 transition-colors duration-150 hover:bg-zinc-800/60 hover:text-zinc-100 active:scale-95';
		const active = 'bg-zinc-800/80 text-emerald-400 border-zinc-700/50';

		return isActive ? `${base} ${active}` : base;
	}
</script>

<nav class="sticky top-0 z-50 border-b border-zinc-800/50 bg-[#0B0C10]/80 backdrop-blur-md">
	<div class="mx-auto flex max-w-7xl items-center justify-between px-4 py-2.5 sm:px-6">
		<a href="/" class="group flex items-center gap-3">
			<img
				src="/favicon.svg"
				alt="Fluxmeter Logo"
				class="h-8 w-8 rounded-lg transition-transform duration-150 select-none group-hover:scale-105 group-active:scale-97"
			/>
			<span
				class="hidden text-lg font-semibold tracking-tight text-zinc-200 transition-colors duration-150 group-hover:text-white sm:block"
			>
				Fluxmeter
			</span>
		</a>

		<a href="/settings" class={navClass('/settings')} aria-label="Settings">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="h-5 w-5"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l.546.947a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-.546.948a1.125 1.125 0 0 1-1.37.491l-1.216-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.37-.49l-.546-.947a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l.546-.947a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.281Z"
				/>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
				/>
			</svg>
		</a>
	</div>
</nav>
