import { Edit2Icon } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { ICONS } from "@/lib/icons";
import type { List } from "@/types";
import { useUpdateList } from "../hooks/useLists";

export function EditListDialog({ list }: { list: List }) {
	const [open, setOpen] = useState(false);
	const [name, setName] = useState(list.name);
	const [icon, setIcon] = useState(list.icon);

	const updateList = useUpdateList({ onSuccess: () => setOpen(false) });

	function handleSubmit(e: React.SubmitEvent<HTMLFormElement>) {
		e.preventDefault();

		updateList.mutate({
			id: list.id,
			data: {
				name: name,
				icon: icon ?? undefined,
			},
		});
	}

	return (
		<Dialog
			open={open}
			onOpenChange={(open) => {
				if (open) {
					setName(list.name);
					setIcon(list.icon);
				}

				setOpen(open);
			}}
		>
			<DialogTrigger
				render={
					<Button size="icon" variant="secondary">
						<Edit2Icon />
					</Button>
				}
			/>
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Edit List</DialogTitle>
					<DialogDescription>Edit your list.</DialogDescription>
				</DialogHeader>
				<form onSubmit={handleSubmit}>
					<div>
						<label htmlFor="name">Name:</label>

						<Input
							id="name"
							value={name}
							onChange={(e) => setName(e.target.value)}
							placeholder="Watchlist"
						/>
					</div>

					<div>
						<label htmlFor="icon">Icon:</label>

						<Select
							id="icon"
							value={icon ?? "none"}
							onValueChange={(value) =>
								setIcon(value === "none" ? null : value)
							}
						>
							<SelectTrigger className="w-[180px]">
								<SelectValue placeholder="Choose an icon" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="none">No icon</SelectItem>

								{Object.entries(ICONS).map(([name, item]) => {
									const Icon = item.icon;

									return (
										<SelectItem key={name} value={name}>
											<div className="flex items-center gap-2">
												<Icon className="size-4" />

												{item.label}
											</div>
										</SelectItem>
									);
								})}
							</SelectContent>
						</Select>
					</div>
					<DialogFooter>
						<Button
							type="button"
							variant="outline"
							onClick={() => setOpen(false)}
						>
							Cancel
						</Button>
						<Button type="submit" disabled={updateList.isPending}>
							{updateList.isPending ? "Saving..." : "Save"}
						</Button>
					</DialogFooter>
				</form>
			</DialogContent>
		</Dialog>
	);
}
