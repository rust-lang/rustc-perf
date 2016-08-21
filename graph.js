var chart;

function init_graph(data, series, field, total_label, yAxisLabel) {
    if (!total_label) {
        total_label = "total";
    }

    var values = [];

    for (var i = 0; i < series.length; i++) {
        values.push([]);
    }

    for (var x = 0; x < data.length; x++) {
        let datum = data[x];
        var date = new Date(datum.date);

        var url = null;
        if (x > 0) {
            url = "https://github.com/rust-lang/rust/compare/" + data[x - 1].commit + "..." + data[x].commit;
        }

        for (var i = 0; i < series.length; i++) {
            let value = {
                commit: datum.commit,
                url: url,
                y: 0,
                x: date.valueOf() // number of milliseconds since 1970
            };
            if (series[i] in datum.data) {
                if (field in datum.data[series[i]]) {
                    value.y = datum.data[series[i]][field];
                }
            }
            value.name = date.toLocaleString();
            values[i].push(value);
        }
    }

    var datasets = [];
    for (var i = 0; i < series.length; i++) {
        var label = series[i];
        if (label == "total") {
            label = total_label;
        }
        datasets.push({
            name: label,
            animation: false,
            allowPointSelect: true,
            data: values[i],
            marker: {
                enabled: true
            },
        });
    }

    if (chart) {
        chart.destroy();
    }
    chart = new Highcharts.Chart({
        chart: {
            zoomType: "x",
            renderTo: "chart-container",
            type: "line",
        },
        title: {
            text: "",
        },
        rangeSelector: {
            selected: 1,
        },
        series: datasets,
        tooltip: {
            shared: true,
            crosshairs: [true],
            formatter: function () {
                var date = new Date(this.x);
                var commit = this.points[0].point.commit.substr(0, 10);
                var s = "<b>" + date.toLocaleString() + " - " + commit + "</b>";
                for (let point of this.points) {
                    s += "<br/>" + point.series.name + ": " + point.y;
                }
                return s;
            }
        },
        xAxis: {
            type: "datetime",
        },
        yAxis: {
            min: 0,
            title: {
                text: yAxisLabel,
            }
        },
        plotOptions: {
            line: {
                point: {
                    events: {
                        click: function (event) {
                            if (this.options.url) {
                                window.open(this.options.url, "_blank");
                            }
                            return false;
                        }
                    }
                }
            }
        }
    });
}
