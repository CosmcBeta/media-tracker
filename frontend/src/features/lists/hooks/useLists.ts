import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/api/client";
import type { UpdateList } from "@/types";

const LISTS_KEY = ["lists"] as const;

export function useLists() {
	return useQuery({
		queryKey: LISTS_KEY,
		queryFn: api.getLists,
	});
}

export function useCreateList(options?: { onSuccess?: () => void }) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: api.createList,
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: LISTS_KEY,
			});
			options?.onSuccess?.();
		},
	});
}

export function useUpdateList(options?: { onSuccess?: () => void }) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: ({ id, data }: { id: string; data: UpdateList }) =>
			api.updateList(id, data),
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: LISTS_KEY,
			});
			options?.onSuccess?.();
		},
	});
}

export function useDeleteList(options?: { onSuccess?: () => void }) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: api.deleteList,
		onSuccess: () => {
			queryClient.invalidateQueries({
				queryKey: LISTS_KEY,
			});
			options?.onSuccess?.();
		},
	});
}
