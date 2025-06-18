import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Filter, Download } from "lucide-react";

interface ElasticsearchQueryProps {
  query: any;
  filtersCount: number;
}

export function ElasticsearchQuery({
  query,
  filtersCount,
}: ElasticsearchQueryProps) {
  if (!query) return null;

  const copyQuery = () => {
    navigator.clipboard.writeText(JSON.stringify(query, null, 2));
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Filter className="h-5 w-5 text-purple-600" />
          <span>Query Elasticsearch Gerada</span>
        </CardTitle>
        <CardDescription>
          Query no formato do Elasticsearch baseada nos filtros aplicados
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <Label className="text-sm font-medium">Query JSON:</Label>
            <Button variant="outline" size="sm" onClick={copyQuery}>
              <Download className="h-4 w-4 mr-2" />
              Copiar Query
            </Button>
          </div>
          <pre className="p-4 bg-muted rounded-lg text-sm overflow-x-auto border">
            {JSON.stringify(query, null, 2)}
          </pre>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-4 border-t">
            <div>
              <Label className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
                Filtros Aplicados
              </Label>
              <p className="text-sm mt-1">{filtersCount} filtro(s) ativo(s)</p>
            </div>
            <div>
              <Label className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
                Tipo de Query
              </Label>
              <p className="text-sm mt-1">
                {filtersCount === 0 ? "match_all" : "bool query"}
              </p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
