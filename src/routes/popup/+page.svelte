<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";

  let text = $derived(page.url.searchParams.get("text") ?? "");
  let icon = $derived(page.url.searchParams.get("icon") ?? "");
  let displayMs = $derived(
    Number(page.url.searchParams.get("display_ms")) || 1500,
  );
  let fadeMs = $derived(Number(page.url.searchParams.get("fade_ms")) || 500);

  let phase: "visible" | "fading" | "done" = $state("visible");

  function dismiss() {
    // Close the window via Tauri
    import("@tauri-apps/api/window").then(({ getCurrentWindow }) => {
      getCurrentWindow().close();
    });
  }

  onMount(() => {
    // Start fade after display period
    const fadeTimer = setTimeout(() => {
      phase = "fading";
    }, displayMs);

    // Close after fade completes
    const closeTimer = setTimeout(() => {
      phase = "done";
      dismiss();
    }, displayMs + fadeMs);

    return () => {
      clearTimeout(fadeTimer);
      clearTimeout(closeTimer);
    };
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="popup-container"
  style="--fade-ms: {fadeMs}ms"
  class:fading={phase === "fading"}
  onclick={dismiss}
>
  <div class="popup-pill">
    {#if icon}
      <span class="popup-icon">{icon}</span>
    {/if}
    <span class="popup-text">{text}</span>
  </div>
</div>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    padding: 0;
    background: #1e1e1e;
    overflow: hidden;
  }

  .popup-container {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100vw;
    height: 100vh;
    cursor: pointer;
    opacity: 1;
    transition: opacity var(--fade-ms) ease-out;
  }

  .popup-container.fading {
    opacity: 0;
  }

  .popup-pill {
    display: flex;
    align-items: center;
    gap: 12px;
    background: #1e1e1e;
    border-radius: 12px;
    padding: 12px 24px;
    border: 1px solid #333;
  }

  .popup-icon {
    font-size: 20px;
    color: #fff;
    line-height: 1;
  }

  .popup-text {
    font-family: "Inter", "SF Pro", "Segoe UI", system-ui, sans-serif;
    font-size: 15px;
    font-weight: 600;
    color: #fff;
    white-space: nowrap;
  }
</style>
