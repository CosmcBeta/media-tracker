import { useState } from "react";
import { useParams } from "react-router";
import { ItemGrid, useAddItemToList, useListItems } from "@/features/items";
import { SearchDialog } from "@/features/search";

export default function ListDetailPage() {
	const { id } = useParams();
	console.log("listId:", id);

	if (!id) return null;

	return <ListDetailContent listId={id} />;
}

function ListDetailContent({ listId }: { listId: string }) {
	const [searchOpen, setSearchOpen] = useState(false);

	const { data: items, isPending, isError } = useListItems(listId);
	const addItem = useAddItemToList(listId, {
		onSuccess: () => setSearchOpen(false),
	});

	if (isPending) {
		return <div>Loading...</div>;
	}

	if (isError) {
		return <div>Error</div>;
	}

	return (
		<div>
			{/*list info and actions*/}
			<div></div>

			{/*item toolbar*/}
			<button type="button" onClick={() => setSearchOpen(true)}>
				Add Item
			</button>
			<SearchDialog
				open={searchOpen}
				onOpenChange={setSearchOpen}
				onAdd={(candidate) => addItem.mutate(candidate)}
				isPending={addItem.isPending}
			/>

			{/*item grid*/}

			<ItemGrid items={items} />
		</div>
	);
}

// <Page>
//   <ListHeader />       — list name, icon, edit/delete list actions
//   <ItemsToolbar />     — search, add item, bulk select toggle
//   <ItemsGrid />        — the actual items
// </Page>
