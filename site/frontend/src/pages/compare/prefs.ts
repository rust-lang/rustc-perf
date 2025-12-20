import {createStoredValue} from "../../storage";
import {DATE_FMT_24HR} from "../../utils/date-formats";

export const PREF_FILTERS_OPENED = createStoredValue(
  "compare.filters-opened",
  false
);
export const PREF_AGGREGATIONS_OPENED = createStoredValue(
  "compare.aggregations-opened",
  false
);
export const PREF_DATETIME_FORMAT = createStoredValue(
  "general.date-time-format",
  DATE_FMT_24HR
);
