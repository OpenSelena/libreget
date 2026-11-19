<script lang="ts">
  import { t } from "$lib/i18n";
  import { formatBytes } from "$lib/download-format";
  import DialogContainer from "$components/dialog/DialogContainer.svelte";

  export type LeftoverFile = { path: string; name: string; size_bytes: number };

  let {
    open = $bindable(false),
    files = [],
    onRemove,
  }: {
    open?: boolean;
    files: LeftoverFile[];
    onRemove?: (deletePartials: boolean) => void;
  } = $props();

  let total = $derived(files.reduce((sum, f) => sum + f.size_bytes, 0));

  function finish(deletePartials: boolean) {
    open = false;
    onRemove?.(deletePartials);
  }

  function cancel() {
    open = false;
  }
</script>

<DialogContainer bind:isOpen={open} titleId="leftovers-dialog-title" onClose={cancel}>
  <div class="leftovers-content">
    <h3 id="leftovers-dialog-title" class="leftovers-title">
      {$t("downloads.leftovers_title")}
    </h3>
    <p class="leftovers-desc">
      {$t("downloads.leftovers_body", { count: files.length, size: formatBytes(total) })}
    </p>

    <ul class="leftovers-list" role="list">
      {#each files as file (file.path)}
        <li class="leftovers-item" title={file.path}>
          <span class="file-name" dir="ltr">{file.name}</span>
          <span class="file-size">{formatBytes(file.size_bytes)}</span>
        </li>
      {/each}
    </ul>

    <div class="leftovers-actions">
      <button type="button" class="btn btn-secondary" onclick={cancel}>
        {$t("common.cancel")}
      </button>
      <button type="button" class="btn btn-secondary" onclick={() => finish(false)}>
        {$t("downloads.leftovers_keep")}
      </button>
      <button type="button" class="btn btn-destructive" onclick={() => finish(true)}>
        {$t("downloads.leftovers_delete")}
      </button>
    </div>
  </div>
</DialogContainer>

<style>
  .leftovers-content {
    display: flex;
    flex-direction: column;
    padding: var(--space-4, 16px);
    gap: var(--space-3, 12px);
  }

  .leftovers-title {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
    color: var(--text, #fff);
  }

  .leftovers-desc {
    margin: 0;
    font-size: 13px;
    line-height: 1.5;
    color: var(--text-dim, rgba(255, 255, 255, 0.7));
  }

  .leftovers-list {
    list-style: none;
    margin: 0;
    padding: var(--space-2, 8px);
    max-height: 180px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-1, 4px);
    font-size: 12px;
    background: var(--surface, rgba(255, 255, 255, 0.04));
    border: 1px solid var(--border, rgba(255, 255, 255, 0.1));
    border-radius: var(--radius-md, 8px);
  }

  .leftovers-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-3, 12px);
    padding: 4px 6px;
    border-radius: 4px;
  }

  .leftovers-item:hover {
    background: var(--fill-2, rgba(255, 255, 255, 0.06));
  }

  .file-name {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text, #fff);
  }

  .file-size {
    flex-shrink: 0;
    font-variant-numeric: tabular-nums;
    color: var(--text-dim, rgba(255, 255, 255, 0.6));
  }

  .leftovers-actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: var(--space-2, 8px);
    margin-top: var(--space-2, 8px);
  }

  .btn {
    min-height: 32px;
    padding: 0 var(--space-3, 12px);
    border: none;
    border-radius: var(--radius-md, 8px);
    font: inherit;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background var(--duration-fast, 120ms) ease;
  }

  .btn:focus-visible {
    outline: var(--focus-ring, 2px solid var(--accent));
    outline-offset: var(--focus-ring-offset, 2px);
  }

  .btn-secondary {
    background: var(--surface, rgba(255, 255, 255, 0.06));
    box-shadow: inset 0 0 0 var(--hairline, 1px) var(--border, rgba(255, 255, 255, 0.12));
    color: var(--text, #fff);
  }

  .btn-secondary:hover {
    background: var(--fill-2, rgba(255, 255, 255, 0.12));
  }

  .btn-destructive {
    background: var(--danger, #e53935);
    color: #fff;
    font-weight: 600;
  }

  .btn-destructive:hover {
    filter: brightness(1.1);
  }
</style>
