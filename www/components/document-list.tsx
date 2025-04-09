import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardFooter,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { RefreshCw } from "lucide-react";

type Document = {
  id: string;
  index: string;
  source: Record<string, any>;
};

type DocumentListProps = {
  documents: Document[];
  onCheckIntegrity: (index: string) => void;
};

export function DocumentList({
  documents,
  onCheckIntegrity,
}: DocumentListProps) {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {documents.map((doc) => (
        <Card key={doc.id} className="overflow-hidden flex flex-col">
          <CardHeader className="bg-muted h-12">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg">ID: {doc.id}</CardTitle>
            </div>
          </CardHeader>
          <CardContent className="p-4 flex-grow">
            <div className="bg-muted rounded-md p-2 overflow-auto max-h-40 mb-2">
              <pre className="text-xs">
                {JSON.stringify(doc.source, null, 2)}
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
              onClick={() => onCheckIntegrity(doc.index)}
            >
              <RefreshCw className="w-4 h-4 mr-2" />
              Check Integrity
            </Button>
          </CardFooter>
        </Card>
      ))}
    </div>
  );
}
