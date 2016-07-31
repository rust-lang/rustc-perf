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

let http = require("http");
let url = require("url");
let path = require("path");
let fs = require("fs");
let nodegit = require("nodegit");
let sortedList = require("sortedlist");

const WEEKS_IN_SUMMARY = 12;

// Data stored by test name and date, each date maps to a table storing the times by crate and by phase.
// See make_data() for the object layout.
var data = {};
var opts = { compare: function(x, y) { return x.date - y.date; }};
data.rustc = sortedList.create(opts);
data.benchmarks = sortedList.create(opts);
var summary = {};
summary.rustc = [];
summary.benchmarks = [];
var benchmarks = [];

var last_date;
var crate_list = [];
var phase_list = [];

init(get_repo_arg(), start_server);

function get_repo_arg() {
	var args = process.argv;
	if (args.length < 3) {
		console.log("No argument supplied, needs location of data repo");
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

function read_files(repo_loc, callback) {
	// Read all files from repo_loc/processed
	let files = fs.readdirSync(path.resolve(__dirname, repo_loc, "processed"));

	let no_times = 0;
	let c_bootstraps = 0;
	let c_benchmarks = 0;
	let c_benchmarks_add = 0;
	for (let filename of files) {
		let full_name = path.resolve(__dirname, repo_loc, "processed", filename);
		let file_contents = fs.readFileSync(full_name, "utf8");
		if (file_contents.length === 0) {
			no_times += 1;
			continue;
		}
		let contents = JSON.parse(file_contents);
		let header = contents.header;
		let times = contents.times;
		if (times.length == 0) {
			no_times += 1;
			continue;
		}
		let test_name = filename.substring(0, filename.indexOf("--"));

		let date = new Date(header.date);
		if (isNaN(date)) {
			let date_components = [0, 0, 0, 0, 0, 0];
			let actual = header.date.split("-").map(comp => parseInt(comp, 10));

			for (let i = 0; i < actual.length; i++) {
				date_components[i] = actual[i];
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

		if (test_name == "rustc") {
			c_bootstraps += 1;
			data.rustc.insertOne(make_data(date, header, times, true));

			for (let timing of times) {
				if (!crate_list.includes(timing.crate)) {
					crate_list.push(timing.crate);
				}
			}
		} else {
			if (!benchmarks.includes(test_name)) {
				benchmarks.push(test_name);
			}
			var key = data.benchmarks.key({date: date});
			if (key == null) {
				c_benchmarks += 1;
				data.benchmarks.insertOne(make_data(date, header, times, false));
			} else {
				c_benchmarks_add += 1;
				data.benchmarks[key].by_crate[test_name] = make_times(times, false)[times[0].crate];
			}
		}

		for (let timing of times) {
			for (let phase in timing.times)
				if (!phase_list.includes(phase)) {
					phase_list.push(phase);
				}
			}
		}

		if (!last_date || last_date < date) {
			last_date = date;
		}
	}

	// Post processing to generate the summary data.
	let summary_rustc = {};
	let summary_benchmarks = {};
	// We only want data for the previous 12 weeks, which means we need 13
	// data points.
	for (let i = 0; i <= WEEKS_IN_SUMMARY; i++) {
		// The date for this data point.
		var date = new Date(last_date.getTime());
		date.setDate(date.getDate() - 7 * i);

		summary_rustc = summarise_data(i, date, "rustc", summary_rustc);
		summary_benchmarks = summarise_data(i, date, "benchmarks", summary_benchmarks);
	}

	console.log("read", files.length, "files");
	console.log("found", no_times, "files without times");
	console.log(c_bootstraps, "bootstrap times");
	console.log(c_benchmarks, "benchmarks times");
	console.log(c_benchmarks_add, "benchmarks times (appended)");
}

// Compute summary data for a given date and kind (rustc bootstrap/benchmarks).
// For each date we calculate some summary, then for each but the first we
// calculate the difference from the previous week. Save it all in the summary
// global.
function summarise_data(i, date, kind, summary_state) {
	function get_percent_change(previous, current) {
		return ((previous - current) / current) * 100;
	}

	let by_crate = {};
	summary[kind][i] = { date, by_crate: {} };

	// For a given date we'll get the three most recent sets of data and take the
	// the median for each value.
	let start_idx = mk_start_idx(date, kind);
	let weeks = [];
	for (let j = 0; j < 3; ++j) {
	    let idx = Math.max(start_idx - j, 0);
	    weeks.push(data[kind][idx].by_crate);
	}
	// let week = data[kind][idx].by_crate;

	// Compute the difference as a percent for every value of data.
	for (let crate_name in weeks[0]) {
	    if (!weeks[1][crate_name] || !weeks[2][crate_name]) {
	    	continue;
	    }

		by_crate[crate_name] = {};
		let cur_summary = {};
		for (let phase of Object.keys(weeks[0][crate_name])) {
		    // Find the median value.
			let values = [
				week_data[0][crate_name][phase].time,
				week_data[1][crate_name][phase].time,
				week_data[2][crate_name][phase].time
			];
			values.sort((a, b) => a - b);
			let median = values[1];

			by_crate[crate_name][phase] = median;

			// Previous summary summarized this kind, had this crate, and
			// get it's value for the phase
			let previous_median = summary_state[crate_name] &&
				summary_state[crate_name][phase];

			if (previous_median > 0) {
				cur_summary[phase] =
					get_percent_change(previous_median, median);

				if (i == WEEKS_IN_SUMMARY &&
					summary[kind][0].by_crate[crate_name] &&
					summary[kind][0].by_crate[crate_name][phase] &&
					summary[kind][0].by_crate[crate_name][phase] > 0) {
					// We're computing the last week, so let's also
					// compute the total difference.
					summary[kind][0].by_crate[crate_name][phase] =
						get_percent_change(summary[kind][0].by_crate[crate_name][phase], median);
				}
			} else if (i == 0) {
				// Save the first week we look at so we can compute the
				// total difference, see above.
				cur_summary[phase] = median;
			}
		}

		summary[kind][i].by_crate[crate_name] = cur_summary;
	}

	return by_crate;
}


function make_data(date, header, times, rustc_crate) {
	return {
		date: date,
		commit: header.commit,
		by_crate: make_times(times, rustc_crate),
	};
}

function make_times(timings, rustc_crate) {
	let by_crate = {};
	let totals = {};

	for (let timing of timings) {
		let times = timing.times;
		let mem_values = [];
		if (timing.rss) {
			for (let phase in times) {
				times[phase].rss = timing.rss[phase];
			}

			for (let phase in timing.rss) {
				mem_values.push(timing.rss[phase]);
			}
		}

		times.total = {
			percent: 100,
			time: timing.total,
			rss: timing.rss ? Math.max.apply(null, mem_values) : 0,
		};

		for (let phase in times) {
			if (!totals[phase]) {
				totals[phase] = {
					percent: 0,
					time: 0,
					rss: 0
				};
			}
			totals[phase].time += times[phase].time;
			totals[phase].rss = Math.max(times[phase].rss, totals[phase].rss);
		}

		by_crate[timing.crate] = times;
	}

	// TODO percentages
	if (rustc_crate) {
		by_crate.total = totals;
	}
	return by_crate;
}

function start_server() {
	console.log("starting server; listening on 2346");

	function handle_path(func, req, res) {
		combine_chunks(req, function(body) {
			try {
				if (body == "") {
					respond(res, func());
				} else {
					let json = JSON.parse(body);
					respond(res, func(json));
					console.log(json);
				}
			} catch(e) {
				respond(res, "Error: " + e);
			}
		});
	}

	function respond(res, response) {
		res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
		if (typeof response == "string") {
			console.log("responding:", response);
			res.end(response);
		} else {
			res.end(JSON.stringify(response));
		}
	}

	http.createServer(function (req, res) {
		var parsed_url = url.parse(req.url, true);
		var pathname = parsed_url.pathname;
		console.log("Path parsed: " + pathname);
		if (pathname == "/data") {
			handle_path(get_data, req, res);
		} else if (pathname == "/get") {
			handle_path(get_days, req, res);
		} else if (pathname == "/get_tabular") {
			handle_path(get_tabular, req, res);
		} else if (pathname == "/stats") {
			handle_path(get_stats, req, res);
		} else if (pathname == "/info") {
			handle_path(get_info, req, res);
		} else if (pathname == "/summary") {
			handle_path(get_summary, req, res);
		} else if (pathname == "/onpush") {
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
	var body = "";
	req.on("data", function(chunk) {
		body += chunk;
	});
	req.on("end", function() {
		callback(body);
	});
}

function on_push(/*json*/) {
	// FIXME we are throwing everything away and starting again. It would be better
	// to read the added file from json and just read that file, adding it to data.
	console.log("received onpush hook");
	data = {
		rustc: sortedList.create(opts),
		benchmarks: sortedList.create(opts),
	};
	benchmarks = [];
	init(get_repo_arg(), function() {});
}

function get_info() {
	return {
		crates: crate_list,
		phases: phase_list,
		benchmarks: benchmarks,
	};
}

function get_summary() {
	let dates = summary.rustc.map(s => s.date);
	dates = dates.slice(0, 12);

	// overall number for each week
	let summaries = summary.benchmarks.map((s, i) => {
		let sum = 0;
		let count = 0;
		for (let crate_name in s.by_crate) {
			if (s.by_crate[crate_name].total) {
				sum += s.by_crate[crate_name].total;
				count += 1;
			}
		}

		if (summary.rustc[i].by_crate.total.total) {
			sum += 2 * summary.rustc[i].by_crate.total.total;
			count += 2;
		}

		return (sum / count).toFixed(1);
	});

	// per benchmark, per week
	let breakdown = summary.benchmarks.map((s, i) => {
		let per_bench = {};

		for (let crate_name in s.by_crate) {
			if (s.by_crate[crate_name].total) {
				per_bench[crate_name] = s.by_crate[crate_name].total.toFixed(1);
			} else {
				per_bench[crate_name] = 0.0;
			}
		}

		if (summary.rustc[i].by_crate.total.total) {
			per_bench.bootstrap = summary.rustc[i].by_crate.total.total.toFixed(1);
		} else {
			per_bench.bootstrap = 0.0;
		}

		return per_bench;
	});

	return {
		total_summary: summaries.shift(),
		total_breakdown: breakdown.shift(),
		breakdown: breakdown,
		summaries: summaries,
		dates: dates,
	};
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     start: Date,     // optional
//     end: Date,       // optional
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str]
// }
// crate (rustc only) or phase can be 'total'
function get_stats(body) {
	let kind = body.kind;
	if (kind != "rustc" && kind != "benchmarks") {
		return "Error: bad value kind: " + kind;
	}

	if (kind == "benchmarks" && body.crates.includes("total")) {
		return "Error: bad value crate for benchmarks";
	}

	let start = start_date(body);
	let end = end_date(body);

	let counted = [];
	// Iterate over date range.
	let start_idx = mk_start_idx(start, kind);
	let end_idx = mk_end_idx(end, kind);
	for (let i = start_idx; i <= end_idx; i++) {
		let today_data = data[kind][i];
		if (Object.keys(today_data.by_crate).length > 0) {
			if (counted.length === 0) {
				start = today_data.date;
			}
			end = today_data.date;
			counted.push(today_data);
		}
	}

	let crates = {};
	for (let crate_name of body.crates) {
		crates[crate_name] = mk_stats(counted, crate_name, body.phases);
	}

	return {
		startDate: start,
		endDate: end,
		crates: crates,
	};
}

function mk_stats(data, crate, phases) {
	let result = {};
	let count = 0;
	let first = 0;
	let skip_list = data.map((d, i) => {
		if (d.by_crate[crate] && Object.keys(d.by_crate[crate]).length > 0) {
			if (count == 0) {
				first = i;
			}

			count += 1;
			return false;
		} else {
			return true;
		}
	});

	if (count == 0) {
		result.first = 0;
		result.last = 0;
		result.min = 0;
		result.max = 0;
		result.mean = 0;
		result.variance = 0;
		result.trend = 0;
		result.trend_b = 0;
		result.n = 0;
		return result;
	}

	let sums = data.map(function(d, i) {
		if (skip_list[i]) {
			return 0;
		}

		return sumTimes(d, crate, phases);
	});

	result.first = sums[first];
	result.last = sums[sums.length - 1];

	let min = result.first;
	let max = result.first;
	const q1_idx = Math.floor(data.length / 4);
	const q4_idx = Math.floor(3 * data.length / 4);
	let total = 0;
	let q1Total = 0;
	let q4Total = 0;
	for (let i = 0; i < data.length; i++) {
		if (skip_list[i]) continue;
		let cur = sums[i];
		total += cur;
		min = Math.min(min, cur);
		max = Math.max(max, cur);
		if (i < q1_idx) { // Within the first quartile
			q1Total += cur;
		}
		if (i >= q4_idx) { // Within the fourth quartile
			q4Total += cur;
		}
	}

	// Calculate the variance
	result.mean = total / count;
	let varTotal = 0;
	for (let i = 0; i < data.length; i++) {
		if (skip_list[i]) continue;
		let diff = sums[i] - result.mean;
		varTotal += diff * diff;
	}
	result.variance = varTotal / (count - 1);

	result.n = count;
	result.min = min;
	result.max = max;
	if (count >= 10 && count == data.length) {
		let q1Mean = q1Total / q1_idx;
		let q4Mean = q4Total / (data.length - q4_idx);
		result.trend = 100 * ((q4Mean - q1Mean) / result.first);
	} else {
		result.trend = 0;
	}
	result.trend_b = 100 * ((result.last - result.first) / result.first);

	return result;
}

function sumTimes(data, crate, phases) {
	let c = data.by_crate[crate];
	let sum = 0;
	for (let phase of phases) {
		sum += c[phase]["time"];
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
	let kind = body.kind;
	if (kind != "rustc" && kind != "benchmarks") {
		return "Error: bad value kind: " + kind;
	}
	let group_by = body.group_by;
	if (group_by != "crate" && group_by != "phase") {
		return "Error: bad value group_by: " + group_by;
	}

	let result = [];
	for (let orig_date of body.dates) {
		let date = new Date(orig_date);
		if (isNaN(date)) {
			console.log("bad date:", orig_date);
			continue;
		}
		let idx = data[kind].bsearch({date: date});
		if (idx < 0 || idx >= data[kind].length) {
			idx = data[kind].length - 1;
		}
		let day = get_data_for_date(kind, idx, body.crates, body.phases, group_by);
		result.push(day);
	}

	return result;
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     date: Date
// }
function get_tabular(body, response) {
	let kind = body.kind;
	if (kind != "rustc" && kind != "benchmarks") {
		return "Error: bad value kind: " + kind;
	}
	let date = new Date(body.date);
	if (isNaN(date)) {
		return "Error: bad date: " + body.date;
	}
	let idx = data[kind].bsearch({date: date});
	if (idx < 0 || idx >= data[kind].length) {
		idx = data[kind].length - 1;
	}

	let day = data[kind][idx];

	return {
		date: day.date.toString(),
		commit: day.commit,
		data: day.by_crate
	};
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
function get_data(body) {
	let start = start_date(body);
	let end = end_date(body);

	let kind = body.kind;
	if (kind != "rustc" && kind != "benchmarks") {
		console.log("Error: bad value kind: " + kind);
		return "Error: bad value kind: " + kind;
	}
	let group_by = body.group_by;
	if (group_by != "crate" && group_by != "phase") {
		console.log("Error: bad value group_by: " + group_by);
		return "Error: bad value group_by: " + group_by;
	}

	let result = [];
	let first_idx = -1;
	let last_idx = 0;
	// Iterate over date range.
	let start_idx = mk_start_idx(start, kind);
	let end_idx = mk_end_idx(end, kind);
	for (let i = start_idx; i <= end_idx; ++i) {
		let today_data = get_data_for_date(kind, i, body.crates, body.phases, group_by);
		result.push(today_data);

		if (Object.keys(today_data.data).length > 0) {
			last_idx = i - start_idx;
			if (first_idx == -1) {
				first_idx = i - start_idx;
			}
		}
	}
	// Trim the data
	result = result.slice(first_idx, last_idx+1);

	return result;
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

function end_date(body) {
	// Handle missing end by using the last available date.
	return body.end ? new Date(body.end) : last_date;
}

function start_date(body) {
	// Handle missing start by returning 30 days before end.
	if (body.start) {
		return new Date(body.start);
	} else {
		let end = end_date(body);
		let start = new Date(end);
		start.setDate(end.getDate() - 30);
		return start;
	}
}

function get_data_for_date(kind, index, crates, phases, group_by) {
	let day = data[kind][index];

	let result = {
		"date": day.date.toString(),
		"commit": day.commit,
		"data": {},
	};

	if (group_by == "crate") {
		for (let crate_name of crates) {
			let mem = 0;
			let total = 0;
			if (crate_name in day.by_crate) {
				let cur_crate = day.by_crate[crate_name];
				for (let phase of phases) {
					if (phase in cur_crate) {
						total += cur_crate[phase]["time"];
						mem = Math.max(mem, cur_crate[phase]["rss"]);
					}
				}
				result.data[crate_name] = {
					time: total,
					rss: mem
				};
			}
		}
	} else {
		// group_by == 'phase'
		for (let phase of phases) {
			let total = 0;
			let mem = 0;
			for (let crate_name of crates) {
				if (crate_name in day.by_crate) {
					let cur_crate = day.by_crate[crate_name];
					if (phase in cur_crate) {
						total += cur_crate[phase]["time"];
						mem = Math.max(mem, cur_crate[phase]["rss"]);
					}
				}
			}
			result.data[phase] = {
				time: total,
				rss: mem
			};
		}
	}

	return result;
}
