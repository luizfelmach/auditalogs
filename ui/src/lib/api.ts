import type { ElasticsearchDocument } from "@/types/search";

const BASE_URL = import.meta.env.AUDITA_URL ?? "";

export async function searchDocuments(
  query: any,
): Promise<ElasticsearchDocument[]> {
  const url = `${BASE_URL}/elastic/search`;

  const response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(query),
  });

  if (!response.ok) {
    throw new Error(`Failed to find documents: ${response.statusText}`);
  }

  const data = await response.json();

  return data.results.map(
    (item: any): ElasticsearchDocument => ({
      id: item._id,
      index: item._index,
      source: { ...item._source },
    }),
  );
}

export async function retrieveElasticHash(
  index: string,
): Promise<string | null> {
  const url = `${BASE_URL}/elastic/${index}`;
  const response = await fetch(url);
  if (!response.ok) {
    return null;
  }
  const data = await response.json();
  if (!data.hash) return null;
  return data.hash;
}

export async function retrieveEthereumHash(
  index: string,
): Promise<string | null> {
  const url = `${BASE_URL}/ethereum/${index}`;
  const response = await fetch(url);
  if (!response.ok) {
    return null;
  }
  const data = await response.json();
  if (!data.hash) return null;
  return data.hash;
}
