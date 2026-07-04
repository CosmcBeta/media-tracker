import { useQuery } from "@tanstack/react-query";
import { api } from "@/api/client";
import type { MediaType } from "@/types";

export function useSearch(query: string, mediaType: MediaType) {
	return useQuery({
		queryKey: ["search", query, mediaType],
		queryFn: () => api.searchItems({ q: query, media_type: mediaType }),
		enabled: query.length > 2,
		staleTime: 100 * 60 * 5,
		placeholderData: undefined,
	});
}
