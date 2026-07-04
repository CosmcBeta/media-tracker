import { cn } from "@/lib/utils";
import type { MediaType } from "@/types";

const styles: Record<MediaType, string> = {
	movie: "bg-purple-100 text-purple-800",
	show: "bg-blue-100 text-blue-800",
	album: "bg-pink-100 text-pink-800",
	artist: "bg-pink-100 text-pink-800",
	book: "bg-amber-100 text-amber-800",
	game: "bg-green-100 text-green-800",
	podcast: "bg-orange-100 text-orange-800",
};

interface Props {
	type: MediaType;
	className?: string;
}

export function MediaTypeBadge({ type, className }: Props) {
	return (
		<span
			className={cn(
				"text-xs font-medium px-2 py-0.5 rounded-md shrink-0",
				styles[type],
				className,
			)}
		>
			{type}
		</span>
	);
}
