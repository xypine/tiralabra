import {
  createEffect,
  createMemo,
  createSignal,
  onCleanup,
  type Component,
} from "solid-js";
import type { Location2D, Tile } from "aaltofunktionromautus";
import type { WorkerRequest, WorkerResponse } from "./worker";
import Worker from "./worker?worker";
import Map from "./Map";

import styles from "./App.module.css";

export type VisualGrid = Map<Location2D, Tile>;

export const W = 30;
export const H = 30;

function locationToIndex(location: Location2D, width: number): number {
  return location.y * width + location.x;
}

const App: Component = () => {
  const [map, setMap] = createSignal<Tile[] | null>(null);
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
      let tiles = data.tiles;
      if (data.type === "tick_update") {
        const res = data.result;
        if (res === undefined) {
          setTickActive(false);
        }
      }

      requestAnimationFrame(() => {
        setMap(tiles);
        setWaitingForWorker(false);
      });
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
    const m = map();
    if (!m) {
      return null;
    }
    let arr = [];
    for (let x = -1; x++, x < W; ) {
      let buffer = [];
      for (let y = -1; y++, y < H; ) {
        const t = m[locationToIndex({ x, y }, W)];
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
    timeout = setTimeout(t, 16);
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
    <div class={styles.App}>
      <header class={styles.header}>
        <Map
          width={() => W}
          height={() => H}
          tiles={() => tiles() ?? []}
          onTileClick={(x, y) => {
            console.debug({ x, y });
            collapse(x, y);
          }}
        />
        <button onClick={() => setTickActive(!tickActive())}>
          toggle tick
        </button>
        <button onClick={() => reset()}>reset</button>
      </header>
    </div>
  );
};

export default App;
