import React, {
  useEffect,
  useState,
  useRef,
  MouseEventHandler,
  EventHandler,
} from "react";
import throttle from "lodash.throttle";

const noop = () => {};
const frame = (f = noop) =>
  new Promise((resolve) => {
    requestAnimationFrame(() => {
      f();
      resolve();
    });
  });

const toColor = (value: number) => {
  return Math.floor(255.99 * Math.sqrt(value));
};

export default React.lazy(async () => {
  let { Scene } = await import("../../../ray-tracing-wasm");

  let RayTracing: React.FC<{ counts: number }> = function RustRayTracing(
    props
  ) {
    let [time, setTime] = useState(0);
    let ref = useRef<HTMLCanvasElement | null>(null);
    let deubgRef = useRef<HTMLCanvasElement | null>(null);
    let width = 200;
    let height = 100;

    useEffect(() => {
      if (!ref.current) return;

      let canvas = ref.current;
      let debugCanvas = deubgRef.current;
      let ctx = canvas.getContext("2d");

      if (!ctx) return;

      ctx.clearRect(0, 0, canvas.clientWidth, canvas.clientHeight);

      let imageData = ctx.createImageData(width, height);
      let ray = new Scene(width, height, 50);
      let data = new Float32Array(width * height * 4);
      let prevImageData = new Float32Array(width * height * 4);
      let currImageData = new Float32Array(width * height * 4);
      let renderCount = new Float32Array(width * height * 4);
      let innerCount = 1;
      let innerTime = 0;
      let tid: any = null;
      let over = false;

      ray.random_scene(100);

      let reset = () => {
        if (!ctx) return;
        imageData = ctx.createImageData(width, height);
        data = new Float32Array(width * height * 4);
        prevImageData = new Float32Array(width * height * 4);
        currImageData = new Float32Array(width * height * 4);
        renderCount = new Float32Array(width * height * 4);
        innerCount += 1;
        innerTime = 0;
        tid = null;
        over = false;
      };

      let renderToCanvas = () => {
        if (!ctx) return;
        ctx.fillStyle = `rgba(0, 0, 0, 1)`;
        ctx.clearRect(0, 0, width, height);
        ctx.putImageData(imageData, 0, 0);
      };

      let render = async function () {
        if (over) return
        let currentCount = innerCount;
        let start = Date.now();
        let duration = 0;
        let list = ray.render();

        for (let i = 0; i < list.length; ) {
          let offset = (i / 3) * 4;
          let r = list[i++];
          let g = list[i++];
          let b = list[i++];
          let a = 1;

          data[offset + 0] += r;
          data[offset + 1] += g;
          data[offset + 2] += b;
          data[offset + 3] += a;

          renderCount[offset + 0] += 1;
          renderCount[offset + 1] += 1;
          renderCount[offset + 2] += 1;
          renderCount[offset + 3] += 1;

          prevImageData[offset + 0] = imageData.data[offset + 0];
          prevImageData[offset + 1] = imageData.data[offset + 1];
          prevImageData[offset + 2] = imageData.data[offset + 2];
          prevImageData[offset + 3] = imageData.data[offset + 3];

          imageData.data[offset + 0] = toColor(
            data[offset + 0] / renderCount[offset + 0]
          );
          imageData.data[offset + 1] = toColor(
            data[offset + 1] / renderCount[offset + 1]
          );
          imageData.data[offset + 2] = toColor(
            data[offset + 2] / renderCount[offset + 2]
          );
          imageData.data[offset + 3] = toColor(
            data[offset + 3] / renderCount[offset + 3]
          );

          currImageData[offset + 0] = imageData.data[offset + 0];
          currImageData[offset + 1] = imageData.data[offset + 1];
          currImageData[offset + 2] = imageData.data[offset + 2];
          currImageData[offset + 3] = imageData.data[offset + 3];
        }

        if (over) return
        setTime(innerTime + duration);
        await frame(() => {
          if (currentCount === innerCount) {
            renderToCanvas();
          }
          let duration = Date.now() - start;
          if (over) return
          setTime((innerTime += duration));
        });

        tid = requestAnimationFrame(render);
      };

      render();

      let handleKeyDown = throttle((event: HTMLElementEventMap["keydown"]) => {
        let number = 0;

        switch (event.key) {
          case "ArrowUp":
          case "w":
            number = 0;
            break;
          case "ArrowDown":
          case "s":
            number = 1;
            break;

          case "ArrowLeft":
          case "a":
            number = 2;
            break;
          case "ArrowRight":
          case "d":
            number = 3;
            break;
          default:
            return;
        }

        ray.process_keyboard(number);
        reset();
      }, 200);

      document.addEventListener("keydown", handleKeyDown, false);

      return () => {
        document.removeEventListener("keydown", handleKeyDown, false);
        over = true;
        clearTimeout(tid);
      };
    }, [props.counts]);

    return (
      <>
        <h2>Ray tracing via Rust</h2>
        <h3>Time: {(time / 1000).toFixed(2)}s</h3>
        <div>Press arrow keys or w/s/a/d to change the viewpoint</div>
        <div
          style={{
            height: 400,
          }}
        >
          <canvas
            width={width}
            height={height}
            ref={ref}
            style={{
              display: "block",
              marginLeft: "auto",
              marginRight: "auto",
              marginTop: (400 - height) / 2,
              background: "#000",
              transform: "scale(4)",
            }}
          />
        </div>
      </>
    );
  };

  return { default: RayTracing };
});
