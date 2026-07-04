import { useState } from "react";
import {
	Card,
	CardAction,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { ItemDetailsDialog } from "@/features/items/components/ItemDetailsDialog";
import { formatDateTime } from "@/lib/date";
import type { Item } from "@/types";
import { DeleteItemFromListDialog } from "./DeleteItemFromListDialog";

export function ItemCard({ item, listId }: { item: Item; listId: string }) {
	const [open, setOpen] = useState(false);

	return (
		<>
			<Card
				className="w-full mx-auto max-w-sm cursor-pointer transition-colors hover:bg-accent/50"
				onClick={() => setOpen(true)}
			>
				<CardHeader>
					<CardTitle className="flex items-center gap-2">
						{item.title}
					</CardTitle>
					<CardAction
						className="flex gap-2"
						onClick={(e) => {
							e.stopPropagation();
						}}
					>
						<DeleteItemFromListDialog item={item} listId={listId} />
					</CardAction>
				</CardHeader>
				<CardFooter className="flex flex-col items-start gap-1 text-sm text-muted-foreground">
					<div>Created {formatDateTime(item.created_at)}</div>
					<div>Updated {formatDateTime(item.updated_at)}</div>
				</CardFooter>
			</Card>

			<ItemDetailsDialog itemId={item.id} open={open} onOpenChange={setOpen} />
		</>
	);
}
