<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface AppEntry {
    id: string;
    name: string;
    description: string;
    icon: string;
    command: string;
    terminal: boolean;
  }

  const apps: AppEntry[] = [
    {
      id: "grok-media",
      name: "Grok Media",
      description: "Gerar imagens e vídeos com a API do Grok (xAI)",
      icon: "🎨",
      command: "python3 ~/grok-media/app.py",
      terminal: true,
    },
  ];

  let launching = $state<string | null>(null);

  async function launch(app: AppEntry) {
    launching = app.id;
    try {
      await invoke("launch_app", {
        command: app.command,
        terminal: app.terminal,
      });
    } catch (e) {
      console.error("Failed to launch:", e);
      alert("Erro ao abrir: " + e);
    } finally {
      setTimeout(() => (launching = null), 1000);
    }
  }
</script>

<div class="apps-page">
  <header class="page-header">
    <h2>Apps</h2>
    <p class="subtitle">Atalhos para aplicativos integrados</p>
  </header>

  <section class="section">
    <div class="app-grid">
      {#each apps as app}
        <button
          class="app-card"
          onclick={() => launch(app)}
          disabled={launching === app.id}
        >
          <span class="app-icon">{app.icon}</span>
          <div class="app-info">
            <span class="app-name">{app.name}</span>
            <span class="app-desc">{app.description}</span>
          </div>
          <span class="app-launch">
            {launching === app.id ? "Abrindo..." : "Abrir"}
          </span>
        </button>
      {/each}
    </div>
  </section>
</div>

<style>
  .apps-page {
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

  .app-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .app-card {
    display: flex;
    align-items: center;
    gap: 16px;
    background: #16213e;
    border: 1px solid #0f3460;
    border-radius: 12px;
    padding: 18px 20px;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: left;
    color: inherit;
    font-family: inherit;
  }

  .app-card:hover {
    border-color: #e94560;
    background: #1a2545;
  }

  .app-card:disabled {
    opacity: 0.6;
    cursor: wait;
  }

  .app-icon {
    font-size: 32px;
    flex-shrink: 0;
  }

  .app-info {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .app-name {
    font-size: 16px;
    font-weight: 600;
    color: #f0f0f0;
  }

  .app-desc {
    font-size: 12px;
    color: #888;
    margin-top: 3px;
  }

  .app-launch {
    background: #e94560;
    color: white;
    border: none;
    border-radius: 8px;
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 600;
    flex-shrink: 0;
    pointer-events: none;
  }
</style>
