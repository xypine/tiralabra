import { Component, For } from "solid-js";
import type { Tile } from "aaltofunktionromautus";

import styles from "./Map.module.css";

const Map: Component<{ tiles: Tile[][] }> = ({ tiles }) => {
  return (
    <>
      <For each={tiles}>
        {(row, x) => (
          <div
            data-index={x()}
            style={{
              display: "flex",
            }}
          >
            {
              <For each={row}>
                {(item, y) => (
                  <div data-index={y()}>
                    <p
                      class={styles.tile}
                      data-tile={
                        item.possible_states.length === 1
                          ? item.possible_states[0]
                          : undefined
                      }
                    >
                      {item.possible_states.length === 1
                        ? item.possible_states[0]
                        : item.possible_states.length + "?"}
                    </p>
                  </div>
                )}
              </For>
            }
          </div>
        )}
      </For>
    </>
  );
};

export default Map;
