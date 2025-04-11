import Solver from "./Solver";

import styles from "./App.module.css";
import { Component, createMemo, createSignal } from "solid-js";
import { InbuiltRuleSet } from "./worker";

const MAX = 100;
const MIN = 2;
const DEFAULT = 30;

const App: Component = () => {
  const [size, setSize] = createSignal<number>(DEFAULT);
  const [rules, setRules] = createSignal<InbuiltRuleSet>("terrain");
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
        <div
          style={{
            "font-size": "1rem",
            display: "flex",
            "flex-direction": "column",
            gap: "0.5rem",
          }}
        >
          <label
            style={{
              display: "flex",
              "flex-direction": "column",
              gap: "0.2rem",
            }}
          >
            Ruleset
            <select
              value={rules()}
              onChange={(e) => {
                const val = e.target.value;
                setRules(val as any);
              }}
            >
              <option value="terrain">terrain</option>
              <option value="flowers_singlepixel">flowers</option>
              <option value="terrain_simple">terrain simple</option>
              <option value="checkers">checkers</option>
              <option value="stripes">stripes</option>
            </select>
          </label>
          <label
            style={{
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
        </div>
        <Solver dimensions={dimensions} rules={rules} />
      </header>
    </div>
  );
};

export default App;
