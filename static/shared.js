var BASE_URL = window.location.origin + "/perf";

function getDate(id) {
    var result = document.getElementById(id).value;
    var as_date = new Date(result);
    if (isNaN(as_date.getTime())) {
        return "";
    } else {
        return as_date.toISOString();
    }
}

function gatherChecks(name) {
    if (document.getElementById(name + "-total") &&
        document.getElementById(name + "-total").checked) {
        return { list: "All", content: null }; // Decoded as all variants
    }
    let result = [];
    let elements = document.getElementsByName(name);
    for (let element of elements) {
        if (element.checked && element.id && element.id != name + "-total") {
            result.push(element.id);
        }
    }

    return { list: "List", content: result };
}

function toList(list_object, type) {
    if (list_object.list == "All") {
        let result = [];
        let elements = document.getElementsByName(name);
        for (let element of elements) {
            if (element.checked && element.id && element.id != `check-${type}-total`) {
                result.push(element.id);
            }
        }

        return result;
    }

    return list_object.content;
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
            return `<label><input type="checkbox" checked="true" name="${name}" id="${id}">${body}</label></br>`;
        } else {
            return `<label><input type="checkbox" name="${name}" id="${id}">${body}</label></br>`;
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
    // Clear checkboxes
    var ck_crates = document.getElementsByName('check-crate');
    for (var i in ck_crates) {
        ck_crates[i].checked = false;
    }

    var ck_phases = document.getElementsByName('check-phase');
    for (var i in ck_phases) {
        ck_phases[i].checked = false;
    }

    var totalCratesSet = crates.list == "All";
    var totalPhasesSet = phases.list == "All";
    var crates = toList(crates);
    var phases = toList(phases);

    if (totalCratesSet) {
        document.getElementById("check-crate-total").checked = true;
    }
    if (totalPhasesSet) {
        document.getElementById("check-phase-total").checked = true;
    }

    // Check crates/benchmarks/phases checkboxes.
    for (let id of crates) {
        let ck = document.getElementById(id);
        if (ck) {
            ck.checked = true;
        } else {
            console.log("Couldn't find", id);
        }
    }
    for (let id of phases) {
        let ck = document.getElementById(id);
        if (ck) {
            ck.checked = true;
        } else {
            console.log("Couldn't find", id);
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
            value = JSON.parse(value);
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
        } else if (typeof state[k] == "string") {
            result += k + "=" + encodeURIComponent(state[k]);
        } else {
            result += k + "=" + encodeURIComponent(JSON.stringify(state[k]));
        }
    }
    return result;
}

// This one is for making the request we send to the backend.
function make_request(path, body) {
    return fetch(BASE_URL + path, {
        method: "POST",
        body: JSON.stringify(body),
        mode: "cors"
    });
}

make_as_of();
