import "./style.css";
import App from "./App.svelte";

import { getEncodings } from "./backend/encode";
import { setEncodings } from "./domain/encode";
import { log, logError, TO_BACKEND } from "./utils/log";

// Initialize ─────────────────────────────────────────── //

try {
  log("main - starting", TO_BACKEND);
  setEncodings(await getEncodings());
}
catch (error) {
  logError("main - initialize", error);
}; 

// Start App ──────────────────────────────────────────── //

const app = new App({
  target: document.getElementById("app"),
});

export default app;