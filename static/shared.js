var BASE_URL = window.location.origin + "/perf";

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
    let total_checked = false;
    if (document.getElementById(name + "-total") && document.getElementById(name + "-total").checked) {
        total_checked = true;
    }
    var result = [];
    var elements = document.getElementsByName(name);
    for (var i in elements) {
        if ((elements[i].checked || total_checked) &&
            elements[i].id && elements[i].id != name + "-total") {
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
            return `<input type="checkbox" checked="true" name="${name}" id="${id}">${body}</input></br>`;
        } else {
            return `<input type="checkbox" name="${name}" id="${id}">${body}</input></br>`;
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
            addTotalHandler("check-crate");

            var phases_html = checkbox("check-phase", "check-phase-total", true, total_label);
            for (let phase of data.phases) {
                phases_html += checkbox("check-phase", phase, false, truncate_name(phase));
            }
            document.getElementById("phases").innerHTML = phases_html;
            addTotalHandler("check-phase");

            var groupByCrate = document.getElementById("group-by-crate");
            if (groupByCrate) {
                groupByCrate.checked = true;
            }

            if (callback) {
                // Respond to back/forwards properly.
                window.onpopstate = function() {
                    dispatch_on_params(callback);
                };

                // Load the page from a URL.
                dispatch_on_params(callback);
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

function set_date(id, date) {
    let d = new Date(date);
    if (!Number.isNaN(d.getTime())) {
        document.getElementById(id).value = new Date(date).toISOString().split('T')[0];
    } else {
        console.warn("could not set", id, "to:", date, "; invalid date");
    }
}

function set_check_boxes(crates, phases, group_by) {
    let is_total_crate_checked = document.getElementById("check-crate-total").checked;
    let is_total_phase_checked = document.getElementById("check-phase-total").checked;

    // Clear checkboxes
    var ck_crates = document.getElementsByName('check-crate');
    for (var i in ck_crates) {
        if (ck_crates[i].id !== "check-crate-total") {
            ck_crates[i].checked = false;
        }
    }

    var ck_phases = document.getElementsByName('check-phase');
    for (var i in ck_phases) {
        if (ck_phases[i].id !== "check-phase-total") {
            ck_phases[i].checked = false;
        }
    }

    // Check crates/benchmarks/phases checkboxes.
    if (!is_total_crate_checked) {
        for (let id of crates) {
            if (!id) {
                continue;
            }
            if (id == "total") {
                id = "check-crate-total";
            }
            let ck = document.getElementById(id);
            if (ck) {
                ck.checked = true;
            } else {
                console.log("Couldn't find", id, i, crates[i]);
            }
        }
    }
    if (!is_total_phase_checked) {
        for (let id of phases) {
            if (!id) {
                continue;
            }
            var ck = document.getElementById(id);
            if (ck) {
                ck.checked = true;
            } else {
                console.log("Couldn't find", id, i, phases[i]);
            }
        }
    }

    if (group_by) {
        var radios = document.getElementsByName("groupBy");
        for (var r in radios) {
            radios[r].checked = radios[r].value == group_by;
        }
    }
}

function get_group_by() {
    // group by
    let group_by;
    let elements = document.getElementsByName("groupBy");
    for (let element of elements) {
        if (element.checked) {
            group_by = element.value;
            break;
        }
    }
    if (!group_by) {
        group_by = "crate";
        document.getElementById("group-by-crate").checked = true;
    }

    return group_by;
}

// A bunch of helper functions for helping with keeping URLs up to date and
// interacting with the browser history.

// Dispatches on query string state. This examines the ?... part of the URL.
//
// The query string may not contain all of the parameters. If it doesn't, we simply pass what
// exists.
function dispatch_on_params(f) {
    if (!window.location.search) {
        f({}, false);
        return;
    }

    let params = new URLSearchParams(window.location.search.substring(1));
    let state = {};
    for (let param of params) {
        let key = param[0];
        let value = param[1];

        if (key == "crates" || key == "phases" || key == "dates") {
            value = value.split(",");
        }

        state[key] = value;
    }
    f(state, false);
}

// Add an entry into the browser's history to allow the back button to work correctly.
//
// This does not refresh the page in any way.
//
// For more information, see:
// https://developer.mozilla.org/en-US/docs/Web/API/History_API#The_pushState()_method
function push_state_to_history(state) {
    var query_string = query_string_for_state(state);
    if (query_string !== "") {
        history.pushState(state, "", query_string);
    }
}

function query_string_for_state(state) {
    var result = "?";
    for (k in state) {
        if (result.length > 1) {
            result += "&";
        }
        // Best known way to check if passed state is a date object.
        if (state[k].toISOString) {
            result += k + "=" + encodeURIComponent(state[k].toISOString());
        } else {
            result += k + "=" + encodeURIComponent(state[k]);
        }
    }
    return result;
}

// This one is for making the request we send to the backend.
function make_request(body) {
    return {
        method: "POST",
        body: JSON.stringify(body),
        mode: "cors"
    };
}

make_as_of();
