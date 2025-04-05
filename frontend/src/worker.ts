import initSync, { Grid, Rules, Dimensions, Tile } from "aaltofunktionromautus";

console.log("Worker loaded");

export type State = {
  tiles: Tile[];
} & Dimensions;

export const INBUILT_RULE_SETS = [
  "terrain",
  "terrain_simple",
  "checkers",
] as const;
export type InbuiltRuleSet = (typeof INBUILT_RULE_SETS)[number];

export type BaseSettings = {
  dimensions?: Dimensions;
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
);

let ready = false;
async function reset(dimensions?: Dimensions, ruleset?: InbuiltRuleSet) {
  if (!ready) {
    await initSync();
    ready = true;
    console.info("Worker loaded wasm!");
  }
  let rules: Rules = Rules.terrain();
  switch (ruleset) {
    case "terrain":
      rules = Rules.terrain();
      break;
    case "terrain_simple":
      rules = Rules.terrain_simple();
      break;
    case "checkers":
      rules = Rules.checkers();
      break;
  }
  console.debug("Rules loaded");
  if (grid && !dimensions) {
    dimensions = grid.get_dimensions();
  }
  const g = new Grid(rules, dimensions?.width ?? 30, dimensions?.height ?? 30);
  console.debug("Grid created");
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
  // console.log("Worker got message", e.data);

  if (grid === undefined) {
    grid = await reset(e.data.dimensions, e.data.rules);
  }
  if (grid.is_finished()) {
    if (e.data.type === "tick") {
      console.info("persiting image for a bit");
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
    grid = await reset(e.data.dimensions, e.data.rules);
  }

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
  if (data.type !== "collapse") {
    throw new Error("Unexpected message type");
  }
  grid.collapse(data.x, data.y);
  const resp: WorkerResponse = {
    type: "state_update",
    state: state(),
  };
  self.postMessage(resp);
}
