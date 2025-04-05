import initSync, { Grid, Rules, Dimensions, Tile } from "aaltofunktionromautus";

console.log("Worker loaded");

export type State = {
  tiles: Tile[];
} & Dimensions;

export type WorkerRequest =
  | {
      type: "reset";
    }
  | {
      type: "tick";
    }
  | {
      type: "collapse";
      x: number;
      y: number;
    };

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
async function reset() {
  if (!ready) {
    await initSync();
    ready = true;
    console.info("Worker loaded wasm!");
  }
  const rules = Rules.terrain();
  console.debug("Rules loaded");
  const g = new Grid(rules, 30, 30);
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
    grid = await reset();
  }

  if (e.data.type === "reset") {
    grid = await reset();
    const resp: WorkerResponse = {
      type: "state_update",
      state: state(),
    };
    self.postMessage(resp);
  } else if (e.data.type === "tick") {
    await tick(e.data);
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
