import type { Item } from "@/types";
import type {
	AlbumMetadata,
	ArtistMetadata,
	GameMetadata,
	MovieMetadata,
	ParsedItemMetadata,
	ShowMetadata,
} from "../types/metadata";

export function parseItemMetadata(item: Item): ParsedItemMetadata | null {
	if (!item.metadata) return null;

	const parsed: unknown = item.metadata;

	switch (item.media_type) {
		case "album":
			return { mediaType: "Album", data: parsed as AlbumMetadata };
		case "artist":
			return { mediaType: "Artist", data: parsed as ArtistMetadata };
		case "game":
			return { mediaType: "Game", data: parsed as GameMetadata };
		case "movie":
			return { mediaType: "Movie", data: parsed as MovieMetadata };
		case "show":
			return { mediaType: "Show", data: parsed as ShowMetadata };
		default:
			return null;
	}
}
