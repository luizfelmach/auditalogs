import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Plus, Filter } from "lucide-react";
import { operators } from "../lib/operators";
import type { NewFilter } from "../types/search";

interface FilterFormProps {
  newFilter: NewFilter;
  setNewFilter: (filter: NewFilter) => void;
  onAddFilter: () => void;
}

export function FilterForm({
  newFilter,
  setNewFilter,
  onAddFilter,
}: FilterFormProps) {
  const isFormValid =
    newFilter.field &&
    newFilter.type &&
    newFilter.operator &&
    newFilter.value &&
    (newFilter.operator !== "between" || newFilter.value2);

  return (
    <div className="space-y-4">
      <div className="flex items-center space-x-2">
        <Filter className="h-4 w-4 text-blue-600" />
        <h3 className="font-medium">Adicionar Filtros Personalizados</h3>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-6 gap-4">
        <div className="space-y-2">
          <Label>Chave do Campo</Label>
          <Input
            value={newFilter.field}
            onChange={(e) =>
              setNewFilter({ ...newFilter, field: e.target.value })
            }
            placeholder="Ex: message, user_id, timestamp"
          />
        </div>

        <div className="space-y-2">
          <Label>Tipo do Campo</Label>
          <Select
            value={newFilter.type}
            onValueChange={(value: "string" | "int" | "date") =>
              setNewFilter({ ...newFilter, type: value, operator: "" })
            }
          >
            <SelectTrigger>
              <SelectValue placeholder="Selecione o tipo" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="string">String</SelectItem>
              <SelectItem value="int">Integer</SelectItem>
              <SelectItem value="date">Date</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label>Operação</Label>
          <Select
            value={newFilter.operator}
            onValueChange={(value) =>
              setNewFilter({ ...newFilter, operator: value })
            }
            disabled={!newFilter.type}
          >
            <SelectTrigger>
              <SelectValue placeholder="Selecione a operação" />
            </SelectTrigger>
            <SelectContent>
              {newFilter.type &&
                operators[newFilter.type].map((op) => (
                  <SelectItem key={op.value} value={op.value}>
                    {op.label}
                  </SelectItem>
                ))}
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label>Valor</Label>
          {newFilter.type === "date" ? (
            <Input
              type={
                newFilter.operator?.includes("last_")
                  ? "number"
                  : "datetime-local"
              }
              value={newFilter.value}
              onChange={(e) =>
                setNewFilter({ ...newFilter, value: e.target.value })
              }
              placeholder={
                newFilter.operator?.includes("last_") ? "Número" : "Data/Hora"
              }
            />
          ) : (
            <Input
              type={newFilter.type === "int" ? "number" : "text"}
              value={newFilter.value}
              onChange={(e) =>
                setNewFilter({ ...newFilter, value: e.target.value })
              }
              placeholder="Digite o valor"
            />
          )}
        </div>

        {newFilter.operator === "between" && (
          <div className="space-y-2">
            <Label>Valor Final</Label>
            <Input
              type={
                newFilter.type === "int"
                  ? "number"
                  : newFilter.type === "date"
                    ? "datetime-local"
                    : "text"
              }
              value={newFilter.value2}
              onChange={(e) =>
                setNewFilter({ ...newFilter, value2: e.target.value })
              }
              placeholder="Valor final"
            />
          </div>
        )}

        <div className="flex items-end">
          <Button
            onClick={onAddFilter}
            disabled={!isFormValid}
            className="bg-gradient-to-r from-blue-600 to-indigo-600"
          >
            <Plus className="h-4 w-4 mr-2" />
            Adicionar
          </Button>
        </div>
      </div>
    </div>
  );
}
