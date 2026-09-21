<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { DEV } from '$lib';

	import InputField from '$lib/components/InputField.svelte';
	import Button from '$lib/components/Button.svelte';

	let downloadSize = $state(50);
	let uploadSize = $state(50);
	let submitting = $state(false);

	$effect(() => {
		if (!browser) return;

		const savedDownload = localStorage.getItem('downloadSize');
		const savedUpload = localStorage.getItem('uploadSize');

		if (savedDownload !== null) downloadSize = Number(savedDownload);
		if (savedUpload !== null) uploadSize = Number(savedUpload);
	});

	function handleSave(e: SubmitEvent) {
		e.preventDefault();
		if (submitting) return;
		if (downloadSize <= 0 || uploadSize <= 0) {
			return;
		}

		if (browser) {
			localStorage.setItem('downloadSize', downloadSize.toString());
			localStorage.setItem('uploadSize', uploadSize.toString());

			if (DEV) {
				console.log('updated settings:', { downloadSize, uploadSize });
			}
		}
	}
</script>

<svelte:head>
	<title>Fluxmeter - Settings</title>
</svelte:head>

<div class="mx-auto max-w-2xl px-4 py-6">
	<div class="rounded-2xl border border-zinc-800/80 bg-zinc-900/40 p-5 backdrop-blur-md sm:p-6">
		<div class="mb-6 border-b border-zinc-800/60 pb-4">
			<h2 class="text-lg font-semibold tracking-tight text-zinc-100">Test Configuration</h2>
			<p class="mt-1 text-xs text-zinc-400">Adjust payload sizes used for bandwidth measurement.</p>
		</div>

		<form onsubmit={handleSave} class="flex flex-col gap-5">
			<div class="flex flex-col gap-1.5 sm:flex-row sm:items-center sm:justify-between">
				<div>
					<label for="downloadSize" class="text-sm font-medium text-zinc-200">
						Download Payload Size
					</label>
					<p class="text-xs text-zinc-500">Amount of data transferred during the download phase.</p>
				</div>
				<InputField
					id="download-size"
					type="number"
					bind:value={downloadSize}
					min="5"
					max="500"
					step="5"
					prefix="MB"
				/>
			</div>

			<div class="flex flex-col gap-1.5 sm:flex-row sm:items-center sm:justify-between">
				<div>
					<label for="uploadSize" class="text-sm font-medium text-zinc-200">
						Upload Payload Size
					</label>
					<p class="text-xs text-zinc-500">Amount of data transferred during the upload phase.</p>
				</div>
				<InputField
					id="upload-size"
					type="number"
					bind:value={uploadSize}
					min="5"
					max="500"
					step="5"
					prefix="MB"
				/>
			</div>

			<div class="mt-4 flex justify-end border-t border-zinc-800/60 pt-4">
				<Button type="submit" disabled={submitting}>Save Settings</Button>
			</div>
		</form>
	</div>
</div>
