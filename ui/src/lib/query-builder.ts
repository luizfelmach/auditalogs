import type { SearchFilter } from "../types/search";

export const buildElasticsearchQuery = (filters: SearchFilter[]) => {
  if (filters.length === 0) {
    return {
      query: {
        match_all: {},
      },
    };
  }

  const mustClauses: any[] = [];
  const mustNotClauses: any[] = [];

  filters.forEach((filter) => {
    const { field, operator, value, type } = filter;

    switch (operator) {
      case "equals":
        mustClauses.push({ term: { [field]: value } });
        break;

      case "contains":
        mustClauses.push({ wildcard: { [field]: `*${value}*` } });
        break;

      case "starts_with":
        mustClauses.push({ prefix: { [field]: value } });
        break;

      case "ends_with":
        mustClauses.push({ wildcard: { [field]: `*${value}` } });
        break;

      case "not_equals":
        mustNotClauses.push({ term: { [field]: value } });
        break;

      case "regex":
        mustClauses.push({ regexp: { [field]: value } });
        break;

      case "greater_than":
        mustClauses.push({
          range: {
            [field]: { gt: type === "int" ? Number.parseInt(value) : value },
          },
        });
        break;

      case "less_than":
        mustClauses.push({
          range: {
            [field]: { lt: type === "int" ? Number.parseInt(value) : value },
          },
        });
        break;

      case "greater_equal":
        mustClauses.push({
          range: {
            [field]: { gte: type === "int" ? Number.parseInt(value) : value },
          },
        });
        break;

      case "less_equal":
        mustClauses.push({
          range: {
            [field]: { lte: type === "int" ? Number.parseInt(value) : value },
          },
        });
        break;

      case "between": {
        const [val1, val2] = value.split("|");
        const rangeQuery: any = { range: { [field]: {} } };

        if (type === "int") {
          rangeQuery.range[field].gte = Number.parseInt(val1);
          rangeQuery.range[field].lte = Number.parseInt(val2);
        } else if (type === "date") {
          rangeQuery.range[field].gte = val1;
          rangeQuery.range[field].lte = val2;
        } else {
          rangeQuery.range[field].gte = val1;
          rangeQuery.range[field].lte = val2;
        }

        mustClauses.push(rangeQuery);
        break;
      }

      case "after":
        mustClauses.push({ range: { [field]: { gt: value } } });
        break;

      case "before":
        mustClauses.push({ range: { [field]: { lt: value } } });
        break;

      case "last_hours": {
        const hoursAgo = new Date();
        hoursAgo.setHours(hoursAgo.getHours() - Number.parseInt(value));
        mustClauses.push({
          range: { [field]: { gte: hoursAgo.toISOString() } },
        });
        break;
      }

      case "last_days": {
        const daysAgo = new Date();
        daysAgo.setDate(daysAgo.getDate() - Number.parseInt(value));
        mustClauses.push({
          range: { [field]: { gte: daysAgo.toISOString() } },
        });
        break;
      }
    }
  });

  const boolQuery: any = {};

  if (mustClauses.length > 0) {
    boolQuery.must = mustClauses;
  }

  if (mustNotClauses.length > 0) {
    boolQuery.must_not = mustNotClauses;
  }

  return {
    query: {
      bool: boolQuery,
    },
  };
};
