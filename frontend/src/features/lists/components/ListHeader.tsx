import { ListIcon } from "@/components/ListIcon";
import type { List } from "@/types";
import { DeleteListDialog } from "./DeleteListDialog";
import { EditListDialog } from "./EditListDialog";

export function ListHeader({ list }: { list: List }) {
	return (
		<div className="flex items-center justify-between border-b pb-4">
			<div className="flex items-center gap-3 min-w-0">
				<ListIcon
					name={list.icon}
					className="w-7 h-7 shrink-0 text-muted-foreground"
				/>
				<h1 className="text-2xl font-semibold truncate">{list.name}</h1>
			</div>
			<div className="flex gap-2 shrink-0">
				<EditListDialog list={list} />
				<DeleteListDialog list={list} />
			</div>
		</div>
	);
}
