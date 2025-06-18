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
import {
  Search,
  Plus,
  X,
  Filter,
  Download,
  Shield,
  CheckCircle,
  AlertCircle,
} from "lucide-react";

export interface ElasticsearchDocument {
  id: string;
  index: string;
  timestamp: string;
  source: Record<string, any>;
}

export interface VerificationResult {
  documentId: string;
  hashElastic: string;
  hashEthereum: string | null;
  isIntact: boolean;
  verifiedAt: string;
}

interface SearchFilter {
  id: string;
  field: string;
  operator: string;
  value: string;
  type: "string" | "int" | "date";
}

const operators = {
  string: [
    { value: "equals", label: "Igual a" },
    { value: "contains", label: "Contém" },
    { value: "starts_with", label: "Começa com" },
    { value: "ends_with", label: "Termina com" },
    { value: "not_equals", label: "Diferente de" },
    { value: "regex", label: "Regex" },
  ],
  int: [
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
    type: "" as "string" | "int" | "date" | "",
    operator: "",
    value: "",
    value2: "",
  });
  const [searchResults, setSearchResults] = useState<ElasticsearchDocument[]>(
    [],
  );
  const [verificationResults, setVerificationResults] = useState<
    VerificationResult[]
  >([]);
  const [isSearching, setIsSearching] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);

  const addFilter = () => {
    if (
      newFilter.field &&
      newFilter.type &&
      newFilter.operator &&
      newFilter.value
    ) {
      const filter: SearchFilter = {
        id: Date.now().toString(),
        field: newFilter.field,
        operator: newFilter.operator,
        value: newFilter.value,
        type: newFilter.type,
      };

      // For "between" operations, combine values
      if (newFilter.operator === "between" && newFilter.value2) {
        filter.value = `${newFilter.value}|${newFilter.value2}`;
      }

      setFilters([...filters, filter]);
      setNewFilter({
        field: "",
        type: "",
        operator: "",
        value: "",
        value2: "",
      });
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
      const mockResults: ElasticsearchDocument[] = [
        {
          id: "doc_001",
          index: "logs-2024.01",
          timestamp: "2024-01-15T14:30:25Z",
          source: {
            event_type: "error",
            message: "Database connection failed",
            user_id: "admin_001",
            application: "auth-service",
            client_ip: "192.168.1.100",
            response_code: 500,
            processing_time_ms: 1250,
            environment: "production",
          },
        },
        {
          id: "doc_002",
          index: "user-activity-2024.01",
          timestamp: "2024-01-15T14:28:15Z",
          source: {
            action: "login",
            username: "john.doe",
            success: true,
            session_id: "sess_abc123",
            device_type: "mobile",
            location: "New York",
            duration_seconds: 85,
          },
        },
        {
          id: "doc_003",
          index: "system-metrics-2024.01",
          timestamp: "2024-01-15T14:25:10Z",
          source: {
            metric_name: "memory_usage",
            value: 87.5,
            unit: "percentage",
            host: "server-01",
            datacenter: "us-east-1",
            alert_threshold: 85.0,
            status: "warning",
          },
        },
      ];
      setSearchResults(mockResults);
      setIsSearching(false);
    }, 2000);
  };

  const verifyIntegrity = async () => {
    if (searchResults.length === 0) return;

    setIsVerifying(true);

    // Simulate verification process
    setTimeout(() => {
      const mockVerificationResults: VerificationResult[] = searchResults.map(
        (doc, index) => ({
          documentId: doc.id,
          hashElastic: `0x${Math.random().toString(16).substr(2, 64)}`,
          hashEthereum:
            Math.random() > 0.4
              ? `0x${Math.random().toString(16).substr(2, 64)}`
              : null, // 60% chance of having Ethereum hash
          isIntact: Math.random() > 0.3, // 70% chance of being intact
          verifiedAt: new Date().toISOString(),
        }),
      );

      setVerificationResults(mockVerificationResults);
      setIsVerifying(false);
    }, 3000);
  };

  const exportResults = () => {
    const exportData = {
      searchResults,
      verificationResults,
      filters,
      exportedAt: new Date().toISOString(),
    };

    const dataStr = JSON.stringify(exportData, null, 2);
    const dataUri =
      "data:application/json;charset=utf-8," + encodeURIComponent(dataStr);
    const exportFileDefaultName = `elasticsearch-results-${new Date().toISOString().split("T")[0]}.json`;

    const linkElement = document.createElement("a");
    linkElement.setAttribute("href", dataUri);
    linkElement.setAttribute("download", exportFileDefaultName);
    linkElement.click();
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

  const getVerificationBadge = (result: VerificationResult) => {
    if (!result.hashEthereum) {
      return (
        <Badge variant="outline" className="bg-gray-100 text-gray-600">
          <AlertCircle className="h-3 w-3 mr-1" />
          Não Registrado
        </Badge>
      );
    }

    return result.isIntact ? (
      <Badge variant="default" className="bg-green-100 text-green-800">
        <CheckCircle className="h-3 w-3 mr-1" />
        Íntegro
      </Badge>
    ) : (
      <Badge variant="destructive">
        <AlertCircle className="h-3 w-3 mr-1" />
        Comprometido
      </Badge>
    );
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
          {/* Add Filters Section */}
          <div className="space-y-4">
            <div className="flex items-center space-x-2">
              <Filter className="h-4 w-4 text-blue-600" />
              <h3 className="font-medium">Adicionar Filtros Personalizados</h3>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-6 gap-4">
              <div className="space-y-2">
                <Label>Chave do Campo</Label>
                <Input
                  value={newFilter.field}
                  onChange={(e) =>
                    setNewFilter({ ...newFilter, field: e.target.value })
                  }
                  placeholder="Ex: message, user_id, timestamp"
                />
              </div>

              <div className="space-y-2">
                <Label>Tipo do Campo</Label>
                <Select
                  value={newFilter.type}
                  onValueChange={(value: "string" | "int" | "date") =>
                    setNewFilter({ ...newFilter, type: value, operator: "" })
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Selecione o tipo" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="string">String</SelectItem>
                    <SelectItem value="int">Integer</SelectItem>
                    <SelectItem value="date">Date</SelectItem>
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
                  disabled={!newFilter.type}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Selecione a operação" />
                  </SelectTrigger>
                  <SelectContent>
                    {newFilter.type &&
                      operators[newFilter.type].map((op) => (
                        <SelectItem key={op.value} value={op.value}>
                          {op.label}
                        </SelectItem>
                      ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Valor</Label>
                {newFilter.type === "date" ? (
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
                    type={newFilter.type === "int" ? "number" : "text"}
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
                      newFilter.type === "int"
                        ? "number"
                        : newFilter.type === "date"
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
                    !newFilter.type ||
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
                      <Badge variant="outline" className="text-xs">
                        {filter.type}
                      </Badge>
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
              <>
                <Button
                  onClick={verifyIntegrity}
                  disabled={isVerifying}
                  className="flex items-center space-x-2 bg-gradient-to-r from-green-600 to-emerald-600"
                >
                  <Shield className="h-4 w-4" />
                  <span>
                    {isVerifying ? "Verificando..." : "Verificar Integridade"}
                  </span>
                </Button>

                <Button variant="outline" onClick={exportResults}>
                  <Download className="h-4 w-4 mr-2" />
                  Exportar Resultados
                </Button>
              </>
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
            <CardTitle>Documentos Encontrados</CardTitle>
            <CardDescription>
              {isSearching
                ? "Executando busca com filtros..."
                : `${searchResults.length} documentos encontrados`}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {isSearching ? (
              <div className="flex items-center justify-center py-8">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
              </div>
            ) : searchResults.length > 0 ? (
              <div className="space-y-4">
                {searchResults.map((doc) => (
                  <Card key={doc.id} className="border-l-4 border-l-blue-500">
                    <CardHeader className="pb-3">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center space-x-4">
                          <Badge
                            variant="outline"
                            className="font-mono text-xs"
                          >
                            {doc.id}
                          </Badge>
                          <Badge variant="secondary" className="text-xs">
                            {doc.index}
                          </Badge>
                          <span className="text-sm text-muted-foreground font-mono">
                            {new Date(doc.timestamp).toLocaleString()}
                          </span>
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
                                <Badge
                                  variant={value ? "default" : "secondary"}
                                >
                                  {value ? "True" : "False"}
                                </Badge>
                              ) : typeof value === "number" ? (
                                <span className="font-mono">{value}</span>
                              ) : typeof value === "string" &&
                                value.length > 50 ? (
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

                      {/* Raw JSON View Toggle */}
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
                    Mostrando {searchResults.length} documentos | Filtros
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

      {/* Verification Results */}
      {verificationResults.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Shield className="h-5 w-5 text-green-600" />
              <span>Resultados da Verificação de Integridade</span>
            </CardTitle>
            <CardDescription>
              {isVerifying
                ? "Verificando integridade dos documentos..."
                : `${verificationResults.length} documentos verificados`}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {isVerifying ? (
              <div className="flex items-center justify-center py-8">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
              </div>
            ) : (
              <div className="space-y-4">
                <div className="border rounded-lg">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>ID do Documento</TableHead>
                        <TableHead>Hash Elasticsearch</TableHead>
                        <TableHead>Hash Ethereum</TableHead>
                        <TableHead>Status</TableHead>
                        <TableHead>Verificado em</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {verificationResults.map((result) => (
                        <TableRow key={result.documentId}>
                          <TableCell className="font-mono text-sm">
                            {result.documentId}
                          </TableCell>
                          <TableCell className="font-mono text-xs max-w-xs truncate">
                            {result.hashElastic}
                          </TableCell>
                          <TableCell className="font-mono text-xs max-w-xs truncate">
                            {result.hashEthereum || (
                              <span className="text-muted-foreground italic">
                                Não registrado
                              </span>
                            )}
                          </TableCell>
                          <TableCell>{getVerificationBadge(result)}</TableCell>
                          <TableCell className="font-mono text-sm">
                            {new Date(result.verifiedAt).toLocaleString()}
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>

                <div className="flex items-center justify-between">
                  <div className="flex space-x-4">
                    <p className="text-sm text-green-600">
                      ✓{" "}
                      {
                        verificationResults.filter(
                          (r) => r.hashEthereum && r.isIntact,
                        ).length
                      }{" "}
                      documentos íntegros
                    </p>
                    <p className="text-sm text-red-600">
                      ✗{" "}
                      {
                        verificationResults.filter(
                          (r) => r.hashEthereum && !r.isIntact,
                        ).length
                      }{" "}
                      documentos comprometidos
                    </p>
                    <p className="text-sm text-gray-600">
                      ⚪{" "}
                      {
                        verificationResults.filter((r) => !r.hashEthereum)
                          .length
                      }{" "}
                      não registrados na blockchain
                    </p>
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Tempo de verificação: 2.8s
                  </p>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Usage Guide */}
      <Card>
        <CardHeader>
          <CardTitle>Guia de Uso - Filtros Personalizados</CardTitle>
          <CardDescription>
            Como criar filtros efetivos com chaves e tipos personalizados
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
                  • <strong>Chaves comuns:</strong> message, user, service
                </li>
                <li>
                  • <strong>Igual a:</strong> Correspondência exata
                </li>
                <li>
                  • <strong>Contém:</strong> Busca substring
                </li>
                <li>
                  • <strong>Regex:</strong> Expressão regular
                </li>
              </ul>
            </div>
            <div>
              <h4 className="font-medium mb-3 flex items-center space-x-2">
                <Badge variant="outline">Integer</Badge>
                <span>Campos Numéricos</span>
              </h4>
              <ul className="text-sm text-muted-foreground space-y-1">
                <li>
                  • <strong>Chaves comuns:</strong> status_code, duration, bytes
                </li>
                <li>
                  • <strong>Maior que:</strong> Valor superior
                </li>
                <li>
                  • <strong>Entre:</strong> Faixa de valores
                </li>
                <li>
                  • <strong>Igual a:</strong> Valor exato
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
                  • <strong>Chaves comuns:</strong> timestamp, created_at
                </li>
                <li>
                  • <strong>Depois de:</strong> Data posterior
                </li>
                <li>
                  • <strong>Últimas X horas:</strong> Período relativo
                </li>
                <li>
                  • <strong>Entre:</strong> Período específico
                </li>
              </ul>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
