import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardFooter,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { RefreshCw } from "lucide-react";

type Document = {
  id: number;
  title: string;
  status: "indexed" | "not_indexed";
  integrity?: boolean;
  content: Record<string, any>;
  index: string;
};

type DocumentListProps = {
  documents: Document[];
  onRecheckIntegrity: (id: number) => void;
};

export function DocumentList({
  documents,
  onRecheckIntegrity,
}: DocumentListProps) {
  const getStatusColor = (status: string, integrity?: boolean) => {
    if (status === "not_indexed") return "bg-gray-500";
    if (integrity) return "bg-green-500";
    return "bg-red-500";
  };

  const getStatusText = (status: string, integrity?: boolean) => {
    if (status === "not_indexed") return "Not Indexed";
    if (integrity) return "Indexed (Integrity: Valid)";
    return "Indexed (Integrity: Invalid)";
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {documents.map((doc) => (
        <Card key={doc.id} className="overflow-hidden flex flex-col">
          <CardHeader className="bg-muted">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg">{doc.title}</CardTitle>
              <Badge className={getStatusColor(doc.status, doc.integrity)}>
                {getStatusText(doc.status, doc.integrity)}
              </Badge>
            </div>
          </CardHeader>
          <CardContent className="p-4 flex-grow">
            <div className="bg-muted rounded-md p-2 overflow-auto max-h-40 mb-2">
              <pre className="text-xs">
                {JSON.stringify(doc.content, null, 2)}
              </pre>
            </div>
            <div className="text-sm text-muted-foreground">
              Index: <span className="font-medium">{doc.index}</span>
            </div>
          </CardContent>
          <CardFooter className="bg-muted/50 p-2">
            <Button
              variant="outline"
              size="sm"
              className="w-full"
              onClick={() => onRecheckIntegrity(doc.id)}
              disabled={doc.status === "not_indexed"}
            >
              <RefreshCw className="w-4 h-4 mr-2" />
              Recheck Integrity
            </Button>
          </CardFooter>
        </Card>
      ))}
    </div>
  );
}
