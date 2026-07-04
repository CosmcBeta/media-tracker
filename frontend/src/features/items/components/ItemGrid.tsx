import type { Item } from "@/types";
import { ItemCard } from "./ItemCard";

export function ItemGrid({ items }: { items: Item[] }) {
	return (
		<div className="grid gap-4 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 p-4">
			{items.map((item) => {
				return <ItemCard key={item.id} item={item} />;
			})}
		</div>
	);
}
