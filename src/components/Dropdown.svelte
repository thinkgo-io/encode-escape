<script lang="ts">
  import type { ListItem } from "../types/listItem";
  import { createEventDispatcher } from "svelte";

  /* Parameters ─────────────────────────────────────────── */
  export let items: ListItem[];
  export let selected: string;
  export let unselectedLabel: string;

  /* Init ───────────────────────────────────────────────── */

  const dispatch = createEventDispatcher();

  $: label = getLabel(items, unselectedLabel);

  // Functions ──────────────────────────────────────────── //

  function isSelected(item: ListItem): boolean {
    return item.id === selected;
  }

  function getLabel(items: ListItem[], unselectedLabel: string): string {
    for (const item of items) if (isSelected(item)) return item.label;
    return unselectedLabel;
  }

  /* Action Handlers ─────────────────────────────────────── */

  const onSelect = (item: ListItem) => {
    label = item.label;
    dispatch("select", { id: item.id });
  };
</script>

<div class="dropdown item">
  <button
    class="btn btn-secondary dropdown-toggle toolbar-item"
    type="button"
    data-bs-toggle="dropdown"
    aria-expanded="false"
  >
    {label}
  </button>
  <ul class="dropdown-menu dropdown-menu-dark toolbar-item">
    {#each items as item}
      <li
        class="toolbar-item"
        on:click={() => onSelect(item)}
        on:keydown={() => onSelect(item)}
      >
        <a class="dropdown-item">
          {item.label}
        </a>
      </li>
    {/each}
  </ul>
</div>

<style>
  button {
    min-width: 8em;
  }
  ul,
  li {
    font-size: 1.1em;
  }
  .dropdown-toggle:after {
    margin-left: 0.5em;
    vertical-align: 0.21em;
  }
</style>
