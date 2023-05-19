import {STATUS_DATA_URL} from "../urls";

import {getJson} from "../utils/requests";

interface Commit {
    sha: string;
    date: string;
    type: "Try" | "Master";
}

interface BenchmarkStatus {
    name: string;
    error: string;
}

interface Step {
    step: string;
    is_done: boolean;
    expected_duration: number;
    current_progress: number;
}

/**
 * The `any` types in the interface below were chosen because the types are quite complex
 * on the Rust side (they are modeled with enums encoded in a way that is not so simple to model in
 * TS).
 */
interface CurrentState {
    artifact: any;
    progress: [Step];
}

interface StatusResponse {
    last_commit: Commit | null
    benchmarks: [BenchmarkStatus],
    missing: [[Commit, any]],
    current: CurrentState | null,
    most_recent_end: number | null,
}

function populate_data(data: StatusResponse) {
    let state_div = document.querySelector("#benchmark-state");
    if (data.last_commit !== null) {
        let element = document.createElement("p");
        element.innerHTML = `SHA: ${data.last_commit.sha}, date: ${data.last_commit.date}`;
        state_div.appendChild(element);
    }
    for (let benchmark of data.benchmarks) {
        let element = document.createElement("div");
        element.innerHTML = `<details open>
                    <summary>${benchmark.name} - error</summary>
                    <pre class="benchmark-error"></pre>
                </details>`;
        element.querySelector<HTMLElement>(".benchmark-error").innerText = benchmark.error;
        state_div.appendChild(element);
    }
    let missing_div = document.querySelector("#data-insert-js");
    if (data.current !== null) {
        let table = document.createElement("table");
        let tr = document.createElement("tr");
        let th = document.createElement("th");
        th.innerText = "Step";
        tr.appendChild(th);
        th = document.createElement("th");
        tr.appendChild(th);
        th = document.createElement("th");
        th.innerText = "Took";
        tr.appendChild(th);
        th = document.createElement("th");
        th.innerText = "Expected";
        tr.appendChild(th);
        table.appendChild(tr);

        let left = 0;
        for (let step of data.current.progress) {
            let tr = document.createElement("tr");
            let td = document.createElement("td");
            td.innerText = step.step;
            tr.appendChild(td);
            td = document.createElement("td");
            let progress = document.createElement("progress");
            progress.setAttribute("max", step.expected_duration.toString());
            progress.setAttribute("value", (step.is_done ? step.expected_duration : step.current_progress).toString());
            td.appendChild(progress);
            tr.appendChild(td);
            td = document.createElement("td");
            td.innerHTML = step.current_progress == 0 ? "" :
                format_duration(step.current_progress);
            td.style.textAlign = "right";
            tr.appendChild(td);
            td = document.createElement("td");
            td.innerHTML = format_duration(step.expected_duration);
            td.style.textAlign = "right";
            tr.appendChild(td);
            if (!step.is_done) {
                left += step.expected_duration - step.current_progress;
            }
            table.appendChild(tr);
        }
        let element = document.createElement("p");
        let artifact_desc = "";
        if (data.current.artifact.Commit) {
            artifact_desc = commit_url(data.current.artifact.Commit);
        } else {
            artifact_desc = data.current.artifact.Tag;
        }
        element.innerHTML = `Currently benchmarking: ${artifact_desc}.
                <br>Time left: ${format_duration(left)}`;
        missing_div.appendChild(element);
        missing_div.appendChild(table);
    } else {
        let element = document.createElement("p");
        if (data.most_recent_end) {
            let end = new Date(data.most_recent_end * 1000);
            element.innerHTML = `No current collection in progress. Last one
                    finished at ${end.toLocaleString()} local time,
                    ${format_duration(Math.trunc((Date.now() - end.getTime()) / 1000))} ago.`;
        } else {
            element.innerHTML = "No current collection in progress.";
        }
        missing_div.appendChild(element);
    }
    {
        let element = document.createElement("p");
        element.innerHTML = `Queue (${data.missing.length} total):<br> Times are local.`;
        missing_div.appendChild(element);
    }
    let table = document.createElement("table");
    table.id = "missing-commits";
    {
        let row = document.createElement("tr");
        row.innerHTML = `<th>Commit Date</th><th>SHA</th><th>Reason</th>`;
        table.appendChild(row);
    }
    for (let [commit, reason] of data.missing) {
        let row = document.createElement("tr");
        {
            let date = new Date(commit.date);
            let element = document.createElement("td");
            if (date.getUTCFullYear() == 2001) {
                element.innerHTML = "try commit";
                element.style.textAlign = "center";
            } else {
                element.innerHTML = new Date(commit.date).toLocaleString();
            }
            row.appendChild(element);
        }
        {
            let element = document.createElement("td");
            element.innerHTML = commit_url(commit);
            row.appendChild(element);
        }
        {
            let element = document.createElement("td");
            element.innerHTML = reason_to_string(reason);
            row.appendChild(element);
        }
        table.appendChild(row);
    }
    missing_div.appendChild(table);
}

function reason_to_string(reason) {
    if (typeof reason == 'string') {
        return reason;
    } else if (reason.InProgress) {
        return `${reason_to_string(reason.InProgress)} - in progress`;
    } else if (reason["Master"] != undefined && reason.Master.pr) {
        return `<a href="https://github.com/rust-lang/rust/pull/${reason["Master"].pr}">
                    #${reason["Master"].pr}</a>${reason.Master.is_try_parent ? " - Try commit parent" : ""
        }`;
    } else if (reason["Master"] != undefined && reason.Master.pr == 0) {
        return "Master";
    } else if (reason["Try"] != undefined && reason.Try.pr) {
        return `
                Try for
                <a href="https://github.com/rust-lang/rust/pull/${reason["Try"].pr}">
                    #${reason["Try"].pr}
                </a>`;
    } else {
        // Should never happen, but a good fallback
        return JSON.stringify(reason);
    }
}

function commit_url(commit) {
    return `<a href="https://github.com/rust-lang/rust/commit/${commit.sha}">${commit.sha.substr(0, 13)}</a>`;
}

function format_duration(seconds) {
    let secs = seconds % 60;
    let mins = Math.trunc(seconds / 60);
    let hours = Math.trunc(mins / 60);
    mins -= hours * 60;

    let s = "";
    if (hours > 0) {
        s = `${hours}h${mins < 10 ? "0" + mins : mins}m${secs < 10 ? "0" + secs : secs}s`;
    } else {
        s = `${mins < 10 ? " " + mins : mins}m${secs < 10 ? "0" + secs : secs}s`;
    }
    return s;
}

async function make_data() {
    const response = await getJson<StatusResponse>(STATUS_DATA_URL);
    populate_data(response);
}

make_data();
