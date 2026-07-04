import { Trash2Icon } from "lucide-react";
import { useState } from "react";
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
	AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import type { Item } from "@/types";
import { useDeleteItemFromList } from "../hooks/useItems";

export function DeleteItemFromListDialog({
	item,
	listId,
}: {
	item: Item;
	listId: string;
}) {
	const [open, setOpen] = useState(false);
	const deleteItemFromList = useDeleteItemFromList(listId, {
		onSuccess: () => setOpen(false),
	});
	return (
		<AlertDialog open={open} onOpenChange={setOpen}>
			<AlertDialogTrigger
				render={
					<Button size="icon" variant="destructive">
						<Trash2Icon />
					</Button>
				}
			/>
			<AlertDialogContent size="sm">
				<AlertDialogHeader>
					<AlertDialogTitle>
						Remove "{item.title}" from this list?
					</AlertDialogTitle>
					<AlertDialogDescription>
						This won't delete the item itself, just removes it from this list.
					</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel variant="outline">Cancel</AlertDialogCancel>
					<AlertDialogAction
						variant="destructive"
						onClick={() => deleteItemFromList.mutate(item.id)}
						disabled={deleteItemFromList.isPending}
					>
						{deleteItemFromList.isPending ? "Removing..." : "Remove"}
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
