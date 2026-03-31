# Epic 02 — Windows Keyboard Engine

> **Goal:** Key remapping e text macros **funcionam** no Windows. O core do app entrega valor real.

## Rationale

Este é o epic mais complexo e o que entrega mais valor. Sem ele, o app no Windows é uma janela bonita que não faz nada. Ao final deste epic, um usuário Windows pode:

1. Abrir o DeskForge
2. Togglear CapsLock → Escape com um clique
3. Criar e usar text macros via atalhos de teclado

## Technical Context

### Key Remapping no Windows — Duas abordagens

| Abordagem | Prós | Contras |
|-----------|------|---------|
| **Registry Scancode Map** | Persiste reboot, kernel-level, zero CPU | Exige reboot/logoff para aplicar, precisa de admin |
| **Low-level keyboard hook** (`SetWindowsHookEx`) | Instantâneo, toggle sem reboot, user-level | Usa CPU (minimal), pode ser bloqueado por antivírus |

**Decisão:** Implementar **ambas**, com keyboard hook como default (toggle instantâneo como no Linux) e Registry Scancode Map como opção "persist across reboot" (equivalente ao nosso `persist_fnmode`).

### Text Macros — Migração de ferramentas

| Linux | Windows (crate) | Função |
|-------|-----------------|--------|
| `xclip` | `arboard` | Ler/escrever clipboard |
| `xdotool key ctrl+v` | `enigo` | Simular Ctrl+V |
| `xdotool type --delay 12` | `enigo` | Simular digitação |

## Acceptance Criteria

- [ ] CapsLock → Escape toggle funciona instantaneamente (sem reboot)
- [ ] Toggle persiste enquanto o app estiver rodando
- [ ] UI mostra status correto do remap (ativo/inativo)
- [ ] Tray menu toggle funciona
- [ ] Global shortcut `Super+Escape` funciona
- [ ] Text macros: método "clipboard" funciona (cola texto via Ctrl+V)
- [ ] Text macros: método "type" funciona (simula digitação)
- [ ] Macros funcionam em apps normais (Notepad, Chrome, VS Code)
- [ ] Documentar limitação: macros não funcionam em apps elevados (admin)

## Dependencies

- Epic 01 completo (app compila no Windows)

## Key Risks

| Risco | Mitigação |
|-------|-----------|
| Keyboard hook interceptado por antivírus (Windows Defender, etc) | Assinar o binário; documentar whitelist; hook é user-level, não kernel |
| `enigo` não funciona em alguns apps | Testar em Notepad, Chrome, VS Code, Discord; documentar limitações |
| `arboard` falha com clipboard vazio | Tratar `Err` no save_clipboard gracefully |
