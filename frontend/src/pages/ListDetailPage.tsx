import { useState } from "react";
import { useParams } from "react-router";
import { Skeleton } from "@/components/ui/skeleton";
import { ItemGrid, useAddItemToList, useListItems } from "@/features/items";
import { ItemsToolbar, ListHeader, useList } from "@/features/lists";
import { SearchDialog } from "@/features/search";

export default function ListDetailPage() {
	const { id } = useParams();
	if (!id) return null;
	return <ListDetailContent listId={id} />;
}

function ListDetailContent({ listId }: { listId: string }) {
	const [searchOpen, setSearchOpen] = useState(false);
	const { data: list, isPending: isListPending } = useList(listId);
	const {
		data: items,
		isPending: isItemsPending,
		isError,
	} = useListItems(listId);
	const addItem = useAddItemToList(listId, {
		onSuccess: () => setSearchOpen(false),
	});

	if (isListPending || isItemsPending) {
		return (
			<div className="max-w-4xl mx-auto p-6 flex flex-col gap-6">
				<Skeleton className="h-9 w-1/3" />
				<Skeleton className="h-5 w-1/5" />
				<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
					{["a", "b", "c", "d"].map((key) => (
						<Skeleton key={key} className="h-24 w-full" />
					))}
				</div>
			</div>
		);
	}

	if (isError || !list) {
		return (
			<div className="max-w-4xl mx-auto p-6">
				<p className="text-sm text-destructive">Couldn't load this list.</p>
			</div>
		);
	}

	return (
		<div className="p-6 flex flex-col gap-6">
			<ListHeader list={list} />

			<div className="mx-auto w-full flex flex-col gap-6">
				<ItemsToolbar
					itemCount={items?.length ?? 0}
					onAddClick={() => setSearchOpen(true)}
				/>

				<SearchDialog
					open={searchOpen}
					onOpenChange={setSearchOpen}
					onAdd={(candidate) => addItem.mutate(candidate)}
					isPending={addItem.isPending}
				/>

				{items && items.length > 0 ? (
					<ItemGrid items={items} listId={listId} />
				) : (
					<div className="flex flex-col items-center justify-center gap-2 py-16 text-center">
						<p className="text-sm text-muted-foreground">
							No items in this list yet.
						</p>
						<button
							type="button"
							className="text-sm underline underline-offset-2"
							onClick={() => setSearchOpen(true)}
						>
							Add your first item
						</button>
					</div>
				)}
			</div>
		</div>
	);
}
