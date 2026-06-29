import { useNavigate } from "react-router";
import { ListIcon } from "@/components/ListIcon";
import {
	Card,
	CardAction,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { formatDateTime } from "@/lib/date";
import type { List } from "@/types";
import { DeleteListDialog } from "./DeleteListDialog";
import { EditListDialog } from "./EditListDialog";

export function ListCard({ list }: { list: List }) {
	const navigate = useNavigate();
	return (
		<Card
			className="w-full mx-auto max-w-sm cursor-pointer transition-colors hover:bg-accent/50"
			onClick={() => navigate(`/lists/${list.id}`)}
		>
			<CardHeader>
				<CardTitle className="flex items-center gap-2">
					<ListIcon name={list.icon} className="w-5 h-5" /> {list.name}
				</CardTitle>
				<CardAction
					className="flex gap-2"
					onClick={(e) => {
						e.stopPropagation();
					}}
				>
					<DeleteListDialog list={list} />

					<EditListDialog list={list} />
				</CardAction>
			</CardHeader>
			<CardFooter className="flex flex-col items-start gap-1 text-sm text-muted-foreground">
				<div>Created {formatDateTime(list.created_at)}</div>
				<div>Updated {formatDateTime(list.updated_at)}</div>
			</CardFooter>
		</Card>
	);
}
