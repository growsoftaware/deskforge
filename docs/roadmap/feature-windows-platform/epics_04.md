# Epic 04 — Polish, CI/CD & Installer

> **Goal:** O app é distribuível para usuários Windows. Build automatizado, installer profissional, zero rough edges.

## Rationale

Um app que funciona no dev machine mas não tem installer nem CI/CD não é um produto — é um projeto. Este epic transforma o DeskForge num app que um usuário pode baixar, instalar em 2 cliques, e usar.

## Acceptance Criteria

- [ ] GitHub Actions workflow builda para Linux (.deb, .AppImage) e Windows (.msi, .exe)
- [ ] Build é triggered em push para `main` e PRs
- [ ] Artifacts são anexados automaticamente a GitHub Releases
- [ ] `.msi` installer funciona no Windows 10/11 limpo
- [ ] App aparece no "Adicionar ou Remover Programas" do Windows
- [ ] Uninstall limpa autostart Registry key
- [ ] `tauri.conf.json` tem ícone Windows (.ico) correto
- [ ] README.md tem instruções de instalação para ambas plataformas
- [ ] CHANGELOG.md documenta v0.2.0
- [ ] Tag `v0.2.0` criada

## Dependencies

- Epics 01, 02, 03 completos
