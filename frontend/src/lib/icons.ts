import {
	Book,
	Film,
	Gamepad2,
	List,
	type LucideIcon,
	Music,
} from "lucide-react";

export const ICONS = {
	list: {
		label: "List",
		icon: List,
	},
	movie: {
		label: "Movie",
		icon: Film,
	},
	book: {
		label: "Book",
		icon: Book,
	},
	game: {
		label: "Game",
		icon: Gamepad2,
	},
	music: {
		label: "Music",
		icon: Music,
	},
} satisfies Record<
	string,
	{
		label: string;
		icon: LucideIcon;
	}
>;

export type IconName = keyof typeof ICONS;
