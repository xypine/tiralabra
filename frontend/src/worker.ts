import initSync, {
  Grid,
  Rules,
  Dimensions,
  TileState,
  Direction2D,
  TileVisual,
} from "aaltofunktionromautus";
import { pickRandomSeed, Seed } from "./utils";

const worker_id = Math.floor(Math.random() * 1000);
console.log(`Worker ${worker_id} loaded`);

export type State = {
  // tiles: Tile[];
  rendered: string;
  history_len: number;
  history_position?: number;
  seed: number;
} & Dimensions;

export const INBUILT_RULE_SETS = [
  "terrain",
  "flowers_singlepixel",
  "flowers",
  "terrain_simple",
  "checkers",
  "stripes",
] as const;
export type InbuiltRuleSet = (typeof INBUILT_RULE_SETS)[number];

export type BaseSettings = {
  dimensions: Dimensions;
  rules: InbuiltRuleSet;
  seed: Seed;
};

export type WorkerRequest = BaseSettings &
  (
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
  );

export type WorkerResponse = {
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
  );

function getRules(ruleset: InbuiltRuleSet) {
  let rules;
  switch (ruleset) {
    case "terrain":
      rules = Rules.terrain();
      break;
    case "flowers_singlepixel":
      rules = Rules.flowers_singlepixel();
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
    // default:
    //   throw new Error("Unknown ruleset: '" + ruleset + "'");
  }
  return rules;
}

let initPromise: Promise<void>;
async function initWasm() {
  await initSync();
  console.info("Web Assembly Loaded");
}

async function reset(
  seed: number,
  dimensions: Dimensions,
  ruleset: InbuiltRuleSet,
  cause: string,
) {
  console.info("reset", { dimensions, ruleset, cause });
  const rules = getRules(ruleset);
  const tileset = rules.get_visual_tileset();
  const grid = new Grid(
    BigInt(seed),
    rules,
    dimensions.width,
    dimensions.height,
  );
  persistent_state = { grid, tileset, seed, history_cache: new Map() };
  return persistent_state;
}

type PersistentState = {
  grid: Grid;
  tileset: TileVisual[];
  seed: number;
  history_cache: Map<number, State>;
};
type PersistentStateUpdate = PersistentState & { was_reset?: boolean };
let persistent_state: PersistentState | undefined = undefined;
async function usePersistentState(basics: BaseSettings) {
  let state = persistent_state;
  let was_reset = false;
  if (state === undefined) {
    state = await reset(
      basics.seed.value,
      basics.dimensions,
      basics.rules,
      "init",
    );
    persistent_state = state;
    was_reset = true;
  }
  return { ...state, was_reset };
}

function state(s: PersistentState, t?: number): State {
  const dimensions = s.grid.get_dimensions();
  const history_len = s.grid.get_history_len();
  // let tiles;
  // if (t !== undefined) {
  //   let existing = s.history_cache.get(t);
  //   tiles = s.grid.dump_at_time(t);
  // } else {
  //   tiles = s.grid.dump();
  // }
  let rendered = s.grid.render(500, 500, t);
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

  let s: PersistentStateUpdate = await usePersistentState(e.data);
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
      console.info("persiting image or a bit");
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }

    s = await reset(
      seedForReset,
      e.data.dimensions,
      e.data.rules,
      "grid was finished",
    );
  }

  if (e.data.type === "reset") {
    if (!s.was_reset) {
      s = await reset(
        seedForReset,
        e.data.dimensions,
        e.data.rules,
        "requested",
      );
    }
    const resp: WorkerResponse = {
      type: "state_update",
      state: state(s),
      tileset: s.tileset,
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
      state: state(s),
    };
    self.postMessage(resp);
  } else if (e.data.type === "read_past") {
    const resp: WorkerResponse = {
      type: "state_update",
      state: state(s, e.data.t),
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
  s.grid.tick();
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(s),
    tileset: s.was_reset ? s.tileset : undefined,
  };
  self.postMessage(resp);
}

async function run(data: WorkerRequest, s: PersistentStateUpdate) {
  if (data.type !== "run") {
    throw new Error("Unexpected message type");
  }
  const dimensions = s.grid.get_dimensions();
  s.grid.run(dimensions.width * dimensions.height * 100);
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(s),
    tileset: s.was_reset ? s.tileset : undefined,
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
    state: state(s),
    tileset: s.was_reset ? s.tileset : undefined,
  };
  self.postMessage(resp);
}
