import {createStoredValue} from "../../storage";

export const PREF_FILTERS_OPENED = createStoredValue(
  "compare.filters-opened",
  false
);
export const PREF_AGGREGATIONS_OPENED = createStoredValue(
  "compare.aggregations-opened",
  false
);
