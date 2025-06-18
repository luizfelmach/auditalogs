export const operators = {
  string: [
    { value: "equals", label: "Igual a" },
    { value: "contains", label: "Contém" },
    { value: "starts_with", label: "Começa com" },
    { value: "ends_with", label: "Termina com" },
    { value: "not_equals", label: "Diferente de" },
    { value: "regex", label: "Regex" },
  ],
  int: [
    { value: "equals", label: "Igual a" },
    { value: "greater_than", label: "Maior que" },
    { value: "less_than", label: "Menor que" },
    { value: "greater_equal", label: "Maior ou igual" },
    { value: "less_equal", label: "Menor ou igual" },
    { value: "between", label: "Entre" },
  ],
  date: [
    { value: "equals", label: "Igual a" },
    { value: "after", label: "Depois de" },
    { value: "before", label: "Antes de" },
    { value: "between", label: "Entre" },
    { value: "last_hours", label: "Últimas X horas" },
    { value: "last_days", label: "Últimos X dias" },
  ],
};
