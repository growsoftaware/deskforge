<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Remap {
    id: string;
    source: string;
    target: string;
    active: boolean;
    icon: string | null;
    label_on: string;
    label_off: string;
  }

  interface Config {
    keyboard: {
      enabled: boolean;
      remaps: Remap[];
    };
  }

  let config: Config | null = $state(null);

  onMount(async () => {
    config = await invoke<Config>("get_config");
  });
</script>

<div class="keyboard-page">
  <header class="page-header">
    <h2>Keyboard Manager</h2>
    <p class="subtitle">Remapeamento de teclas e atalhos</p>
  </header>

  <section class="section">
    <h3>Remapeamentos</h3>
    {#if config}
      <div class="card-list">
        {#each config.keyboard.remaps as remap}
          <div class="card">
            <div class="card-left">
              <span class="card-icon">{remap.icon ?? "⌨"}</span>
              <div class="card-info">
                <span class="card-title">{remap.source} → {remap.target}</span>
                <span class="card-status">
                  {remap.active ? remap.label_on : remap.label_off}
                </span>
              </div>
            </div>
            <label class="toggle">
              <input type="checkbox" checked={remap.active} disabled />
              <span class="toggle-slider"></span>
            </label>
          </div>
        {/each}
      </div>
    {:else}
      <p class="loading">Carregando...</p>
    {/if}
  </section>

  <section class="section">
    <h3>Macros de Texto</h3>
    <p class="empty-state">Nenhuma macro configurada. Em breve...</p>
  </section>
</div>

<style>
  .keyboard-page {
    max-width: 640px;
  }

  .page-header {
    margin-bottom: 28px;
  }

  .page-header h2 {
    font-size: 22px;
    font-weight: 700;
    color: #f0f0f0;
  }

  .subtitle {
    color: #888;
    font-size: 13px;
    margin-top: 4px;
  }

  .section {
    margin-bottom: 28px;
  }

  .section h3 {
    font-size: 13px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: #666;
    margin-bottom: 12px;
  }

  .card-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: #16213e;
    border: 1px solid #0f3460;
    border-radius: 10px;
    padding: 14px 18px;
    transition: border-color 0.15s;
  }

  .card:hover {
    border-color: #e94560;
  }

  .card-left {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .card-icon {
    font-size: 24px;
  }

  .card-info {
    display: flex;
    flex-direction: column;
  }

  .card-title {
    font-weight: 600;
    font-size: 14px;
    color: #e0e0e0;
  }

  .card-status {
    font-size: 12px;
    color: #888;
    margin-top: 2px;
  }

  .toggle {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    cursor: pointer;
  }

  .toggle input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    inset: 0;
    background: #333;
    border-radius: 24px;
    transition: 0.2s;
  }

  .toggle-slider::before {
    content: "";
    position: absolute;
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background: #888;
    border-radius: 50%;
    transition: 0.2s;
  }

  .toggle input:checked + .toggle-slider {
    background: #e94560;
  }

  .toggle input:checked + .toggle-slider::before {
    transform: translateX(20px);
    background: #fff;
  }

  .loading {
    color: #666;
    font-style: italic;
  }

  .empty-state {
    color: #555;
    font-size: 13px;
    padding: 16px;
    background: #16213e;
    border: 1px dashed #0f3460;
    border-radius: 10px;
    text-align: center;
  }
</style>
