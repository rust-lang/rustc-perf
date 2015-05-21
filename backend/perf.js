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

// Data stored by test name and date, each date maps to a table storing the times by crate and by phase.
// See make_data() for the object layout.
var data = {};
data['rustc'] = {};
data['benchmarks'] = {};
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
        files.map(function (filename) {
            var full_name = path.resolve(__dirname, repo_loc, 'processed', filename);
            var contents = JSON.parse(fs.readFileSync(full_name, 'utf8'));
            var header = contents.header;
            var times = contents.times;
            var test_name = filename.substring(0, filename.indexOf('--'));

            var date = new Date(header.date);
            if (!data[test_name]) {
                data[test_name] = {};
                if (test_name != 'rustc') {
                    benchmarks.push(test_name);
                }
            }
            if (test_name == 'rustc') {
                data['rustc'][date] = make_data(header, times, true);
            } else {
                if (!data['benchmarks'][date]) {
                    data['benchmarks'][date] = make_data(header, times, false);
                } else {
                    data['benchmarks'][date].by_crate[test_name] = make_times(times, false)[times[0].crate];
                }
            }
            if (!last_date || last_date < date) {
                last_date = date;
            }
        });

        console.log(benchmarks);
        console.log(crate_list);
    });
}


function make_data(header, times, rustc_crate) {
    if (!header || !times || times.length == 0) {
        return {};
    }

    return {
        'date': header.date,
        'commit': header.commit,
        'by_crate': make_times(times, rustc_crate),
    };
}

function make_times(times, rustc_crate) {
    var by_crate = {};
    var totals = {}
    times.map(function(t) {
        by_crate[t.crate] = t.times;
        by_crate[t.crate]['total'] = {
            "percent": 100, 
            "time": t.total
        };
        for (var phase in by_crate[t.crate]) {
            if (!totals[phase]) {
                totals[phase] = {
                    "percent": 0, 
                    "time": 0                    
                };
            }
            totals[phase].time += by_crate[t.crate][phase].time;
            record_phase(phase);
        }
        if (rustc_crate) {
            record_crate(t.crate);
        }
    });
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
        // TODO support getting breakdowns for single days
        if (pathname == '/data') {
            combine_chunks(req, function(body) {
                try {
                    var json = JSON.parse(body);
                    get_data(json, res);
                } catch(e) {
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Error: " + e);
                }
            });
        } else if (pathname == '/info') {
            get_info(res);
        } else if (pathname == '/onpush') {
            combine_chunks(req, function(body) {
                try {
                    var json = JSON.parse(body);
                    on_push(json);
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Success?");
                } catch (e) {
                    res.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
                    res.end("Error: " + e);
                }
            });
        } else {
            res.writeHead(404, {"Content-Type": "text/plain"});
            res.write("404 Not Found\n");
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
        return;
    } catch (e) {
        console.log(e);
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: " + e);
        return;
    }
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     start: Date,     // optional
//     end: Date,       // optional
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str],
//     group_by: 'crate' | 'phase'
// }
// crate or phase can be 'total'
function get_data(body, response) {
    try {
        // Handle missing end by using the last available date.
        var end = body.end;
        if (end) {
            end = new Date(end);
        } else {
            end = last_date;
        }
        end.setHours(23,59,59,999);
        // Handle missing start by returning 60 days before end.
        var start = body.start;
        if (start) {
            start = new Date(start);
        } else {
            start = new Date();
            start.setDate(end.getDate() - 60);
        }

        var kind = body.kind;
        if (kind != 'rustc' && kind != 'benchmarks') {
            response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
            response.end("Error: bad value kind: " + kind);
            return;
        }
        var crates = body.crates;
        var phases = body.phases;
        var group_by = body.group_by;
        if (group_by != 'crate' && group_by != 'phase') {
            response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
            response.end("Error: bad value group_by: " + group_by);
            return;
        }

        var result = [];
        var first_idx = 0;
        var last_idx = 0;
        // Iterate over date range.
        for (var d = start, i = 0; start <= end; d.setDate(d.getDate() + 1, ++i)) {
            var today_data = get_data_for_date(kind, d, crates, phases, group_by);
            result.push(today_data);

            var empty = Object.keys(today_data.data).length == 0;
            if (!empty) {
                last_idx = i;
                if (first_idx == 0) {
                    first_idx = i;
                }
            }
        }

        // Trim the data
        result = result.slice(first_idx, last_idx+1);

        response.writeHead(200, {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"});
        response.end(JSON.stringify(result));
        return;
    } catch (e) {
        console.log(e);
        response.writeHead(200, {"Content-Type": "text/plain", "Access-Control-Allow-Origin": "*"});
        response.end("Error: " + e);
        return;
    }
}

function get_data_for_date(kind, date, crates, phases, group_by) {
    var day = data[kind][date];
    if (!day || Object.keys(day).length == 0) {
        // No data for that date.
        return {
            'date': date.toDateString(),
            'commit': '',
            'data': {},
        };
    }

    var result = {
        'date': day.date,
        'commit': day.commit,
        'data': {},
    };
    if (group_by == 'crate') {
        for (var c in crates) {
            var crate = crates[c];
            if (crate in day.by_crate) {
                var cur_crate = day.by_crate[crate];
                var total = 0;
                for (var p in phases) {
                    var phase = phases[p];
                    if (phase in cur_crate) {
                        total += cur_crate[phase]['time'];
                    }
                }
                result.data[crates[c]] = total;
            }
        }
    } else {
        // group_by == 'phase'
        for (var p in phases) {
            var total = 0;
            var phase = phases[p];
            for (var c in crates) {
                var crate = crates[c];
                if (crate in day.by_crate) {
                    var cur_crate = day.by_crate[crate];
                    if (phase in cur_crate) {
                        total += cur_crate[phase]['time'];
                    }
                }
            }
            result.data[phases[p]] = total;
        }
    }

    return result;
}
