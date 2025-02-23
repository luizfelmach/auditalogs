"use client";

import type React from "react";

import { useState } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Search } from "lucide-react";
import { DocumentList } from "@/components/document-list";

export function SearchInterface() {
  const [searchQuery, setSearchQuery] = useState("");
  const [documents, setDocuments] = useState([]);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    // Simulated search results with JSON content and index information
    const mockResults = [
      {
        id: 1,
        title: "Document 1",
        status: "indexed",
        integrity: true,
        content: {
          name: "John Doe",
          age: 30,
          email: "john@example.com",
        },
        index: "users",
      },
      {
        id: 2,
        title: "Document 2",
        status: "indexed",
        integrity: false,
        content: {
          product: "Laptop",
          price: 999.99,
          inStock: true,
        },
        index: "products",
      },
      {
        id: 3,
        title: "Document 3",
        status: "not_indexed",
        content: {
          city: "New York",
          population: 8419000,
          country: "USA",
        },
        index: "cities",
      },
      {
        id: 4,
        title: "Document 4",
        status: "indexed",
        integrity: true,
        content: {
          title: "The Great Gatsby",
          author: "F. Scott Fitzgerald",
          year: 1925,
        },
        index: "books",
      },
    ];
    setDocuments(mockResults);
  };

  const handleRecheckIntegrity = (id: number) => {
    // Simulated integrity recheck
    setDocuments((prevDocs) =>
      prevDocs.map((doc) =>
        doc.id === id ? { ...doc, integrity: Math.random() > 0.5 } : doc
      )
    );
  };

  return (
    <div className="space-y-6">
      <form onSubmit={handleSearch} className="flex gap-2">
        <Input
          type="search"
          placeholder="Search documents..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="flex-grow"
        />
        <Button type="submit">
          <Search className="mr-2 h-4 w-4" /> Search
        </Button>
      </form>
      <DocumentList
        documents={documents}
        onRecheckIntegrity={handleRecheckIntegrity}
      />
    </div>
  );
}
