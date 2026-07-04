import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/api/client";
import type { SearchCandidate } from "@/types";

const itemKeys = {
	all: ["lists"] as const,
	detail: (id: string) => ["items", id] as const,
	byList: (listId: string) => ["lists", listId, "items"] as const,
};

export function useListItems(id: string) {
	return useQuery({
		queryKey: itemKeys.byList(id),
		queryFn: () => api.getListItems(id),
	});
}

export function useAddItemToList(
	listId: string,
	options?: { onSuccess?: () => void },
) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: async (candidate: SearchCandidate) => {
			const item = await api.importItem(candidate);
			return api.addItemToList(listId, { item_id: item.id });
		},
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: itemKeys.byList(listId) });
			options?.onSuccess?.();
		},
	});
}

// export interface AddItemToList {
// 	item_id: string;
// }

// export function useCreateList(options?: { onSuccess?: () => void }) {
// 	const queryClient = useQueryClient();
// 	return useMutation({
// 		mutationFn: api.addItemToList,
// 		onSuccess: () => {
// 			queryClient.invalidateQueries({
// 				queryKey: LISTS_KEY,
// 			});
// 			options?.onSuccess?.();
// 		},
// 	});
// }

// export function useUpdateList(options?: { onSuccess?: () => void }) {
// 	const queryClient = useQueryClient();
// 	return useMutation({
// 		mutationFn: ({ id, data }: { id: string; data: UpdateList }) =>
// 			api.importItem(id, data),
// 		onSuccess: () => {
// 			queryClient.invalidateQueries({
// 				queryKey: LISTS_KEY,
// 			});
// 			options?.onSuccess?.();
// 		},
// 	});
// }

// export function useDeleteList(options?: { onSuccess?: () => void }) {
// 	const queryClient = useQueryClient();
// 	return useMutation({
// 		mutationFn: api.deleteItemFromList,
// 		onSuccess: () => {
// 			queryClient.invalidateQueries({
// 				queryKey: LISTS_KEY,
// 			});
// 			options?.onSuccess?.();
// 		},
// 	});
// }
