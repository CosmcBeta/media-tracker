import { CreateListDialog, ListGrid, useLists } from "@/features/lists";

export default function ListsPage() {
	const { data: lists, isPending, isError } = useLists();

	if (isPending) {
		return <div>Loading...</div>;
	}

	if (isError) {
		return <div>Error</div>;
	}

	return (
		<div>
			<CreateListDialog />

			<ListGrid lists={lists} />
		</div>
	);
}

// <Page>

//     <PageTitle />

//     <Toolbar />

//     <ListGrid />

// </Page>
//
// <Toolbar>
//  <SearchLists />

//  <SortDropdown />

//  <CreateListButton />
// </Toolbar>
