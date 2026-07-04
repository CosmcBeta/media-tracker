import { Dialog, DialogContent } from "@/components/ui/dialog";
import { ItemDetailsContent } from "./ItemDetailsContent";

interface ItemDetailsDialogProps {
	itemId: string | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
}

export function ItemDetailsDialog({
	itemId,
	open,
	onOpenChange,
}: ItemDetailsDialogProps) {
	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-2xl">
				{itemId && <ItemDetailsContent itemId={itemId} />}
			</DialogContent>
		</Dialog>
	);
}
