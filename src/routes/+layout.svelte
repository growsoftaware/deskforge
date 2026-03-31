<script lang="ts">
  import "../app.css";
  import { page } from "$app/state";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  const modules = [
    { name: "Keyboard", path: "/keyboard", icon: "⌨" },
    { name: "Apps", path: "/apps", icon: "🚀" },
  ];

  let { children } = $props();
  let autostart = $state(false);

  onMount(async () => {
    try {
      autostart = await invoke<boolean>("get_autostart");
    } catch {}
  });

  async function toggleAutostart() {
    try {
      await invoke("set_autostart", { enabled: !autostart });
      autostart = !autostart;
    } catch (e) {
      console.error("Failed to toggle autostart:", e);
    }
  }
</script>

<div class="app-shell">
  <nav class="sidebar">
    <div class="sidebar-header">
      <h1>DeskForge</h1>
    </div>
    <ul class="nav-list">
      {#each modules as mod}
        <li>
          <a
            href={mod.path}
            class:active={page.url.pathname.startsWith(mod.path)}
          >
            <span class="nav-icon">{mod.icon}</span>
            <span class="nav-label">{mod.name}</span>
          </a>
        </li>
      {/each}
    </ul>
    <div class="sidebar-footer">
      <label class="autostart-toggle">
        <input type="checkbox" checked={autostart} onchange={toggleAutostart} />
        <span>Iniciar com o sistema</span>
      </label>
      <span class="version">v0.1.0</span>
    </div>
  </nav>
  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  .app-shell {
    display: flex;
    height: 100vh;
  }

  .sidebar {
    width: 180px;
    background: var(--mantle);
    border-right: 1px solid var(--surface0);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar-header {
    padding: 20px 16px 16px;
    border-bottom: 1px solid var(--surface0);
  }

  .sidebar-header h1 {
    font-size: 15px;
    font-weight: 700;
    color: var(--accent);
    letter-spacing: 1.5px;
    text-transform: uppercase;
  }

  .nav-list {
    list-style: none;
    padding: 8px;
    flex: 1;
  }

  .nav-list a {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 9px 12px;
    border-radius: 6px;
    color: var(--overlay1);
    text-decoration: none;
    font-weight: 500;
    font-size: 13px;
    transition: all 0.15s ease;
  }

  .nav-list a:hover {
    background: var(--surface0);
    color: var(--text);
  }

  .nav-list a.active {
    background: var(--accent-dim);
    color: var(--accent);
  }

  .nav-icon {
    font-size: 16px;
  }

  .nav-label {
    font-size: 13px;
  }

  .sidebar-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--surface0);
  }

  .autostart-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    color: var(--overlay0);
    cursor: pointer;
    margin-bottom: 8px;
  }

  .autostart-toggle input {
    accent-color: var(--accent);
    cursor: pointer;
  }

  .version {
    font-size: 10px;
    color: var(--surface2);
    font-family: inherit;
  }

  .content {
    flex: 1;
    padding: 24px;
    overflow-y: auto;
  }
</style>
