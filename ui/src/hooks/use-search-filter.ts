import { useState } from "react";
import type { SearchFilter, NewFilter } from "../types/search";

export const useSearchFilters = () => {
  const [filters, setFilters] = useState<SearchFilter[]>([]);
  const [newFilter, setNewFilter] = useState<NewFilter>({
    field: "",
    type: "",
    operator: "",
    value: "",
    value2: "",
  });

  const addFilter = () => {
    if (
      newFilter.field &&
      newFilter.type &&
      newFilter.operator &&
      newFilter.value
    ) {
      const filter: SearchFilter = {
        id: Date.now().toString(),
        field: newFilter.field,
        operator: newFilter.operator,
        value: newFilter.value,
        type: newFilter.type,
      };

      // For "between" operations, combine values
      if (newFilter.operator === "between" && newFilter.value2) {
        filter.value = `${newFilter.value}|${newFilter.value2}`;
      }

      setFilters([...filters, filter]);
      setNewFilter({
        field: "",
        type: "",
        operator: "",
        value: "",
        value2: "",
      });
    }
  };

  const removeFilter = (id: string) => {
    setFilters(filters.filter((filter) => filter.id !== id));
  };

  const clearFilters = () => {
    setFilters([]);
  };

  return {
    filters,
    newFilter,
    setNewFilter,
    addFilter,
    removeFilter,
    clearFilters,
  };
};
