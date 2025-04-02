import { createEffect, createSignal, type Component } from "solid-js";
import initSync, { Grid, Rules, Location2D, Tile } from "aaltofunktionromautus";

import logo from "./logo.svg";
import styles from "./App.module.css";

const App: Component = () => {
  const [wasmReady, setWasmReady] = createSignal(false);
  initSync().then(() => {
    setWasmReady(true);
  });

  createEffect(() => {
    if (wasmReady()) {
      const rules = Rules.checkers();
      const g = new Grid(rules, 5, 5);
      const img: Map<Location2D, Tile> = g.image();
      console.debug(img);
    }
  });

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid
        </a>
      </header>
    </div>
  );
};

export default App;
