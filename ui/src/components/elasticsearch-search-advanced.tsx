import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Search, Shield, Download, X } from "lucide-react";

// Hooks
import { useSearchFilters } from "../hooks/use-search-filter";
import { useElasticsearchSearch } from "../hooks/use-elasticsearch-search";
import { useIntegrityVerification } from "../hooks/use-integrity-verification";

// Components
import { FilterForm } from "./filter-form";
import { ActiveFilters } from "./active-filters";
import { ElasticsearchQuery } from "./elasticsearch-query";
import { SearchResults } from "./search-results";
import { VerificationResults } from "./verification-results";
import { UsageGuideSearch } from "./usage-guide-search";

// Utils
import { exportToJson } from "../lib/helpers";

export function ElasticsearchSearchAdvanced() {
  const {
    filters,
    newFilter,
    setNewFilter,
    addFilter,
    removeFilter,
    clearFilters,
  } = useSearchFilters();

  const {
    searchResults,
    isSearching,
    hasSearched,
    elasticsearchQuery,
    performSearch,
  } = useElasticsearchSearch();

  const { verificationResults, isVerifying, verifyIntegrity } =
    useIntegrityVerification();

  const handleSearch = () => {
    performSearch(filters);
  };

  const handleVerifyIntegrity = () => {
    verifyIntegrity(searchResults);
  };

  const handleExportResults = () => {
    const exportData = {
      searchResults,
      verificationResults,
      filters,
      exportedAt: new Date().toISOString(),
    };
    exportToJson(exportData, "elasticsearch-results");
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Search className="h-5 w-5 text-blue-600" />
            <span>Busca Avançada no Elasticsearch</span>
          </CardTitle>
          <CardDescription>
            Configure filtros personalizados definindo tipo de campo e chave
            manualmente
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <FilterForm
            newFilter={newFilter}
            setNewFilter={setNewFilter}
            onAddFilter={addFilter}
          />

          <ActiveFilters filters={filters} onRemoveFilter={removeFilter} />

          <Separator />

          <div className="flex items-center space-x-4">
            <Button
              onClick={handleSearch}
              disabled={isSearching}
              className="flex items-center space-x-2 bg-gradient-to-r from-blue-600 to-indigo-600"
            >
              <Search className="h-4 w-4" />
              <span>{isSearching ? "Buscando..." : "Executar Busca"}</span>
            </Button>

            {searchResults.length > 0 && (
              <>
                <Button
                  onClick={handleVerifyIntegrity}
                  disabled={isVerifying}
                  className="flex items-center space-x-2 bg-gradient-to-r from-green-600 to-emerald-600"
                >
                  <Shield className="h-4 w-4" />
                  <span>
                    {isVerifying ? "Verificando..." : "Verificar Integridade"}
                  </span>
                </Button>

                <Button variant="outline" onClick={handleExportResults}>
                  <Download className="h-4 w-4 mr-2" />
                  Exportar Resultados
                </Button>
              </>
            )}

            {filters.length > 0 && (
              <Button variant="outline" onClick={clearFilters}>
                <X className="h-4 w-4 mr-2" />
                Limpar Filtros
              </Button>
            )}
          </div>
        </CardContent>
      </Card>

      <ElasticsearchQuery
        query={elasticsearchQuery}
        filtersCount={filters.length}
      />

      <SearchResults
        results={searchResults}
        isSearching={isSearching}
        hasSearched={hasSearched}
        filtersCount={filters.length}
      />

      <VerificationResults
        results={verificationResults}
        isVerifying={isVerifying}
      />

      <UsageGuideSearch />
    </div>
  );
}
