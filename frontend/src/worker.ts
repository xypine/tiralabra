import initSync, {
  Grid,
  Rules,
  Dimensions,
  TileState,
  Direction2D,
  TileVisual,
  Backtracker2D,
  BacktrackerVariant,
  new_backtracker,
  OverlappingBitmapExtractorOptions,
} from "aaltofunktionromautus";
import { CustomRule, pickRandomSeed, Seed } from "./utils";

const worker_id = Math.floor(Math.random() * 1000);
console.log(`Worker ${worker_id} loaded`);

export type State = {
  rendered: string;
  history_len: number;
  history_position?: number;
  seed: number;
} & Dimensions;

export const INBUILT_RULE_SETS = [
  "terrain",
  "flowers_singlepixel",
  "bubblewrap",
  "terrain_simple",
  "checkers",
  "stripes",
  "flowers",
  "link",
  "village",
  "simple_wall",
  "skyline2",
] as const;
export type InbuiltRuleSet = (typeof INBUILT_RULE_SETS)[number];

export type BaseSettings = {
  outputSize: number;
  dimensions: Dimensions;
  rules: InbuiltRuleSet | CustomRule;
  seed: Seed;
  backtracker: BacktrackerVariant | null;
};

export type WorkerRequest =
  | (BaseSettings &
      (
        | {
            type: "setCustomRules";
            customRules: CustomRule[];
          }
        | {
            type: "reset";
          }
        | {
            type: "tick";
          }
        | {
            type: "run";
          }
        | {
            type: "read_past";
            t: number;
          }
        | {
            type: "collapse";
            x: number;
            y: number;
            state?: bigint;
          }
        | {
            type: "rule_check";
            from: TileState[];
            target: TileState[];
            direction: Direction2D;
          }
      ))
  | {
      type: "extract_rules";
      name: string;
      source: Uint8Array;
      options: OverlappingBitmapExtractorOptions;
    };

export type WorkerResponse =
  | ({
      state: State;
    } & {
      tileset?: TileVisual[];
    } & (
        | {
            type: "state_update";
          }
        | {
            type: "tick_update";
            result: boolean | undefined;
          }
        | {
            type: "rule_check";
            allowed: TileState[];
          }
      ))
  | {
      type: "extracted_rules";
      result: CustomRule;
    };

function getRules(ruleset: InbuiltRuleSet | CustomRule) {
  let rules;
  switch (ruleset) {
    case "terrain":
      rules = Rules.terrain();
      break;
    case "flowers_singlepixel":
      rules = Rules.flowers_singlepixel();
      break;
    case "bubblewrap":
      rules = Rules.bubblewrap();
      break;
    case "terrain_simple":
      rules = Rules.terrain_simple();
      break;
    case "checkers":
      rules = Rules.checkers();
      break;
    case "stripes":
      rules = Rules.stripes();
      break;
    case "flowers":
      rules = Rules.flowers();
      break;
    case "link":
      rules = Rules.link();
      break;
    case "village":
      rules = Rules.village();
      break;
    case "simple_wall":
      rules = Rules.simple_wall();
      break;
    case "skyline2":
      rules = Rules.skyline2();
      break;
    default:
      rules = Rules.from_json(ruleset.rules);
      break;
    //   throw new Error("Unknown ruleset: '" + ruleset + "'");
  }
  return rules;
}

function getBacktracker(variant: BacktrackerVariant | null) {
  let backtracker = null;
  if (variant !== null) {
    backtracker = new_backtracker(variant);
  }
  return backtracker;
}

let initPromise: Promise<void>;
async function initWasm() {
  await initSync();
  console.info("Web Assembly Loaded");
}

async function reset(
  seed: number,
  dimensions: Dimensions,
  ruleset: InbuiltRuleSet | CustomRule,
  backtrackerVariant: BacktrackerVariant | null,
  customRules: CustomRule[],
  cause: string,
) {
  console.info("reset", { dimensions, ruleset, backtrackerVariant, cause });
  console.time("getRules");
  const rules = getRules(ruleset);
  console.timeEnd("getRules");
  // const tileset = rules.get_visual_tileset();
  console.time("reset");
  const grid = new Grid(
    BigInt(seed),
    rules,
    dimensions.width,
    dimensions.height,
  );
  console.timeEnd("reset");
  const backtracker = getBacktracker(backtrackerVariant);
  persistent_state = {
    grid,
    seed,
    history_cache: new Map(),
    tickBacktracker: backtracker,
    customRules,
  };
  console.info({ persistent_state });
  return persistent_state;
}

type PersistentState = {
  grid: Grid;
  // tileset: TileVisual[];
  tickBacktracker: Backtracker2D | null;
  seed: number;
  history_cache: Map<number, State>;
  customRules: CustomRule[];
};
type PersistentStateUpdate = PersistentState & { was_reset?: boolean };
let persistent_state: PersistentState | undefined = undefined;
async function usePersistentState(
  basics: BaseSettings,
  customRules?: CustomRule[],
) {
  let state = persistent_state;
  let was_reset = false;
  if (state === undefined) {
    state = await reset(
      basics.seed.value,
      basics.dimensions,
      basics.rules,
      basics.backtracker,
      customRules ?? [],
      "init",
    );
    persistent_state = state;
    was_reset = true;
  } else if (customRules) {
    state.customRules = customRules;
  }
  return { ...state, was_reset };
}

function state(s: PersistentState, outputSize: number, t?: number): State {
  const dimensions = s.grid.get_dimensions();
  const history_len = s.grid.get_history_len();
  // let tiles;
  // if (t !== undefined) {
  //   let existing = s.history_cache.get(t);
  //   tiles = s.grid.dump_at_time(t);
  // } else {
  //   tiles = s.grid.dump();
  // }
  let rendered = s.grid.render(outputSize, outputSize, t);
  return {
    ...dimensions,
    // tiles,
    rendered,
    history_len,
    history_position: t,
    seed: s.seed,
  };
}
self.onmessage = async (e: MessageEvent<WorkerRequest>) => {
  // console.log("Worker got message", e.data);

  // wait until wasm has been loaded
  if (!initPromise) {
    initPromise = initWasm();
  }
  await initPromise;
  if (e.data.type === "extract_rules") {
    const rules = Rules.extract_rules_from_bitmap(
      e.data.source,
      e.data.options,
    );
    postMessage({
      type: "extracted_rules",
      result: {
        name: e.data.name,
        options: e.data.options,
        rules: rules,
      },
    } satisfies WorkerResponse);
    return;
  }
  let s: PersistentStateUpdate = await usePersistentState(
    e.data,
    e.data.type === "setCustomRules" ? e.data.customRules : undefined,
  );
  let seedForReset = e.data.seed.value;
  if (e.data.seed.allowRandomization && e.data.type !== "reset") {
    seedForReset = pickRandomSeed();
  }

  if (
    s.grid.is_finished() &&
    (
      ["tick", "run", "collapse"] satisfies WorkerRequest["type"][] as string[]
    ).includes(e.data.type)
  ) {
    if (e.data.type === "tick") {
      console.info("persisting image for a bit");
      await new Promise((resolve) => setTimeout(resolve, 2000));
    }

    s = await reset(
      seedForReset,
      e.data.dimensions,
      e.data.rules,
      e.data.backtracker,
      s.customRules,
      "grid was finished",
    );
  }

  if (e.data.type === "setCustomRules") {
    const resp: WorkerResponse = {
      type: "state_update",
      state: state(s, e.data.outputSize),
    };
    self.postMessage(resp);
  } else if (e.data.type === "reset") {
    if (!s.was_reset) {
      s = await reset(
        seedForReset,
        e.data.dimensions,
        e.data.rules,
        e.data.backtracker,
        s.customRules,
        "requested",
      );
    }
    const resp: WorkerResponse = {
      type: "state_update",
      state: state(s, e.data.outputSize),
      // tileset: s.tileset,
    };
    self.postMessage(resp);
  } else if (e.data.type === "tick") {
    await tick(e.data, s);
  } else if (e.data.type === "run") {
    await run(e.data, s);
  } else if (e.data.type === "collapse") {
    await collapse(e.data, s);
  } else if (e.data.type === "rule_check") {
    const rules = getRules(e.data.rules);
    console.debug({ rules });
    let allowed = [];
    const targetB = BigUint64Array.from(e.data.target.map((v) => BigInt(v)));
    const fromB = BigUint64Array.from(e.data.from.map((v) => BigInt(v)));
    console.debug({ targetB, fromB });
    const res = rules.check(targetB, fromB, e.data.direction);
    allowed = Array.from(res).map((v) => Number(v));
    console.debug({ allowed });
    const resp: WorkerResponse = {
      type: "rule_check",
      allowed,
      state: state(s, e.data.outputSize),
    };
    self.postMessage(resp);
  } else if (e.data.type === "read_past") {
    const resp: WorkerResponse = {
      type: "state_update",
      state: state(s, e.data.outputSize, e.data.t),
    };
    self.postMessage(resp);
  } else {
    console.warn("Unknown worker request", e.data);
  }
};

async function tick(data: WorkerRequest, s: PersistentStateUpdate) {
  if (data.type !== "tick") {
    throw new Error("Unexpected message type");
  }
  console.time("tick");
  s.grid.tick(s.tickBacktracker);
  console.timeEnd("tick");
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(s, data.outputSize),
    // tileset: s.was_reset ? s.tileset : undefined,
  };
  self.postMessage(resp);
}

async function run(data: WorkerRequest, s: PersistentStateUpdate) {
  if (data.type !== "run") {
    throw new Error("Unexpected message type");
  }
  const dimensions = s.grid.get_dimensions();
  console.time("run");
  s.grid.run(dimensions.width * dimensions.height * 100, data.backtracker);
  console.timeEnd("run");
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(s, data.outputSize),
    // tileset: s.was_reset ? s.tileset : undefined,
  };
  self.postMessage(resp);
}

async function collapse(data: WorkerRequest, s: PersistentStateUpdate) {
  // console.debug("collapse", { data });
  if (data.type !== "collapse") {
    throw new Error("Unexpected message type");
  }
  s.grid.collapse(
    data.x,
    data.y,
    data.state !== undefined ? data.state : undefined,
  );
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(s, data.outputSize),
    // tileset: s.was_reset ? s.tileset : undefined,
  };
  self.postMessage(resp);
}
