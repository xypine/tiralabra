import initSync, {
  Grid,
  Rules,
  Dimensions,
  Tile,
  TileState,
} from "aaltofunktionromautus";
import { Direction2D } from "../pkg/aaltofunktionromautus";

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
  console.debug("getRules", { ruleset });
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
  console.debug("Rules loaded");
  const g = new Grid(getRules(ruleset), dimensions.width, dimensions.height);
  console.debug("Grid created");
  if (ruleset === "flowers_singlepixel") {
    console.debug("collapsing ground");
    g.collapse(0, g.get_dimensions().height - 1, BigInt(0));
  }
  return g;
}

let grid: Grid;
function state(): State {
  const dimensions = grid.get_dimensions();
  const tiles = grid.dump();
  return {
    ...dimensions,
    tiles,
  };
}
self.onmessage = async (e: MessageEvent<WorkerRequest>) => {
  console.log("Worker got message", e.data);

  // wait until wasm has been loaded
  if (!initPromise) {
    initPromise = initWasm();
  }
  await initPromise;

  if (grid === undefined) {
    grid = await reset(e.data.dimensions, e.data.rules);
  }
  console.debug("grid is set");
  if (grid.is_finished()) {
    console.debug("grid has finished");
    if (e.data.type === "tick") {
      console.info("persiting image for a bit");
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
    grid = await reset(e.data.dimensions, e.data.rules);
  }
  console.debug("grid has been checked");

  if (e.data.type === "reset") {
    grid = await reset(e.data.dimensions, e.data.rules);
    const resp: WorkerResponse = {
      type: "state_update",
      state: state(),
    };
    self.postMessage(resp);
  } else if (e.data.type === "tick") {
    await tick(e.data);
  } else if (e.data.type === "run") {
    await run(e.data);
  } else if (e.data.type === "collapse") {
    await collapse(e.data);
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
      state: state(),
    };
    self.postMessage(resp);
  } else {
    console.warn("Unknown worker request", e.data);
  }
};

async function tick(data: WorkerRequest) {
  if (data.type !== "tick") {
    throw new Error("Unexpected message type");
  }
  grid.tick();
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(),
  };
  self.postMessage(resp);
}

async function run(data: WorkerRequest) {
  if (data.type !== "run") {
    throw new Error("Unexpected message type");
  }
  const dimensions = grid.get_dimensions();
  grid.run(dimensions.width * dimensions.height);
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(),
  };
  self.postMessage(resp);
}

async function collapse(data: WorkerRequest) {
  console.debug("collapse", { data });
  if (data.type !== "collapse") {
    throw new Error("Unexpected message type");
  }
  grid.collapse(
    data.x,
    data.y,
    data.state !== undefined ? BigInt(data.state) : undefined,
  );
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(),
  };
  self.postMessage(resp);
}
