import { useQuery } from "@tanstack/react-query";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { DocumentItem } from "@/components/document-item";
import { Skeleton } from "@/components/ui/skeleton";
import { AlertCircle } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { searchDocuments } from "@/lib/api";
import type { SearchParams } from "@/types/search";
import { Badge } from "@/components/ui/badge";
import { FileText } from "lucide-react";

interface SearchResultsProps {
  searchParams: SearchParams;
}

export function SearchResults({ searchParams }: SearchResultsProps) {
  const { data, isLoading, error } = useQuery({
    queryKey: ["documents", searchParams],
    queryFn: () => searchDocuments(searchParams),
  });

  if (isLoading) {
    return (
      <Card className="border-[#00BFB3]/20 shadow-md">
        <CardHeader className="border-b border-slate-100">
          <CardTitle className="text-xl font-semibold text-slate-800">
            <Skeleton className="h-6 w-48" />
          </CardTitle>
        </CardHeader>
        <CardContent className="pt-6">
          <div className="space-y-6">
            {Array.from({ length: 3 }).map((_, i) => (
              <div key={i} className="border rounded-lg p-4 space-y-4">
                <div className="flex justify-between items-start">
                  <div>
                    <Skeleton className="h-5 w-40 mb-2" />
                    <Skeleton className="h-4 w-24" />
                  </div>
                  <Skeleton className="h-9 w-32" />
                </div>
                <Skeleton className="h-24 w-full mt-2" />
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Alert variant="destructive">
        <AlertCircle className="h-4 w-4" />
        <AlertTitle>Error</AlertTitle>
        <AlertDescription>
          Failed to fetch documents. Please try again later.
        </AlertDescription>
      </Alert>
    );
  }

  return (
    <Card className="border-[#00BFB3]/20 shadow-md">
      <CardHeader className="border-b border-slate-100">
        <CardTitle className="text-xl font-semibold text-slate-800">
          Search Results
          <Badge
            variant="outline"
            className="ml-2 bg-[#00BFB3]/10 text-[#00BFB3] border-[#00BFB3]/20"
          >
            {data?.length || 0} documents found
          </Badge>
        </CardTitle>
      </CardHeader>
      <CardContent className="pt-6">
        {data?.length === 0 ? (
          <div className="text-center py-12 text-muted-foreground">
            <div className="mx-auto w-16 h-16 mb-4 rounded-full bg-slate-100 flex items-center justify-center">
              <FileText className="h-8 w-8 text-slate-400" />
            </div>
            <p className="text-lg font-medium text-slate-600">
              No documents found matching your search criteria.
            </p>
            <p className="text-sm text-slate-500 mt-1">
              Try adjusting your search parameters.
            </p>
          </div>
        ) : (
          <div className="space-y-6">
            {data?.map((document) => (
              <DocumentItem key={document.id} document={document} />
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
