import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import type { ProgressKind } from "@/types";
import { useCreateProgress } from "../hooks/useProgress";

interface LogProgressDialogProps {
	itemId: string;
	open: boolean;
	onOpenChange: (open: boolean) => void;
}

const KIND_LABELS: Record<ProgressKind, string> = {
	episode: "Episode",
	page: "Page",
	percentage: "Percentage",
	complete: "Complete",
};

export function LogProgressDialog({
	itemId,
	open,
	onOpenChange,
}: LogProgressDialogProps) {
	const [kind, setKind] = useState<ProgressKind>("percentage");
	const [value, setValue] = useState("");
	const [note, setNote] = useState("");
	const { mutate, isPending } = useCreateProgress(itemId);

	const needsValue = kind !== "complete";

	function reset() {
		setKind("percentage");
		setValue("");
		setNote("");
	}

	function handleSubmit() {
		mutate(
			{
				kind,
				value: needsValue && value ? value : undefined,
				note: note || undefined,
			},
			{
				onSuccess: () => {
					reset();
					onOpenChange(false);
				},
			},
		);
	}

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle>Log progress</DialogTitle>
				</DialogHeader>

				<div className="flex flex-col gap-4">
					<div className="flex flex-col gap-2">
						<Label htmlFor="progress-kind">Type</Label>
						<Select
							value={kind}
							onValueChange={(v) => setKind(v as ProgressKind)}
						>
							<SelectTrigger id="progress-kind">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								{(Object.keys(KIND_LABELS) as ProgressKind[]).map((k) => (
									<SelectItem key={k} value={k}>
										{KIND_LABELS[k]}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>

					{needsValue && (
						<div className="flex flex-col gap-2">
							<Label htmlFor="progress-value">
								{kind === "episode" && "Episode number"}
								{kind === "page" && "Page number"}
								{kind === "percentage" && "Percentage"}
							</Label>
							<Input
								id="progress-value"
								type="number"
								min={0}
								max={kind === "percentage" ? 100 : undefined}
								value={value}
								onChange={(e) => setValue(e.target.value)}
							/>
						</div>
					)}

					<div className="flex flex-col gap-2">
						<Label htmlFor="progress-note">Note (optional)</Label>
						<Textarea
							id="progress-note"
							value={note}
							onChange={(e) => setNote(e.target.value)}
						/>
					</div>
				</div>

				<DialogFooter>
					<Button variant="outline" onClick={() => onOpenChange(false)}>
						Cancel
					</Button>
					<Button onClick={handleSubmit} disabled={isPending}>
						{isPending ? "Saving..." : "Save"}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
