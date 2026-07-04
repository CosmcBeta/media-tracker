export function tmdbImage(
	path: string | null,
	size: "w342" | "w500" | "original" = "w500",
) {
	return path ? `https://image.tmdb.org/t/p/${size}${path}` : null;
}

export function igdbCover(
	url: string,
	size: "t_cover_big" | "t_720p" = "t_cover_big",
) {
	// IGDB returns protocol-relative urls at t_thumb size, e.g. //images.igdb.com/igdb/image/upload/t_thumb/xyz.jpg
	const upgraded = url.replace("t_thumb", size);
	return upgraded.startsWith("//") ? `https:${upgraded}` : upgraded;
}

export function albumCoverArt(releaseGroupId: string) {
	return `https://coverartarchive.org/release-group/${releaseGroupId}/front`;
}

export function formatSecondsAsHours(seconds: number): string {
	const hours = seconds / 3600;
	return `${hours % 1 === 0 ? hours : hours.toFixed(1)} hr`;
}
