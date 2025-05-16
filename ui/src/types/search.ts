export interface SearchParams {
  ipAddress?: string;
  dateFrom?: Date;
  dateTo?: Date;
}

export interface ElasticsearchDocument {
  id: string;
  index: string;
  timestamp: string;
  source: Record<string, any>;
}

export interface VerificationResult {
  documentId: string;
  hashElastic: string;
  hashEthereum: string;
  isIntact: boolean;
  verifiedAt: string;
}
