import { Badge } from "@/components/ui/badge";
import { albumCoverArt } from "../../lib/mediaImages";
import type { AlbumMetadata } from "../../types/metadata";

export function AlbumMetadataView({ data }: { data: AlbumMetadata }) {
	const artists = data["artist-credit"]
		.map((c) => c.name ?? c.artist.name)
		.join(", ");

	return (
		<div className="flex gap-6">
			<img
				src={albumCoverArt(data.id)}
				alt={data.title}
				className="w-40 h-40 rounded-lg object-cover bg-muted shrink-0"
				onError={(e) => (e.currentTarget.style.display = "none")}
			/>
			<div className="flex flex-col gap-2 min-w-0">
				<h2 className="text-xl font-semibold">{data.title}</h2>
				{artists && <p className="text-muted-foreground">{artists}</p>}
				<div className="flex flex-wrap gap-2 mt-1">
					{data["primary-type"] && (
						<Badge variant="secondary">{data["primary-type"]}</Badge>
					)}
					{data["first-release-date"] && (
						<Badge variant="outline">{data["first-release-date"]}</Badge>
					)}
				</div>
				{data.tags.length > 0 && (
					<div className="flex flex-wrap gap-1.5 mt-2">
						{data.tags.map((tag) => (
							<Badge key={tag.name} variant="outline" className="text-xs">
								{tag.name}
							</Badge>
						))}
					</div>
				)}
			</div>
		</div>
	);
}
