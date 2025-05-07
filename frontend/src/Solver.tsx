import {
  Accessor,
  createEffect,
  createMemo,
  createResource,
  createSignal,
  on,
  onCleanup,
  onMount,
  Setter,
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
import RulesetDebug from "./RulesetDebug";
import { CustomRule, pickRandomSeed } from "./utils";
import { untrack } from "solid-js/web";
import { BacktrackerVariant } from "../pkg/aaltofunktionromautus";
import { Extractor } from "./Extractor";

export type VisualGrid = Map<Location2D, Tile>;

const SOLVE_DELAY = 0 as const;

const Solver: Component<{
  dimensions: Accessor<Dimensions>;
  rules: Accessor<InbuiltRuleSet | CustomRule>;
  backtracker: Accessor<BacktrackerVariant | null>;
  customRules: Accessor<CustomRule[]>;
  setCustomRules: Setter<CustomRule[]>;
}> = ({ dimensions, rules, backtracker, customRules, setCustomRules }) => {
  const [size, setSize] = createSignal(500);
  const calculateMinViewportSize = () => {
    const vw = window.innerWidth * 0.75;
    const vh = window.innerHeight * 0.75;
    const min = Math.min(vw, vh);
    console.debug({ min });
    setSize(min);
  };
  onMount(() => {
    calculateMinViewportSize();
    window.addEventListener("resize", calculateMinViewportSize);
  });

  const [randomSeed, setRandomSeed] = createSignal<boolean>(true);
  const [seed, setSeed] = createSignal<number>(pickRandomSeed());
  const [state, setState] = createSignal<State | null>(null);
  // const [tileSet, setTileSet] = createSignal<TileVisual[]>([]);
  const [tickActive, setTickActive] = createSignal(false);
  const [timeTravelIndex, setTimeTravelIndex] = createSignal<
    number | undefined
  >(0);

  const tooLargeForTick = createMemo(() => {
    const dim = dimensions();
    return dim.width > 55 || dim.height > 55;
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
        outputSize: size(),
        rules: rules(),
        seed: {
          allowRandomization: randomSeed(),
          value: seed(),
        },
        backtracker: backtracker(),
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
        outputSize: size(),
        seed: {
          allowRandomization: randomSeed(),
          value: seed(),
        },
        backtracker: backtracker(),
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
      if (data.type === "extracted_rules") {
        return;
      }
      // console.debug("ui got message", { data });
      let newState = data.state;
      if (newState.seed !== seed()) {
        setSeed(newState.seed);
      }
      // let newTileSet = data.tileset;
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
        // if (newTileSet) {
        //   setTileSet(newTileSet);
        // }
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

  onMount(() => {
    console.debug("init");
    createT(SOLVE_DELAY);
  });

  createMemo(() => {
    console.debug("reset (dimensions / rules changed)");
    const d = dimensions();
    const r = rules();
    const b = backtracker();
    untrack(() => {
      calculateMinViewportSize();
      reset(d, r, seed(), randomSeed(), b);
    });
  });

  createMemo(() => {
    const cr = customRules();
    console.info("updating custom rules");
    untrack(() => {
      postMessage({
        type: "setCustomRules",
        customRules: cr,

        outputSize: size(),
        dimensions: dimensions(),
        rules: rules(),
        seed: {
          allowRandomization: randomSeed(),
          value: seed(),
        },
        backtracker: backtracker(),
      });
    });
  });

  // const tiles = () => {
  //   const s = state();
  //   if (!s) {
  //     return null;
  //   }
  //   let arr = [];
  //   for (let x = -1; x++, x < s.width; ) {
  //     let buffer = [];
  //     for (let y = -1; y++, y < s.height; ) {
  //       const t = s.tiles[locationToIndex({ x, y }, s.width)];
  //       buffer.push(t);
  //     }
  //     arr.push(buffer);
  //   }
  //   return arr;
  // };

  function reset(
    d: Dimensions,
    r: InbuiltRuleSet | CustomRule,
    seed: number,
    allowRandomization: boolean,
    backtracker: BacktrackerVariant | null,
    activate = false,
  ) {
    postMessage({
      type: "reset",
      outputSize: size(),
      dimensions: d,
      rules: r,
      seed: {
        allowRandomization,
        value: seed,
      },
      backtracker,
    });
    setTickActive(activate);
  }
  function tick() {
    postMessage({
      type: "tick",
      dimensions: dimensions(),
      outputSize: size(),
      rules: rules(),
      seed: {
        allowRandomization: randomSeed(),
        value: seed(),
      },
      backtracker: backtracker(),
    });
  }
  function collapse(x: number, y: number, state?: bigint) {
    postMessage({
      type: "collapse",
      dimensions: dimensions(),
      outputSize: size(),
      rules: rules(),
      x,
      y,
      state,
      seed: {
        allowRandomization: randomSeed(),
        value: seed(),
      },
      backtracker: backtracker(),
    });
  }
  function run() {
    setTickActive(false);
    postMessage({
      type: "run",
      dimensions: dimensions(),
      outputSize: size(),
      rules: rules(),
      seed: {
        allowRandomization: randomSeed(),
        value: seed(),
      },
      backtracker: backtracker(),
    });
  }

  let timeout: number | undefined = undefined;
  function createT(delay: number) {
    timeout = setTimeout(t, delay);
  }
  function t() {
    function next() {
      createT(SOLVE_DELAY);
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
      <div>
        {/* <Map */}
        {/*   width={() => state()?.width ?? 0} */}
        {/*   height={() => state()?.height ?? 0} */}
        {/*   tiles={() => tiles() ?? []} */}
        {/*   onTileClick={(x, y) => { */}
        {/*     console.debug({ x, y }); */}
        {/*     collapse(x, y); */}
        {/*   }} */}
        {/*   onTileRightClick={(x, y) => { */}
        {/*     const s = state(); */}
        {/*     if (!s) { */}
        {/*       return; */}
        {/*     } */}
        {/*     const possible = s.tiles[locationToIndex({ x, y }, s.width)] */}
        {/*       .possible_states as unknown as bigint[]; */}
        {/*     const chosen = prompt( */}
        {/*       "which state? Possible: " + possible.join(", "), */}
        {/*     ); */}
        {/*     if (chosen) { */}
        {/*       const asNum = BigInt(chosen); */}
        {/*       if (possible.includes(asNum)) { */}
        {/*         console.debug({ x, y, chosen }); */}
        {/*         collapse(x, y, asNum); */}
        {/*       } */}
        {/*     } */}
        {/*   }} */}
        {/* /> */}
        <div
          style={{ "min-width": size() + "px", "min-height": size() + "px" }}
          innerHTML={state()?.rendered}
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
            {/* <button */}
            {/*   onClick={() => reset(dimensions(), rules(), seed(), randomSeed())} */}
            {/* > */}
            {/*   reset */}
            {/* </button> */}
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
                "flex-wrap": "wrap",
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
      <div
        style={{
          display: "flex",
          "flex-direction": "column",
          "font-size": "1rem",
          gap: "0.2rem",
        }}
      >
        <label
          style={{
            display: "flex",
            "flex-direction": "column",
            gap: "0.2rem",
          }}
        >
          random seed
          <input
            type="checkbox"
            checked={randomSeed()}
            onChange={(e) => setRandomSeed(e.target.checked)}
          />
        </label>
        <label
          style={{
            display: "flex",
            "flex-direction": "column",
            gap: "0.2rem",
          }}
        >
          seed
          <input
            type="number"
            value={seed()}
            disabled={randomSeed()}
            onChange={(e) => {
              const val = e.target.value;
              let valn = val ? +val : undefined;
              if (valn) {
                setSeed(valn);
                reset(dimensions(), rules(), valn, false, backtracker());
              }
            }}
          />
        </label>
      </div>
      {/* <Show when={false}> */}
      {/*   <RulesetDebug */}
      {/*     possible_states={() => tileSet().map(([v]) => v)} */}
      {/*     postMessage={postMessage} */}
      {/*     state={ruleCheckerState} */}
      {/*     setState={setRuleCheckerState} */}
      {/*     baseSettings={() => ({ */}
      {/*       rules: rules(), */}
      {/*       dimensions: dimensions(), */}
      {/*       outputSize: size(), */}
      {/*       seed: { */}
      {/*         allowRandomization: randomSeed(), */}
      {/*         value: seed(), */}
      {/*       }, */}
      {/*     })} */}
      {/*   /> */}
      {/* </Show> */}
    </div>
  );
};

export default Solver;
