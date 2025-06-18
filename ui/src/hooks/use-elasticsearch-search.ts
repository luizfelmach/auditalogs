import { useState } from "react";
import type { ElasticsearchDocument, SearchFilter } from "../types/search";
import { buildElasticsearchQuery } from "../lib/query-builder";
import { searchDocuments } from "@/lib/api";

export const useElasticsearchSearch = () => {
  const [searchResults, setSearchResults] = useState<ElasticsearchDocument[]>(
    [],
  );
  const [isSearching, setIsSearching] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);
  const [elasticsearchQuery, setElasticsearchQuery] = useState<any>(null);

  const performSearch = async (filters: SearchFilter[]) => {
    setIsSearching(true);
    setHasSearched(true);

    const query = buildElasticsearchQuery(filters);
    setElasticsearchQuery(query);

    const results = await searchDocuments(query);
    setSearchResults(results);
    setIsSearching(false);
  };

  return {
    searchResults,
    isSearching,
    hasSearched,
    elasticsearchQuery,
    performSearch,
  };
};
