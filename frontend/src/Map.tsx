import { Component, For } from "solid-js";
import type { Tile } from "aaltofunktionromautus";

import styles from "./Map.module.css";

const Map: Component<{
  tiles: Tile[][];
  onTileClick: (x: number, y: number) => void;
}> = ({ tiles, onTileClick }) => {
  return (
    <div
      style={{
        display: "flex",
        "flex-direction": "column",
      }}
    >
      <For each={tiles}>
        {(row, y) => (
          <div
            data-index={y()}
            style={{
              display: "flex",
            }}
          >
            {
              <For each={row}>
                {(item, x) => (
                  <div data-index={y()}>
                    <div
                      class={styles.tile_container}
                      style={{
                        "--states": item.possible_states.length,
                      }}
                      data-tile={
                        item.possible_states.length === 1
                          ? item.possible_states[0]
                          : undefined
                      }
                      onClick={() => {
                        onTileClick(x(), y());
                      }}
                    >
                      <For each={item.possible_states}>
                        {(state) => (
                          <div class={styles.tile} data-tile={state} />
                        )}
                      </For>
                    </div>
                  </div>
                )}
              </For>
            }
          </div>
        )}
      </For>
    </div>
  );
};

export default Map;
