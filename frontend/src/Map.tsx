import { Component, For } from "solid-js";
import type { Location2D, Tile } from "aaltofunktionromautus";

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
                      style={{
                        margin: 0,
                        width: "2rem",
                        height: "2rem",
                        padding: "0.5rem",
                      }}
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
