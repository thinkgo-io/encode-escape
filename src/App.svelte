<script lang="ts">
  import Button from "./components/Button.svelte";
  import RadioBar from "./components/RadioBar.svelte";
  import Dropdown from "./components/Dropdown.svelte";
  import Copy from "./icons/Copy.svelte";
  import Swap from "./icons/Swap.svelte";

  import { log, logError } from "./utils/log";

  import type { ListItem } from "./types/listItem";

  import { input, result } from "./store/content";
  import { encodingList, encoding, variantList, variant } from "./store/encode";

  import { encode } from "./backend/encode";
  import { setEncodingByName, swap } from "./domain/encode";

  /* Functions  ─────────────────────────────────────────── */

  function reportError(context: string, error: Error) {
    result.set(error.message);
    logError(context, error);
  }

  /* Event Handlers ─────────────────────────────────────── */

  async function onCopy() {
    try {
      log("On Copy");
      navigator.clipboard.writeText($result);
    } catch (error) {
      reportError("App - onCopy", error);
    }
  }

  async function onEncode() {
    try {
      log("On Encode: " + $encoding + " - " + $variant);
      result.set(await encode($encoding, $variant, $input));
    } catch (error) {
      reportError("App - onJustDoIt", error);
    }
  }

  async function onSwap() {
    try {
      log("On Swap");
      log("Input\n" + $input);
      swap();
    } catch (error) {
      reportError("App - onSwap", error);
    }
  }

  async function onSelectEncoding(event) {
    try {
      log("On Select Encoding: " + JSON.stringify(event.detail));
      setEncodingByName(event.detail.id);
    } catch (error) {
      reportError("App - onSwap", error);
    }
  }

  async function onSelectVariant(event) {
    try {
      log("On Select Variant: " + JSON.stringify(event.detail));
      variant.set(event.detail.id);
    } catch (error) {
      reportError("App - onSwap", error);
    }
  }
</script>

<div class="toolbar">
  <Dropdown
    items={$encodingList}
    selected={$encoding}
    unselectedLabel="Set Encoding"
    on:select={(event) => onSelectEncoding(event)}
  />

  <RadioBar
    name="variant"
    bind:items={$variantList}
    bind:selected={$variant}
    on:select={(event) => onSelectVariant(event)}
  />

  <Button on:click={onSwap}>
    <Swap />
    Swap
  </Button>

  <Button on:click={onCopy}>
    <Copy />
    Copy
  </Button>
</div>

<div class="content">
  <textarea
    id="input"
    class="item form-control code"
    placeholder="Input here"
    bind:value={$input}
    on:input={() => onEncode()}
  />
  <textarea
    id="result"
    class="item form-control code"
    placeholder="Results here"
    bind:value={$result}
  />
</div>
