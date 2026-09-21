<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { onDestroy } from 'svelte';

	let ping = $state(0);
	let downloadMbps = $state(0);
	let uploadMbps = $state(0);
	let isTesting = $state(false);

	let worker: Worker | null = null;

	async function runPing() {
		const times: number[] = [];
		for (let i = 0; i < 5; i++) {
			const start = performance.now();
			await fetch('/api/v1/ping', { cache: 'no-store' });
			times.push(performance.now() - start);
		}
		ping = Math.round(times.reduce((a, b) => a + b, 0) / times.length);
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
		const targetBytes = 50_000_000; // 50 MB
		await runWorkerTest('start_download', `/api/v1/download?bytes=${targetBytes}`);
	}

	async function runUpload() {
		await runWorkerTest('start_upload', '/api/v1/upload');
	}

	async function startTest() {
		isTesting = true;
		downloadMbps = 0;
		uploadMbps = 0;

		try {
			await runPing();
			await runDownload();
			await runUpload();
		} catch (err) {
			console.error('Speed test failed:', err);
		} finally {
			isTesting = false;
		}
	}

	onDestroy(() => {
		worker?.terminate();
	});
</script>

<svelte:head>
	<title>Fluxmeter</title>
</svelte:head>

<div class="flex flex-col items-center gap-3">
	<button
		onclick={startTest}
		disabled={isTesting}
		class="active:bg-blue-450 cursor-pointer rounded-lg bg-blue-500 px-3 py-2 transition-colors duration-150 hover:bg-blue-400"
	>
		{isTesting ? 'Testing...' : 'Start Test'}
	</button>

	<div>
		<p>Ping: {ping} ms</p>
		<p>Download: {downloadMbps.toFixed(0)} Mbps</p>
		<p>Upload: {uploadMbps.toFixed(0)} Mbps</p>
	</div>
</div>
