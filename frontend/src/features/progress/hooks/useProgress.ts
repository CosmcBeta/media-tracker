import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/api/client";
import type { CreateProgress } from "@/types";

const progressKey = (itemId: string) => ["progress", itemId] as const;

export function useItemProgress(itemId: string) {
	return useQuery({
		queryKey: progressKey(itemId),
		queryFn: () => api.getItemProgress(itemId),
		enabled: !!itemId,
	});
}

export function useCreateProgress(itemId: string) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: (data: CreateProgress) => api.createItemProgress(itemId, data),
		onSuccess: () =>
			queryClient.invalidateQueries({ queryKey: progressKey(itemId) }),
	});
}

export function useDeleteProgress(itemId: string) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: (id: string) => api.deleteItemProgress(id),
		onSuccess: () =>
			queryClient.invalidateQueries({ queryKey: progressKey(itemId) }),
	});
}
