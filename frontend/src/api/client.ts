import type {
	AddItemToList,
	CreateItem,
	CreateList,
	CreateProgress,
	Item,
	List,
	Progress,
	SearchCandidate,
	SearchParams,
	UpdateItem,
	UpdateList,
} from "../types";

const BASE_URL = "/api/v1";

export const api = {
	getItems,
	getItem,
	createItem,
	updateItem,
	deleteItem,
	searchItems,
	importItem,
	getLists,
	createList,
	updateList,
	deleteList,
	getListItems,
	addItemToList,
	deleteItemFromList,
	getItemProgress,
	createItemProgress,
	deleteItemProgress,
};

async function getItems(): Promise<Item[]> {
	const response = await fetch(`${BASE_URL}/items`);
	if (!response.ok) {
		throw new Error(`Failed to fetch items: ${response.status}`);
	}
	return response.json();
}

async function getItem(id: string): Promise<Item> {
	const response = await fetch(`${BASE_URL}/items/${id}`);
	if (!response.ok) {
		throw new Error(`Failed to fetch item: ${response.status}`);
	}
	return response.json();
}

async function createItem(data: CreateItem): Promise<Item> {
	const response = await fetch(`${BASE_URL}/items`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data),
	});
	if (!response.ok) {
		throw new Error(`Failed to create item: ${response.status}`);
	}
	return response.json();
}

async function updateItem(id: string, data: UpdateItem): Promise<Item> {
	const response = await fetch(`${BASE_URL}/items/${id}`, {
		method: "PATCH",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data),
	});
	if (!response.ok) {
		throw new Error(`Failed to update item: ${response.status}`);
	}
	return response.json();
}

async function deleteItem(id: string): Promise<void> {
	const response = await fetch(`${BASE_URL}/items/${id}`, {
		method: "DELETE",
	});
	if (!response.ok) {
		throw new Error(`Failed to delete item: ${response.status}`);
	}
}

async function searchItems(params: SearchParams): Promise<SearchCandidate[]> {
	const query = new URLSearchParams({
		q: params.q,
		media_type: params.media_type,
	});
	const response = await fetch(`${BASE_URL}/items/search?${query}`);
	if (!response.ok) {
		throw new Error(`Failed to search items: ${response.status}`);
	}
	return response.json();
}

async function importItem(candidate: SearchCandidate): Promise<Item> {
	const response = await fetch(`${BASE_URL}/items/import`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(candidate),
	});
	if (!response.ok) {
		throw new Error(`Failed to import item: ${response.status}`);
	}
	return response.json();
}

async function getLists(): Promise<List[]> {
	const response = await fetch(`${BASE_URL}/lists`);
	if (!response.ok) {
		throw new Error(`Failed to fetch lists: ${response.status}`);
	}
	return response.json();
}

async function createList(data: CreateList): Promise<List> {
	const response = await fetch(`${BASE_URL}/lists`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data),
	});
	if (!response.ok) {
		throw new Error(`Failed to create list: ${response.status}`);
	}
	return response.json();
}

async function updateList(id: string, data: UpdateList): Promise<List> {
	const response = await fetch(`${BASE_URL}/lists/${id}`, {
		method: "PATCH",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data),
	});
	if (!response.ok) {
		throw new Error(`Failed to update list: ${response.status}`);
	}
	return response.json();
}

async function deleteList(id: string): Promise<void> {
	const response = await fetch(`${BASE_URL}/lists/${id}`, {
		method: "DELETE",
	});
	if (!response.ok) {
		throw new Error(`Failed to delete list: ${response.status}`);
	}
}

async function getListItems(id: string): Promise<Item[]> {
	const response = await fetch(`${BASE_URL}/lists/${id}/items`);
	if (!response.ok) {
		throw new Error(`Failed to fetch list items: ${response.status}`);
	}
	return response.json();
}

async function addItemToList(
	listId: string,
	data: AddItemToList,
): Promise<void> {
	const response = await fetch(`${BASE_URL}/lists/${listId}/items`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data),
	});
	if (!response.ok) {
		throw new Error(`Failed to add item to list: ${response.status}`);
	}
}

async function deleteItemFromList(
	listId: string,
	itemId: string,
): Promise<void> {
	const response = await fetch(`${BASE_URL}/lists/${listId}/items/${itemId}`, {
		method: "DELETE",
	});
	if (!response.ok) {
		throw new Error(`Failed to delete item from list: ${response.status}`);
	}
}

async function getItemProgress(itemId: string): Promise<Progress[]> {
	const response = await fetch(`${BASE_URL}/items/${itemId}/progress`);
	if (!response.ok) {
		throw new Error(`Failed to fetch item progress: ${response.status}`);
	}
	return response.json();
}

async function createItemProgress(
	itemId: string,
	data: CreateProgress,
): Promise<Progress> {
	const response = await fetch(`${BASE_URL}/items/${itemId}/progress`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(data),
	});
	if (!response.ok) {
		throw new Error(`Failed to create item progress: ${response.status}`);
	}
	return response.json();
}

async function deleteItemProgress(id: string): Promise<void> {
	const response = await fetch(`${BASE_URL}/progress/${id}`, {
		method: "DELETE",
	});
	if (!response.ok) {
		throw new Error(`Failed to delete item progress: ${response.status}`);
	}
}
