// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

if (!String.prototype.capitalize) {
	String.prototype.capitalize = function (this: string): string {
		if (!this) return '';
		return this.charAt(0).toUpperCase() + this.slice(1);
	};
}
