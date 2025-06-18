import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Search } from "lucide-react";
import type { ElasticsearchDocument } from "../types/search";

interface SearchResultsProps {
  results: ElasticsearchDocument[];
  isSearching: boolean;
  hasSearched: boolean;
  filtersCount: number;
}

export function SearchResults({
  results,
  isSearching,
  hasSearched,
  filtersCount,
}: SearchResultsProps) {
  if (!hasSearched) return null;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Documentos Encontrados</CardTitle>
        <CardDescription>
          {isSearching
            ? "Executando busca com filtros..."
            : `${results.length} documentos encontrados`}
        </CardDescription>
      </CardHeader>
      <CardContent>
        {isSearching ? (
          <div className="flex items-center justify-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          </div>
        ) : results.length > 0 ? (
          <div className="space-y-4">
            {results.map((doc) => (
              <Card key={doc.id} className="border-l-4 border-l-blue-500">
                <CardHeader className="pb-3">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-4">
                      <Badge variant="outline" className="font-mono text-xs">
                        {doc.id}
                      </Badge>
                      <Badge variant="secondary" className="text-xs">
                        {doc.index}
                      </Badge>
                    </div>
                  </div>
                </CardHeader>
                <CardContent className="pt-0">
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {Object.entries(doc.source).map(([key, value]) => (
                      <div key={key} className="space-y-1">
                        <Label className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
                          {key.replace(/_/g, " ")}
                        </Label>
                        <div className="text-sm">
                          {typeof value === "boolean" ? (
                            <Badge variant={value ? "default" : "secondary"}>
                              {value ? "True" : "False"}
                            </Badge>
                          ) : typeof value === "number" ? (
                            <span className="font-mono">{value}</span>
                          ) : typeof value === "string" && value.length > 50 ? (
                            <div className="truncate" title={value}>
                              {value}
                            </div>
                          ) : (
                            <span>{String(value)}</span>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>

                  <details className="mt-4">
                    <summary className="cursor-pointer text-sm text-muted-foreground hover:text-foreground">
                      Ver JSON completo
                    </summary>
                    <pre className="mt-2 p-3 bg-muted rounded-md text-xs overflow-x-auto">
                      {JSON.stringify(doc.source, null, 2)}
                    </pre>
                  </details>
                </CardContent>
              </Card>
            ))}

            <div className="flex items-center justify-between pt-4">
              <p className="text-sm text-muted-foreground">
                Mostrando {results.length} documentos | Filtros aplicados:{" "}
                {filtersCount}
              </p>
              <p className="text-sm text-muted-foreground">
                Tempo de busca: 0.45s
              </p>
            </div>
          </div>
        ) : (
          <div className="text-center py-8">
            <Search className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
            <p className="text-muted-foreground">Nenhum resultado encontrado</p>
            <p className="text-sm text-muted-foreground mt-2">
              Tente ajustar os filtros ou verificar a conexão com o
              Elasticsearch
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
