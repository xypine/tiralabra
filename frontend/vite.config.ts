import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [wasm(), topLevelAwait(), solidPlugin()],
  optimizeDeps: {
    exclude: ["aaltofunktionromautus"],
  },
  server: {
    port: 3000,
  },
  build: {
    target: "esnext",
  },
});
