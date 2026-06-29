import type { List } from "@/types";
import { ListCard } from "./ListCard";

export function ListGrid({ lists }: { lists: List[] }) {
	return (
		<div className="grid gap-4 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 p-4">
			{lists.map((list) => {
				return <ListCard key={list.id} list={list} />;
			})}
		</div>
	);
}
