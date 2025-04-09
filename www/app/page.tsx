import { SearchInterface } from "@/components/search-interface";

export default function Home() {
  return (
    <main className="container mx-auto p-4 max-w-4xl">
      <h1 className="text-3xl font-bold mb-6">Elasticsearch</h1>
      <SearchInterface />
    </main>
  );
}
