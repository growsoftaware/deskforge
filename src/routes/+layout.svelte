<script lang="ts">
  import "../app.css";
  import { page } from "$app/state";

  const modules = [
    { name: "Keyboard", path: "/keyboard", icon: "⌨" },
  ];

  let { children } = $props();
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
    width: 200px;
    background: #16213e;
    border-right: 1px solid #0f3460;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar-header {
    padding: 20px 16px 16px;
    border-bottom: 1px solid #0f3460;
  }

  .sidebar-header h1 {
    font-size: 18px;
    font-weight: 700;
    color: #e94560;
    letter-spacing: 0.5px;
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
    padding: 10px 12px;
    border-radius: 8px;
    color: #a0a0b8;
    text-decoration: none;
    font-weight: 500;
    transition: all 0.15s ease;
  }

  .nav-list a:hover {
    background: #1a1a3e;
    color: #e0e0e0;
  }

  .nav-list a.active {
    background: #0f3460;
    color: #e94560;
  }

  .nav-icon {
    font-size: 18px;
  }

  .nav-label {
    font-size: 14px;
  }

  .sidebar-footer {
    padding: 12px 16px;
    border-top: 1px solid #0f3460;
  }

  .version {
    font-size: 11px;
    color: #555;
  }

  .content {
    flex: 1;
    padding: 24px;
    overflow-y: auto;
  }
</style>
