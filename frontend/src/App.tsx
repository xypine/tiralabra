import Solver from "./Solver";

import styles from "./App.module.css";
import { Component, createMemo, createSignal } from "solid-js";

const MAX = 100;
const MIN = 3;
const DEFAULT = 4;

const App: Component = () => {
  const [size, setSize] = createSignal<number>(DEFAULT);
  const dimensions = createMemo(() => {
    const s = size();
    return {
      width: s,
      height: s,
    };
  });
  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <label
          style={{
            "font-size": "1rem",
            display: "flex",
            "flex-direction": "column",
            gap: "0.2rem",
          }}
        >
          grid size
          <input
            type="number"
            min={3}
            max={100}
            value={size()}
            onChange={(e) => {
              const val = e.target.value;
              let valn = val ? +val : undefined;
              if (valn) {
                if (valn < MIN) {
                  valn = MIN;
                }
                if (valn > MAX) {
                  valn = MAX;
                }
                setSize(valn);
              }
            }}
          />
        </label>
        <Solver dimensions={dimensions} />
      </header>
    </div>
  );
};

export default App;
