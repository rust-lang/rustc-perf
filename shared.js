var BASE_URL = "http://perf.rust-lang.org/perf";

// Radio button event listener.
var radios = document.getElementsByName("kind");
var prev_kind = null;
for (var r in radios) {
    radios[r].onclick = radio_handler;
}

function radio_handler() {
    if (prev_kind !== this) {
        prev_kind = this;

        if (this.value == "rustc") {
            document.getElementById("crates_label").innerHTML = "Crates";
            document.getElementById("crates").style.display = "block";
            document.getElementById("benchmarks").style.display = "none";
        } else {
            document.getElementById("crates_label").innerHTML = "Benchmarks";
            document.getElementById("crates").style.display = "none";
            document.getElementById("benchmarks").style.display = "block";
        }
    }
}

function get_kind() {
    var radios = document.getElementsByName('kind');

    for (var r in radios) {
        if (radios[r].checked) {
            return radios[r].value;
        }
    }
}

function getDate(id) {
    var result = document.getElementById(id).value;
    if (result) {
        var as_date = new Date(result);
        if (isNaN(as_date.getTime())) {
            result = "";
            document.getElementById(id).value = "Invalid date";
        } else {
            result = as_date.toISOString();
        }
    }

    return result;
}

function gatherChecks(name) {
    if (document.getElementById(name + "-total") && document.getElementById(name + "-total").checked) {
        return ["total"];
    }
    var result = [];
    var elements = document.getElementsByName(name);
    for (var i in elements) {
        if (elements[i].checked)
        {
            result.push(elements[i].id);
        }
    }

    return result;
}

function addTotalHandler(name) {
    var elements = document.getElementsByName(name);
    for (var i in elements) {
        if (elements[i].id != name + "-total") {
            elements[i].onclick = function() {
                document.getElementById(name + "-total").checked = false;
            };
        }
    }
    document.getElementById(name + "-total").onclick = function() {
        for (var i in elements) {
            if (elements[i].id != name + "-total") {
                elements[i].checked = false;
            }
        }
    };
}

// Get lists of the available crates and phases from the server and make
// the lists of checkboxes and other settings.
// Assumes the initial graph is total/total/by crate
function make_settings(callback, total_label) {
    function checkbox(name, id, checked, body) {
        if (checked) {
            return "<input type=\"checkbox\" checked=\"true\" name=\"" + name + "\" id=\"" + id + "\">" + body + "</input></br>";
        } else {
            return "<input type=\"checkbox\" name=\"" + name + "\" id=\"" + id + "\">" + body + "</input></br>";
        }
    }

    if (!total_label) {
        total_label = "total";
    }
    return fetch(BASE_URL + "/info", {}).then(function(response) {
        response.json().then(function(data) {
            var crates_html = checkbox("check-crate", "check-crate-total", true, total_label);
            for (let crate of data.crates) {
                crates_html += checkbox("check-crate", crate, false, truncate_name(crate));
            }
            document.getElementById("crates").innerHTML = crates_html;

            var bench_html = "";
            for (let bench of data.benchmarks) {
                bench_html += checkbox("check-bench", bench, false, truncate_name(bench));
            }
            document.getElementById("benchmarks").innerHTML = bench_html;

            var phases_html = checkbox("check-phase", "check-phase-total", true, total_label);
            for (let phase of data.phases) {
                if (phase != "total") {
                    phases_html += checkbox("check-phase", phase, false, truncate_name(phase));
                }
            }
            document.getElementById("phases").innerHTML = phases_html;

            var groupByCrate = document.getElementById("group-by-crate");
            if (groupByCrate) {
                groupByCrate.checked = true;
            }

            addTotalHandler("check-crate");
            addTotalHandler("check-phase");

            if (callback) {
                callback();
            }
        });
    }, function(err) {
        document.getElementById("settings").innerHTML = "Error fetching info";
        console.log("Error fetching info:");
        console.log(err);
    });
}

function make_as_of() {
    return fetch(BASE_URL + "/info", {}).then(function(response) {
        response.json().then(function(data) {
            document.getElementById("as-of").innerHTML = "Updated as of: " + (new Date(data.as_of)).toLocaleString();
        });
    }, function(err) {
        document.getElementById("as-of").innerHTML = "Error fetching info";
        console.log("Error fetching info:");
        console.log(err);
    });
}

function truncate_name(name) {
    if (name.length > 30) {
        return name.substring(0, 30) + "...";
    }

    return name;
}

function set_dates(start_date, end_date) {
    if (!start_date || !end_date) {
        return;
    }

    document.getElementById("start-date").value = new Date(start_date).toISOString().split('T')[0];
    document.getElementById("end-date").value = new Date(end_date).toISOString().split('T')[0];
}

function set_date(date) {
    document.getElementById("date").value = new Date(date).toISOString().split('T')[0];
}

function set_check_boxes(kind, crates, phases, group_by) {
    // Set the kind radio button and hide/show other groups as appropriate.
    var radios = document.getElementsByName('kind');
    for (var r in radios) {
        radios[r].checked = radios[r].value == kind;
    }
    var crates_total_name = null;
    if (kind == "rustc") {
        crates_total_name = "check-crate-total";
        document.getElementById("crates_label").innerHTML = "Crates";
        document.getElementById("crates").style.display = "block";
        document.getElementById("benchmarks").style.display = "none";

        // Clear checkboxes
        var ck_crates = document.getElementsByName('check-crate');
        for (var i in ck_crates) {
            ck_crates[i].checked = false;
        }
    } else {
        document.getElementById("crates_label").innerHTML = "Benchmarks";
        document.getElementById("crates").style.display = "none";
        document.getElementById("benchmarks").style.display = "block";

        // Clear checkboxes
        var ck_benches = document.getElementsByName('check-bench');
        for (var i in ck_benches) {
            ck_benches[i].checked = false;
        }
    }

    var ck_phases = document.getElementsByName('check-phase');
    for (var i in ck_phases) {
        ck_phases[i].checked = false;
    }

    // Check crates/benchmarks/phases checkboxes.
    for (var i in crates) {
        var id = crates[i];
        if (!id) {
            continue;
        }
        if (id == "total") {
            id = crates_total_name;
        }
        var ck = document.getElementById(id);
        if (ck) {
            ck.checked = true;
        } else {
            console.log("Couldn't find", id, i, crates[i]);
        }
    }
    for (var i in phases) {
        var id = phases[i];
        if (!id) {
            continue;
        }
        if (id == "total") {
            id = "check-phase-total";
        }
        var ck = document.getElementById(id);
        if (ck) {
            ck.checked = true;
        } else {
            console.log("Couldn't find", id, i, phases[i]);
        }
    }

    if (group_by) {
        var radios = document.getElementsByName("groupBy");
        for (var r in radios) {
            radios[r].checked = radios[r].value == group_by;
        }
    }
}

// A bunch of helper functions for helping with keeping URLs up to date and
// interacting with the browser history.

function dispatch_on_state(f, state, keys) {
    if (state) {
        var args = keys.map(k => state[k]);
        args.push(false);
        f.apply(null, args);
        return true;
    }
    return false;
}

function dispatch_on_params(f, keys) {
    if (!window.location.search) {
        return false;
    }
    var params = new URLSearchParams(window.location.search.substring(1));
    if (keys.map(k => params.has(k)).reduce((a, b) => a && b, true)) {
        var args = keys.map(k => get_param(k, params));
        args.push(false);
        f.apply(null, args);
        return true;
    }
    return false;
}

function get_param(key, params) {
    var result = params.get(key);
    if (key == "crates" || key == "phases" || key == "dates") {
        result = result.split(',');
    }
    return result;
}

function push_state_to_history(keys, values) {
    var state = {};
    for (k in keys) {
        state[keys[k]] = values[k];
    }
    var query_string = query_string_for_state(state);
    history.pushState(state, "", query_string);
}

function query_string_for_state(state) {
    var result = "?";
    for (k in state) {
        if (result.length > 1) {
            result += "&";
        }
        result += k + "=" + encodeURIComponent(state[k]);
    }
    return result;
}

// This one is for making the request we send to the backend.
function make_request(keys, values) {
    var body = {};
    for (k in keys) {
        body[keys[k]] = values[k];
    }

    return {
        method: "POST",
        body: JSON.stringify(body),
        mode: "cors"
    };
}
