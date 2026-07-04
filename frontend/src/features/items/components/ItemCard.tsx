import { useNavigate } from "react-router";
import {
	Card,
	CardAction,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { formatDateTime } from "@/lib/date";
import type { Item } from "@/types";

export function ItemCard({ item }: { item: Item }) {
	const navigate = useNavigate();
	return (
		<Card
			className="w-full mx-auto max-w-sm cursor-pointer transition-colors hover:bg-accent/50"
			onClick={() => navigate(`/items/${item.id}`)}
		>
			<CardHeader>
				<CardTitle className="flex items-center gap-2">{item.title}</CardTitle>
				{/*<CardAction
					className="flex gap-2"
					onClick={(e) => {
						e.stopPropagation();
					}}
				>
					<DeleteListDialog list={list} />

					<EditListDialog list={list} />
				</CardAction>*/}
			</CardHeader>
			<CardFooter className="flex flex-col items-start gap-1 text-sm text-muted-foreground">
				<div>Created {formatDateTime(item.created_at)}</div>
				<div>Updated {formatDateTime(item.updated_at)}</div>
			</CardFooter>
		</Card>
	);
}
