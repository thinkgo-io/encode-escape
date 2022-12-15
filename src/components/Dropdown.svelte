<script lang="ts">
  // Events ─────────────────────────────────────────────── //

  // "select" - Fires when an item is selected from the dropdown.

  // Imports ────────────────────────────────────────────── //

  import type { ListItem } from "../types/listItem";
  import { createEventDispatcher } from "svelte";

  /* Parameters ─────────────────────────────────────────── */

  export let items: ListItem[];
  export let selected: string;
  export let unselectedLabel: string;

  /* Init ───────────────────────────────────────────────── */

  const dispatch = createEventDispatcher();

  $: selectedBind = { items, selected };
  $: label = getLabel(selected);
  $: tooltip = getTooltip(selected);

  // Functions ──────────────────────────────────────────── //

  function isSelected(item: ListItem, selected): boolean {
    return item.id === selected;
  }

  function getLabel(selected: string): string {
    return getSelected(selected)?.label || unselectedLabel;
  }

  function getSelected(selected: string): ListItem | null {
    for (const item of items) if (isSelected(item, selected)) return item;
    return null;
  }

  function getTooltip(selected: string): string {
    return getSelected(selected)?.tooltip || "";
  }

  /* Action Handlers ─────────────────────────────────────── */

  const onSelect = (item: ListItem) => {
    // label = item.label;
    dispatch("select", { id: item.id });
  };
</script>

<div class="dropdown toolbar-item item">
  {#if tooltip}
    <button
      class="btn theme dropdown-toggle toolbar-item"
      type="button"
      data-bs-toggle="dropdown"
      data-toggle="dropdown"
      data-placement="bottom"
      title={tooltip}
      aria-expanded="false"
    >
      {label}
    </button>
  {:else}
    <button
      class="btn theme dropdown-toggle toolbar-item"
      type="button"
      data-bs-toggle="dropdown"
      aria-expanded="false"
    >
      {label}
    </button>
  {/if}
  <ul class="dropdown-menu toolbar-item theme">
    {#each items as item}
      <li
        class="toolbar-item"
        on:click={() => onSelect(item)}
        on:keydown={() => onSelect(item)}
        selected={item.id === selected || null}
      >
        {#if item.tooltip}
          <a
            class="dropdown-item"
            data-bs-toggle="tooltip"
            data-bs-placement="bottom"
            title={item.tooltip}
          >
            {item.label}
          </a>
        {:else}
          <a class="dropdown-item">
            {item.label}
          </a>
        {/if}
      </li>
    {/each}
  </ul>
</div>

<style>
  button {
    min-width: 8rem;
  }
  .dropdown-toggle:after {
    margin-left: 0.5rem;
    vertical-align: 0.21rem;
  }
  .item {
    display:inline-flex;
  }
</style>
