import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  ChevronDown,
  ChevronUp,
  FileText,
  ShieldCheck,
  Clock,
  Database,
} from "lucide-react";
import { format } from "date-fns";
import { useMutation } from "@tanstack/react-query";
import { verifyDocumentIntegrity } from "@/lib/api";
import type { ElasticsearchDocument } from "@/types/search";
import { toast } from "sonner";

interface DocumentItemProps {
  document: ElasticsearchDocument;
}

export function DocumentItem({ document }: DocumentItemProps) {
  const [isContentVisible, setIsContentVisible] = useState(false);

  const { mutate: verifyIntegrity, isPending } = useMutation({
    mutationFn: () => verifyDocumentIntegrity(document.index),
    onSuccess: (result) => {
      if (result.isIntact) {
        toast("Document Integrity Verified", {
          description: "The document is intact and has not been modified.",
        });
      } else {
        toast("Document Integrity Issue", {
          description: "The document may have been modified or corrupted.",
        });
      }
    },
    onError: () => {
      toast("Verification Failed", {
        description: "Unable to verify document integrity. Please try again.",
      });
    },
  });

  return (
    <Card className="overflow-hidden border-slate-200 hover:border-[#00BFB3]/30 transition-colors">
      <CardContent className="p-0">
        <div className="p-4 border-b border-slate-100">
          <div className="flex justify-between items-start">
            <div>
              <h3 className="text-lg font-medium text-slate-800 flex items-center gap-2">
                <Database className="h-4 w-4 text-[#00BFB3]" />
                {document.id}
              </h3>
              <div className="flex flex-wrap items-center gap-2 mt-2">
                <Badge
                  variant="outline"
                  className="bg-[#00BFB3]/10 text-[#00BFB3] border-[#00BFB3]/20"
                >
                  {document.index}
                </Badge>
                <div className="flex items-center text-sm text-slate-500">
                  <Clock className="h-3.5 w-3.5 mr-1" />
                  {document.timestamp
                    ? format(new Date(document.timestamp), "PPP 'at' p")
                    : "No date"}
                </div>
              </div>
            </div>
            <Button
              variant="outline"
              size="sm"
              onClick={() => verifyIntegrity()}
              disabled={isPending}
              className="border-[#00BFB3]/30 text-[#00BFB3] hover:bg-[#00BFB3]/10 hover:text-[#00BFB3]"
            >
              <ShieldCheck className="h-4 w-4 mr-2" />
              Verify Integrity
            </Button>
          </div>
        </div>

        <div className="border-b border-slate-100">
          <button
            onClick={() => setIsContentVisible(!isContentVisible)}
            className="w-full flex items-center justify-between p-3 text-sm hover:bg-slate-50 transition-colors"
          >
            <div className="flex items-center gap-2 text-slate-600">
              <FileText className="h-4 w-4 text-[#00BFB3]" />
              <span>Document Raw Content</span>
            </div>
            {isContentVisible ? (
              <ChevronUp className="h-4 w-4 text-slate-400" />
            ) : (
              <ChevronDown className="h-4 w-4 text-slate-400" />
            )}
          </button>
        </div>

        {isContentVisible && (
          <div className="p-4 bg-slate-50">
            <pre className="bg-white border border-slate-200 p-4 rounded-md text-sm overflow-auto max-h-96 text-slate-800">
              {JSON.stringify(document.source, null, 2)}
            </pre>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
