import {
  Accessor,
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
import { Dimensions } from "../pkg/aaltofunktionromautus";

export type VisualGrid = Map<Location2D, Tile>;

function locationToIndex(location: Location2D, width: number): number {
  return location.y * width + location.x;
}

const Solver: Component<{ dimensions: Accessor<Dimensions> }> = ({
  dimensions,
}) => {
  const [state, setState] = createSignal<State | null>(null);
  const [tickActive, setTickActive] = createSignal(false);
  const tooLargeForTick = createMemo(() => {
    const dim = dimensions();
    return dim.width > 40 || dim.height > 40;
  });

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
      dimensions: dimensions(),
    });
    setTickActive(activate);
  }
  function tick() {
    postMessage({
      type: "tick",
    });
  }
  function collapse(x: number, y: number) {
    postMessage({
      type: "collapse",
      x,
      y,
    });
  }
  function run() {
    setTickActive(false);
    postMessage({
      type: "run",
    });
  }

  let timeout: number | undefined = undefined;
  function createT(delay: number) {
    timeout = setTimeout(t, delay);
  }
  function t() {
    function next() {
      createT(8);
    }
    if (!tickActive() || waitingForWorker()) {
      return next();
    }
    tick();
    return next();
  }
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
        <button
          onClick={() => setTickActive(!tickActive())}
          disabled={tooLargeForTick()}
        >
          toggle tick
        </button>
        <fieldset
          style={{ display: "contents" }}
          disabled={tickActive() || waitingForWorker()}
        >
          <button onClick={() => run()}>complete</button>
          <button onClick={() => reset()}>reset</button>
        </fieldset>
      </div>
    </div>
  );
};

export default Solver;
