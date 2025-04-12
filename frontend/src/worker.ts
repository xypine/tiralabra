import initSync, {
  Grid,
  Rules,
  Dimensions,
  Tile,
  TileState,
  Direction2D,
  TileVisual,
} from "aaltofunktionromautus";

const worker_id = Math.floor(Math.random() * 1000);
console.log(`Worker ${worker_id} loaded`);

export type State = {
  tiles: Tile[];
} & Dimensions;

export const INBUILT_RULE_SETS = [
  "terrain",
  "flowers_singlepixel",
  "terrain_simple",
  "checkers",
  "stripes",
] as const;
export type InbuiltRuleSet = (typeof INBUILT_RULE_SETS)[number];

export type BaseSettings = {
  dimensions: Dimensions;
  rules: InbuiltRuleSet;
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
        type: "collapse";
        x: number;
        y: number;
        state?: number;
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

function getRules(ruleset?: InbuiltRuleSet) {
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
    default:
      throw new Error("Unknown ruleset: '" + ruleset + "'");
  }
  return rules;
}

let initPromise: Promise<void>;
async function initWasm() {
  await initSync();
  console.info("Web Assembly Loaded");
}

async function reset(dimensions: Dimensions, ruleset?: InbuiltRuleSet) {
  const rules = getRules(ruleset);
  const tileset = rules.get_visual_tileset();
  const grid = new Grid(rules, dimensions.width, dimensions.height);
  if (ruleset === "flowers_singlepixel") {
    console.debug("collapsing ground");
    grid.collapse(0, grid.get_dimensions().height - 1, BigInt(0));
  }
  persistent_state = { grid, tileset };
  return persistent_state;
}

type PersistentState = { grid: Grid; tileset: TileVisual[] };
type PersistentStateUpdate = PersistentState & { was_reset?: boolean };
let persistent_state: PersistentState | undefined = undefined;
async function usePersistentState(basics: BaseSettings) {
  let state = persistent_state;
  let was_reset = false;
  if (state === undefined) {
    state = await reset(basics.dimensions, basics.rules);
    persistent_state = state;
    was_reset = true;
  }
  return { ...state, was_reset };
}

function state(s: PersistentState): State {
  const dimensions = s.grid.get_dimensions();
  const tiles = s.grid.dump();
  return {
    ...dimensions,
    tiles,
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

  if (s.grid.is_finished()) {
    if (e.data.type === "tick") {
      console.info("persiting image or a bit");
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
    s = await reset(e.data.dimensions, e.data.rules);
  }

  if (e.data.type === "reset") {
    if (!s.was_reset) {
      s = await reset(e.data.dimensions, e.data.rules);
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
  s.grid.run(dimensions.width * dimensions.height);
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
    data.state !== undefined ? BigInt(data.state) : undefined,
  );
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(s),
    tileset: s.was_reset ? s.tileset : undefined,
  };
  self.postMessage(resp);
}
