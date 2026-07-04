import { Badge } from "@/components/ui/badge";
import { tmdbImage } from "../../lib/mediaImages";
import type { ShowMetadata } from "../../types/metadata";

export function ShowMetadataView({ data }: { data: ShowMetadata }) {
	const poster = tmdbImage(data.poster_path);

	return (
		<div className="flex gap-6">
			{poster && (
				<img
					src={poster}
					alt={data.name}
					className="w-40 h-60 rounded-lg object-cover bg-muted shrink-0"
				/>
			)}
			<div className="flex flex-col gap-2 min-w-0">
				<h2 className="text-xl font-semibold">{data.name}</h2>
				<div className="flex flex-wrap gap-2">
					{data.first_air_date && (
						<Badge variant="outline">{data.first_air_date}</Badge>
					)}
					{data.vote_average != null && (
						<Badge variant="secondary">★ {data.vote_average.toFixed(1)}</Badge>
					)}
					{data.origin_country.map((c) => (
						<Badge key={c} variant="outline">
							{c}
						</Badge>
					))}
				</div>
				{data.overview && (
					<p className="text-sm leading-relaxed">{data.overview}</p>
				)}
			</div>
		</div>
	);
}
