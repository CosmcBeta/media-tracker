import { ImageIcon } from "lucide-react";
import { MediaTypeBadge } from "@/components/MediaTypeBadge";
import { cn } from "@/lib/utils";
import type { SearchCandidate } from "@/types";

interface Props {
	candidate: SearchCandidate;
	selected: boolean;
	onSelect: () => void;
}

export function SearchResultCard({ candidate, selected, onSelect }: Props) {
	return (
		<button
			type="button"
			onClick={onSelect}
			className={cn(
				"flex items-center gap-3 px-2 py-2 rounded-md cursor-pointer hover:bg-accent/50 w-full text-left",
				selected && "bg-accent",
			)}
		>
			{candidate.poster_url ? (
				<img
					src={candidate.poster_url}
					alt={candidate.title}
					className="w-10 h-14 object-cover rounded shrink-0"
				/>
			) : (
				<div className="w-10 h-14 rounded bg-muted flex items-center justify-center shrink-0">
					<ImageIcon className="w-4 h-4 text-muted-foreground" />
				</div>
			)}
			<div className="flex-1 min-w-0">
				<div className="text-sm font-medium truncate">{candidate.title}</div>
				{candidate.year && (
					<div className="text-xs text-muted-foreground mt-0.5">
						{candidate.year.slice(0, 4)}
					</div>
				)}
			</div>
			<MediaTypeBadge type={candidate.media_type} />
		</button>
	);
}
