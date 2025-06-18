import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Shield, CheckCircle, AlertCircle, Loader2 } from "lucide-react";
import type { VerificationResult } from "../types/search";

interface VerificationResultsProps {
  results: VerificationResult[];
  isVerifying: boolean;
}

function SkeletonHash() {
  return (
    <div className="flex items-center space-x-2">
      <Loader2 className="h-3 w-3 animate-spin text-muted-foreground" />
      <div className="h-4 bg-muted animate-pulse rounded w-32"></div>
    </div>
  );
}

function SkeletonBadge() {
  return (
    <div className="flex items-center space-x-2">
      <Loader2 className="h-3 w-3 animate-spin text-muted-foreground" />
      <div className="h-6 bg-muted animate-pulse rounded w-20"></div>
    </div>
  );
}

export function VerificationResults({
  results,
  isVerifying,
}: VerificationResultsProps) {
  if (results.length === 0) return null;

  const getVerificationBadge = (result: VerificationResult) => {
    if (result.hashEthereumLoading || result.hashElasticLoading) {
      return <SkeletonBadge />;
    }

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

  const completedResults = results.filter(
    (r) => !(r.hashElasticLoading || r.hashEthereumLoading),
  );
  const loadingCount = results.filter(
    (r) => r.hashEthereumLoading || r.hashElasticLoading,
  ).length;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Shield className="h-5 w-5 text-green-600" />
          <span>Resultados da Verificação de Integridade</span>
          {isVerifying && (
            <div className="flex items-center space-x-2 text-sm text-muted-foreground">
              <Loader2 className="h-4 w-4 animate-spin" />
              <span>Verificando...</span>
            </div>
          )}
        </CardTitle>
        <CardDescription>
          {isVerifying
            ? `Verificando ${results.length} documentos... (${completedResults.length}/${results.length} concluídos)`
            : `${results.length} documentos verificados`}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {/* Progress Bar */}
          {isVerifying && (
            <div className="space-y-2">
              <div className="flex justify-between text-sm text-muted-foreground">
                <span>Progresso da verificação</span>
                <span>
                  {completedResults.length}/{results.length}
                </span>
              </div>
              <div className="w-full bg-muted rounded-full h-2">
                <div
                  className="bg-green-600 h-2 rounded-full transition-all duration-500 ease-out"
                  style={{
                    width: `${(completedResults.length / results.length) * 100}%`,
                  }}
                ></div>
              </div>
            </div>
          )}

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
                {results.map((result) => (
                  <TableRow
                    key={result.documentId}
                    className={
                      result.hashElasticLoading || result.hashEthereumLoading
                        ? "opacity-75"
                        : ""
                    }
                  >
                    <TableCell className="font-mono text-sm">
                      {result.documentId}
                    </TableCell>
                    <TableCell className="font-mono text-xs max-w-xs truncate">
                      {result.hashElasticLoading ? (
                        <SkeletonHash />
                      ) : (
                        result.hashElastic
                      )}
                    </TableCell>
                    <TableCell className="font-mono text-xs max-w-xs truncate">
                      {result.hashEthereumLoading ? (
                        <SkeletonHash />
                      ) : result.hashEthereum ? (
                        result.hashEthereum
                      ) : (
                        <span className="text-muted-foreground italic">
                          Não registrado
                        </span>
                      )}
                    </TableCell>
                    <TableCell>{getVerificationBadge(result)}</TableCell>
                    <TableCell className="font-mono text-sm">
                      {result.hashElasticLoading ||
                      result.hashEthereumLoading ? (
                        <div className="h-4 bg-muted animate-pulse rounded w-24"></div>
                      ) : (
                        new Date(result.verifiedAt!).toLocaleString()
                      )}
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
                  completedResults.filter((r) => r.hashEthereum && r.isIntact)
                    .length
                }{" "}
                documentos íntegros
              </p>
              <p className="text-sm text-red-600">
                ✗{" "}
                {
                  completedResults.filter((r) => r.hashEthereum && !r.isIntact)
                    .length
                }{" "}
                documentos comprometidos
              </p>
              <p className="text-sm text-gray-600">
                ⚪ {completedResults.filter((r) => !r.hashEthereum).length} não
                registrados na blockchain
              </p>
              {loadingCount > 0 && (
                <p className="text-sm text-blue-600">
                  <Loader2 className="h-3 w-3 animate-spin inline mr-1" />
                  {loadingCount} verificando...
                </p>
              )}
            </div>
            <p className="text-sm text-muted-foreground">
              {isVerifying
                ? "Verificação em andamento..."
                : "Verificação concluída"}
            </p>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
