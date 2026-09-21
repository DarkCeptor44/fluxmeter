<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import type { TestStage } from '$lib/types';
	import { onDestroy } from 'svelte';
	import { median, sleep } from '$lib/utils';

	let ping = $state(0);
	let downloadMbps = $state(0);
	let uploadMbps = $state(0);
	let stage = $state<TestStage>('idle');

	let worker: Worker | null = null;
	let gaugeProgress = $derived.by(() => {
		let currentSpeed = 0;
		if (stage === 'download') currentSpeed = downloadMbps;
		else if (stage === 'upload') currentSpeed = uploadMbps;
		else if (stage === 'complete') currentSpeed = Math.max(downloadMbps, uploadMbps);

		if (currentSpeed <= 0) return 0;

		const logSpeed = Math.log10(currentSpeed + 1);
		const maxLog = Math.log10(1001);
		return Math.min(100, Math.round((logSpeed / maxLog) * 100));
	});

	const radius = 120;
	const circumference = 2 * Math.PI * radius;
	let strokeDashoffset = $derived(circumference - (gaugeProgress / 100) * (circumference * 0.75));

	async function runPing() {
		stage = 'ping';

		const times: number[] = [];
		for (let i = 0; i < 5; i++) {
			const start = performance.now();
			await fetch('/api/v1/ping', { cache: 'no-store' });

			times.push(performance.now() - start);
		}

		ping = median(times);
	}

	function runWorkerTest(type: 'start_download' | 'start_upload', url: string): Promise<void> {
		return new Promise((resolve, reject) => {
			worker = new Worker(new URL('$lib/speedtest.worker.ts', import.meta.url), {
				type: 'module'
			});

			worker.onmessage = (event) => {
				const { type: msgType, downloadMbps: dSpeed, uploadMbps: uSpeed, message } = event.data;

				if (msgType === 'progress') {
					downloadMbps = dSpeed;
				} else if (msgType === 'progress_upload') {
					uploadMbps = uSpeed;
				} else if (msgType === 'complete' || msgType === 'complete_upload') {
					worker?.terminate();
					worker = null;
					resolve();
				} else if (msgType === 'error') {
					worker?.terminate();
					worker = null;
					reject(new Error(message));
				}
			};

			worker.postMessage({ type, url });
		});
	}

	async function runDownload() {
		stage = 'download';
		const targetBytes = 50_000_000; // 50 MB
		await runWorkerTest('start_download', `/api/v1/download?bytes=${targetBytes}`);
	}

	async function runUpload() {
		stage = 'upload';
		await runWorkerTest('start_upload', '/api/v1/upload');
	}

	async function startTest() {
		downloadMbps = 0;
		uploadMbps = 0;

		try {
			await runPing();
			await sleep(1000);

			await runDownload();
			await sleep(1000);

			await runUpload();
			await sleep(500);

			stage = 'complete';
		} catch (err) {
			console.error('Speed test failed:', err);
			stage = 'idle';
		}
	}

	onDestroy(() => {
		worker?.terminate();
	});
</script>

<svelte:head>
	<title>Fluxmeter</title>
</svelte:head>

<div class="mx-auto flex max-w-4xl flex-col items-center gap-4 px-4 py-2">
	<div
		class="flex items-center gap-6 rounded-full border border-zinc-800/80 bg-zinc-900/40 px-5 py-1.5 text-xs font-medium tracking-wider text-zinc-400 uppercase backdrop-blur-md"
	>
		<span class={stage === 'ping' ? 'font-bold text-emerald-400' : ''}>1. Ping</span>
		<span class="text-zinc-700">•</span>
		<span class={stage === 'download' ? 'font-bold text-emerald-400' : ''}>2. Download</span>
		<span class="text-zinc-700">•</span>
		<span class={stage === 'upload' ? 'font-bold text-emerald-400' : ''}>3. Upload</span>
	</div>

	<div class="relative flex h-64 w-64 items-center justify-center">
		<svg class="h-full w-full -rotate-225 transform" viewBox="0 0 300 300">
			<circle
				cx="150"
				cy="150"
				r={radius}
				fill="transparent"
				stroke="currentColor"
				stroke-width="12"
				stroke-dasharray={circumference}
				stroke-dashoffset={circumference * 0.25}
				stroke-linecap="round"
				class="text-zinc-800/60"
			/>
			<circle
				cx="150"
				cy="150"
				r={radius}
				fill="transparent"
				stroke="url(#flux-gradient)"
				stroke-width="14"
				stroke-dasharray={circumference}
				stroke-dashoffset={strokeDashoffset}
				stroke-linecap="round"
				class="transition-[stroke-dashoffset] duration-150 ease-out"
			/>
			<defs>
				<linearGradient id="flux-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
					<stop offset="0%" stop-color="#10b981" />
					<stop offset="100%" stop-color="#06b6d4" />
				</linearGradient>
			</defs>
		</svg>

		<div class="absolute flex flex-col items-center justify-center text-center">
			{#if stage === 'idle' || stage === 'complete'}
				<button
					onclick={startTest}
					class="group relative flex h-28 w-28 cursor-pointer items-center justify-center rounded-full bg-linear-to-tr from-emerald-500 to-teal-600 font-bold text-zinc-950 shadow-lg shadow-emerald-500/20 transition-all duration-200 hover:scale-105 active:scale-95"
				>
					<span class="text-lg tracking-wider uppercase">Start</span>
				</button>
			{:else}
				<div class="flex flex-col items-center">
					<span class="font-mono text-5xl font-extrabold tracking-tight text-zinc-100">
						{#if stage === 'download'}
							{downloadMbps.toFixed(0)}
						{:else if stage === 'upload'}
							{uploadMbps.toFixed(0)}
						{:else}
							--
						{/if}
					</span>
					<span class="mt-1 text-xs font-semibold tracking-wider text-emerald-400 uppercase">
						Mbps
					</span>
					<span class="mt-1 text-[10px] font-medium text-zinc-500 uppercase">
						{stage}ing...
					</span>
				</div>
			{/if}
		</div>
	</div>

	<div class="grid w-full grid-cols-3 gap-3">
		<div
			class="flex flex-col rounded-xl border border-zinc-800/60 bg-zinc-900/30 p-4 backdrop-blur-md"
		>
			<span class="text-xs font-semibold tracking-wider text-zinc-400 uppercase">Ping</span>
			<div class="mt-1 flex items-baseline gap-1">
				<span class="font-mono text-2xl font-bold text-zinc-100">{ping.toFixed(2)}</span>
				<span class="text-xs text-zinc-500">ms</span>
			</div>
		</div>

		<div
			class="flex flex-col rounded-xl border border-zinc-800/60 bg-zinc-900/30 p-4 backdrop-blur-md"
		>
			<span class="text-xs font-semibold tracking-wider text-zinc-400 uppercase">Download</span>
			<div class="mt-1 flex items-baseline gap-1">
				<span class="font-mono text-2xl font-bold text-zinc-100">{downloadMbps.toFixed(0)}</span>
				<span class="text-xs text-zinc-500">Mbps</span>
			</div>
		</div>

		<div
			class="flex flex-col rounded-xl border border-zinc-800/60 bg-zinc-900/30 p-4 backdrop-blur-md"
		>
			<span class="text-xs font-semibold tracking-wider text-zinc-400 uppercase">Upload</span>
			<div class="mt-1 flex items-baseline gap-1">
				<span class="font-mono text-2xl font-bold text-zinc-100">{uploadMbps.toFixed(0)}</span>
				<span class="text-xs text-zinc-500">Mbps</span>
			</div>
		</div>
	</div>
</div>
