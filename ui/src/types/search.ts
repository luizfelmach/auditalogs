export interface ElasticsearchDocument {
  id: string;
  index: string;
  source: Record<string, any>;
}

export interface VerificationResult {
  documentId: string;
  hashElastic: string;
  hashEthereum: string | null;
  isIntact: boolean;
  verifiedAt: string;
}

export interface SearchFilter {
  id: string;
  field: string;
  operator: string;
  value: string;
  type: "string" | "int" | "date";
}

export interface NewFilter {
  field: string;
  type: "string" | "int" | "date" | "";
  operator: string;
  value: string;
  value2: string;
}
