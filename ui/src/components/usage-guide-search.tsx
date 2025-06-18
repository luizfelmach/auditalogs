import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export function UsageGuideSearch() {
  return (
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
  );
}
