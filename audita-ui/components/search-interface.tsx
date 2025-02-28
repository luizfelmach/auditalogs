"use client";

import type React from "react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Search } from "lucide-react";
import { DocumentList } from "@/components/document-list";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import axios from "axios";
import { useState } from "react";
import { toast } from "sonner";

type Document = {
  id: string;
  index: string;
  source: Record<string, any>;
};

const searchFetch = async (searchQuery: string): Promise<Document[]> => {
  const response = await axios.post(`http://localhost:8081/search`, {
    match: {
      ip: searchQuery,
    },
  });
  return response.data;
};

export function SearchInterface() {
  const [searchQuery, setSearchQuery] = useState<string>("");
  const queryClient = useQueryClient();

  const { data, isLoading } = useQuery<Document[]>({
    queryKey: ["search", searchQuery],
    queryFn: () => searchFetch(searchQuery),
    enabled: !!searchQuery,
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    queryClient.invalidateQueries({ queryKey: ["search"] });
  };

  return (
    <div className="space-y-6">
      <form onSubmit={handleSearch} className="flex gap-2">
        <Input
          type="search"
          placeholder="Search documents..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="flex-grow"
        />
        <Button type="submit">
          <Search className="mr-2 h-4 w-4" /> Search
        </Button>
      </form>
      {!isLoading && data && (
        <DocumentList
          documents={data}
          onCheckIntegrity={async (index) => {
            try {
              const response = await axios.get(
                `http://localhost:8081/proof/${index}`
              );
              if (response.data.blockchain_match) {
                toast.success(
                  "Integridade verificada com sucesso! Os dados estão íntegros."
                );
              } else if (!response.data.blockchain_match) {
                toast.error(
                  "Falha na verificação de integridade. Os dados não está íntegros."
                );
              }
            } catch (error) {
              toast.error("Erro ao verificar integridade");
              return false;
            }
          }}
        />
      )}
    </div>
  );
}
