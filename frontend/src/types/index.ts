export type MediaType =
	| "movie"
	| "show"
	| "album"
	| "artist"
	| "book"
	| "game"
	| "podcast";

export type ProgressKind = "episode" | "page" | "percentage" | "complete";

export interface Item {
	id: string;
	media_type: MediaType;
	title: string;
	external_id: string | null;
	metadata: Record<string, unknown> | null;
	created_at: string;
	updated_at: string;
}

export interface List {
	id: string;
	name: string;
	icon: string | null;
	created_at: string;
	updated_at: string;
}

export interface Progress {
	id: string;
	item_id: string;
	kind: ProgressKind;
	value: string | null;
	note: string | null;
	logged_at: string;
}

export interface SearchCandidate {
	external_id: string;
	title: string;
	media_type: MediaType;
	year: string | null;
	description: string | null;
	poster_url: string | null;
	metadata: string;
}

export interface CreateItem {
	media_type: MediaType;
	title: string;
}

export interface CreateList {
	name: string;
	icon?: string;
}

export interface CreateProgress {
	kind: ProgressKind;
	value?: string;
	note?: string;
	logged_at?: string;
}

export interface UpdateItem {
	media_type?: MediaType;
	title?: string;
	external_id?: string;
	metadata?: Record<string, unknown>;
}

export interface UpdateList {
	name?: string;
	icon?: string;
}

export interface AddItemToList {
	item_id: string;
}

export interface SearchParams {
	q: string;
	media_type: MediaType;
}
