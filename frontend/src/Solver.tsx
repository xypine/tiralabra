import {
  createEffect,
  createMemo,
  createSignal,
  onCleanup,
  type Component,
} from "solid-js";
import type { Location2D, Tile } from "aaltofunktionromautus";
import type { State, WorkerRequest, WorkerResponse } from "./worker";
import Worker from "./worker?worker";
import Map from "./Map";

export type VisualGrid = Map<Location2D, Tile>;

function locationToIndex(location: Location2D, width: number): number {
  return location.y * width + location.x;
}

const Solver: Component = () => {
  const [state, setState] = createSignal<State | null>(null);
  const [tickActive, setTickActive] = createSignal(false);

  const [waitingForWorker, setWaitingForWorker] = createSignal(false);
  const worker = createMemo(() => {
    return new Worker();
  });
  const postMessage = (msg: WorkerRequest) => {
    setWaitingForWorker(true);
    worker().postMessage(msg);
  };
  createEffect(() => {
    worker().onmessage = (event) => {
      let data: WorkerResponse = event.data;
      // console.debug({ data });
      let newState = data.state;
      if (data.type === "tick_update") {
        const res = data.result;
        if (res === undefined) {
          setTickActive(false);
        }
      }

      requestAnimationFrame(() => {
        setState(newState);
      });
      setWaitingForWorker(false);
    };
    worker().onerror = (event) => {
      throw event.error;
    };
  });
  createEffect(() => {
    console.debug("init");
    reset();
    createT();
  });

  const tiles = () => {
    const s = state();
    if (!s) {
      return null;
    }
    let arr = [];
    for (let x = -1; x++, x < s.width; ) {
      let buffer = [];
      for (let y = -1; y++, y < s.height; ) {
        const t = s.tiles[locationToIndex({ x, y }, s.width)];
        buffer.push(t);
      }
      arr.push(buffer);
    }
    return arr;
  };

  function reset(activate = false) {
    postMessage({
      type: "reset",
    });
    setTickActive(activate);
  }
  function tick() {
    postMessage({
      type: "tick",
    });
  }
  function collapse(x: number, y: number) {
    setTickActive(false);
    postMessage({
      type: "collapse",
      x,
      y,
    });
  }

  let timeout: number | undefined = undefined;
  function createT() {
    timeout = setTimeout(t, 8);
  }
  function t() {
    function next() {
      createT();
    }
    if (!tickActive() || waitingForWorker()) {
      return next();
    }
    tick();
    return next();
  }
  // const interval = setInterval(() => {
  //   tick();
  // }, 100);
  onCleanup(() => {
    if (timeout !== undefined) {
      clearTimeout(timeout);
    }
  });

  return (
    <div>
      <Map
        width={() => state()?.width ?? 0}
        height={() => state()?.height ?? 0}
        tiles={() => tiles() ?? []}
        onTileClick={(x, y) => {
          console.debug({ x, y });
          collapse(x, y);
        }}
      />
      <div
        style={{
          "margin-top": "1.5rem",
          display: "flex",
          gap: "1rem",
          "justify-content": "center",
          "align-items": "center",
        }}
      >
        <button onClick={() => setTickActive(!tickActive())}>
          toggle tick
        </button>
        <button onClick={() => reset()}>reset</button>
      </div>
    </div>
  );
};

export default Solver;
