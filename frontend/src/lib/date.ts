export function formatDate(date: string) {
	return new Intl.DateTimeFormat("en-US", {
		dateStyle: "medium",
	}).format(new Date(date));
}

export function formatDateTime(date: string) {
	return new Intl.DateTimeFormat("en-US", {
		dateStyle: "medium",
		timeStyle: "short",
	}).format(new Date(date));
}
