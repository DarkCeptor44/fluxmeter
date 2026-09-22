// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/**
 * Generate a random ID
 * @returns A random ID string
 */
export function generateId(): string {
	if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
		return crypto.randomUUID();
	}
	if (typeof crypto !== 'undefined' && typeof crypto.getRandomValues === 'function') {
		return (([1e7] as unknown as string) + -1e3 + -4e3 + -8e3 + -1e11).replace(/[018]/g, (c) =>
			(
				Number(c) ^
				(crypto.getRandomValues(new Uint8Array(1))[0] & (15 >> (Number(c) / 4)))
			).toString(16)
		);
	}
	return Math.random().toString(36).substring(2, 11);
}

export function median(arr: number[]): number {
	return arr.sort((a, b) => a - b)[Math.floor(arr.length / 2)];
}

export const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
