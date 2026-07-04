import { Badge } from "@/components/ui/badge";
import { tmdbImage } from "../../lib/mediaImages";
import type { MovieMetadata } from "../../types/metadata";

export function MovieMetadataView({ data }: { data: MovieMetadata }) {
	const poster = tmdbImage(data.poster_path);

	return (
		<div className="flex gap-6">
			{poster && (
				<img
					src={poster}
					alt={data.title}
					className="w-40 h-60 rounded-lg object-cover bg-muted shrink-0"
				/>
			)}
			<div className="flex flex-col gap-2 min-w-0">
				<h2 className="text-xl font-semibold">{data.title}</h2>
				<div className="flex flex-wrap gap-2">
					{data.release_date && (
						<Badge variant="outline">{data.release_date}</Badge>
					)}
					{data.vote_average != null && (
						<Badge variant="secondary">★ {data.vote_average.toFixed(1)}</Badge>
					)}
					{data.original_language && (
						<Badge variant="outline">
							{data.original_language.toUpperCase()}
						</Badge>
					)}
				</div>
				{data.overview && (
					<p className="text-sm leading-relaxed">{data.overview}</p>
				)}
			</div>
		</div>
	);
}
