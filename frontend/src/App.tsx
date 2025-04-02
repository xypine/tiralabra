import { createSignal, onCleanup, Show, type Component } from "solid-js";
import initSync, { Grid, Rules, Location2D, Tile } from "aaltofunktionromautus";
import Map from "./Map";

import styles from "./App.module.css";

export type VisualGrid = Map<Location2D, Tile>;

export const W = 30;
export const H = 30;

function locationToIndex(location: Location2D, width: number): number {
  return location.y * width + location.x;
}

const App: Component = () => {
  const [wasmReady, setWasmReady] = createSignal(false);
  const [grid, setGrid] = createSignal<Grid | null>(null);
  const [tickActive, setTickActive] = createSignal(true);

  initSync().then(() => {
    setWasmReady(true);
    const rules = Rules.terrain();
    const g = new Grid(rules, W, H);
    setGrid(g);
  });

  const map = () => {
    const g = grid();
    if (!g) {
      return null;
    }
    const img = g.dump();
    return img;
  };
  const tiles = () => {
    const m = map();
    if (!m) {
      return null;
    }
    let arr = [];
    for (let y = -1; y++, y < H; ) {
      let buffer = [];
      for (let x = -1; x++, x < W; ) {
        const t = m[locationToIndex({ x, y }, W)];
        buffer.push(t);
      }
      arr.push(buffer);
    }
    return arr;
  };

  function tick() {
    setTickActive(false);
    let g = grid();
    if (g) {
      const res = g.tick();
      if (res) {
        const rules = Rules.terrain();
        g = new Grid(rules, W, H);
      }
      setGrid(null);
      setGrid(g);
      if (res === undefined) {
        setTickActive(false);
      } else {
        setTickActive(true);
      }
    }
  }
  const interval = setInterval(() => {
    if (!tickActive() || !wasmReady()) {
      return;
    }
    tick();
  }, 2);
  onCleanup(() => clearInterval(interval));

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <Show when={tiles() != null}>
          <Map tiles={tiles()!} />
        </Show>
        <button onClick={() => setTickActive(!tickActive())}>
          toggle tick
        </button>
      </header>
    </div>
  );
};

export default App;
