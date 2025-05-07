import Solver from "./Solver";

import styles from "./App.module.css";
import {
  Component,
  createMemo,
  createResource,
  createSignal,
  For,
} from "solid-js";
import { InbuiltRuleSet } from "./worker";
import { BacktrackerVariant } from "aaltofunktionromautus";
import { makePersisted } from "@solid-primitives/storage";
import { CustomRule } from "./utils";
import { Extractor } from "./Extractor";

const MAX = 150;
const MIN = 2;
const DEFAULT = 30;

const App: Component = () => {
  const [customRules, setCustomRules, init] = makePersisted(
    createSignal<CustomRule[]>([]),
    { name: "custom_rules" },
  );
  createResource(() => init)[0]();
  const [size, setSize] = createSignal<number>(DEFAULT);
  const [rules, setRules] = createSignal<InbuiltRuleSet | CustomRule>(
    "terrain",
  );
  const [backtracker, setBacktracker] = createSignal<BacktrackerVariant | null>(
    BacktrackerVariant.GradualReset,
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
      <div class={styles.header}>
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
              value={(() => {
                const r = rules();
                return typeof r === "string" ? r : r.name;
              })()}
              onChange={(e) => {
                const val = e.target.value;
                const custom = customRules().find((r) => r.name === val);
                setRules(custom ?? (val as any));
              }}
            >
              <optgroup label="Samples">
                <option value="flowers">flowers</option>
                <option value="link">link</option>
                <option value="village">village</option>
                <option value="simple_wall">wall simple</option>
                <option value="skyline2">skyline 2</option>
              </optgroup>
              <optgroup label="Test Patterns">
                <option value="terrain">terrain</option>
                <option value="flowers_singlepixel">flowers</option>
                <option value="terrain_simple">terrain simple</option>
                <option value="checkers">checkers</option>
                <option value="stripes">stripes</option>
              </optgroup>
              <optgroup label="Custom Rules">
                <For each={customRules()}>
                  {(r) => <option value={r.name}>{r.name}</option>}
                </For>
              </optgroup>
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
              min={MIN}
              max={MAX}
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
          <details style={{ background: "#1D2021", padding: "0.2rem 0.1rem" }}>
            <summary style={{ cursor: "pointer" }}>Rule Creator</summary>
            <Extractor
              customRules={customRules}
              setCustomRules={setCustomRules}
              setRules={setRules}
            />
          </details>
        </div>
        <Solver
          dimensions={dimensions}
          rules={rules}
          backtracker={backtracker}
          customRules={customRules}
          setCustomRules={setCustomRules}
        />
      </div>
    </div>
  );
};

export default App;
