#!/usr/bin/env bash
# Configura o atalho Super+Escape para toggle CapsLock/Escape
# Verifica se já existe antes de criar

SHORTCUT_PATH="/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/capslock-toggle/"
SHORTCUT_NAME="Toggle CapsLock/Escape"
SHORTCUT_CMD="/home/ndr/andre-os-utilitarios/toggle-capslock-esc.sh"
SHORTCUT_BINDING="<Super>Escape"

SCHEMA="org.gnome.settings-daemon.plugins.media-keys.custom-keybinding"
SCHEMA_PATH="org.gnome.settings-daemon.plugins.media-keys"

# Verifica se o atalho já existe
EXISTING=$(dconf read "$SHORTCUT_PATH"command 2>/dev/null)
if [ -n "$EXISTING" ]; then
    echo "Atalho já existe:"
    echo "  Nome:    $(dconf read "${SHORTCUT_PATH}name")"
    echo "  Comando: $EXISTING"
    echo "  Tecla:   $(dconf read "${SHORTCUT_PATH}binding")"
    echo "Nada a fazer."
    exit 0
fi

# Configura o atalho
gsettings set "$SCHEMA:$SHORTCUT_PATH" name "$SHORTCUT_NAME"
gsettings set "$SCHEMA:$SHORTCUT_PATH" command "$SHORTCUT_CMD"
gsettings set "$SCHEMA:$SHORTCUT_PATH" binding "$SHORTCUT_BINDING"

# Registra na lista de custom keybindings (sem duplicar)
CURRENT=$(gsettings get "$SCHEMA_PATH" custom-keybindings)
if echo "$CURRENT" | grep -q "$SHORTCUT_PATH"; then
    echo "Path já registrado na lista."
else
    if [ "$CURRENT" = "@as []" ] || [ "$CURRENT" = "[]" ]; then
        NEW="['$SHORTCUT_PATH']"
    else
        NEW=$(echo "$CURRENT" | sed "s|]|, '$SHORTCUT_PATH']|")
    fi
    gsettings set "$SCHEMA_PATH" custom-keybindings "$NEW"
fi

echo "Atalho criado com sucesso!"
echo "  Super+Escape -> Toggle CapsLock/Escape"
