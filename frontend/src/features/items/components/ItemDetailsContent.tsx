import { Skeleton } from "@/components/ui/skeleton";
import { ProgressSection } from "@/features/progress";
import { useItem } from "../hooks/useItems";
import { parseItemMetadata } from "../lib/parseMetadata";
import { AlbumMetadataView } from "./metadata/AlbumMetadataView";
import { ArtistMetadataView } from "./metadata/ArtistMetadataView";
import { GameMetadataView } from "./metadata/GameMetadataView";
import { MovieMetadataView } from "./metadata/MovieMetadataView";
import { ShowMetadataView } from "./metadata/ShowMetadataView";

export function ItemDetailsContent({ itemId }: { itemId: string }) {
	const { data: item, isLoading, isError } = useItem(itemId);

	if (isLoading) {
		return (
			<div className="flex gap-6">
				<Skeleton className="w-40 h-56 rounded-lg shrink-0" />
				<div className="flex flex-col gap-3 flex-1">
					<Skeleton className="h-6 w-2/3" />
					<Skeleton className="h-4 w-1/3" />
					<Skeleton className="h-20 w-full" />
				</div>
			</div>
		);
	}

	if (isError || !item) {
		return <p className="text-sm text-destructive">Couldn't load this item.</p>;
	}

	const metadata = parseItemMetadata(item);

	function renderMetadata() {
		if (!metadata)
			return <h2 className="text-xl font-semibold">{item.title}</h2>;
		switch (metadata.mediaType) {
			case "Album":
				return <AlbumMetadataView data={metadata.data} />;
			case "Artist":
				return <ArtistMetadataView data={metadata.data} />;
			case "Game":
				return <GameMetadataView data={metadata.data} />;
			case "Movie":
				return <MovieMetadataView data={metadata.data} />;
			case "Show":
				return <ShowMetadataView data={metadata.data} />;
		}
	}

	return (
		<div>
			{renderMetadata()}
			<ProgressSection itemId={itemId} />
		</div>
	);
}
