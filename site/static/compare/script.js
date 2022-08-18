function findQueryParam(name) {
    let urlParams = window.location.search?.substring(1).split("&").map(x => x.split("="));
    let pair = urlParams?.find(x => x[0] === name)
    if (pair) {
        return unescape(pair[1]);
    }
}

function createDefaultFilter() {
    return {
        name: null,
        nonRelevant: false,
        profile: {
            check: true,
            debug: true,
            opt: true,
            doc: true
        },
        scenario: {
            full: true,
            incrFull: true,
            incrUnchanged: true,
            incrPatched: true
        },
        category: {
            primary: true,
            secondary: true
        }
    };
}

const app = Vue.createApp({
    mounted() {
        const app = this;
        loadState(state => makeData(state, app));

        document.getElementById("filters-toggle").onclick = (e) => {
            toggleSection("filters-content", "filters-toggle-indicator");
        };
        document.getElementById("search-toggle").onclick = (e) => {
            toggleSection("search-content", "search-toggle-indicator");
        };
        document.getElementById("aggregations-toggle").onclick = (e) => {
            toggleSection("aggregations-content", "aggregations-toggle-indicator");
        };
    },
    data() {
        return {
            filter: createDefaultFilter(),
            showRawData: false,
            data: null,
            dataLoading: false
        }
    },
    computed: {
        notContinuous() {
            return !this.data.is_contiguous;
        },
        prevLink() {
            return `/compare.html?start=${this.data.prev}&end=${this.data.a.commit}&stat=${this.stat}`;
        },
        nextLink() {
            return `/compare.html?start=${this.data.b.commit}&end=${this.data.next}&stat=${this.stat}`;
        },
        compareLink() {
            return `https://github.com/rust-lang/rust/compare/${this.data.a.commit}...${this.data.b.commit}`;
        },
        testCases() {
            let data = this.data;
            const filter = this.filter;
            const benchmarkMap = this.benchmarkMap;

            function profileFilter(profile) {
                if (profile === "check") {
                    return filter.profile.check;
                } else if (profile === "debug") {
                    return filter.profile.debug;
                } else if (profile === "opt") {
                    return filter.profile.opt;
                } else if (profile === "doc") {
                    return filter.profile.doc;
                } else {
                    return true;
                }
            }

            function scenarioFilter(scenario) {
                if (scenario === "full") {
                    return filter.scenario.full;
                } else if (scenario === "incr-full") {
                    return filter.scenario.incrFull;
                } else if (scenario === "incr-unchanged") {
                    return filter.scenario.incrUnchanged;
                } else if (scenario.startsWith("incr-patched")) {
                    return filter.scenario.incrPatched;
                } else {
                    // Unknown, but by default we should show things
                    return true;
                }
            }

            function categoryFilter(category) {
                if (category === 'primary' && !filter.category.primary) return false;
                if (category === 'secondary' && !filter.category.secondary) return false;
                return true;
            }

            function shouldShowTestCase(testCase) {
                const name = `${testCase.benchmark} ${testCase.profile} ${testCase.scenario}`;
                let nameFilter = filter.name && filter.name.trim();
                nameFilter = !nameFilter || name.includes(nameFilter);

                const relevanceFilter = filter.nonRelevant ? true : testCase.isRelevant;

                return (
                    profileFilter(testCase.profile) &&
                    scenarioFilter(testCase.scenario) &&
                    categoryFilter(testCase.category) &&
                    relevanceFilter &&
                    nameFilter
                );
            }

            let testCases =
                data.comparisons
                    .map(c => {
                        const datumA = c.statistics[0];
                        const datumB = c.statistics[1];
                        const percent = 100 * ((datumB - datumA) / datumA);
                        return {
                            benchmark: c.benchmark,
                            profile: c.profile,
                            scenario: c.scenario,
                            category: (benchmarkMap[c.benchmark] || {}).category || "secondary",
                            isRelevant: c.is_relevant,
                            significanceFactor: c.significance_factor,
                            datumA,
                            datumB,
                            percent,
                        };
                    })
                    .filter(tc => shouldShowTestCase(tc))

            // Sort by name first, so that there is a canonical ordering
            // of test cases. This ensures the overall order is stable, even if
            // individual benchmarks have the same largestChange value.
            testCases.sort((a, b) => a.benchmark.localeCompare(b.benchmark));
            testCases.sort((a, b) => Math.abs(b.percent) - Math.abs(a.percent));

            return testCases;
        },
        bootstrapTotals() {
            const a = this.data.a.bootstrap_total / 1e9;
            const b = this.data.b.bootstrap_total / 1e9;
            return {a, b};
        },
        bootstraps() {
            return Object.entries(this.data.a.bootstrap).map(e => {
                const name = e[0];

                const format = datum => datum ? datum.toLocaleString('en-US', {
                    minimumFractionDigits: 3,
                    maximumFractionDigits: 3
                }) : "";
                const a = e[1] / 1e9;
                const b = this.data.b.bootstrap[name] / 1e9;
                return {
                    name,
                    a: format(a),
                    b: format(b),
                    percent: 100 * (b - a) / a
                };
            }).sort((a, b) => {
                let bp = Math.abs(b.percent);
                if (Number.isNaN(bp)) {
                    bp = 0;
                }
                let ap = Math.abs(a.percent);
                if (Number.isNaN(ap)) {
                    ap = 0;
                }
                if (bp < ap) {
                    return -1;
                } else if (bp > ap) {
                    return 1;
                } else {
                    return a.name.localeCompare(b.name);
                }
            });
        },
        before() {
            if (!this.data) {
                const start = findQueryParam("start");
                // 0,10 extracts "YYYY-MM-DD".
                return start ? start.substring(0, 10) : "???";
            }
            if (this.data.a.pr) {
                return `#${this.data.a.pr}`;
            }
            if (this.data.a.date) {
                return this.formatDate(this.data.a.date);
            }

            // 0,7 extracts 7 chars from the git commit id/tag, which is probably
            // enough to distinguish it. (It is only for display purposes.)
            return this.data.a.commit.substring(0, 7);
        },
        after() {
            if (!this.data) {
                const end = findQueryParam("end");
                // 0,10 extracts "YYYY-MM-DD".
                return end ? end.substring(0, 10) : "???";
            }

            if (this.data.b.pr) {
                return `#${this.data.b.pr}`;
            }
            if (this.data.b.date) {
                return this.formatDate(this.data.b.date);
            }

            // 0,7 extracts 7 chars from the git commit id/tag, which is probably
            // enough to distinguish it. (It is only for display purposes.)
            return this.data.b.commit.substring(0, 7);
        },
        stat() {
            return findQueryParam("stat") || "instructions:u";
        },
        // Returns summary of currently filtered data
        filteredSummary() {
            return computeSummary(this.testCases);
        },
        benchmarkMap() {
            if (!this.data) return {};

            const benchmarks = {};
            for (const benchmark of this.data.benchmark_data) {
                benchmarks[benchmark.name] = {
                    "category": benchmark.category
                };
            }
            return benchmarks;
        }
    },
    methods: {
        short(comparison) {
            return shortCommit(comparison.commit);
        },
        prLink(pr) {
            return `https://github.com/rust-lang/rust/pull/${pr}`;
        },
        diffClass(diff) {
            let klass = "";
            if (diff > 1) {
                klass = 'positive';
            } else if (diff < -1) {
                klass = 'negative';
            }
            return klass;

        },
        commitLink(commit) {
            return `https://github.com/rust-lang/rust/commit/${commit}`;
        },
        formatDate(date) {
            date = new Date(date);

            function padStr(i) {
                return (i < 10) ? "0" + i : "" + i;
            }

            return `${date.getUTCFullYear()}-${padStr(date.getUTCMonth() + 1)}-${padStr(date.getUTCDate())} `;
        },
        trimBenchName(name) {
            let result = name.substring(0, 25)
            if (result != name) {
                result = result + "...";

            }
            return result;
        },
        createUrlForMetric(metric) {
            let start = findQueryParam("start");
            let end = findQueryParam("end");

            return createUrlFromParams(createSearchParamsForMetric(metric, start, end));
        },
        resetFilter() {
            this.filter = createDefaultFilter();
        }
    },
});

app.component('test-cases-table', {
    props: ['cases', 'showRawData', 'commitA', 'commitB', 'before', 'after', 'title', 'stat'],
    methods: {
        detailedQueryLink(commit, testCase) {
            return `/detailed-query.html?commit=${commit.commit}&benchmark=${testCase.benchmark + "-" + testCase.profile}&scenario=${testCase.scenario}`;
        },
        percentLink(commit, baseCommit, testCase) {
            return `/detailed-query.html?commit=${commit.commit}&base_commit=${baseCommit.commit}&benchmark=${testCase.benchmark + "-" + testCase.profile}&scenario=${testCase.scenario}`;
        },
        benchmarkLink(benchmark) {
            return "https://github.com/rust-lang/rustc-perf/tree/master/collector/benchmarks/" + benchmark;
        },
        graphLink(commit, stat, testCase) {
            let date = new Date(commit.date);
            // Move to `30 days ago` to display history of the test case
            date.setUTCDate(date.getUTCDate() - 30);
            let year = date.getUTCFullYear();
            let month = (date.getUTCMonth() + 1).toString().padStart(2, '0');
            let day = date.getUTCDate().toString().padStart(2, '0');
            let start = `${year}-${month}-${day}`;

            let end = commit.commit;
            return `/index.html?start=${start}&end=${end}&benchmark=${testCase.benchmark}&profile=${testCase.profile}&scenario=${testCase.scenario}&stat=${stat}`;
        },
        prettifyRawNumber(number) {
            return number.toLocaleString();
        },
    },
    template: `
<div class="bench-table">
<div class="category-title">{{ title }} benchmarks</div>
<div v-if="cases.length === 0" style="text-align: center;">
  No results
</div>
<table v-else class="benches compare">
    <thead>
        <tr>
            <th>Benchmark</th>
            <th>Profile</th>
            <th>Scenario</th>
            <th>% Change</th>
            <th>
                Significance Factor<span class="tooltip">?
                    <span class="tooltiptext">
                        How much a particular result is over the significance threshold. A factor of 2.50x
                        means
                        the result is 2.5 times over the significance threshold. You can see <a
                            href="https://github.com/rust-lang/rustc-perf/blob/master/docs/comparison-analysis.md#what-makes-a-test-result-significant">
                            here</a> how the significance threshold is calculated.
                    </span>
                </span>
            </th>
            <th v-if="showRawData">{{ before }}</th>
            <th v-if="showRawData">{{ after }}</th>
        </tr>
    </thead>
    <tbody>
        <template v-for="testCase in cases">
            <tr>
                <td>
                  <a v-bind:href="benchmarkLink(testCase.benchmark)"
                     class="silent-link"
                     target="_blank">
                     {{ testCase.benchmark }}
                 </a>
                </td>
                <td>
                  <a v-bind:href="graphLink(commitB, stat, testCase)" target="_blank" class="silent-link">
                    {{ testCase.profile }}
                  </a>
                </td>
                <td>{{ testCase.scenario }}</td>
                <td>
                    <a v-bind:href="percentLink(commitB, commitA, testCase)">
                        <span v-bind:class="percentClass(testCase.percent)">
                            {{ testCase.percent.toFixed(2) }}%
                        </span>
                    </a>
                </td>
                <td>
                    {{ testCase.significanceFactor ? testCase.significanceFactor.toFixed(2) + "x" : "-" }}
                </td>
                <td v-if="showRawData" class="numeric">
                  <a v-bind:href="detailedQueryLink(commitA, testCase)">
                    <abbr :title="testCase.datumA">{{ prettifyRawNumber(testCase.datumA) }}</abbr>
                  </a>
                </td>
                <td v-if="showRawData" class="numeric">
                  <a v-bind:href="detailedQueryLink(commitB, testCase)">
                    <abbr :title="testCase.datumB">{{ prettifyRawNumber(testCase.datumB) }}</abbr>
                  </a>
                </td>
            </tr>
        </template>
    </tbody>
</table>
</div>
`
});

const SummaryPercentValue = {
    props: {
        value: Number,
        padWidth: {
            type: Number,
            default: null
        }
    },
    template: `
<span><span v-html="padSpaces" />{{ formattedValue }}%</span>
`,
    computed: {
        formattedValue() {
            return `${this.signIfPositive(this.value)}${this.value.toFixed(2)}`;
        },
        padSpaces() {
            let value = this.formattedValue;
            if (value.length < this.padWidth) {
                return "&nbsp;".repeat(this.padWidth - value.length);
            }
            return "";
        }
    }
};
const SummaryRange = {
    props: {
        range: Array,
    },
    template: `
<div v-if="range.length > 0">
  [<SummaryPercentValue :value="range[0]" :padWidth="6" />, <SummaryPercentValue :value="range[1]" :padWidth="6" />]
</div>
<div v-else style="text-align: center;">-</div>
`, components: {SummaryPercentValue}
};
const SummaryCount = {
    props: {
        cases: Number,
        benchmarks: Number
    },
    template: `
<span :title="cases + ' test case(s), ' + benchmarks + ' unique benchmark(s)'">{{ cases }} ({{ benchmarks }})</span>
`
};

app.component('summary-table', {
    props: {
        summary: Object,
        withLegend: {
            type: Boolean,
            default: true
        }
    },
    template: `
<div v-if="summary.all.count === 0" style="flex-grow: 1; display: flex; flex-direction: column; justify-content: center;">
  <span>No results</span>
</div>
<table v-else class="summary-table">
    <thead v-if="withLegend">
      <th><!-- icon --></th>
      <th>Range</th>
      <th>Mean</th>
      <th>Count</th>
    </thead>
    <tbody>
        <tr class="positive">
            <td title="Regressions" v-if="withLegend">❌</td>
            <template v-if="summary.regressions.count !== 0">
                <td><SummaryRange :range="summary.regressions.range" /></td>
                <td><SummaryPercentValue :value="summary.regressions.average" /></td>
                <td><SummaryCount :cases="summary.regressions.count" :benchmarks="summary.regressions.benchmarks" /></td>
            </template>
            <template v-else>
              <td colspan="3" style="text-align: center;">No regressions</td>
            </template>
        </tr>
        <tr class="negative">
            <td title="Improvements" v-if="withLegend">✅</td>
            <template v-if="summary.improvements.count !== 0">
                <td><SummaryRange :range="summary.improvements.range" /></td>
                <td><SummaryPercentValue :value="summary.improvements.average" /></td>
                <td><SummaryCount :cases="summary.improvements.count" :benchmarks="summary.improvements.benchmarks" /></td>
            </template>
            <template v-else>
              <td colspan="3" style="text-align: center;">No improvements</td>
            </template>
        </tr>
        <tr>
            <td title="All changes" v-if="withLegend">❌,✅</td>
            <td><SummaryRange :range="summary.all.range" /></td>
            <td><SummaryPercentValue :value="summary.all.average" /></td>
            <td><SummaryCount :cases="summary.all.count" :benchmarks="summary.all.benchmarks" /></td>
        </tr>
    </tbody>
</table>
`,
    components: {SummaryRange, SummaryPercentValue, SummaryCount}
});

app.component("aggregations", {
    props: {
        cases: Array
    },
    template: `
<div>
  <div class="aggregation-section">
    <div class="header">Profile</div>
    <div class="groups">
      <div class="group" v-for="profile in ['check', 'debug', 'opt', 'doc']">
        <div class="group-header">{{ profile }}</div>
        <summary-table :summary="calculateSummary('profile', profile)" :withLegend="false"></summary-table>
      </div>
    </div>
  </div>
  <div class="aggregation-section">
    <div class="header">Scenario</div>
    <div class="groups">
      <div class="group" v-for="scenario in ['full', 'incr-full', 'incr-unchanged', 'incr-patched']">
        <div class="group-header">{{ scenario }}</div>
        <summary-table :summary="calculateSummary('scenario', scenario)" :withLegend="false"></summary-table>
      </div>
    </div>
  </div>
  <div class="aggregation-section">
    <div class="header">Category</div>
    <div class="groups">
      <div class="group" v-for="category in ['primary', 'secondary']">
        <div class="group-header">{{ category }}</div>
        <summary-table :summary="calculateSummary('category', category)" :withLegend="false"></summary-table>
      </div>
    </div>
  </div>
</div>
`,
    methods: {
        calculateSummary(keyAttribute, keyValue) {
            const benchmarks = [];
            for (const benchmark of this.cases) {
                if (benchmark[keyAttribute].startsWith(keyValue)) {
                    benchmarks.push(benchmark);
                }
            }
            return computeSummary(benchmarks);
        }
    }
});

app.mixin({
    methods: {
        percentClass(pct) {
            let klass = "";
            if (pct > 1) {
                klass = 'positive';
            } else if (pct > 0) {
                klass = 'slightly-positive';
            } else if (pct < -1) {
                klass = 'negative';
            } else if (pct < -0) {
                klass = 'slightly-negative';
            }
            return klass;
        },
        signIfPositive(pct) {
            if (pct >= 0) {
                return "+";
            }
            return "";
        },
    }
});

function toggleSection(id, toggle) {
    let styles = document.getElementById(id).style;
    let indicator = document.getElementById(toggle);
    if (styles.display != "none") {
        indicator.innerHTML = " ▶"
        styles.display = "none";
    } else {
        indicator.innerHTML = " ▼"
        styles.display = "block";
    }
}

toggleSection("filters-content", "filters-toggle-indicator");
toggleSection("search-content", "search-toggle-indicator");
toggleSection("aggregations-content", "aggregations-toggle-indicator");

/**
 * Computes summaries of improvements, regressions and all changes from the given `comparisons`.
 * Returns a dictionary {improvements, regressions, all}.
 */
function computeSummary(testCases) {
    const regressions = {
        values: [],
        benchmarks: new Set(),
    };
    const improvements = {
        values: [],
        benchmarks: new Set(),
    };
    const all = {
        values: [],
        benchmarks: new Set(),
    };

    const handleTestCase = (items, testCase) => {
        items.benchmarks.add(testCase.benchmark);
        items.values.push(testCase.percent);
    };

    for (const testCase of testCases) {
        if (testCase.percent > 0) {
            handleTestCase(regressions, testCase);
        } else if (testCase.percent < 0) {
            handleTestCase(improvements, testCase);
        }
        handleTestCase(all, testCase);
    }

    const computeSummary = (data) => {
        const values = data.values;
        const benchmarks = data.benchmarks;

        const count = values.length;
        let range = [];
        if (count > 0) {
            range = [
                Math.min.apply(null, values),
                Math.max.apply(null, values),
            ];
        }

        const sum = values.reduce((acc, item) => acc + item, 0);
        const average = sum / Math.max(count, 1);

        return {
            count,
            benchmarks: benchmarks.size,
            average,
            range,
        }
    };

    return {
        improvements: computeSummary(improvements),
        regressions: computeSummary(regressions),
        all: computeSummary(all)
    };
}

function unique(arr) {
    return arr.filter((value, idx) => arr.indexOf(value) == idx);
}

function shortCommit(commit) {
    return commit.substring(0, 8);
}

function makeData(state, app) {
    app.dataLoading = true;
    let values = Object.assign({}, {
        start: "",
        end: "",
        stat: "instructions:u",
    }, state);
    makeRequest("/get", values).then(function (data) {
        app.data = data;
    }).finally(function () {
        app.dataLoading = false;
    });
}

function createSearchParamsForMetric(stat, start, end) {
    let params = new URLSearchParams();
    if (start !== undefined) {
        params.append("start", start);
    }
    if (end !== undefined) {
        params.append("end", end);
    }
    if (stat !== undefined) {
        params.append("stat", stat);
    }
    return params.toString();
}

function createUrlFromParams(params) {
    return window.location.protocol + "//" + window.location.host + window.location.pathname + "?" + params;
}

function submitSettings() {
    let stat = getSelected("stats");
    let start = document.getElementById("start-bound").value;
    let end = document.getElementById("end-bound").value;
    let params = createSearchParamsForMetric(stat, start, end);
    window.location.search = params.toString();
}

app.mount('#app');
