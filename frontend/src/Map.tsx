import { Accessor, Component, createMemo, For } from "solid-js";
import type { Tile } from "aaltofunktionromautus";

import styles from "./Map.module.css";

const Map: Component<{
  tiles: Accessor<Tile[][]>;
  onTileClick: (x: number, y: number) => void;
  width: Accessor<number>;
  height: Accessor<number>;
}> = ({ width, height, tiles, onTileClick }) => {
  const mtiles = createMemo(tiles);
  const heightIterator = createMemo(() =>
    Array.from({ length: height() }, (_, i) => i),
  );
  const widthIterator = createMemo(() =>
    Array.from({ length: width() }, (_, i) => i),
  );
  const getTile = (x: number, y: number) => {
    return mtiles().at(x)?.at(y);
  };
  return (
    <div
      style={{
        "--unit": "min(1vw, 1vh)",
        "--target-area": "calc(var(--unit) * 85)",
        "--w": width(),
        "--h": height(),
        "--tile-w": "calc(var(--target-area) / var(--w))",
        "--tile-h": "calc(var(--target-area) / var(--h))",
        width: "var(--target-area)",
        height: "var(--target-area)",
        display: "flex",
        "flex-direction": "column",
      }}
    >
      <For each={heightIterator()}>
        {(y) => (
          <div
            data-index={y}
            style={{
              display: "flex",
            }}
          >
            <For each={widthIterator()}>
              {(x) => (
                <Tile
                  x={x}
                  y={y}
                  onTileClick={() => onTileClick(x, y)}
                  tile={() => getTile(x, y)}
                />
              )}
            </For>
          </div>
        )}
      </For>
    </div>
  );
};

const Tile: Component<{
  tile: Accessor<Tile | undefined>;
  onTileClick: () => void;
  x: number;
  y: number;
}> = ({ x, y, tile, onTileClick }) => {
  const mtile = createMemo(tile);
  return (
    <div data-x={x} data-y={y}>
      <div
        class={styles["tile-container"]}
        style={{
          "--states": mtile()?.possible_states.length ?? 0,
        }}
        data-collapsed={
          mtile()?.possible_states.length === 1 ? true : undefined
        }
        onClick={() => {
          onTileClick();
        }}
      >
        <For each={mtile()?.possible_states}>
          {(state) => <div class={styles.tile} data-tile={state} />}
        </For>
      </div>
    </div>
  );
};

export default Map;
