if (!String.prototype.capitalize) {
	String.prototype.capitalize = function (this: string): string {
		if (!this) return '';
		return this.charAt(0).toUpperCase() + this.slice(1);
	};
}
