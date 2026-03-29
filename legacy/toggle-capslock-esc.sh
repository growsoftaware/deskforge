#!/usr/bin/env bash
# Toggle CapsLock entre Escape e comportamento normal (CapsLock)

CURRENT=$(gsettings get org.gnome.desktop.input-sources xkb-options)

if echo "$CURRENT" | grep -q "caps:escape"; then
    # Remove caps:escape, mantendo outras opcoes
    NEW=$(echo "$CURRENT" | sed "s/'caps:escape'//g" | sed "s/, ,/,/g" | sed "s/\[, /[/g" | sed "s/, \]/]/g")
    gsettings set org.gnome.desktop.input-sources xkb-options "$NEW"
    echo "CapsLock = CapsLock (normal)"
    python3 "$(dirname "$0")/popup-overlay.py" "CapsLock" "⇪" &
else
    # Adiciona caps:escape junto com as opcoes existentes
    if [ "$CURRENT" = "@as []" ] || [ "$CURRENT" = "[]" ]; then
        NEW="['caps:escape']"
    else
        NEW=$(echo "$CURRENT" | sed "s/\]/, 'caps:escape']/g")
    fi
    gsettings set org.gnome.desktop.input-sources xkb-options "$NEW"
    echo "CapsLock = Escape"
    python3 "$(dirname "$0")/popup-overlay.py" "Escape" "⎋" &
fi
