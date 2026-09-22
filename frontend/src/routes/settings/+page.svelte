<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { toast } from '$lib/toast.svelte';
	import { DEV } from '$lib';
	import { t } from '$lib/i18n/index.svelte';

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

			toast.show(t('settings.saved'));
		}
	}
</script>

<svelte:head>
	<title>Fluxmeter - Settings</title>
</svelte:head>

<div class="mx-auto max-w-2xl px-4 py-6">
	<div class="rounded-2xl border border-zinc-800/80 bg-zinc-900/40 p-5 sm:p-6">
		<div class="mb-6 border-b border-zinc-800/60 pb-4">
			<h2 class="text-lg font-semibold tracking-tight text-zinc-100">{t('settings.title')}</h2>
			<p class="mt-1 text-xs text-zinc-400">{t('settings.description')}</p>
		</div>

		<form onsubmit={handleSave} class="flex flex-col gap-5">
			<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
				<div>
					<label for="download-size" class="text-sm font-medium text-zinc-200">
						{t('settings.download')}
					</label>
					<p class="text-xs text-zinc-500">{t('settings.downloadDescription')}</p>
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

			<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
				<div>
					<label for="upload-size" class="text-sm font-medium text-zinc-200">
						{t('settings.upload')}
					</label>
					<p class="text-xs text-zinc-500">{t('settings.uploadDescription')}</p>
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
				<Button type="submit" disabled={submitting}>{t('settings.save')}</Button>
			</div>
		</form>
	</div>
</div>
