// Copyright 2015 authors shown at
// https://github.com/nrc/rustc-perf/graphs/contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Main entry point for the backend server. Maintains a clone of the data repo
// and exposes an API for getting views of that data.
//
// start with `nohup nodejs perf.js path/to/repo &`

// On receipt of a web hook from GH, update the repo and our db.

// On request for data, produce data in JSON format from our db.

var http = require('http');
var url = require('url');
var async = require('async');
var path = require("path");
var fs = require('fs');
var nodegit = require('nodegit');
var sortedList = require('sortedlist');

var WEEKS_IN_SUMMARY = 12;

// Data stored by test name and date, each date maps to a table storing the times by crate and by phase.
// See make_data() for the object layout.
var data = {};
var opts = { compare: function(x, y) { return x.date - y.date; }}
data['rustc'] = sortedList.create(opts);
data['benchmarks'] = sortedList.create(opts);
var summary = {};
summary['rustc'] = [];
summary['benchmarks'] = [];
var benchmarks = [];

var last_date;
var crate_list = [];
var phase_list = [];

init(get_repo_arg(), start_server);

function get_repo_arg() {
    var args = process.argv;
    if (args.length < 3) {
        console.log('No argument supplied, needs location of data repo');
        process.exit(1);
    }

    return args[2];
}

// Startup - update the repo and create our runtime representation of the data.
function init(repo_loc, callback) {
    console.log('pulling', repo_loc);

    // git pull origin master == fetch; merge
    var repository;
    nodegit.Repository.open(path.resolve(__dirname, repo_loc))
        .then(function(repo) {
            repository = repo;
            return repository.fetch('origin', {}, {});
        })
        .then(function() {
            return repository.mergeBranches('master', 'origin/master');
        })
        .done(function() {
            var opts = new nodegit.CheckoutOptions();
            opts.checkoutStrategy = nodegit.Checkout.STRATEGY.FORCE;
            nodegit.Checkout.head(repository, opts).then(function() {
                console.log('success - pulled');

                read_files(repo_loc);

                callback();
            });
        });
}

function read_files(repo_loc) {
    // Read all files from repo_loc/processed
    fs.readdir(path.resolve(__dirname, repo_loc, 'processed'), function (err, files) {
        if (err) {
           throw err;
        }
        var no_times = 0;
        var c_bootstraps = 0;
        var c_benchmarks = 0;
        var c_benchmarks_add = 0;
        files.map(function (filename) {
            var full_name = path.resolve(__dirname, repo_loc, 'processed', filename);
            var file_contents = fs.readFileSync(full_name, 'utf8');
            if (!file_contents) {
                no_times += 1;
                return;
            }
            var contents = JSON.parse(file_contents);
            var header = contents.header;
            var times = contents.times;
            if (times.length == 0) {
                no_times += 1;
                return;
            }
            var test_name = filename.substring(0, filename.indexOf('--'));

            var date = new Date(header.date);
            if (isNaN(date)) {
                var date_components = [0, 0, 0, 0, 0, 0];
                var actual_components = header.date.split('-');
                for (i in actual_components) {
                    date_components[i] = parseInt(actual_components[i], 10);
                }
                if (date_components[1] > 0) {
                    date_components[1] -= 1;
                }

                date = new Date(date_components[0],
                                date_components[1],
                                date_components[2],
                                date_components[3],
                                date_components[4],
                                date_components[5]);
            }

            if (test_name == 'rustc') {
                c_bootstraps += 1;
                data['rustc'].insertOne(make_data(date, header, times, true));
            } else {
                if (benchmarks.indexOf(test_name) < 0) {
                    benchmarks.push(test_name);
                }
                var key = data['benchmarks'].key({date: date});
                if (key == null) {
                    c_benchmarks += 1;
                    data['benchmarks'].insertOne(make_data(date, header, times, false));
                } else {
                    c_benchmarks_add += 1;
                    data['benchmarks'][key].by_crate[test_name] = make_times(times, false)[times[0].crate];
                }
            }
            if (!last_date || last_date < date) {
                last_date = date;
            }
        });

        // Post processing to generate the summary data.
        var prev_summary = [];
        // We only want data for the previous 12 weeks, which means we need 13
        // data points.
        for (var i = 0; i < WEEKS_IN_SUMMARY + 1; ++i) {
            // The date for this data point.
            var date = new Date(last_date.getTime());
            date.setDate(date.getDate() - 7 * i);

            summarise_data(i, date, 'rustc', prev_summary);
            summarise_data(i, date, 'benchmarks', prev_summary);
        }

        console.log("read", files.length, "files");
        console.log("found", no_times, "files without times");
        console.log(c_bootstraps, "bootstrap times");
        console.log(c_benchmarks, "benchmarks times");
        console.log(c_benchmarks_add, "benchmarks times (appended)");
    });
}

// Compute summary data for a given date and kind (rustc bootstrap/benchmarks).
// For each date we calculate some summary, then for each but the first we
// calculate the difference from the previous week. Save it all in the summary
// global.
function summarise_data(i, date, kind, prev_summary) {
    var cur_summary = [];
    cur_summary[kind] = { 'date': date, 'by_crate': {} };
    summary[kind][i] = { 'date': date, 'by_crate': {} };

    // For a given date we'll get the three most recent sets of data and take the
    // the median for each value.
    var start_idx = mk_start_idx(date, kind);
    var week_data = [];
    for (var j = 0; j < 3; ++j) {
        var idx = Math.max(start_idx - 1, 0);
        week_data.push(data[kind][idx]);
    }

    // Compute the difference as a percent for every value of data.
    for (var c in week_data[0].by_crate) {
        if (c in week_data[1].by_crate && c in week_data[2].by_crate) {
            cur_summary[kind].by_crate[c] = {};
            summary[kind][i].by_crate[c] = {};
            // p is phase
            for (var p in week_data[0].by_crate[c]) {
                if (p in week_data[1].by_crate[c] && p in week_data[2].by_crate[c]) {
                    // Find the median value.
                    var values = [week_data[0].by_crate[c][p].time, week_data[1].by_crate[c][p].time, week_data[2].by_crate[c][p].time];
                    values.sort(function(a, b) {
                        return a - b;
                    });
                    cur_summary[kind].by_crate[c][p] = values[1];

                    if (prev_summary[kind] && c in prev_summary[kind].by_crate && p in prev_summary[kind].by_crate[c] && prev_summary[kind].by_crate[c][p] > 0) {
                        var difference = prev_summary[kind].by_crate[c][p] - values[1];
                        var percent = 100 * difference / values[1];
                        summary[kind][i].by_crate[c][p] = percent;

                        if (i == WEEKS_IN_SUMMARY && c in summary[kind][0].by_crate && p in summary[kind][0].by_crate[c] && summary[kind][0].by_crate[c][p] > 0) {
                            // We're computing the last week, so let's also compute the total
                            // difference.
                            var difference = summary[kind][0].by_crate[c][p] - values[1];
                            var percent = 100 * difference / values[1];
                            summary[kind][0].by_crate[c][p] = percent;
                        }
                    } else if (i == 0) {
                        // Save the first week we look at so we can compute the total difference,
                        // see above.
                        summary[kind][i].by_crate[c][p] = values[1]
                    }
                }
            }
        }
    }

    prev_summary[kind] = cur_summary[kind];
}


function make_data(date, header, times, rustc_crate) {
    if (!header || !times || times.length == 0) {
        return {};
    }

    return {
        'date': date,
        'commit': header.commit,
        'by_crate': make_times(times, rustc_crate),
    };
}

function make_times(times, rustc_crate) {
    var by_crate = {};
    var totals = {}

    for (i in times) {
        var t = times[i];

        var t_times = t.times;
        var mem_values = [0];
        if (t.rss) {
            for (p in t_times) {
                t_times[p].rss = t.rss[p];
            }
            mem_values = Object.keys(t.rss).map(function(v) { return t.rss[v]; });
        }
        by_crate[t.crate] = t_times;


        by_crate[t.crate]['total'] = {
            "percent": 100,
            "time": t.total,
            "rss": Math.max.apply(null, mem_values)
        };
        for (var phase in by_crate[t.crate]) {
            if (!totals[phase]) {
                totals[phase] = {
                    "percent": 0,
                    "time": 0,
                    "rss": 0
                };
            }
            totals[phase].time += by_crate[t.crate][phase].time;
            totals[phase].rss = Math.max(by_crate[t.crate][phase].rss, totals[phase].rss);
            record_phase(phase);
        }
        if (rustc_crate) {
            record_crate(t.crate);
        }
    }

    // TODO percentages
    if (rustc_crate) {
        by_crate['total'] = totals;
    }
    return by_crate;
}

function start_server() {
    console.log('starting server; listening on 2346')
    http.createServer(function (req, res) {
        var parsed_url = url.parse(req.url, true);
        var pathname = parsed_url.pathname;
        console.log("Path parsed: " + pathname);
        if (pathname == '/data') {
            combine_chunks(req, function(body) {
                try {
                    var json = JSON.parse(body);
                    get_data(json, res);
                    console.log(json);
                } catch(e) {
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Error: " + e);
                }
            });
        } else if (pathname == '/get') {
            combine_chunks(req, function(body) {
                try {
                    var json = JSON.parse(body);
                    get_days(json, res);
                    console.log(json);
                } catch(e) {
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Error: " + e);
                    console.log(e);
                }
            });
        } else if (pathname == '/get_tabular') {
            combine_chunks(req, function(body) {
                try {
                    var json = JSON.parse(body);
                    get_tabular(json, res);
                    console.log(json);
                } catch(e) {
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Error: " + e);
                    console.log(e);
                }
            });
        } else if (pathname == '/stats') {
            combine_chunks(req, function(body) {
                try {
                    var json = JSON.parse(body);
                    get_stats(json, res);
                    console.log(json);
                } catch(e) {
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Error: " + e);
                    console.log(e);
                }
            });
        } else if (pathname == '/summary') {
            try {
                get_summary(res);
            } catch (e) {
                res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                res.end("Error: " + e);
                console.log(e);
            }
        } else if (pathname == '/info') {
            get_info(res);
        } else if (pathname == '/onpush') {
            combine_chunks(req, function(body) {
                try {
                    var json = JSON.parse(body);
                    console.log(json);
                    on_push(json);
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Success?");
                } catch (e) {
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Error: " + e);
                    console.log(e);
                }
            });
        } else {
            res.writeHead(404, {"Content-Type": "text/plain"});
            res.write("404 Not Found\n");
            console.log("404 Not Found");
            res.end();
          }
    }).listen(2346);
}

function combine_chunks(req, callback) {
    var body = '';
    req.on('data', function(chunk) {
        body += chunk;
    });
    req.on('end', function() {
        callback(body);
    });
}

function on_push(json) {
    // FIXME we are throwing everything away and starting again. It would be better
    // to read the added file from json and just read that file, adding it to data.
    console.log("received onpush hook")
    data = {};
    data['rustc'] = sortedList.create(opts);
    data['benchmarks'] = sortedList.create(opts);
    benchmarks = [];
    init(get_repo_arg(), function() {});
}


function record_crate(crate_name) {
    if (crate_list.indexOf(crate_name) < 0) {
        crate_list.push(crate_name);
    }
}


function record_phase(phase_name) {
    if (phase_list.indexOf(phase_name) < 0) {
        phase_list.push(phase_name);
    }
}

function get_info(response) {
    try {
        var result = {};
        result.crates = crate_list;
        result.phases = phase_list;
        result.benchmarks = benchmarks;
        response.writeHead(200, {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"});
        response.end(JSON.stringify(result));
    } catch (e) {
        console.log(e);
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: " + e);
    }
}

function get_summary(response) {
    // TODO we're computing and saving a lot of data which we never use
    var result = {};
    // dates for each summary/breakdown
    result.dates = summary['rustc'].map(function(s) { return s.date; });
    // Remove the last date.
    result.dates.length = 12;
    // overall number for each week
    result.summaries = [];
    var sums = summary['benchmarks'].map(function(s) {
        var sum = 0;
        var count = 0;
        for (var c in s.by_crate) {
            if ('total' in s.by_crate[c]) {
                sum += s.by_crate[c]['total'];
                count += 1;
            }
        }

        return { 'sum': sum, 'count': count };
    });
    for (var i in sums) {
        if ('total' in summary['rustc'][i].by_crate['total']) {
            sums[i].sum += 2 * summary['rustc'][i].by_crate['total']['total'];
            sums[i].count += 2;
        }
        result.summaries[i] = (sums[i].sum / sums[i].count).toFixed(1);
    }

    // per benchmark, per week
    result.breakdown = summary['benchmarks'].map(function(s) {
        var per_bench = {};

        for (var c in s.by_crate) {
            if ('total' in s.by_crate[c]) {
                per_bench[c] = s.by_crate[c]['total'].toFixed(1);
            } else {
                per_bench[c] = 0.0;
            }
        }

        return per_bench;
    });
    for (var i in result.breakdown) {
        if ('total' in summary['rustc'][i].by_crate['total']) {
            result.breakdown[i]['bootstrap'] = summary['rustc'][i].by_crate['total']['total'].toFixed(1);
        } else {
            result.breakdown[i]['bootstrap'] = 0.0;
        }
    }

    result.total_summary = result.summaries.shift();
    result.total_breakdown = result.breakdown.shift();

    response.writeHead(200, {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"});
    response.end(JSON.stringify(result));
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     start: Date,     // optional
//     end: Date,       // optional
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str]
// }
// crate (rustc only) or phase can be 'total'
function get_stats(body, response) {
    try {
        var dates = start_and_end_dates(body);
        var start = dates[0];
        var end = dates[1];

        var kind = body.kind;
        if (kind != 'rustc' && kind != 'benchmarks') {
            response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
            response.end("Error: bad value kind: " + kind);
            return;
        }
        var crates = body.crates;
        var phases = body.phases;

        if (kind != 'rustc' && crates.indexOf('total') >= 0) {
            response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
            response.end("Error: bad value crate for benchmarks");
            return;
        }

        var result = {};
        // These should get overwritten with more accurate data later.
        result.startDate = start;
        result.endDate = end;

        var counted = [];
        var first_idx = -1;
        var last_idx = 0;
        // Iterate over date range.
        var start_idx = mk_start_idx(start, kind);
        var end_idx = mk_end_idx(end, kind);
        for (var i = start_idx; i <= end_idx; ++i) {
            var today_data = data[kind][i];
            counted.push(today_data);

            if (Object.keys(today_data.by_crate).length > 0) {
                last_idx = i;
                result.endDate = today_data.date;
                if (first_idx < 0) {
                    first_idx = i;
                    result.startDate = today_data.date;
                }
            }
        }
        // Trim the data
        counted = counted.slice(Math.max(first_idx, 0), last_idx+1);

        result.crates = {};
        for (var c in crates) {
            result.crates[crates[c]] = mk_stats(counted, crates[c], phases);
        }

        response.writeHead(200, {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"});
        response.end(JSON.stringify(result));
    } catch (e) {
        console.log(e);
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: " + e);
    }
}

function mk_stats(data, crate, phases) {
    var result = {};
    var len = data.length;
    var count = 0;
    var skip_list = data.map(function(d) {
        if (d.by_crate[crate] && Object.keys(d.by_crate[crate]).length > 0) {
            count += 1;
            return false;
        } else {
            return true;
        }
    });

    if (count == 0) {
        result.min = 0;
        result.max = 0;
        result.mean = 0;
        result.variance = 0;
        result.first = 0;
        result.last = 0;
        result.trend = 0;
        result.trend_b = 0;
        result.n = 0;
        return result;
    }

    var first = 0;
    for (var i in skip_list) {
        if (!skip_list[i]) {
            first = i;
            break;
        }
    }
    result.first = sumTimes(data[first], crate, phases);
    result.last = sumTimes(data[len-1], crate, phases);
    var min = result.first;
    var max = result.first;
    var q1 = Math.floor(len / 4);
    var q4 = Math.floor(3 * len / 4);
    var total = 0;
    var q1Total = 0;
    var q4Total = 0;
    var sums = data.map(function(d, i) {
        if (skip_list[i]) {
            return 0;
        }

        return sumTimes(d, crate, phases);
    });
    for (var i in data) {
        if (skip_list[i]) {
            continue;
        }
        var cur = sums[i];
        total += cur;
        if (cur < min) {
            min = cur;
        }
        if (cur > max) {
            max = cur;
        }
        if (i < q1) {
            q1Total += cur;
        }
        if (i >= q4) {
            q4Total += cur;
        }
    }

    // Calculate the variance
    result.mean = total / count;
    var varTotal = 0;
    for (var i in data) {
        if (skip_list[i]) {
            continue;
        }
        var diff = sums[i] - result.mean;
        varTotal += diff * diff;
    }
    result.variance = varTotal / (count - 1);

    result.n = count;
    result.min = min;
    result.max = max;
    if (count >= 10 && count == len) {
        var q1Mean = q1Total / q1;
        var q4Mean = q4Total / (len - q4);
        result.trend = 100 * (q4Mean - q1Mean) / result.first;
    } else {
        result.trend = 0;
    }
    result.trend_b = 100 * (result.last - result.first) / result.first;

    return result;
}

function sumTimes(data, crate, phases) {
    var c = data.by_crate[crate];
    var sum = 0;
    for (var p in phases) {
        sum += c[phases[p]]['time'];
    }

    return sum;
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     dates: [Date],
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str],
//     group_by: 'crate' | 'phase'
// }
// crate or phase can be 'total'
function get_days(body, response) {
    var kind = body.kind;
    if (kind != 'rustc' && kind != 'benchmarks') {
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: bad value kind: " + kind);
        console.log("Error: bad value kind: " + kind);
        return;
    }
    var crates = body.crates;
    var phases = body.phases;
    var group_by = body.group_by;
    if (group_by != 'crate' && group_by != 'phase') {
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: bad value group_by: " + group_by);
        console.log("Error: bad value group_by: " + group_by);
        return;
    }

    var result = [];
    for (var i in body.dates) {
        var date = new Date(body.dates[i]);
        if (isNaN(date)) {
            console.log("bad date:", body.dates[i]);
            continue;
        }
        var idx = data[kind].bsearch({date: date});
        if (idx < 0 || idx >= data[kind].length) {
            idx = data[kind].length - 1;
        }
        var day = get_data_for_date(kind, idx, crates, phases, group_by);
        result.push(day);
    }

    response.writeHead(200, {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"});
    response.end(JSON.stringify(result));
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     date: Date
// }
function get_tabular(body, response) {
    var kind = body.kind;
    if (kind != 'rustc' && kind != 'benchmarks') {
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: bad value kind: " + kind);
        console.log("Error: bad value kind: " + kind);
        return;
    }
    var date = new Date(body.date);
    if (isNaN(date)) {
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: bad date: " + body.date);
        console.log("bad date:", body.date);
        return;
    }
    var idx = data[kind].bsearch({date: date});
    if (idx < 0 || idx >= data[kind].length) {
        idx = data[kind].length - 1;
    }

    var day = data[kind][idx];

    var result = {
        'date': day.date.toString(),
        'commit': day.commit,
        'data': day.by_crate
    };

    response.writeHead(200, {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"});
    response.end(JSON.stringify(result));
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     start: Date,     // optional
//     end: Date,       // optional
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str],
//     group_by: 'crate' | 'phase',
// }
// crate (rustc only) or phase can be 'total'
function get_data(body, response) {
    try {
        var dates = start_and_end_dates(body);
        var start = dates[0];
        var end = dates[1];

        var kind = body.kind;
        if (kind != 'rustc' && kind != 'benchmarks') {
            response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
            response.end("Error: bad value kind: " + kind);
            console.log("Error: bad value kind: " + kind);
            return;
        }
        var crates = body.crates;
        var phases = body.phases;
        var group_by = body.group_by;
        if (group_by != 'crate' && group_by != 'phase') {
            response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
            response.end("Error: bad value group_by: " + group_by);
            console.log("Error: bad value group_by: " + group_by);
            return;
        }

        var result = [];
        var first_idx = -1;
        var last_idx = 0;
        // Iterate over date range.
        var start_idx = mk_start_idx(start, kind);
        var end_idx = mk_end_idx(end, kind);
        for (var i = start_idx; i <= end_idx; ++i) {
            var today_data = get_data_for_date(kind, i, crates, phases, group_by);
            result.push(today_data);

            var empty = Object.keys(today_data.data).length == 0;
            if (!empty) {
                last_idx = i - start_idx;
                if (first_idx == -1) {
                    first_idx = i - start_idx;
                }
            }
        }
        // Trim the data
        result = result.slice(first_idx, last_idx+1);

        response.writeHead(200, {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"});
        response.end(JSON.stringify(result));
    } catch (e) {
        console.log(e);
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: " + e);
    }
}

function mk_start_idx(start, kind) {
    var start_idx = data[kind].bsearch({date: start});
    if (start_idx < 0) {
        start_idx = 0;
    }
    return start_idx;
}

function mk_end_idx(end, kind) {
    var end_idx = data[kind].bsearch({date: end});
    if (end_idx < 0 || end_idx >= data[kind].length) {
        end_idx = data[kind].length - 1;
    }
    return end_idx;
}

function start_and_end_dates(body) {
    // Handle missing end by using the last available date.
    var end = body.end;
    if (end) {
        end = new Date(end);
    } else {
        end = last_date;
    }

    // Handle missing start by returning 60 days before end.
    var start = body.start;
    if (start) {
        start = new Date(start);
    } else {
        start = new Date(end);
        start.setDate(end.getDate() - 30);
    }

    return [start, end];
}

function get_data_for_date(kind, index, crates, phases, group_by) {
    var day = data[kind][index];

    var result = {
        'date': day.date.toString(),
        'commit': day.commit,
        'data': {},
    };

    if (group_by == 'crate') {
        for (var c in crates) {
            var crate = crates[c];
            if (crate in day.by_crate) {
                var cur_crate = day.by_crate[crate];
                var total = 0;
                var mem = 0;
                for (var p in phases) {
                    var phase = phases[p];
                    if (phase in cur_crate) {
                        total += cur_crate[phase]['time'];
                        mem = Math.max(mem, cur_crate[phase]['rss']);
                    }
                }
                result.data[crates[c]] = {
                    time: total,
                    rss: mem
                };
            }
        }
    } else {
        // group_by == 'phase'
        for (var p in phases) {
            var total = 0;
            var mem = 0;
            var phase = phases[p];
            for (var c in crates) {
                var crate = crates[c];
                if (crate in day.by_crate) {
                    var cur_crate = day.by_crate[crate];
                    if (phase in cur_crate) {
                        total += cur_crate[phase]['time'];
                        mem = Math.max(mem, cur_crate[phase]['rss']);
                    }
                }
            }
            result.data[phases[p]] = {
                time: total,
                rss: mem
            };
        }
    }

    return result;
}
