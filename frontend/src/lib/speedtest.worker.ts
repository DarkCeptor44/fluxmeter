self.onmessage = async (event: MessageEvent<{ type: string; url: string }>) => {
	if (event.data.type === 'start_download') {
		try {
			const start = performance.now();
			const response = await fetch(event.data.url, { cache: 'no-store' });
			if (!response.body) {
				self.postMessage({ type: 'error', message: 'No response body' });
				return;
			}

			const reader = response.body.getReader();
			let receivedBytes = 0;

			while (true) {
				const result = await reader.read();
				if (result.done) break;

				if (result.value) {
					receivedBytes += result.value.length;
					const durationSeconds = (performance.now() - start) / 1000;
					const downloadMbps = Number(
						((receivedBytes * 8) / (durationSeconds * 1_000_000)).toFixed(2)
					);

					self.postMessage({ type: 'progress', downloadMbps });
				}
			}

			self.postMessage({ type: 'complete' });
		} catch (err) {
			self.postMessage({ type: 'error', message: String(err) });
		}
	} else if (event.data.type === 'start_upload') {
		try {
			const targetBytes = 50_000_000; // 50 MB payload
			const payload = new Uint8Array(targetBytes);
			const start = performance.now();
			const xhr = new XMLHttpRequest();
			xhr.open('POST', event.data.url, true);
			xhr.setRequestHeader('Content-Type', 'application/octet-stream');

			xhr.upload.onprogress = (e) => {
				if (e.lengthComputable || e.loaded) {
					const durationSeconds = (performance.now() - start) / 1000;
					if (durationSeconds > 0) {
						const uploadMbps = Number(((e.loaded * 8) / (durationSeconds * 1_000_000)).toFixed(2));
						self.postMessage({ type: 'progress_upload', uploadMbps });
					}
				}
			};

			xhr.onload = () => {
				if (xhr.status === 200) {
					self.postMessage({ type: 'complete_upload' });
				} else {
					self.postMessage({ type: 'error', message: `Upload failed with status ${xhr.status}` });
				}
			};

			xhr.onerror = () => {
				self.postMessage({ type: 'error', message: 'Upload network error' });
			};

			xhr.send(payload);
		} catch (err) {
			self.postMessage({ type: 'error', message: String(err) });
		}
	}
};
