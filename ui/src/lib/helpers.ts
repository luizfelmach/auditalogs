import { operators } from "./operators";
import type { SearchFilter } from "../types/search";

export const getOperatorLabel = (type: string, operator: string) => {
  const ops = operators[type as keyof typeof operators] || [];
  return ops.find((op) => op.value === operator)?.label || operator;
};

export const formatFilterValue = (filter: SearchFilter) => {
  if (filter.operator === "between") {
    const [val1, val2] = filter.value.split("|");
    return `${val1} - ${val2}`;
  }
  return filter.value;
};

export const exportToJson = (data: any, filename: string) => {
  const dataStr = JSON.stringify(data, null, 2);
  const dataUri =
    "data:application/json;charset=utf-8," + encodeURIComponent(dataStr);
  const exportFileDefaultName = `${filename}-${new Date().toISOString().split("T")[0]}.json`;

  const linkElement = document.createElement("a");
  linkElement.setAttribute("href", dataUri);
  linkElement.setAttribute("download", exportFileDefaultName);
  linkElement.click();
};
