import { PlusIcon } from "lucide-react";
import { Button } from "@/components/ui/button";

export function ItemsToolbar({
	itemCount,
	onAddClick,
}: {
	itemCount: number;
	onAddClick: () => void;
}) {
	return (
		<div className="flex items-center justify-between">
			<p className="text-sm text-muted-foreground">
				{itemCount} {itemCount === 1 ? "item" : "items"}
			</p>
			<Button size="sm" onClick={onAddClick} className="gap-1.5">
				<PlusIcon className="w-4 h-4" />
				Add Item
			</Button>
		</div>
	);
}
