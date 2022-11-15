<script lang="ts">
  // Events ─────────────────────────────────────────────── //

  // "select" - When an item is selected from the dropdown.

  // Imports ────────────────────────────────────────────── //

  import { createEventDispatcher } from "svelte";
  import { beforeUpdate, afterUpdate, onMount } from "svelte";
  import type { ListItem } from "../types/listItem";
  import { log } from "../utils/log";

  /* Parameters ─────────────────────────────────────────── */

  export let name: string; // Used to create unique ids for the elements.
  export let items: ListItem[];
  export let selected: string;

  /* Init ───────────────────────────────────────────────── */

  const dispatch = createEventDispatcher();
  const groupId = "radio-bar-" + name;

  $: selectedBind = { items, selected };

  /* Functions ──────────────────────────────────────────── */

  function getChecked(item: ListItem): string {
    return isSelected(item) ? "checked" : "";
  }

  function isSelected(item: ListItem): boolean {
    return item.id === selected;
  }

  /* Action Handlers ─────────────────────────────────────── */

  const onSelect = (item: ListItem) => {
    dispatch("select", { id: item.id });
  };
</script>

<div
  class="btn-group toolbar-group item"
  role="group"
  aria-label="Basic radio toggle button group"
  id={groupId}
>
  {#each items as item}
    <input
      type="radio"
      class="btn-check"
      {name}
      id="{name}-{item.id}"
      autocomplete="off"
      checked={item.id === selected || null}
    />
    {#if item.tooltip}
      <label
        class="btn btn-outline-secondary toolbar-item"
        for="{name}-{item.id}"
        data-toggle="tooltip"
        data-placement="bottom"
        title={item.tooltip}
        on:click={() => onSelect(item)}
        on:keydown={() => onSelect(item)}>{item.label}</label
      >
    {:else}
      <label
        class="btn btn-outline-secondary toolbar-item"
        for="{name}-{item.id}"
        on:click={() => onSelect(item)}
        on:keydown={() => onSelect(item)}>{item.label}</label
      >
    {/if}
  {/each}
</div>

<style>
  label {
    transition: none;
    min-width: 6em;
  }
</style>
