import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Search, Plus, X, Filter, Download } from "lucide-react";

interface SearchFilter {
  id: string;
  field: string;
  operator: string;
  value: string;
  type: "string" | "number" | "date";
}

const fieldTypes = {
  message: "string",
  level: "string",
  user: "string",
  service: "string",
  ip: "string",
  status: "string",
  timestamp: "date",
  duration: "number",
  response_time: "number",
  status_code: "number",
  bytes: "number",
};

const operators = {
  string: [
    { value: "equals", label: "Igual a" },
    { value: "contains", label: "Contém" },
    { value: "starts_with", label: "Começa com" },
    { value: "ends_with", label: "Termina com" },
    { value: "not_equals", label: "Diferente de" },
    { value: "regex", label: "Regex" },
  ],
  number: [
    { value: "equals", label: "Igual a" },
    { value: "greater_than", label: "Maior que" },
    { value: "less_than", label: "Menor que" },
    { value: "greater_equal", label: "Maior ou igual" },
    { value: "less_equal", label: "Menor ou igual" },
    { value: "between", label: "Entre" },
  ],
  date: [
    { value: "equals", label: "Igual a" },
    { value: "after", label: "Depois de" },
    { value: "before", label: "Antes de" },
    { value: "between", label: "Entre" },
    { value: "last_hours", label: "Últimas X horas" },
    { value: "last_days", label: "Últimos X dias" },
  ],
};

export function ElasticsearchSearchAdvanced() {
  const [filters, setFilters] = useState<SearchFilter[]>([]);
  const [newFilter, setNewFilter] = useState({
    field: "",
    operator: "",
    value: "",
    value2: "", // For "between" operations
  });
  const [searchResults, setSearchResults] = useState<any[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);

  const addFilter = () => {
    if (newFilter.field && newFilter.operator && newFilter.value) {
      const filter: SearchFilter = {
        id: Date.now().toString(),
        field: newFilter.field,
        operator: newFilter.operator,
        value: newFilter.value,
        type:
          fieldTypes[newFilter.field as keyof typeof fieldTypes] || "string",
      };

      // For "between" operations, combine values
      if (newFilter.operator === "between" && newFilter.value2) {
        filter.value = `${newFilter.value}|${newFilter.value2}`;
      }

      setFilters([...filters, filter]);
      setNewFilter({ field: "", operator: "", value: "", value2: "" });
    }
  };

  const removeFilter = (id: string) => {
    setFilters(filters.filter((filter) => filter.id !== id));
  };

  const performSearch = async () => {
    setIsSearching(true);
    setHasSearched(true);

    // Simulate search with filters
    setTimeout(() => {
      const mockResults = [
        {
          id: 1,
          timestamp: "2024-01-15T14:30:25Z",
          level: "ERROR",
          message: "Database connection failed",
          user: "admin",
          service: "auth-service",
          ip: "192.168.1.100",
          status_code: 500,
          duration: 1250,
        },
        {
          id: 2,
          timestamp: "2024-01-15T14:28:15Z",
          level: "INFO",
          message: "User login successful",
          user: "john.doe",
          service: "auth-service",
          ip: "192.168.1.101",
          status_code: 200,
          duration: 85,
        },
        {
          id: 3,
          timestamp: "2024-01-15T14:25:10Z",
          level: "WARN",
          message: "High memory usage detected",
          user: "system",
          service: "monitoring",
          ip: "192.168.1.102",
          status_code: 200,
          duration: 45,
        },
      ];
      setSearchResults(mockResults);
      setIsSearching(false);
    }, 2000);
  };

  const exportResults = () => {
    const dataStr = JSON.stringify(searchResults, null, 2);
    const dataUri =
      "data:application/json;charset=utf-8," + encodeURIComponent(dataStr);
    const exportFileDefaultName = `elasticsearch-results-${new Date().toISOString().split("T")[0]}.json`;

    const linkElement = document.createElement("a");
    linkElement.setAttribute("href", dataUri);
    linkElement.setAttribute("download", exportFileDefaultName);
    linkElement.click();
  };

  const getFieldType = (field: string): "string" | "number" | "date" => {
    return fieldTypes[field as keyof typeof fieldTypes] || "string";
  };

  const getOperatorLabel = (type: string, operator: string) => {
    const ops = operators[type as keyof typeof operators] || [];
    return ops.find((op) => op.value === operator)?.label || operator;
  };

  const formatFilterValue = (filter: SearchFilter) => {
    if (filter.operator === "between") {
      const [val1, val2] = filter.value.split("|");
      return `${val1} - ${val2}`;
    }
    return filter.value;
  };

  const getLevelBadge = (level: string) => {
    switch (level) {
      case "ERROR":
        return <Badge variant="destructive">ERROR</Badge>;
      case "WARN":
        return (
          <Badge variant="secondary" className="bg-yellow-100 text-yellow-800">
            WARN
          </Badge>
        );
      case "INFO":
        return (
          <Badge variant="default" className="bg-blue-100 text-blue-800">
            INFO
          </Badge>
        );
      default:
        return <Badge variant="outline">{level}</Badge>;
    }
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
            Configure filtros avançados com diferentes tipos de campo e
            operações
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Add Filters Section */}
          <div className="space-y-4">
            <div className="flex items-center space-x-2">
              <Filter className="h-4 w-4 text-blue-600" />
              <h3 className="font-medium">Adicionar Filtros Avançados</h3>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
              <div className="space-y-2">
                <Label>Campo</Label>
                <Select
                  value={newFilter.field}
                  onValueChange={(value) =>
                    setNewFilter({ ...newFilter, field: value, operator: "" })
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Selecione o campo" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="message">Mensagem</SelectItem>
                    <SelectItem value="level">Level</SelectItem>
                    <SelectItem value="user">Usuário</SelectItem>
                    <SelectItem value="service">Serviço</SelectItem>
                    <SelectItem value="ip">IP</SelectItem>
                    <SelectItem value="status">Status</SelectItem>
                    <SelectItem value="timestamp">Data/Hora</SelectItem>
                    <SelectItem value="duration">Duração (ms)</SelectItem>
                    <SelectItem value="response_time">
                      Tempo Resposta
                    </SelectItem>
                    <SelectItem value="status_code">Código Status</SelectItem>
                    <SelectItem value="bytes">Bytes</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Operação</Label>
                <Select
                  value={newFilter.operator}
                  onValueChange={(value) =>
                    setNewFilter({ ...newFilter, operator: value })
                  }
                  disabled={!newFilter.field}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Selecione a operação" />
                  </SelectTrigger>
                  <SelectContent>
                    {newFilter.field &&
                      operators[getFieldType(newFilter.field)].map((op) => (
                        <SelectItem key={op.value} value={op.value}>
                          {op.label}
                        </SelectItem>
                      ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Valor</Label>
                {getFieldType(newFilter.field) === "date" ? (
                  <Input
                    type={
                      newFilter.operator?.includes("last_")
                        ? "number"
                        : "datetime-local"
                    }
                    value={newFilter.value}
                    onChange={(e) =>
                      setNewFilter({ ...newFilter, value: e.target.value })
                    }
                    placeholder={
                      newFilter.operator?.includes("last_")
                        ? "Número"
                        : "Data/Hora"
                    }
                  />
                ) : (
                  <Input
                    type={
                      getFieldType(newFilter.field) === "number"
                        ? "number"
                        : "text"
                    }
                    value={newFilter.value}
                    onChange={(e) =>
                      setNewFilter({ ...newFilter, value: e.target.value })
                    }
                    placeholder="Digite o valor"
                  />
                )}
              </div>

              {newFilter.operator === "between" && (
                <div className="space-y-2">
                  <Label>Valor Final</Label>
                  <Input
                    type={
                      getFieldType(newFilter.field) === "number"
                        ? "number"
                        : getFieldType(newFilter.field) === "date"
                          ? "datetime-local"
                          : "text"
                    }
                    value={newFilter.value2}
                    onChange={(e) =>
                      setNewFilter({ ...newFilter, value2: e.target.value })
                    }
                    placeholder="Valor final"
                  />
                </div>
              )}

              <div className="flex items-end">
                <Button
                  onClick={addFilter}
                  disabled={
                    !newFilter.field ||
                    !newFilter.operator ||
                    !newFilter.value ||
                    (newFilter.operator === "between" && !newFilter.value2)
                  }
                  className="bg-gradient-to-r from-blue-600 to-indigo-600"
                >
                  <Plus className="h-4 w-4 mr-2" />
                  Adicionar
                </Button>
              </div>
            </div>
          </div>

          {/* Active Filters */}
          {filters.length > 0 && (
            <div className="space-y-2">
              <h4 className="font-medium text-sm">
                Filtros Ativos ({filters.length})
              </h4>
              <div className="flex flex-wrap gap-2">
                {filters.map((filter) => (
                  <Badge
                    key={filter.id}
                    variant="secondary"
                    className="flex items-center space-x-2 p-2"
                  >
                    <div className="flex items-center space-x-1">
                      <span className="font-medium">{filter.field}</span>
                      <span className="text-xs opacity-75">
                        {getOperatorLabel(filter.type, filter.operator)}
                      </span>
                      <span className="font-mono text-xs">
                        {formatFilterValue(filter)}
                      </span>
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      className="h-4 w-4 p-0 hover:bg-transparent"
                      onClick={() => removeFilter(filter.id)}
                    >
                      <X className="h-3 w-3" />
                    </Button>
                  </Badge>
                ))}
              </div>
            </div>
          )}

          <Separator />

          {/* Search Button */}
          <div className="flex items-center space-x-4">
            <Button
              onClick={performSearch}
              disabled={isSearching}
              className="flex items-center space-x-2 bg-gradient-to-r from-blue-600 to-indigo-600"
            >
              <Search className="h-4 w-4" />
              <span>{isSearching ? "Buscando..." : "Executar Busca"}</span>
            </Button>

            {searchResults.length > 0 && (
              <Button variant="outline" onClick={exportResults}>
                <Download className="h-4 w-4 mr-2" />
                Exportar Resultados
              </Button>
            )}

            {filters.length > 0 && (
              <Button variant="outline" onClick={() => setFilters([])}>
                <X className="h-4 w-4 mr-2" />
                Limpar Filtros
              </Button>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Search Results */}
      {hasSearched && (
        <Card>
          <CardHeader>
            <CardTitle>Resultados da Busca</CardTitle>
            <CardDescription>
              {isSearching
                ? "Executando busca com filtros..."
                : `${searchResults.length} logs encontrados`}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {isSearching ? (
              <div className="flex items-center justify-center py-8">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
              </div>
            ) : searchResults.length > 0 ? (
              <div className="space-y-4">
                <div className="border rounded-lg">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>Timestamp</TableHead>
                        <TableHead>Level</TableHead>
                        <TableHead>Usuário</TableHead>
                        <TableHead>Serviço</TableHead>
                        <TableHead>Status</TableHead>
                        <TableHead>Duração</TableHead>
                        <TableHead>Mensagem</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {searchResults.map((result) => (
                        <TableRow key={result.id}>
                          <TableCell className="font-mono text-sm">
                            {new Date(result.timestamp).toLocaleString()}
                          </TableCell>
                          <TableCell>{getLevelBadge(result.level)}</TableCell>
                          <TableCell>{result.user}</TableCell>
                          <TableCell>{result.service}</TableCell>
                          <TableCell>
                            <Badge
                              variant={
                                result.status_code >= 400
                                  ? "destructive"
                                  : "default"
                              }
                            >
                              {result.status_code}
                            </Badge>
                          </TableCell>
                          <TableCell className="font-mono text-sm">
                            {result.duration}ms
                          </TableCell>
                          <TableCell className="max-w-xs truncate">
                            {result.message}
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>

                <div className="flex items-center justify-between">
                  <p className="text-sm text-muted-foreground">
                    Mostrando {searchResults.length} resultados | Filtros
                    aplicados: {filters.length}
                  </p>
                  <p className="text-sm text-muted-foreground">
                    Tempo de busca: 0.45s
                  </p>
                </div>
              </div>
            ) : (
              <div className="text-center py-8">
                <Search className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
                <p className="text-muted-foreground">
                  Nenhum resultado encontrado
                </p>
                <p className="text-sm text-muted-foreground mt-2">
                  Tente ajustar os filtros ou verificar a conexão com o
                  Elasticsearch
                </p>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Search Tips */}
      <Card>
        <CardHeader>
          <CardTitle>Guia de Busca Avançada</CardTitle>
          <CardDescription>
            Como usar efetivamente os filtros por tipo de campo
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div>
              <h4 className="font-medium mb-3 flex items-center space-x-2">
                <Badge variant="outline">String</Badge>
                <span>Campos de Texto</span>
              </h4>
              <ul className="text-sm text-muted-foreground space-y-1">
                <li>
                  • <strong>Igual a:</strong> Correspondência exata
                </li>
                <li>
                  • <strong>Contém:</strong> Busca substring
                </li>
                <li>
                  • <strong>Começa com:</strong> Prefixo
                </li>
                <li>
                  • <strong>Termina com:</strong> Sufixo
                </li>
                <li>
                  • <strong>Regex:</strong> Expressão regular
                </li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium mb-3 flex items-center space-x-2">
                <Badge variant="outline">Number</Badge>
                <span>Campos Numéricos</span>
              </h4>
              <ul className="text-sm text-muted-foreground space-y-1">
                <li>
                  • <strong>Igual a:</strong> Valor exato
                </li>
                <li>
                  • <strong>Maior que:</strong> Valor superior
                </li>
                <li>
                  • <strong>Menor que:</strong> Valor inferior
                </li>
                <li>
                  • <strong>Entre:</strong> Faixa de valores
                </li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium mb-3 flex items-center space-x-2">
                <Badge variant="outline">Date</Badge>
                <span>Campos de Data</span>
              </h4>
              <ul className="text-sm text-muted-foreground space-y-1">
                <li>
                  • <strong>Depois de:</strong> Data posterior
                </li>
                <li>
                  • <strong>Antes de:</strong> Data anterior
                </li>
                <li>
                  • <strong>Entre:</strong> Período específico
                </li>
                <li>
                  • <strong>Últimas X horas/dias:</strong> Período relativo
                </li>
              </ul>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
