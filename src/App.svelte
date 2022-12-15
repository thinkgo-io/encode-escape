<script lang="ts">
  import Button from "./components/Button.svelte";
  import RadioBar from "./components/RadioBar.svelte";
  import Dropdown from "./components/Dropdown.svelte";
  import Copy from "./icons/Copy.svelte";
  import Swap from "./icons/Swap.svelte";

  import { log, logError } from "./utils/log";

  import { EncodeOperation } from "./types/encoding";
  import type { ListItem } from "./types/listItem";

  import { input, result } from "./store/content";
  import {
    encodingList,
    encoding,
    operationList,
    operation,
  } from "./store/encoding";

  import { encode } from "./backend/encode";
  import {
    setEncodingAndOperationByName,
    setEncodingByName,
    swap,
  } from "./domain/encode";
  import { getCurrentEncodeOperation, getEncodings } from "./backend/encode";
  import { setEncodings } from "./domain/encode";
  import { onMount } from "svelte";

  onMount(async () => {
    setEncodings(await getEncodings());
    let encodeOperation = await getCurrentEncodeOperation();
    setEncodingAndOperationByName(
      encodeOperation.encoding,
      encodeOperation.operation
    );
  });

  /* Functions  ─────────────────────────────────────────── */

  function clear() {
    input.set("");
    result.set("");
  }

  async function encodeIt() {
    let encode_operation = new EncodeOperation($encoding, $operation);
    result.set(
      await encode(new EncodeOperation($encoding, $operation), $input)
    );
  }

  function reportError(context: string, error: Error) {
    result.set(error.message);
    logError(context, error);
  }

  /* Event Handlers ─────────────────────────────────────── */

  function onClear(event) {
    if (event.key !== "Escape") return;
    event.preventDefault();
    clear();
  }

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
      log("On Encode: " + $encoding + " - " + $operation);
      encodeIt();
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
      encodeIt();
    } catch (error) {
      reportError("App - onSwap", error);
    }
  }

  async function onSelectOperation(event) {
    try {
      log("On Select Operation: " + JSON.stringify(event.detail));
      operation.set(event.detail.id);
      encodeIt();
    } catch (error) {
      reportError("App - onSwap", error);
    }
  }
</script>

<div class="toolbar">
  <Dropdown
    items={$encodingList}
    bind:selected={$encoding}
    unselectedLabel="Set Encoding"
    on:select={(event) => onSelectEncoding(event)}
  />

  <RadioBar
    name="operation"
    bind:items={$operationList}
    bind:selected={$operation}
    on:select={(event) => onSelectOperation(event)}
  />

  <Button on:click={onSwap} tooltip={"Swap input and results text."}>
    <Swap />
    Swap
  </Button>

  <Button on:click={onCopy} tooltip={"Copy results to the clipboard."}>
    <Copy />
    Copy
  </Button>
</div>

<div class="content">
  <textarea
    id="input"
    class="item form-control code theme"
    placeholder="Input here"
    bind:value={$input}
    on:input={() => onEncode()}
    on:keypress={(event) => onClear(event)}
  />
  <textarea
    id="result"
    class="item form-control code theme"
    placeholder="Results here"
    bind:value={$result}
    on:keypress={(event) => onClear(event)}
  />
</div>
