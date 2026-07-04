export interface MusicBrainzTag {
	count: number;
	name: string;
}

export interface MusicBrainzLifeSpan {
	begin: string | null;
	end: string | null;
	ended: boolean | null;
}

export interface MusicBrainzArtistRef {
	id: string;
	name: string;
	"sort-name": string;
	type: string | null;
	gender: string | null;
	country: string | null;
	"life-span": MusicBrainzLifeSpan | null;
	tags: MusicBrainzTag[];
}

export interface MusicBrainzArtistCredit {
	name: string | null;
	artist: MusicBrainzArtistRef;
}

export interface MusicBrainzRelease {
	id: string;
	title: string;
	status: string | null;
}

export interface AlbumMetadata {
	id: string;
	title: string;
	"first-release-date": string | null;
	"primary-type": string | null;
	"artist-credit": MusicBrainzArtistCredit[];
	releases: MusicBrainzRelease[];
	tags: MusicBrainzTag[];
}

export interface ArtistMetadata {
	id: string;
	name: string;
	"sort-name": string;
	type: string | null;
	gender: string | null;
	country: string | null;
	"life-span": MusicBrainzLifeSpan | null;
	tags: MusicBrainzTag[];
}

export interface IgdbRef {
	id: number;
	name: string;
}

export interface GameMetadata {
	id: number;
	name: string;
	first_release_date: number;
	cover: { id: number; url: string };
	game_modes: IgdbRef[];
	genres: IgdbRef[];
	platforms: IgdbRef[];
	summary: string;
	time_to_beat?: TimeToBeat;
}

export interface TimeToBeat {
	id: number;
	hastily: number;
	normally: number;
	completely: number;
}

export interface MovieMetadata {
	id: number;
	title: string;
	overview: string | null;
	release_date: string | null;
	poster_path: string | null;
	backdrop_path: string | null;
	genre_ids: number[];
	original_language: string | null;
	vote_average: number | null;
	vote_count: number | null;
	popularity: number | null;
}

export interface ShowMetadata {
	id: number;
	name: string;
	overview: string | null;
	first_air_date: string | null;
	poster_path: string | null;
	backdrop_path: string | null;
	genre_ids: number[];
	origin_country: string[];
	original_language: string | null;
	vote_average: number | null;
	vote_count: number | null;
	popularity: number | null;
}

export type ParsedItemMetadata =
	| { mediaType: "Album"; data: AlbumMetadata }
	| { mediaType: "Artist"; data: ArtistMetadata }
	| { mediaType: "Game"; data: GameMetadata }
	| { mediaType: "Movie"; data: MovieMetadata }
	| { mediaType: "Show"; data: ShowMetadata };
