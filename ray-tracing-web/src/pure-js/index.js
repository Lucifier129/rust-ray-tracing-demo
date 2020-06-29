import React, { useState, useEffect, useRef } from "react";
import createRayTracing from "./ray-tracing";

const noop = () => {};
const frame = (f = noop) =>
  new Promise((resolve) => {
    requestAnimationFrame(() => {
      f();
      resolve();
    });
  });

const getMSE = (listA, listB) => {
  let sum = 0;
  for (let i = 0; i < listA.length; i++) {
    sum += Math.pow(listA[i] - listB[i], 2);
  }
  return sum / listA.length;
};

const incre = function* (start, end, step = 1) {
  while (start <= end) {
    yield start;
    start += step;
  }
};

const decre = function* (start, end, step = -1) {
  while (start >= end) {
    yield start;
    start += step;
  }
};

const range = function (start, end, step = 1) {
  if (start < end) return incre(start, end, Math.abs(step));
  return decre(start, end, -Math.abs(step));
};

const toggle = function* (a, b) {
  while (true) {
    let valueA = a.next();
    let valueB = b.next();
    if (valueA.done && valueB.done) break;
    if (!valueA.done) yield valueA.value;
    if (!valueB.done) yield valueB.value;
  }
};

const spread = function* (start, end, step = 1) {
  let middle = Math.floor((end - start) / 2);
  let source = toggle(
    range(middle, start, step),
    range(middle + step, end, step)
  );
  for (let v of source) yield v;
};

// mean squared error
const sortByMSE = (prev, current) => {
  let result = [];

  for (let i = 0; i < prev.length; i += 4) {
    let index = Math.floor(i / 4);
    let listA = [prev[i + 0], prev[i + 1], prev[i + 2], prev[i + 3]];
    let listB = [
      current[i + 0],
      current[i + 1],
      current[i + 2],
      current[i + 3],
    ];

    result[index] = {
      index,
      value: getMSE(listA, listB),
    };
  }

  return result.sort((a, b) => b.value - a.value);
};

const toColor = (value) => {
  return Math.floor(255.99 * Math.sqrt(value));
};

export default function RayTracing(props) {
  let [time, setTime] = useState(0);
  let ref = useRef();
  let deubgRef = useRef();
  let width = 800;
  let height = 400;

  useEffect(() => {
    let canvas = ref.current;
    let debugCanvas = deubgRef.current;
    let ctx = canvas.getContext("2d");
    let debugCtx = debugCanvas.getContext("2d");
    let imageData = ctx.createImageData(width, height);
    let debugImageData = debugCtx.createImageData(width, height);
    let ray = createRayTracing({ width, height, amount: props.counts || 0 });
    let data = new Float32Array(width * height * 4);
    let prevImageData = new Float32Array(width * height * 4);
    let currImageData = new Float32Array(width * height * 4);
    let renderCount = new Float32Array(width * height * 4);
    let innerCount = 1;
    let innerTime = 0;
    let tid = null;
    let over = false;

    let renderToCanvas = () => {
      ctx.fillColor = `rgba(0, 0, 0, 1)`;
      ctx.clearRect(0, 0, width, height);
      ctx.putImageData(imageData, 0, 0);
    };

    let showDebugInfo = () => {
      let data = renderCount;
      for (let i = 0; i < data.length; i++) {
        if ((i + 1) % 4 === 0) {
          debugImageData.data[i] = 255;
        } else {
          debugImageData.data[i] = data[i];
        }
      }
      debugCtx.clearRect(0, 0, width, height);
      debugCtx.putImageData(debugImageData, 0, 0);
    };

    let renderByPosition = (x, y) => {
      let [r, g, b, a = 1] = ray.renderByPosition(x, y);
      let i = ((height - 1 - y) * width + x) * 4;

      data[i + 0] += r;
      data[i + 1] += g;
      data[i + 2] += b;
      data[i + 3] += a;

      renderCount[i + 0] += 1;
      renderCount[i + 1] += 1;
      renderCount[i + 2] += 1;
      renderCount[i + 3] += 1;

      prevImageData[i + 0] = imageData.data[i + 0];
      prevImageData[i + 1] = imageData.data[i + 1];
      prevImageData[i + 2] = imageData.data[i + 2];
      prevImageData[i + 3] = imageData.data[i + 3];

      imageData.data[i + 0] = toColor(data[i + 0] / renderCount[i + 0]);
      imageData.data[i + 1] = toColor(data[i + 1] / renderCount[i + 1]);
      imageData.data[i + 2] = toColor(data[i + 2] / renderCount[i + 2]);
      imageData.data[i + 3] = toColor(data[i + 3] / renderCount[i + 3]);

      currImageData[i + 0] = imageData.data[i + 0];
      currImageData[i + 1] = imageData.data[i + 1];
      currImageData[i + 2] = imageData.data[i + 2];
      currImageData[i + 3] = imageData.data[i + 3];
    };

    let render = async function (step = 1) {
      let start = Date.now();
      let duration = 0;

      for (let y of spread(0, height - 1, step)) {
        for (let x of range(0, width - 1, step)) {
          if (over) return;

          renderByPosition(x, y);
          duration = Date.now() - start;
          if (duration % 100 === 0) {
            setTime(innerTime + duration);
            await frame(() => {
              renderToCanvas();
              showDebugInfo();
            });
          }
        }
      }

      if (step === 1) innerCount += 1;
      renderToCanvas();
      showDebugInfo();
      setTime((innerTime += duration));
      if (innerCount > 2) {
        scheduleRender();
      } else {
        tid = requestAnimationFrame(() => render());
      }
    };

    let scheduleRender = async (count = 0) => {
      let list = sortByMSE(prevImageData, currImageData);
      let start = Date.now();
      let duration = 0;

      for (let i = 0; i < 20000; i++) {
        let item = list[i];
        let x = item.index % width;
        let y = height - Math.floor(item.index / width);

        renderByPosition(x, y);
        duration = Date.now() - start;

        if (duration % 100 === 0) {
          setTime(innerTime + duration);
          await frame(() => {
            renderToCanvas();
            showDebugInfo();
          });
        }
        if (over) return;
      }

      if (over) return;

      renderToCanvas();
      setTime((innerTime += Date.now() - start));
      tid = requestAnimationFrame(() => {
        if (count < 5) {
          scheduleRender(count + 1);
        } else {
          render();
        }
      });
    };

    render(3);

    return () => {
      over = true;
      clearTimeout(tid);
    };
  }, [props.counts]);

  return (
    <>
      <h2>Ray tracing via JavaScript</h2>
      <h3>Time: {(time / 1000).toFixed(2)}s</h3>
      <canvas
        width={width}
        height={height}
        ref={ref}
        style={{ background: "#000", display: "block", margin: "0 auto" }}
      />
      <br />
      <canvas
        width={width}
        height={height}
        ref={deubgRef}
        style={{ display: "block", margin: "0 auto" }}
      />
    </>
  );
}
