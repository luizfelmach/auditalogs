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
import { Shield, CheckCircle, AlertCircle } from "lucide-react";
import type { VerificationResult } from "../types/search";

interface VerificationResultsProps {
  results: VerificationResult[];
  isVerifying: boolean;
}

export function VerificationResults({
  results,
  isVerifying,
}: VerificationResultsProps) {
  if (results.length === 0 && !isVerifying) return null;

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
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Shield className="h-5 w-5 text-green-600" />
          <span>Resultados da Verificação de Integridade</span>
        </CardTitle>
        <CardDescription>
          {isVerifying
            ? "Verificando integridade dos documentos..."
            : `${results.length} documentos verificados`}
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
                  {results.map((result) => (
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
                  ✓ {results.filter((r) => r.hashEthereum && r.isIntact).length}{" "}
                  documentos íntegros
                </p>
                <p className="text-sm text-red-600">
                  ✗{" "}
                  {results.filter((r) => r.hashEthereum && !r.isIntact).length}{" "}
                  documentos comprometidos
                </p>
                <p className="text-sm text-gray-600">
                  ⚪ {results.filter((r) => !r.hashEthereum).length} não
                  registrados na blockchain
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
  );
}
