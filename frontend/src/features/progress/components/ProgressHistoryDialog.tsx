import { Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from "@/components/ui/dialog";
import type { Progress } from "@/types";
import { useDeleteProgress } from "../hooks/useProgress";

interface ProgressHistoryDialogProps {
	itemId: string;
	entries: Progress[];
	open: boolean;
	onOpenChange: (open: boolean) => void;
}

function formatEntry(kind: string, value: string | null) {
	switch (kind) {
		case "episode":
			return value ? `Episode ${value}` : "Episode";
		case "page":
			return value ? `Page ${value}` : "Page";
		case "percentage":
			return value ? `${value}%` : "Percentage";
		case "complete":
			return "Completed";
		default:
			return kind;
	}
}

export function ProgressHistoryDialog({
	itemId,
	entries,
	open,
	onOpenChange,
}: ProgressHistoryDialogProps) {
	const { mutate: deleteProgress } = useDeleteProgress(itemId);

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle>Progress history</DialogTitle>
				</DialogHeader>

				<div className="flex flex-col gap-2 max-h-96 overflow-y-auto">
					{entries.length === 0 && (
						<p className="text-sm text-muted-foreground">No entries yet.</p>
					)}
					{entries.map((entry) => (
						<div
							key={entry.id}
							className="flex items-start justify-between border-b pb-2 last:border-0"
						>
							<div>
								<p className="text-sm font-medium">
									{formatEntry(entry.kind, entry.value)}
								</p>
								{entry.note && (
									<p className="text-sm text-muted-foreground">{entry.note}</p>
								)}
								<p className="text-xs text-muted-foreground">
									{new Date(entry.logged_at).toLocaleString()}
								</p>
							</div>
							<Button
								variant="ghost"
								size="icon"
								onClick={() => deleteProgress(entry.id)}
							>
								<Trash2 className="h-4 w-4 text-destructive" />
							</Button>
						</div>
					))}
				</div>
			</DialogContent>
		</Dialog>
	);
}
