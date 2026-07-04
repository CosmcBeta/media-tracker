import { Badge } from "@/components/ui/badge";
import { formatSecondsAsHours, igdbCover } from "../../lib/mediaImages";
import type { GameMetadata } from "../../types/metadata";

export function GameMetadataView({ data }: { data: GameMetadata }) {
	const releaseDate = new Date(
		data.first_release_date * 1000,
	).toLocaleDateString();
	const ttb = data.time_to_beat;

	return (
		<div className="flex gap-6">
			<img
				src={igdbCover(data.cover.url)}
				alt={data.name}
				className="w-40 h-52 rounded-lg object-cover bg-muted shrink-0"
			/>
			<div className="flex flex-col gap-2 min-w-0">
				<h2 className="text-xl font-semibold">{data.name}</h2>
				<p className="text-sm text-muted-foreground">{releaseDate}</p>
				{data.summary && (
					<p className="text-sm leading-relaxed">{data.summary}</p>
				)}

				<div className="flex flex-wrap gap-1.5 mt-2">
					{data.genres.map((g) => (
						<Badge key={g.id} variant="secondary" className="text-xs">
							{g.name}
						</Badge>
					))}
				</div>
				<div className="flex flex-wrap gap-1.5">
					{data.platforms.map((p) => (
						<Badge key={p.id} variant="outline" className="text-xs">
							{p.name}
						</Badge>
					))}
				</div>
				{data.game_modes.length > 0 && (
					<p className="text-xs text-muted-foreground mt-1">
						{data.game_modes.map((m) => m.name).join(" · ")}
					</p>
				)}

				{ttb && (
					<div className="mt-3 pt-3 border-t">
						<p className="text-xs font-medium text-muted-foreground mb-1.5">
							Time to beat
						</p>
						<div className="flex gap-4 text-sm">
							<div>
								<span className="text-muted-foreground">Main: </span>
								{formatSecondsAsHours(ttb.hastily)}
							</div>
							<div>
								<span className="text-muted-foreground">Normal: </span>
								{formatSecondsAsHours(ttb.normally)}
							</div>
							<div>
								<span className="text-muted-foreground">Complete: </span>
								{formatSecondsAsHours(ttb.completely)}
							</div>
						</div>
					</div>
				)}
			</div>
		</div>
	);
}
