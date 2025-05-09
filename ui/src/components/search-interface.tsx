import type { SearchParams } from "@/types/search";
import { useState } from "react";
import { SearchForm } from "./search-form";
import { SearchResults } from "./search-results";

export function SearchInterface() {
  const [searchParams, setSearchParams] = useState<SearchParams | null>(null);
  return (
    <main className="container mx-auto py-8 px-4 sm:px-6 lg:px-8">
      <div className="space-y-6">
        <SearchForm onSearch={setSearchParams} />
        {searchParams && <SearchResults searchParams={searchParams} />}
      </div>
    </main>
  );
}
