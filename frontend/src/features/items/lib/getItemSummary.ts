import type { Item } from "@/types";
import { albumCoverArt, igdbCover, tmdbImage } from "./mediaImages";
import { parseItemMetadata } from "./parseMetadata";

interface ItemSummary {
	posterUrl: string | null;
	subtitle: string | null;
}

export function getItemSummary(item: Item): ItemSummary {
	const metadata = parseItemMetadata(item);

	if (!metadata) {
		return { posterUrl: null, subtitle: null };
	}

	switch (metadata.mediaType) {
		case "Album": {
			const artists = metadata.data["artist-credit"]
				.map((c) => c.name ?? c.artist.name)
				.join(", ");
			return {
				posterUrl: albumCoverArt(metadata.data.id),
				subtitle: [artists, metadata.data["first-release-date"]]
					.filter(Boolean)
					.join(" · "),
			};
		}
		case "Artist": {
			const span = metadata.data["life-span"];
			return {
				posterUrl: null,
				subtitle: [metadata.data.type, span?.begin].filter(Boolean).join(" · "),
			};
		}
		case "Game": {
			const year = new Date(
				metadata.data.first_release_date * 1000,
			).getFullYear();
			return {
				posterUrl: igdbCover(metadata.data.cover.url),
				subtitle: [String(year), metadata.data.platforms[0]?.name]
					.filter(Boolean)
					.join(" · "),
			};
		}
		case "Movie": {
			return {
				posterUrl: tmdbImage(metadata.data.poster_path, "w342"),
				subtitle: [
					metadata.data.release_date?.slice(0, 4),
					metadata.data.vote_average != null
						? `★ ${metadata.data.vote_average.toFixed(1)}`
						: null,
				]
					.filter(Boolean)
					.join(" · "),
			};
		}
		case "Show": {
			return {
				posterUrl: tmdbImage(metadata.data.poster_path, "w342"),
				subtitle: [
					metadata.data.first_air_date?.slice(0, 4),
					metadata.data.vote_average != null
						? `★ ${metadata.data.vote_average.toFixed(1)}`
						: null,
				]
					.filter(Boolean)
					.join(" · "),
			};
		}
	}
}
