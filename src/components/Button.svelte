<script lang="ts">
  export let tooltip: string = "";
</script>

{#if tooltip}
  <button
    type="button"
    class="btn theme toolbar-item item"
    data-toggle="tooltip"
    data-placement="bottom"
    title={tooltip}
    on:click
  >
    <slot />
  </button>
{:else}
  <button type="button" class="btn theme toolbar-item item" on:click>
    <slot />
  </button>
{/if}
