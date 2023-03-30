function render(element, name, data, versions) {
    let articles = {"check": "a", "debug": "a", "opt": "an", "doc": "a"};
    new Highcharts.chart(document.getElementById(element), {
        chart: {
            zoomType: "xy",
            renderTo: document.getElementById(element),
            type: "line",
        },
        title: {
            text: `Average time for ${articles[name]} ${name} build`,
        },
        yAxis: {
            title: {text: "Seconds"},
            min: 0,
        },
        xAxis: {
            categories: versions,
            title: {text: "Version"},
        },
        series: [
            {
                name: "full",
                animation: false,
                data: data.clean_averages,
            },
            {
                name: "incremental full",
                animation: false,
                data: data.base_incr_averages,
            },
            {
                name: "incremental unchanged",
                animation: false,
                data: data.clean_incr_averages,
            },
            {
                name: "incremental patched: println",
                animation: false,
                data: data.println_incr_averages,
            },
        ],
    });
}

function populate_data(data) {
    data = data.Ok;
    render("check-average-times", "check", data.check, data.versions);
    render("debug-average-times", "debug", data.debug, data.versions);
    render("opt-average-times", "opt", data.opt, data.versions);
    render("doc-average-times", "doc", data.doc, data.versions);
}

function make_data() {
    fetch(BASE_URL + "/dashboard", {}).then(function (response) {
        response.json().then(data => populate_data(data));
    });
}

make_data();
