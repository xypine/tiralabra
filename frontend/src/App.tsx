import Solver from "./Solver";

import styles from "./App.module.css";
import { Component, createMemo, createSignal } from "solid-js";
import { InbuiltRuleSet } from "./worker";
import { BacktrackerVariant } from "aaltofunktionromautus";

const MAX = 150;
const MIN = 2;
const DEFAULT = 30;

const App: Component = () => {
  const [size, setSize] = createSignal<number>(DEFAULT);
  const [rules, setRules] = createSignal<InbuiltRuleSet>("terrain");
  const [backtracker, setBacktracker] = createSignal<BacktrackerVariant | null>(
    null,
  );
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
              <option value="flowers">flowers 2</option>
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
          <label
            style={{
              display: "flex",
              "flex-direction": "column",
              gap: "0.2rem",
            }}
          >
            Backtracker
            <select
              value={backtracker()?.toString() ?? ""}
              onChange={(e) => {
                const val = e.target.value;
                if (val !== "") {
                  setBacktracker(+val);
                } else {
                  setBacktracker(null);
                }
              }}
            >
              <option value="">no backtracking</option>
              <option value="0">reset</option>
              <option value="1">gradual reset</option>
            </select>
          </label>
        </div>
        <Solver
          dimensions={dimensions}
          rules={rules}
          backtracker={backtracker}
        />
      </header>
    </div>
  );
};

export default App;
