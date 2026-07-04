import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import {
	Card,
	CardAction,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { ItemDetailsDialog } from "@/features/items/components/ItemDetailsDialog";
import { getItemSummary } from "@/features/items/lib/getItemSummary";
import { formatDateTime } from "@/lib/date";
import type { Item } from "@/types";
import { DeleteItemFromListDialog } from "./DeleteItemFromListDialog";

export function ItemCard({ item, listId }: { item: Item; listId: string }) {
	const [open, setOpen] = useState(false);
	const { posterUrl, subtitle } = getItemSummary(item);

	return (
		<>
			<Card
				className="w-full mx-auto max-w-sm cursor-pointer transition-colors hover:bg-accent/50 overflow-hidden"
				onClick={() => setOpen(true)}
			>
				<div className="flex gap-4 p-4">
					{posterUrl ? (
						<img
							src={posterUrl}
							alt={item.title}
							className="w-16 h-16 rounded-md object-cover shrink-0 bg-muted"
						/>
					) : (
						<div className="w-16 h-16 rounded-md bg-muted shrink-0 flex items-center justify-center text-xs text-muted-foreground">
							No image
						</div>
					)}

					<div className="flex flex-col gap-1 min-w-0 flex-1">
						<CardHeader className="p-0">
							<CardTitle className="flex items-center gap-2 truncate">
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

						<div className="flex items-center gap-2">
							<Badge variant="secondary" className="text-xs capitalize">
								{item.media_type}
							</Badge>
						</div>

						{subtitle && (
							<p className="text-sm text-muted-foreground truncate">
								{subtitle}
							</p>
						)}
					</div>
				</div>

				<CardFooter className="flex flex-col items-start gap-1 text-sm text-muted-foreground border-t pt-3">
					<div>Created {formatDateTime(item.created_at)}</div>
					<div>Updated {formatDateTime(item.updated_at)}</div>
				</CardFooter>
			</Card>
			<ItemDetailsDialog itemId={item.id} open={open} onOpenChange={setOpen} />
		</>
	);
}
