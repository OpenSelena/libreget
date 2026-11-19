<script lang="ts">
  import { t } from "$lib/i18n";

  let {
    choice = $bindable("video"),
    isMix = false,
    onChoose,
  }: {
    choice?: "video" | "list";
    isMix?: boolean;
    onChoose?: (next: "video" | "list") => void;
  } = $props();

  function select(next: "video" | "list") {
    choice = next;
    onChoose?.(next);
  }
</script>

<fieldset class="video-or-list">
  <legend>{$t(isMix ? "librebox.video_or_list_mix" : "librebox.video_or_list")}</legend>
  <div class="options">
    <button
      type="button"
      aria-pressed={choice === "video"}
      class:active={choice === "video"}
      onclick={() => select("video")}
    >
      {$t("librebox.video_or_list_video")}
    </button>
    <button
      type="button"
      aria-pressed={choice === "list"}
      class:active={choice === "list"}
      onclick={() => select("list")}
    >
      {$t(isMix ? "librebox.video_or_list_mix_list" : "librebox.video_or_list_list")}
    </button>
  </div>
</fieldset>

<style>
  .video-or-list {
    border: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-2, 8px);
  }

  legend {
    padding: 0;
    margin-block-end: var(--space-2, 8px);
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }

  .options {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2, 8px);
  }

  button {
    flex: 1 1 140px;
    min-height: 34px;
    padding: 0 var(--space-3, 12px);
    border: none;
    border-radius: var(--radius-md, 8px);
    background: var(--surface, rgba(255, 255, 255, 0.04));
    box-shadow: inset 0 0 0 var(--hairline, 1px) var(--border, rgba(255, 255, 255, 0.1));
    color: var(--text);
    font: inherit;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background var(--duration-fast, 120ms) ease, color var(--duration-fast, 120ms) ease;
  }

  button:hover {
    background: var(--fill-2, rgba(255, 255, 255, 0.08));
  }

  button.active {
    background: var(--accent);
    box-shadow: none;
    color: var(--on-accent, #fff);
    font-weight: 600;
  }

  button:focus-visible {
    outline: var(--focus-ring, 2px solid var(--accent));
    outline-offset: var(--focus-ring-offset, 2px);
  }
</style>
