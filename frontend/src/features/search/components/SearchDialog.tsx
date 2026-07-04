import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
	Command,
	CommandEmpty,
	CommandInput,
	CommandList,
} from "@/components/ui/command";
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from "@/components/ui/dialog";
import { cn } from "@/lib/utils";
import type { MediaType, SearchCandidate } from "@/types";
import { useSearch } from "../hooks/useSearch";
import { SearchResultCard } from "./SearchResultCard";

interface Props {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onAdd: (candidate: SearchCandidate) => void;
	isPending: boolean;
}

const TABS: { label: string; value: MediaType; disabled?: boolean }[] = [
	{ label: "Movie", value: "movie" },
	{ label: "Show", value: "show" },
	{ label: "Album", value: "album" },
	{ label: "Artist", value: "artist" },
	{ label: "Book", value: "book", disabled: true },
	{ label: "Game", value: "game" },
	{ label: "Podcast", value: "podcast", disabled: true },
];

export function SearchDialog({ open, onOpenChange, onAdd, isPending }: Props) {
	const [input, setInput] = useState("");
	const [query, setQuery] = useState("");
	const [mediaType, setMediaType] = useState<MediaType>("movie");
	const [selected, setSelected] = useState<SearchCandidate | null>(null);

	const { data: results, isPending: isSearching } = useSearch(query, mediaType);

	useEffect(() => {
		const timer = setTimeout(() => setQuery(input), 300);
		return () => clearTimeout(timer);
	}, [input]);

	function handleOpenChange(next: boolean) {
		if (!next) {
			setInput("");
			setQuery("");
			setSelected(null);
			setMediaType("movie");
		}
		onOpenChange(next);
	}

	function handleTabChange(type: MediaType) {
		setMediaType(type);
		setSelected(null);
	}

	return (
		<Dialog open={open} onOpenChange={handleOpenChange}>
			<DialogContent className="p-0 gap-0 max-w-lg">
				<DialogHeader className="px-4 pt-4 pb-0">
					<DialogTitle>Add item to list</DialogTitle>
				</DialogHeader>

				<div className="px-4 pt-3 flex gap-1.5 flex-wrap">
					{TABS.map((tab) => (
						<button
							key={tab.value}
							type="button"
							disabled={tab.disabled}
							onClick={() => handleTabChange(tab.value)}
							className={cn(
								"text-xs px-3 py-1.5 rounded-md border border-border transition-colors",
								mediaType === tab.value
									? "bg-primary text-primary-foreground border-primary"
									: "text-muted-foreground hover:text-foreground",
							)}
						>
							{tab.label}
						</button>
					))}
				</div>

				<Command shouldFilter={false}>
					<CommandInput
						placeholder={`Search ${mediaType}s...`}
						value={input}
						onValueChange={setInput}
					/>
					<CommandList className="min-h-60 max-h-80">
						{query.length <= 2 && (
							<div className="py-6 text-center text-sm text-muted-foreground">
								Type at least 3 characters to search
							</div>
						)}
						{query.length > 2 && isSearching && (
							<div className="py-6 text-center text-sm text-muted-foreground">
								Searching…
							</div>
						)}
						{results?.map((candidate) => (
							<SearchResultCard
								key={candidate.external_id}
								candidate={candidate}
								selected={selected?.external_id === candidate.external_id}
								onSelect={() => setSelected(candidate)}
							/>
						))}
						{query.length > 2 && !isSearching && results?.length === 0 && (
							<CommandEmpty>No results found.</CommandEmpty>
						)}
					</CommandList>
				</Command>
				<div className="px-4 py-3 border-t">
					<Button variant="outline" onClick={() => handleOpenChange(false)}>
						Cancel
					</Button>
					<Button
						disabled={!selected || isPending}
						onClick={() => selected && onAdd(selected)}
					>
						{isPending ? "Adding…" : "Add to list"}
					</Button>
				</div>
			</DialogContent>
		</Dialog>
	);
}
