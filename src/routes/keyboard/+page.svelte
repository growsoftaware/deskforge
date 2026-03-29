<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface RemapStatus {
    id: string;
    source: string;
    target: string;
    active: boolean;
    icon: string | null;
    label: string;
  }

  interface DeviceFix {
    id: string;
    name: string;
    description: string;
    active: boolean;
    detected: boolean;
  }

  interface TextMacro {
    id: string;
    name: string;
    trigger: string;
    text: string;
    method: string;
  }

  let remaps: RemapStatus[] = $state([]);
  let macrosList: TextMacro[] = $state([]);
  let deviceFixes: DeviceFix[] = $state([]);
  let loading = $state(true);
  let toggling = $state<string | null>(null);
  let togglingDevice = $state<string | null>(null);

  // Macro editor state
  let editing = $state(false);
  let editId = $state<string | null>(null);
  let editName = $state("");
  let editTrigger = $state("");
  let editText = $state("");
  let editMethod = $state("clipboard");
  let recording = $state(false);
  let saving = $state(false);

  async function loadData() {
    try {
      const [r, m, d] = await Promise.all([
        invoke<RemapStatus[]>("get_remap_statuses"),
        invoke<TextMacro[]>("get_macros"),
        invoke<DeviceFix[]>("get_device_fixes"),
      ]);
      remaps = r;
      macrosList = m;
      deviceFixes = d;
    } catch (e) {
      console.error("Failed to load data:", e);
    } finally {
      loading = false;
    }
  }

  async function handleToggle(remapId: string) {
    toggling = remapId;
    try {
      const updated = await invoke<RemapStatus>("toggle_remap", { remapId });
      remaps = remaps.map((r) => (r.id === updated.id ? updated : r));
    } catch (e) {
      console.error("Failed to toggle remap:", e);
    } finally {
      toggling = null;
    }
  }

  async function handleDeviceToggle(fixId: string) {
    togglingDevice = fixId;
    try {
      const nowActive = await invoke<boolean>("toggle_device_fix", { id: fixId });
      deviceFixes = deviceFixes.map((f) =>
        f.id === fixId ? { ...f, active: nowActive } : f,
      );
    } catch (e) {
      console.error("Failed to toggle device fix:", e);
    } finally {
      togglingDevice = null;
    }
  }

  function openEditor(mac?: TextMacro) {
    if (mac) {
      editId = mac.id;
      editName = mac.name;
      editTrigger = mac.trigger;
      editText = mac.text;
      editMethod = mac.method;
    } else {
      editId = null;
      editName = "";
      editTrigger = "";
      editText = "";
      editMethod = "clipboard";
    }
    editing = true;
  }

  function closeEditor() {
    editing = false;
    recording = false;
  }

  function handleKeyRecord(e: KeyboardEvent) {
    if (!recording) return;
    e.preventDefault();
    e.stopPropagation();

    // Ignore lone modifier keys
    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return;

    const parts: string[] = [];
    if (e.ctrlKey) parts.push("Ctrl");
    if (e.shiftKey) parts.push("Shift");
    if (e.altKey) parts.push("Alt");
    if (e.metaKey) parts.push("Super");

    // Map key names
    let key = e.key;
    if (key === " ") key = "Space";
    else if (key === "Escape") key = "Escape";
    else if (key.length === 1) key = key.toUpperCase();

    parts.push(key);
    editTrigger = parts.join("+");
    recording = false;
  }

  async function saveMacro() {
    if (!editName.trim() || !editTrigger.trim() || !editText.trim()) return;

    saving = true;
    try {
      if (editId) {
        await invoke("update_macro", {
          id: editId,
          name: editName.trim(),
          trigger: editTrigger.trim(),
          text: editText,
          method: editMethod,
        });
      } else {
        const id = "macro-" + Date.now();
        await invoke("add_macro", {
          id,
          name: editName.trim(),
          trigger: editTrigger.trim(),
          text: editText,
          method: editMethod,
        });
      }
      closeEditor();
      macrosList = await invoke<TextMacro[]>("get_macros");
    } catch (e) {
      console.error("Failed to save macro:", e);
      alert("Erro ao salvar: " + e);
    } finally {
      saving = false;
    }
  }

  async function deleteMacro(id: string) {
    try {
      await invoke("delete_macro", { id });
      macrosList = await invoke<TextMacro[]>("get_macros");
    } catch (e) {
      console.error("Failed to delete macro:", e);
    }
  }

  onMount(() => {
    loadData();
  });
</script>

<svelte:window onkeydown={handleKeyRecord} />

<div class="keyboard-page">
  <header class="page-header">
    <h2>Keyboard Manager</h2>
    <p class="subtitle">Remapeamento de teclas, atalhos e macros</p>
  </header>

  <!-- Remaps Section -->
  <section class="section">
    <h3>Remapeamentos</h3>
    {#if loading}
      <p class="loading">Carregando...</p>
    {:else if remaps.length === 0}
      <p class="empty-state">Nenhum remapeamento configurado.</p>
    {:else}
      <div class="card-list">
        {#each remaps as remap}
          <div class="card" class:active={remap.active}>
            <div class="card-left">
              <span class="card-icon">{remap.icon ?? "⌨"}</span>
              <div class="card-info">
                <span class="card-title">{remap.source} → {remap.target}</span>
                <span class="card-status" class:status-on={remap.active}>
                  {remap.label}
                </span>
              </div>
            </div>
            <label class="toggle">
              <input
                type="checkbox"
                checked={remap.active}
                disabled={toggling === remap.id}
                onchange={() => handleToggle(remap.id)}
              />
              <span class="toggle-slider"></span>
            </label>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  <!-- Device Fixes Section -->
  {#if deviceFixes.some((f) => f.detected)}
    <section class="section">
      <h3>Dispositivos</h3>
      <div class="card-list">
        {#each deviceFixes.filter((f) => f.detected) as fix}
          <div class="card" class:active={fix.active}>
            <div class="card-left">
              <span class="card-icon">🔧</span>
              <div class="card-info">
                <span class="card-title">{fix.name}</span>
                <span class="card-status" class:status-on={fix.active}>
                  {fix.description}
                  {#if fix.active}— ativo{/if}
                </span>
              </div>
            </div>
            <label class="toggle">
              <input
                type="checkbox"
                checked={fix.active}
                disabled={togglingDevice === fix.id}
                onchange={() => handleDeviceToggle(fix.id)}
              />
              <span class="toggle-slider"></span>
            </label>
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <!-- Macros Section -->
  <section class="section">
    <div class="section-header">
      <h3>Macros de Texto</h3>
      {#if !editing}
        <button class="btn-add" onclick={() => openEditor()}>+ Nova Macro</button>
      {/if}
    </div>

    {#if editing}
      <div class="editor-card">
        <div class="field">
          <label for="macro-name">Nome</label>
          <input
            id="macro-name"
            type="text"
            placeholder="Ex: Email Signature"
            bind:value={editName}
          />
        </div>
        <div class="field">
          <label for="macro-trigger">Atalho</label>
          <div class="trigger-row">
            <input
              id="macro-trigger"
              type="text"
              placeholder="Clique em Gravar e pressione as teclas"
              bind:value={editTrigger}
              readonly
            />
            <button
              class="btn-record"
              class:recording
              onclick={() => (recording = !recording)}
            >
              {recording ? "⏺ Gravando..." : "⌨ Gravar"}
            </button>
          </div>
        </div>
        <div class="field">
          <label for="macro-text">Texto</label>
          <textarea
            id="macro-text"
            placeholder="Texto que será colado ao pressionar o atalho"
            bind:value={editText}
            rows="3"
          ></textarea>
        </div>
        <div class="field">
          <label for="macro-method">Método</label>
          <select id="macro-method" bind:value={editMethod}>
            <option value="clipboard">Clipboard (Ctrl+V)</option>
            <option value="type">Simular digitação</option>
          </select>
        </div>
        <div class="editor-actions">
          <button class="btn-cancel" onclick={closeEditor}>Cancelar</button>
          <button class="btn-save" onclick={saveMacro} disabled={saving}>
            {saving ? "Salvando..." : "Salvar"}
          </button>
        </div>
      </div>
    {/if}

    {#if macrosList.length === 0 && !editing}
      <p class="empty-state">
        Nenhuma macro configurada. Crie uma para colar textos com atalhos de
        teclado.
      </p>
    {:else}
      <div class="card-list">
        {#each macrosList as mac}
          <div class="card">
            <div class="card-left">
              <span class="card-icon">📝</span>
              <div class="card-info">
                <span class="card-title">{mac.name}</span>
                <span class="card-status">
                  <kbd>{mac.trigger}</kbd>
                  → {mac.text.length > 40
                    ? mac.text.slice(0, 40) + "..."
                    : mac.text}
                </span>
              </div>
            </div>
            <div class="card-actions">
              <button class="btn-icon" onclick={() => openEditor(mac)} title="Editar">✏️</button>
              <button class="btn-icon btn-delete" onclick={() => deleteMacro(mac.id)} title="Remover">🗑</button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
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

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .section h3 {
    font-size: 13px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: #666;
    margin-bottom: 0;
  }

  .section-header + .card-list,
  .section-header + .empty-state {
    margin-top: 0;
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

  .card.active {
    border-color: #e9456044;
  }

  .card-left {
    display: flex;
    align-items: center;
    gap: 14px;
    min-width: 0;
    flex: 1;
  }

  .card-icon {
    font-size: 24px;
    flex-shrink: 0;
  }

  .card-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
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
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .card-status.status-on {
    color: #e94560;
  }

  kbd {
    background: #0f3460;
    border: 1px solid #1a3a6e;
    border-radius: 4px;
    padding: 1px 6px;
    font-size: 11px;
    font-family: monospace;
    color: #e94560;
  }

  .card-actions {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }

  .btn-icon {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 16px;
    padding: 4px 6px;
    border-radius: 6px;
    opacity: 0.6;
    transition: opacity 0.15s;
  }

  .btn-icon:hover {
    opacity: 1;
    background: #1a1a3e;
  }

  .btn-delete:hover {
    background: #3e1a1a;
  }

  .toggle {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    cursor: pointer;
    flex-shrink: 0;
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

  .btn-add {
    background: #e94560;
    color: white;
    border: none;
    border-radius: 8px;
    padding: 6px 14px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-add:hover {
    background: #d63a55;
  }

  /* Editor */
  .editor-card {
    background: #16213e;
    border: 1px solid #e94560;
    border-radius: 10px;
    padding: 18px;
    margin-bottom: 12px;
  }

  .field {
    margin-bottom: 14px;
  }

  .field label {
    display: block;
    font-size: 12px;
    font-weight: 600;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 6px;
  }

  .field input,
  .field textarea,
  .field select {
    width: 100%;
    background: #1a1a2e;
    border: 1px solid #0f3460;
    border-radius: 8px;
    padding: 10px 12px;
    color: #e0e0e0;
    font-size: 14px;
    font-family: inherit;
    outline: none;
    transition: border-color 0.15s;
  }

  .field input:focus,
  .field textarea:focus,
  .field select:focus {
    border-color: #e94560;
  }

  .field textarea {
    resize: vertical;
    min-height: 60px;
  }

  .field select {
    cursor: pointer;
  }

  .trigger-row {
    display: flex;
    gap: 8px;
  }

  .trigger-row input {
    flex: 1;
  }

  .btn-record {
    background: #0f3460;
    color: #e0e0e0;
    border: 1px solid #1a3a6e;
    border-radius: 8px;
    padding: 8px 14px;
    font-size: 13px;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s;
  }

  .btn-record:hover {
    background: #1a3a6e;
  }

  .btn-record.recording {
    background: #e94560;
    border-color: #e94560;
    color: white;
    animation: pulse 1s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.7;
    }
  }

  .editor-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 4px;
  }

  .btn-cancel {
    background: none;
    border: 1px solid #333;
    color: #888;
    border-radius: 8px;
    padding: 8px 18px;
    font-size: 13px;
    cursor: pointer;
  }

  .btn-cancel:hover {
    border-color: #666;
    color: #e0e0e0;
  }

  .btn-save {
    background: #e94560;
    color: white;
    border: none;
    border-radius: 8px;
    padding: 8px 18px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
  }

  .btn-save:hover {
    background: #d63a55;
  }

  .btn-save:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
