import {nodeResolve} from "@rollup/plugin-node-resolve"
import {wasm} from "@rollup/plugin-wasm"

export default {
  input: "./script.mjs",
  output: {
    file: "./script.bundle.js",
    format: "iife"
  },
  plugins: [nodeResolve(), wasm()]
}
