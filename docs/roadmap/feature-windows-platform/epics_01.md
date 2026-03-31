# Epic 01 — Platform Abstraction Layer

> **Goal:** O codebase compila no Windows sem erros. Nenhuma feature funciona ainda, mas a estrutura está pronta.

## Rationale

Hoje o projeto **não compila** no Windows. O `platform/mod.rs` só exporta `linux`, e módulos como `devices.rs` fazem `fs::read_to_string("/proc/...")` incondicionalmente. Este epic cria os guardrails para que o código Linux fique isolado e o código Windows tenha onde morar.

É o alicerce. Sem ele, nada mais funciona.

## Acceptance Criteria

- [ ] `cargo check --target x86_64-pc-windows-msvc` passa sem erros
- [ ] `cargo check --target x86_64-unknown-linux-gnu` continua passando
- [ ] Nenhuma funcionalidade Linux é removida ou quebrada
- [ ] `platform/windows.rs` existe com stubs que retornam `Err("Not implemented")`
- [ ] `devices.rs` está gated com `#[cfg(target_os = "linux")]`
- [ ] `macros.rs` tem branch `#[cfg]` com stubs para Windows
- [ ] `popup.rs` tem branch `#[cfg]` com stub para Windows
- [ ] `autostart.rs` tem branch `#[cfg]` com stub para Windows
- [ ] `lib.rs` `launch_app` tem branch `#[cfg]` para shell/terminal detection
- [ ] `Cargo.toml` declara dependências Windows-only com `[target.'cfg(...)']`
- [ ] O app **abre** no Windows (janela vazia com sidebar é suficiente)

## Dependencies

Nenhuma. Este é o primeiro epic.

## Architecture Notes

### Padrão `#[cfg]` — Onde aplicar

```
Arquivo                  │ Estratégia
─────────────────────────┼────────────────────────────────────────
platform/mod.rs          │ Adicionar #[cfg(windows)] pub mod windows;
platform/windows.rs      │ CRIAR — stubs com mesmas assinaturas de linux.rs
devices.rs               │ Envolver TUDO em #[cfg(target_os = "linux")]
                         │ + criar get_all() stub para Windows (retorna vec![])
macros.rs                │ #[cfg] nos bodies de execute() — stub para Windows
popup.rs                 │ #[cfg] no show() — stub para Windows
autostart.rs             │ #[cfg] no is_enabled/enable/disable — stubs
lib.rs launch_app        │ #[cfg] para shell: bash vs cmd.exe
Cargo.toml               │ Adicionar seção [target.'cfg(windows)'.dependencies]
```

### Novas dependências (Cargo.toml)

```toml
[target.'cfg(target_os = "windows")'.dependencies]
winreg = "0.52"          # Windows Registry access
arboard = "3"            # Cross-platform clipboard (substitui xclip)
enigo = "0.2"            # Cross-platform input simulation (substitui xdotool)
```

### Assinaturas que `platform/windows.rs` precisa exportar

Estas são as 5 funções que `remapper.rs` chama. O stub pode retornar `Err`:

```rust
pub fn remap_to_xkb_option(source: &str, target: &str) -> Option<String>
pub fn is_option_active(option: &str) -> Result<bool, String>
pub fn add_option(option: &str) -> Result<bool, String>
pub fn remove_option(option: &str) -> Result<bool, String>
pub fn toggle_option(option: &str) -> Result<bool, String>
```

No Windows o conceito de "XKB option" não existe, então a naming vai mudar no Epic 02 para algo mais genérico. Neste epic os stubs mantêm os mesmos nomes para compilar.
