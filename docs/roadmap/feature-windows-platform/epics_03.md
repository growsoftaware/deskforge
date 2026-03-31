# Epic 03 — System Integration (Windows)

> **Goal:** O app se comporta como um cidadão de primeira classe no Windows — autostart, notificações, launcher, tray polido.

## Rationale

Features de "system integration" são o que fazem um app desktop parecer profissional vs. um protótipo. Sem autostart, o usuário precisa abrir manualmente depois de cada reboot. Sem popup, não tem feedback visual. Sem launcher funcional, metade da UI não faz nada.

## Acceptance Criteria

- [ ] Autostart toggle funciona via Registry (`HKCU\...\Run`)
- [ ] Popup de notificação aparece e some com fade (Tauri webview window)
- [ ] App launcher executa comandos via `cmd.exe` e `powershell`
- [ ] App launcher abre terminal (Windows Terminal ou cmd.exe)
- [ ] Tray icon mostra status correto do CapsLock remap
- [ ] App minimiza para tray no close (não fecha)
- [ ] Apps page mostra apenas apps relevantes para a plataforma atual

## Dependencies

- Epic 01 completo (compilação)
- Epic 02 completo (remaps e macros funcionam — necessário para tray status)
