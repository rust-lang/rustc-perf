function findQueryParam(name) {
    let urlParams = window.location.search?.substring(1).split("&").map(x => x.split("="));
    let pair = urlParams?.find(x => x[0] === name)
    if (pair) {
        return unescape(pair[1]);
    }
}

const app = Vue.createApp({
    mounted() {
        const app = this;
        loadState(state => makeData(state, app));

        document.getElementById("filters-toggle").onclick = (e) => {
            toggleFilters("filters-content", "filters-toggle-indicator");
        };
        document.getElementById("search-toggle").onclick = (e) => {
            toggleFilters("search-content", "search-toggle-indicator");
        };
    },
    data() {
        return {
            filter: {
                name: null,
                showNonRelevant: false,
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
            },
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

                const relevanceFilter = filter.showNonRelevant ? true : testCase.isRelevant;

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
        summary() {
            // Create object with each test case that is not filtered out as a key
            const filtered = this.testCases.reduce((sum, next) => {
                sum[testCaseString(next)] = true;
                return sum;
            }, {});
            const newCount = {
                regressions: 0,
                regressions_avg: 0,
                improvements: 0,
                improvements_avg: 0,
                unchanged: 0,
                average: 0
            };

            const addDatum = (result, datum, percent) => {
                if (percent > 0 && datum.is_relevant) {
                    result.regressions += 1;
                    result.regressions_avg += percent;
                } else if (percent < 0 && datum.is_relevant) {
                    result.improvements += 1;
                    result.improvements_avg += percent;
                } else {
                    result.unchanged += 1;
                }
                result.average += percent;
            };

            let result = {all: {...newCount}, filtered: {...newCount}}
            for (let d of this.data.comparisons) {
                const testCase = testCaseString(d)
                const datumA = d.statistics[0];
                const datumB = d.statistics[1];
                let percent = 100 * ((datumB - datumA) / datumA);
                addDatum(result.all, d, percent);
                if (filtered[testCase]) {
                    addDatum(result.filtered, d, percent);
                }
            }

            const computeAvg = (result) => {
                result.improvements_avg /= Math.max(result.improvements, 1);
                result.regressions_avg /= Math.max(result.regressions, 1);
                result.average /= Math.max(result.regressions + result.improvements + result.unchanged, 1);
            };
            computeAvg(result.all);
            computeAvg(result.filtered);

            return result;

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
        signIfPositive(pct) {
            if (pct >= 0) {
                return "+";
            }
            return "";
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
    }
});

function toggleFilters(id, toggle) {
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

toggleFilters("filters-content", "filters-toggle-indicator");
toggleFilters("search-content", "search-toggle-indicator");

function testCaseString(testCase) {
    return testCase.benchmark + "-" + testCase.profile + "-" + testCase.scenario;
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
    params.append("start", start);
    params.append("end", end);
    params.append("stat", stat);
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
