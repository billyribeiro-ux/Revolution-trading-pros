var main_chart;

(function($) {
	var drawGraph = function (highlight) {

		if (highlight !== "undefined" && date_chart_data.series[highlight]) {
			highlight_series = date_chart_data.series[highlight];

			highlight_series.color = "#9c5d90";

			if (highlight_series.bars) {
				highlight_series.bars.fillColor = "#9c5d90";
			}

			if (highlight_series.lines) {
				highlight_series.lines.lineWidth = 5;
			}
		}

		main_chart = $.plot($(".chart-placeholder.main"), date_chart_data.series, {
			legend: {
				show: false,
			},
			grid: {
				color: "#aaa",
				borderColor: "transparent",
				borderWidth: 0,
				hoverable: true,
			},
			xaxes: [
				{
					color: "#aaa",
					position: "bottom",
					tickColor: "transparent",
					mode: "time",
					timeformat: date_chart_data.time_format,
					monthNames: date_chart_data.month_names,
					tickLength: 1,
					minTickSize: [1, date_chart_data.cart_groupby],
					font: {
						color: "#aaa",
					},
				},
			],
			yaxes: [
				{
					min: 0,
					minTickSize: 1,
					tickDecimals: 0,
					color: "#d4d9dc",
					font: { color: "#aaa" },
				},
				{
					position: "right",
					min: 0,
					tickDecimals: 2,
					alignTicksWithAxis: 1,
					color: "transparent",
					font: { color: "#aaa" },
				},
			],
		});

		$(".chart-placeholder").resize();
	};

	drawGraph();

	$(".highlight_series").hover(
		function () {
			drawGraph($(this).data("series"));
		},
		function () {
			drawGraph();
		},
	);
})(jQuery);
