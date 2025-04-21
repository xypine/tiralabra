import {
  Accessor,
  createEffect,
  createMemo,
  createSignal,
  onCleanup,
  Show,
  type Component,
} from "solid-js";
import type {
  Dimensions,
  Location2D,
  Tile,
  TileState,
  TileVisual,
} from "aaltofunktionromautus";
import type {
  InbuiltRuleSet,
  State,
  WorkerRequest,
  WorkerResponse,
} from "./worker";
import Worker from "./worker?worker";
import Map from "./Map";
import RulesetDebug from "./RulesetDebug";

export type VisualGrid = Map<Location2D, Tile>;

function locationToIndex(location: Location2D, width: number): number {
  return location.y * width + location.x;
}

const Solver: Component<{
  dimensions: Accessor<Dimensions>;
  rules: Accessor<InbuiltRuleSet>;
}> = ({ dimensions, rules }) => {
  const [state, setState] = createSignal<State | null>(null);
  const [tileSet, setTileSet] = createSignal<TileVisual[]>([]);
  const [tickActive, setTickActive] = createSignal(false);
  const [timeTravelIndex, setTimeTravelIndex] = createSignal<
    number | undefined
  >(0);

  const tooLargeForTick = createMemo(() => {
    const dim = dimensions();
    return dim.width > 40 || dim.height > 40;
  });

  createEffect(() => {
    const tti = timeTravelIndex();
    const s = state();
    if (tti !== undefined && s !== null && tti >= s.history_len) {
      setTimeTravelIndex(undefined);
    } else if (tti !== undefined && s !== null && tti !== s.history_position) {
      postMessage({
        type: "read_past",
        t: tti,
        dimensions: dimensions(),
        rules: rules(),
      });
    } else if (
      tti === undefined &&
      s !== null &&
      tti !== s.history_position &&
      s.history_position !== s.history_len
    ) {
      postMessage({
        type: "read_past",
        t: s.history_len,
        dimensions: dimensions(),
        rules: rules(),
      });
    }
  });

  const [ruleCheckerState, setRuleCheckerState] = createSignal<
    TileState[] | undefined
  >(undefined);

  const [waitingForWorker, setWaitingForWorker] = createSignal(false);
  const worker = createMemo(() => {
    console.debug("init worker");
    const w = new Worker();
    w.onmessage = (event) => {
      let data: WorkerResponse = event.data;
      // console.debug("ui got message", { data });
      let newState = data.state;
      let newTileSet = data.tileset;
      if (data.type === "tick_update") {
        const res = data.result;
        if (res === undefined) {
          setTickActive(false);
        }
      }
      if (data.type === "rule_check") {
        setRuleCheckerState(data.allowed);
      }

      requestAnimationFrame(() => {
        if (newTileSet) {
          setTileSet(newTileSet);
        }
        setState(newState);
      });
      setWaitingForWorker(false);
    };
    w.onerror = (event) => {
      console.warn("error", { event });
      throw event.error;
    };
    return w;
  });
  const postMessage = (msg: WorkerRequest) => {
    setWaitingForWorker(true);
    worker().postMessage(msg);
  };
  createEffect(() => {
    console.debug("init");
    reset();
    createT(8);
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
      rules: rules(),
    });
    setTickActive(activate);
  }
  function tick() {
    postMessage({
      type: "tick",
      dimensions: dimensions(),
      rules: rules(),
    });
  }
  function collapse(x: number, y: number, state?: number) {
    postMessage({
      type: "collapse",
      dimensions: dimensions(),
      rules: rules(),
      x,
      y,
      state,
    });
  }
  function run() {
    setTickActive(false);
    postMessage({
      type: "run",
      dimensions: dimensions(),
      rules: rules(),
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
    <>
      <div>
        <Map
          width={() => state()?.width ?? 0}
          height={() => state()?.height ?? 0}
          tiles={() => tiles() ?? []}
          tileset={tileSet}
          onTileClick={(x, y) => {
            console.debug({ x, y });
            collapse(x, y);
          }}
          onTileRightClick={(x, y) => {
            const s = state();
            if (!s) {
              return;
            }
            const possible =
              s.tiles[locationToIndex({ x, y }, s.width)].possible_states;
            const chosen = prompt(
              "which state? Possible: " + possible.join(", "),
            );
            if (chosen) {
              const asNum = +chosen;
              if (possible.includes(asNum)) {
                console.debug({ x, y, chosen });
                collapse(x, y, asNum);
              }
            }
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
        <div
          style={{
            "margin-top": "0.5rem",
            "font-size": "1rem",
            "min-height": "2rem",
          }}
        >
          <Show when={!tickActive() && state()?.history_len}>
            <label
              style={{
                display: "flex",
                "justify-content": "center",
                "align-items": "center",
                gap: "0.5rem",
              }}
            >
              time travel
              <input
                style={{
                  "min-width": "min(500px, 90vw)",
                }}
                type="range"
                min={0}
                max={state()?.history_len ?? 0}
                disabled={
                  state() === null ||
                  tickActive() ||
                  (state()?.history_len ?? 0) < 2
                }
                value={timeTravelIndex() ?? state()?.history_len}
                onInput={(e) => {
                  const v = +e.target.value;
                  console.debug("stti", { v });
                  setTimeTravelIndex(v);
                }}
              />
            </label>
          </Show>
        </div>
      </div>
      <Show when={false}>
        <RulesetDebug
          postMessage={postMessage}
          state={ruleCheckerState}
          setState={setRuleCheckerState}
          baseSettings={() => ({
            rules: rules(),
            dimensions: dimensions(),
          })}
        />
      </Show>
    </>
  );
};

export default Solver;
