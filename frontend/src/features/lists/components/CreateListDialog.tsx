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
import { useCreateList } from "../hooks/useLists";

export function CreateListDialog() {
	const [open, setOpen] = useState(false);
	const [name, setName] = useState("");
	const [icon, setIcon] = useState<string | null>(null);

	const createList = useCreateList({
		onSuccess: () => {
			setName("");
			setIcon(null);
			setOpen(false);
		},
	});

	function handleSubmit(e: React.SubmitEvent<HTMLFormElement>) {
		e.preventDefault();

		createList.mutate({
			name: name,
			icon: icon ?? undefined,
		});
	}

	return (
		<Dialog open={open} onOpenChange={setOpen}>
			<DialogTrigger render={<Button>Create List</Button>} />
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Create New List</DialogTitle>
					<DialogDescription>You can create a new list</DialogDescription>
				</DialogHeader>
				<form onSubmit={handleSubmit}>
					<div>
						<label htmlFor="name">
							Name: <span className="text-destructive">*</span>
						</label>

						<Input
							id="name"
							value={name}
							onChange={(e) => setName(e.target.value)}
							placeholder="Watchlist"
							required
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
						<Button type="submit" disabled={createList.isPending}>
							{createList.isPending ? "Creating..." : "Create"}
						</Button>
					</DialogFooter>
				</form>
			</DialogContent>
		</Dialog>
	);
}
