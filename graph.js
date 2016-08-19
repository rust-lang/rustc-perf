var chart;

Chart.types.Line.extend({
    name: "MissingLine",
    initialize: function(data) {
        // Kind of sad we have to dup all this code from the Line chart,
        // but such is life. This whole function (and draw) is adjusted copy-pasta from
        // https://github.com/nnnick/Chart.js/blob/master/src/Chart.Line.js.
        // See http://jsfiddle.net/leighking2/sLgefm04/6/ for the inspiration.

        var helpers = Chart.helpers;
        this.options.bezierCurve = false;

        //Declare the extension of the default point, to cater for the options passed in to the constructor
        this.PointClass = Chart.Point.extend({
            strokeWidth : this.options.pointDotStrokeWidth,
            radius : this.options.pointDotRadius,
            display: this.options.pointDot,
            hitDetectionRadius : this.options.pointHitDetectionRadius,
            ctx : this.chart.ctx,
            inRange : function(mouseX){
                return (Math.pow(mouseX-this.x, 2) < Math.pow(this.radius + this.hitDetectionRadius,2));
            }
        });

        this.datasets = [];

        //Set up tooltip events on the chart
        if (this.options.showTooltips){
            helpers.bindEvents(this, this.options.tooltipEvents, function(evt){
                var activePoints = (evt.type !== 'mouseout') ? this.getPointsAtEvent(evt) : [];
                this.eachPoints(function(point){
                    point.restore(['fillColor', 'strokeColor']);
                });
                helpers.each(activePoints, function(activePoint){
                    activePoint.fillColor = activePoint.highlightFill;
                    activePoint.strokeColor = activePoint.highlightStroke;
                });
                this.showTooltip(activePoints);
            });
        }

        //Iterate through each of the datasets, and build this into a property of the chart
        helpers.each(data.datasets,function(dataset){
            var datasetObject = {
                label : dataset.label || null,
                fillColor : dataset.fillColor,
                strokeColor : dataset.strokeColor,
                pointColor : dataset.pointColor,
                pointStrokeColor : dataset.pointStrokeColor,
                points : []
            };

            this.datasets.push(datasetObject);

            helpers.each(dataset.data,function(dataPoint,index){
                //Add a new point for each piece of data, passing any required data to draw.
                datasetObject.points.push(new this.PointClass({
                    value : dataPoint,
                    label : data.labels[index],
                    datasetLabel: dataset.label,
                    strokeColor : dataset.pointStrokeColor,
                    fillColor : dataset.pointColor,
                    highlightFill : dataset.pointHighlightFill || dataset.pointColor,
                    highlightStroke : dataset.pointHighlightStroke || dataset.pointStrokeColor,
                    ignore: dataPoint <= 0
                }));
            },this);

            // Skip some labels if there are lots of data.
            var step = Math.max(1, Math.floor(data.labels.length / 12));
            var steppedLabels = [];
            for (var i = 0; i < data.labels.length; ++i) {
                if (i % step == 0) {
                    steppedLabels.push(data.axisLabels[i]);
                } else {
                    steppedLabels.push("");
                }
            }
            this.buildScale(steppedLabels);

            this.eachPoints(function(point, index){
                helpers.extend(point, {
                    x: this.scale.calculateX(index),
                    y: this.scale.endPoint
                });
                point.save();
            }, this);

        },this);

        if (data.datasets.length == 0) {
            this.buildScale([]);
        }

        this.render();
    },

    draw: function(ease) {
        var helpers = Chart.helpers;

        var easingDecimal = ease || 1;
        this.clear();

        var ctx = this.chart.ctx;

        // Some helper methods for getting the next/prev points
        var hasValue = function(item){
            return item.value !== null;
        },
        nextPoint = function(point, collection, index){
            return helpers.findNextWhere(collection, hasValue, index) || point;
        },
        previousPoint = function(point, collection, index){
            return helpers.findPreviousWhere(collection, hasValue, index) || point;
        };

        this.scale.draw(easingDecimal);


        helpers.each(this.datasets,function(dataset){
            var pointsWithValues = helpers.where(dataset.points, hasValue);

            //Transition each point first so that the line and point drawing isn't out of sync
            //We can use this extra loop to calculate the control points of this dataset also in this loop

            helpers.each(dataset.points, function(point, index){
                if (point.hasValue()){
                    point.transition({
                        y : this.scale.calculateY(point.value),
                        x : this.scale.calculateX(index)
                    }, easingDecimal);
                }
            },this);

            //Draw the line between all the points
            ctx.lineWidth = this.options.datasetStrokeWidth;
            ctx.strokeStyle = dataset.strokeColor;
            ctx.beginPath();

            helpers.each(pointsWithValues, function(point, index){
                if (index === 0){
                    ctx.moveTo(point.x, point.y);
                }
                else if (!point.ignore) {
                    ctx.lineTo(point.x,point.y);
                }
            }, this);

            ctx.stroke();

            if (this.options.datasetFill && pointsWithValues.length > 0){
                //Round off the line by going to the base of the chart, back to the start, then fill.
                ctx.lineTo(pointsWithValues[pointsWithValues.length - 1].x, this.scale.endPoint);
                ctx.lineTo(pointsWithValues[0].x, this.scale.endPoint);
                ctx.fillStyle = dataset.fillColor;
                ctx.closePath();
                ctx.fill();
            }

            //Now draw the points over the line
            //A little inefficient double looping, but better than the line
            //lagging behind the point positions
            helpers.each(pointsWithValues,function(point){
                if (!point.ignore) {
                    point.draw();
                }
            });
        },this);
    }
});

function init_graph(data, series, field, total_label) {
    if (!total_label) {
        total_label = "total";
    }

    var ctx = document.getElementById("graph").getContext("2d");

    var labels = [];
    var axisLabels = [];
    var values = [];

    for (var i in series) { values.push([]) }
    for (d in data) {
        var datum = data[d];

        var date = new Date(datum.date);
        labels.push(date.toLocaleString() + " - " + datum.commit.substr(0, 10));
        axisLabels.push(date.toLocaleString());

        for (var i in series) {
            if (series[i] in datum.data) {
                if (field in datum.data[series[i]]) {
                    values[i].push(datum.data[series[i]][field]);
                } else {
                    values[i].push(0);
                }
            } else {
                values[i].push(0);
            }
        }
    }

    var datasets = [];
    for (var i in series) {
        var r = Math.floor(Math.random() * 100 + 155);
        var g = Math.floor(Math.random() * 100 + 155);
        var b = Math.floor(Math.random() * 100 + 155);
        var rgb = "" + r + "," + g + "," + b;
        var label = series[i];
        if (label == "total") {
            label = total_label;
        }
        datasets.push({
            label: label,
            fillColor: "rgba(" + rgb + ",0.2)",
            strokeColor: "rgba(" + rgb + ",1)",
            pointColor: "rgba(" + rgb + ",1)",
            pointStrokeColor: "#fff",
            pointHighlightFill: "#fff",
            pointHighlightStroke: "rgba(" + rgb + ",1)",
            data: values[i]
        });
    }

    var chart_data = {
        labels: labels,
        datasets: datasets,
        axisLabels: axisLabels,
    };

    ctx.clearRect(0, 0, 1000, 600);
    if (chart) {
        chart.destroy();
        chart = null;
    }
    chart = new Chart(ctx).MissingLine(chart_data, {
        pointHitDetectionRadius: 3,
        legendTemplate: "<ul class=\"<%=name.toLowerCase()%>-legend\"><% for (var i=0; i<datasets.length; i++){%><li><span style=\"color:<%=datasets[i].strokeColor%>\"><%if(datasets[i].label){%><%=datasets[i].label%><%}%></span></li><%}%></ul>"
    });
    document.getElementById("legend").innerHTML = chart.generateLegend();
}
