<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
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
  let hasNuphy = $derived(deviceFixes.some((f) => f.detected));
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

    const unlisten = listen<RemapStatus>("remap-changed", (event) => {
      const updated = event.payload;
      remaps = remaps.map((r) => (r.id === updated.id ? updated : r));
    });

    return () => {
      unlisten.then((fn) => fn());
    };
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

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .section h3 {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 1.2px;
    color: var(--overlay0);
    margin-bottom: 0;
  }

  .section-header + .card-list,
  .section-header + .empty-state {
    margin-top: 0;
  }

  .card-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--mantle);
    border: 1px solid var(--surface0);
    border-radius: 8px;
    padding: 12px 16px;
    transition: border-color 0.15s;
  }

  .card:hover {
    border-color: var(--surface2);
  }

  .card.active {
    border-color: var(--accent);
    border-left: 3px solid var(--accent);
  }

  .card-left {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
    flex: 1;
  }

  .card-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .card-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .card-title {
    font-weight: 600;
    font-size: 13px;
    color: var(--text);
  }

  .card-status {
    font-size: 11px;
    color: var(--overlay0);
    margin-top: 2px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .card-status.status-on {
    color: var(--green);
  }

  kbd {
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 4px;
    padding: 1px 6px;
    font-size: 10px;
    font-family: inherit;
    color: var(--accent);
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
    font-size: 14px;
    padding: 4px 6px;
    border-radius: 6px;
    opacity: 0.5;
    transition: all 0.15s;
  }

  .btn-icon:hover {
    opacity: 1;
    background: var(--surface0);
  }

  .btn-delete:hover {
    background: rgba(243, 139, 168, 0.15);
  }

  .toggle {
    position: relative;
    display: inline-block;
    width: 40px;
    height: 22px;
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
    background: var(--surface1);
    border-radius: 22px;
    transition: 0.2s;
  }

  .toggle-slider::before {
    content: "";
    position: absolute;
    height: 16px;
    width: 16px;
    left: 3px;
    bottom: 3px;
    background: var(--overlay0);
    border-radius: 50%;
    transition: 0.2s;
  }

  .toggle input:checked + .toggle-slider {
    background: var(--accent);
  }

  .toggle input:checked + .toggle-slider::before {
    transform: translateX(18px);
    background: var(--crust);
  }

  .btn-add {
    background: var(--accent-dim);
    color: var(--accent);
    border: 1px solid transparent;
    border-radius: 6px;
    padding: 5px 12px;
    font-size: 12px;
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-add:hover {
    background: var(--accent);
    color: var(--crust);
  }

  /* Editor */
  .editor-card {
    background: var(--mantle);
    border: 1px solid var(--accent);
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 12px;
  }

  .field {
    margin-bottom: 14px;
  }

  .field label {
    display: block;
    font-size: 10px;
    font-weight: 600;
    color: var(--overlay0);
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-bottom: 6px;
  }

  .field input,
  .field textarea,
  .field select {
    width: 100%;
    background: var(--base);
    border: 1px solid var(--surface0);
    border-radius: 6px;
    padding: 9px 12px;
    color: var(--text);
    font-size: 13px;
    font-family: inherit;
    outline: none;
    transition: border-color 0.15s;
  }

  .field input:focus,
  .field textarea:focus,
  .field select:focus {
    border-color: var(--accent);
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
    background: var(--surface0);
    color: var(--subtext0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    padding: 8px 14px;
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.15s;
  }

  .btn-record:hover {
    background: var(--surface1);
    color: var(--text);
  }

  .btn-record.recording {
    background: var(--red);
    border-color: var(--red);
    color: var(--crust);
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
    border: 1px solid var(--surface1);
    color: var(--overlay0);
    border-radius: 6px;
    padding: 7px 16px;
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
  }

  .btn-cancel:hover {
    border-color: var(--overlay0);
    color: var(--text);
  }

  .btn-save {
    background: var(--accent);
    color: var(--crust);
    border: none;
    border-radius: 6px;
    padding: 7px 16px;
    font-size: 12px;
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-save:hover {
    background: var(--accent-hover);
  }

  .btn-save:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .loading {
    color: var(--overlay0);
    font-style: italic;
  }

  .empty-state {
    color: var(--overlay0);
    font-size: 12px;
    padding: 16px;
    background: var(--mantle);
    border: 1px dashed var(--surface0);
    border-radius: 8px;
    text-align: center;
  }
</style>
