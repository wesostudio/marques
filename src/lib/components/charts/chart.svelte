<script lang="ts">
  import { onMount } from "svelte";
  import { createChart } from "lightweight-charts";

  onMount(() => {
    const chartOptions = {
      layout: {
        textColor: "white",
        background: { type: "solid", color: "black" },
      },
    };

    const chart = createChart(
      document.getElementById("container") as HTMLElement,
      //@ts-ignore
      chartOptions
    );

    chart.applyOptions({
      rightPriceScale: {
        scaleMargins: {
          top: 0.3, // leave some space for the legend
          bottom: 0.25,
        },
      },
      crosshair: {
        // hide the horizontal crosshair line
        horzLine: {
          visible: false,
          labelVisible: false,
        },
      },
      // hide the grid lines
      grid: {
        vertLines: {
          visible: false,
        },
        horzLines: {
          visible: false,
        },
      },
    });

    const areaSeries = chart.addAreaSeries({
      topColor: "#2962FF",
      bottomColor: "rgba(41, 98, 255, 0.28)",
      lineColor: "#2962FF",
      lineWidth: 2,
      crosshairMarkerVisible: false,
    });

    areaSeries.setData([{ time: "2018-10-19", value: 26.19 }]);

    const symbolName = "ETC USD 7D VWAP";

    const container = document.getElementById("container");

    const legend = document.createElement("div");
    //@ts-ignore
    legend.style = `position: absolute; left: 12px; top: 12px; z-index: 1; font-size: 14px; font-family: sans-serif; line-height: 18px; font-weight: 300;`;
    container?.appendChild(legend);

    const firstRow = document.createElement("div");
    firstRow.innerHTML = symbolName;
    firstRow.style.color = "white";
    legend.appendChild(firstRow);

    chart.subscribeCrosshairMove((param) => {
      let priceFormatted = "";
      if (param.time) {
        const data = param.seriesData.get(areaSeries);
        //@ts-ignore
        const price = data.value !== undefined ? data.value : data.close;
        priceFormatted = price.toFixed(2);
      }
      firstRow.innerHTML = `${symbolName} <strong>${priceFormatted}</strong>`;
    });

    chart.timeScale().fitContent();
  });
</script>

<div class="container"></div>
