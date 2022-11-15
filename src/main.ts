import "./style.css";
import App from "./App.svelte";

import { getEncodings } from "./backend/encode";
import { setEncodings } from "./domain/encode";
import { log, logError, TO_BACKEND } from "./utils/log";

// Start App ──────────────────────────────────────────── //

const app = new App({
  target: document.getElementById("app"),
});

export default app;