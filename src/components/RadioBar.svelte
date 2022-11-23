<script lang="ts">
  // Events ─────────────────────────────────────────────── //

  // "select" - Fires when an item is selected from the radio bar.

  // Imports ────────────────────────────────────────────── //

  import type { ListItem } from "../types/listItem";
  import { createEventDispatcher } from "svelte";

  /* Parameters ─────────────────────────────────────────── */

  export let name: string; // Used to create unique ids for the elements.
  export let items: ListItem[];
  export let selected: string;

  /* Init ───────────────────────────────────────────────── */

  const dispatch = createEventDispatcher();
  const groupId = "radio-bar-" + name;

  $: selectedBind = { items, selected };

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
        class="btn theme-outline toolbar-item"
        for="{name}-{item.id}"
        data-toggle="tooltip"
        data-placement="bottom"
        title={item.tooltip}
        on:click={() => onSelect(item)}
        on:keydown={() => onSelect(item)}>{item.label}</label
      >
    {:else}
      <label
        class="btn theme-outline toolbar-item"
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
