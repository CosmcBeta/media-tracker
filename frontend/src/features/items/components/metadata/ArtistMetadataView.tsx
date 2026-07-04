import { Badge } from "@/components/ui/badge";
import type { ArtistMetadata } from "../../types/metadata";

export function ArtistMetadataView({ data }: { data: ArtistMetadata }) {
	const span = data["life-span"];

	return (
		<div className="flex flex-col gap-2">
			<h2 className="text-xl font-semibold">{data.name}</h2>
			<div className="flex flex-wrap gap-2">
				{data.type && <Badge variant="secondary">{data.type}</Badge>}
				{data.country && <Badge variant="outline">{data.country}</Badge>}
				{span?.begin && (
					<Badge variant="outline">
						{span.begin}
						{span.end ? ` – ${span.end}` : span.ended ? "" : " – present"}
					</Badge>
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
	);
}
