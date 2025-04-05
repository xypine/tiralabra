import Solver from "./Solver";

import styles from "./App.module.css";
import { Component } from "solid-js";

const App: Component = () => {
  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <Solver />
      </header>
    </div>
  );
};

export default App;
