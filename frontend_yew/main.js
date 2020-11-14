import init, { run_app } from "./pkg/bundle.js";
async function main() {
  await init("/pkg/bundle_bg.wasm");
  run_app();
}
main();
