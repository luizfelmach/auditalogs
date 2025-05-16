import type {
  SearchParams,
  ElasticsearchDocument,
  VerificationResult,
} from "@/types/search";

export async function searchDocuments(
  params: SearchParams,
): Promise<ElasticsearchDocument[]> {
  const url = `${import.meta.env.AUDITA_URL}/elastic/search`;

  const response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      ip: params.ipAddress,
      from: params.dateFrom,
      to: params.dateTo,
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to find documents: ${response.statusText}`);
  }

  const data = await response.json();

  return data.results.map(
    (item: any): ElasticsearchDocument => ({
      id: item._id,
      index: item._index,
      timestamp: item._source.timestamp,
      source: { ...item._source },
    }),
  );
}

export async function verifyDocumentIntegrity(
  index: string,
): Promise<VerificationResult> {
  const url1 = `${import.meta.env.AUDITA_URL}/elastic/${index}`;
  const url2 = `${import.meta.env.AUDITA_URL}/ethereum/${index}`;

  const response1 = await fetch(url1);
  const response2 = await fetch(url2);

  if (!response1.ok) {
    throw new Error(`Failed to check documents: ${response1.statusText}`);
  }
  if (!response2.ok) {
    throw new Error(`Failed to check documents: ${response2.statusText}`);
  }
  const data1 = await response1.json();
  const data2 = await response2.json();

  const isIntact = data1.hash === data2.hash;
  return {
    documentId: index,
    hashElastic: data1.hash,
    hashEthereum: data2.hash,
    isIntact,
    verifiedAt: new Date().toISOString(),
  };
}
