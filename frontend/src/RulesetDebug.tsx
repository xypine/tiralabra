import type { Direction2D, TileState } from "aaltofunktionromautus";
import { BaseSettings, WorkerRequest } from "./worker";
import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  For,
  Setter,
} from "solid-js";
import { TileVisual } from "./Map";

const Debugger: Component<{
  possible_states: Accessor<TileState[]>;
  baseSettings: Accessor<BaseSettings>;
  state: Accessor<TileState[] | undefined>;
  setState: Setter<TileState[] | undefined>;
  postMessage: (msg: WorkerRequest) => void;
}> = ({ possible_states, postMessage, state, setState, baseSettings }) => {
  const [from, setFrom] = createSignal<number[]>([]);
  const [target, setTarget] = createSignal<number[]>([]);
  const [dir, setDir] = createSignal<Direction2D>("UP");
  function ask() {
    setState(undefined);
    const rq: WorkerRequest = {
      type: "rule_check",
      from: from(),
      target: target(),
      direction: dir(),
      ...baseSettings(),
    };
    postMessage(rq);
  }
  createEffect(() => {
    ask();
  });
  return (
    <div
      style={{
        "font-size": "1rem",
        "--tile-w": "2rem",
        "--tile-h": "2rem",
      }}
    >
      <div
        style={{
          padding: ".2rem",
          "max-width": "min(75vw, 800px)",
          display: "flex",
          "flex-wrap": "wrap",
          gap: ".2rem",
        }}
      >
        <For each={possible_states()}>
          {(state) => (
            <TileVisual
              x={0}
              y={0}
              onTileClick={() => {}}
              onTileRightClick={() => {}}
              tile={() => ({ collapsed: true, possible_states: [state] })}
            />
          )}
        </For>
      </div>
      <div
        style={{
          display: "flex",
          "flex-direction": "column",
          padding: "1rem",
          gap: "0.5rem",
        }}
      >
        <label>
          current
          <input
            value={target().join(", ")}
            onChange={(e) => {
              setTarget(
                e.target.value
                  .split(",")
                  .flatMap((v) => (v.trim().length ? +v : [])),
              );
            }}
          />
        </label>
        <label>
          direction
          <select
            value={dir()}
            onChange={(e) => setDir(e.target.value as Direction2D)}
          >
            <option value={"UP"}>up</option>
            <option value={"RIGHT"}>right</option>
            <option value={"DOWN"}>down</option>
            <option value={"LEFT"}>left</option>
          </select>
        </label>
        <label>
          neighbour
          <input
            value={from().join(", ")}
            onChange={(e) => {
              setFrom(
                e.target.value
                  .split(",")
                  .flatMap((v) => (v.trim().length ? +v : [])),
              );
            }}
          />
        </label>
        <div
          style={{
            display: "flex",
            gap: "0.5rem",
            "justify-content": "center",
            "align-items": "center",
            padding: "1rem",
          }}
        >
          <For each={state() ?? []}>
            {(state) => (
              <div title={state + ""}>
                <TileVisual
                  x={0}
                  y={0}
                  onTileClick={() => {}}
                  onTileRightClick={() => {}}
                  tile={() => ({ collapsed: true, possible_states: [state] })}
                />
              </div>
            )}
          </For>
        </div>
      </div>
    </div>
  );
};

export default Debugger;
