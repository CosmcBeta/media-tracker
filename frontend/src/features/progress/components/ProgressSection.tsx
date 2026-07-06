import { History, Plus } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { useItemProgress } from "../hooks/useProgress";
import { LogProgressDialog } from "./LogProgressDialog";
import { ProgressHistoryDialog } from "./ProgressHistoryDialog";

function formatProgress(kind: string, value: string | null) {
	switch (kind) {
		case "episode":
			return value ? `Episode ${value}` : "Episode logged";
		case "page":
			return value ? `Page ${value}` : "Page logged";
		case "percentage":
			return value ? `${value}%` : "Progress logged";
		case "complete":
			return "Completed";
		default:
			return kind;
	}
}

export function ProgressSection({ itemId }: { itemId: string }) {
	const [logOpen, setLogOpen] = useState(false);
	const [historyOpen, setHistoryOpen] = useState(false);
	const { data: progress, isLoading } = useItemProgress(itemId);

	const sorted = progress
		? [...progress].sort(
				(a, b) =>
					new Date(b.logged_at).getTime() - new Date(a.logged_at).getTime(),
			)
		: [];
	const mostRecent = sorted[0];

	return (
		<div className="border-t pt-4 mt-4">
			<div className="flex items-center justify-between">
				<div>
					<h3 className="text-sm font-medium text-muted-foreground">
						Progress
					</h3>
					{isLoading ? (
						<p className="text-sm text-muted-foreground">Loading...</p>
					) : mostRecent ? (
						<p className="text-base">
							{formatProgress(mostRecent.kind, mostRecent.value)}
							<span className="text-xs text-muted-foreground ml-2">
								{new Date(mostRecent.logged_at).toLocaleDateString()}
							</span>
						</p>
					) : (
						<p className="text-sm text-muted-foreground">
							No progress logged yet
						</p>
					)}
				</div>
				<div className="flex gap-2">
					{sorted.length > 0 && (
						<Button
							variant="outline"
							size="icon"
							onClick={() => setHistoryOpen(true)}
						>
							<History className="h-4 w-4" />
						</Button>
					)}
					<Button size="icon" onClick={() => setLogOpen(true)}>
						<Plus className="h-4 w-4" />
					</Button>
				</div>
			</div>

			<LogProgressDialog
				itemId={itemId}
				open={logOpen}
				onOpenChange={setLogOpen}
			/>
			<ProgressHistoryDialog
				itemId={itemId}
				entries={sorted}
				open={historyOpen}
				onOpenChange={setHistoryOpen}
			/>
		</div>
	);
}
