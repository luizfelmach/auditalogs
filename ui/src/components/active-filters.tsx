import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { X } from "lucide-react";
import type { SearchFilter } from "../types/search";
import { getOperatorLabel, formatFilterValue } from "../lib/helpers";

interface ActiveFiltersProps {
  filters: SearchFilter[];
  onRemoveFilter: (id: string) => void;
}

export function ActiveFilters({ filters, onRemoveFilter }: ActiveFiltersProps) {
  if (filters.length === 0) return null;

  return (
    <div className="space-y-2">
      <h4 className="font-medium text-sm">Filtros Ativos ({filters.length})</h4>
      <div className="flex flex-wrap gap-2">
        {filters.map((filter) => (
          <Badge
            key={filter.id}
            variant="secondary"
            className="flex items-center space-x-2 p-2"
          >
            <div className="flex items-center space-x-1">
              <Badge variant="outline" className="text-xs">
                {filter.type}
              </Badge>
              <span className="font-medium">{filter.field}</span>
              <span className="text-xs opacity-75">
                {getOperatorLabel(filter.type, filter.operator)}
              </span>
              <span className="font-mono text-xs">
                {formatFilterValue(filter)}
              </span>
            </div>
            <Button
              variant="ghost"
              size="sm"
              className="h-4 w-4 p-0 hover:bg-transparent"
              onClick={() => onRemoveFilter(filter.id)}
            >
              <X className="h-3 w-3" />
            </Button>
          </Badge>
        ))}
      </div>
    </div>
  );
}
