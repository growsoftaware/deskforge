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
    {
      id: "win11-start",
      name: "Iniciar VM Windows",
      description: "Iniciar a máquina virtual Windows 11 (libvirt)",
      icon: "🖥️",
      command: "sudo virsh --connect qemu:///system start win11",
      terminal: true,
    },
    {
      id: "win11-rdp",
      name: "Windows Desktop",
      description: "Conectar ao desktop Windows em tela cheia (RDP)",
      icon: "🪟",
      command: "xfreerdp /v:192.168.122.78 /u:krakenlab /p:Mystapler1 /cert:ignore +clipboard /dynamic-resolution /f",
      terminal: false,
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
    font-size: 18px;
    font-weight: 700;
    color: var(--text);
    letter-spacing: 0.5px;
  }

  .subtitle {
    color: var(--overlay0);
    font-size: 12px;
    margin-top: 4px;
  }

  .section {
    margin-bottom: 28px;
  }

  .app-grid {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .app-card {
    display: flex;
    align-items: center;
    gap: 14px;
    background: var(--mantle);
    border: 1px solid var(--surface0);
    border-radius: 8px;
    padding: 14px 18px;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: left;
    color: inherit;
    font-family: inherit;
  }

  .app-card:hover {
    border-color: var(--surface2);
    background: var(--surface0);
  }

  .app-card:disabled {
    opacity: 0.5;
    cursor: wait;
  }

  .app-icon {
    font-size: 26px;
    flex-shrink: 0;
  }

  .app-info {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .app-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .app-desc {
    font-size: 11px;
    color: var(--overlay0);
    margin-top: 2px;
  }

  .app-launch {
    background: var(--accent-dim);
    color: var(--accent);
    border: none;
    border-radius: 6px;
    padding: 6px 14px;
    font-size: 11px;
    font-weight: 600;
    font-family: inherit;
    flex-shrink: 0;
    pointer-events: none;
    transition: all 0.15s;
  }

  .app-card:hover .app-launch {
    background: var(--accent);
    color: var(--crust);
  }
</style>
