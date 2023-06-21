import {CompileBenchmarkFilter} from "../types";

export const defaultFilter: CompileBenchmarkFilter = {
  name: null,
  nonRelevant: false,
  showRawData: false,
  profile: {
    check: true,
    debug: true,
    opt: true,
    doc: true,
  },
  scenario: {
    full: true,
    incrFull: true,
    incrUnchanged: true,
    incrPatched: true,
  },
  category: {
    primary: true,
    secondary: true,
  },
};
