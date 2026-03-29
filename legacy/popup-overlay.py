#!/usr/bin/env python3
"""Popup overlay minimalista que aparece no centro da tela e esmaece após 2s."""

import sys
import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, Gdk, GLib

DISPLAY_MS = 1500      # tempo visível antes de começar a esmaecer
FADE_MS = 500          # duração do fade out
FADE_STEPS = 20        # frames do fade
BOTTOM_MARGIN = 80     # margem acima da dock

class PopupOverlay(Gtk.Window):
    def __init__(self, text, icon=None):
        super().__init__(type=Gtk.WindowType.POPUP)

        self.set_decorated(False)
        self.set_keep_above(True)
        self.set_skip_taskbar_hint(True)
        self.set_skip_pager_hint(True)
        self.set_can_focus(False)
        self.set_accept_focus(False)
        self.set_app_paintable(True)

        # Transparência
        screen = self.get_screen()
        visual = screen.get_rgba_visual()
        if visual:
            self.set_visual(visual)

        # CSS
        self.css = Gtk.CssProvider()
        self._update_css(1.0)
        css = self.css
        Gtk.StyleContext.add_provider_for_screen(
            screen, css, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )

        # Layout horizontal: ícone à esquerda, texto à direita
        box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
        box.set_name("popup-box")
        box.set_halign(Gtk.Align.CENTER)
        box.set_valign(Gtk.Align.CENTER)

        if icon:
            icon_label = Gtk.Label(label=icon)
            icon_label.set_name("popup-icon")
            box.pack_start(icon_label, False, False, 0)

        label = Gtk.Label(label=text)
        label.set_name("popup-label")
        box.pack_start(label, False, False, 0)

        # Event box para capturar clique
        event_box = Gtk.EventBox()
        event_box.add(box)
        event_box.connect("button-press-event", lambda *_: self.close_now())
        self.add(event_box)

        # Centralizar no monitor onde está o cursor
        self.show_all()
        self.realize()
        display = Gdk.Display.get_default()
        seat = display.get_default_seat()
        pointer = seat.get_pointer()
        _, px, py = pointer.get_position()
        monitor = display.get_monitor_at_point(px, py)
        geom = monitor.get_geometry()
        alloc = self.get_allocation()
        x = geom.x + (geom.width - alloc.width) // 2
        y = geom.y + geom.height - alloc.height - BOTTOM_MARGIN
        self.move(x, y)

        # Timers
        self.opacity_val = 1.0
        GLib.timeout_add(DISPLAY_MS, self.start_fade)

    def start_fade(self):
        interval = FADE_MS // FADE_STEPS
        self.fade_step = 0
        GLib.timeout_add(interval, self.fade_tick)
        return False

    def _update_css(self, opacity):
        bg_alpha = 0.88 * opacity
        text_alpha = opacity
        self.css.load_from_data(f"""
            #popup-box {{
                background-color: rgba(30, 30, 30, {bg_alpha:.3f});
                border-radius: 12px;
                padding: 12px 24px;
            }}
            #popup-icon {{
                font-size: 18px;
                color: rgba(255, 255, 255, {text_alpha:.3f});
            }}
            #popup-label {{
                font-size: 14px;
                font-weight: 600;
                color: rgba(255, 255, 255, {text_alpha:.3f});
                font-family: "Inter", "SF Pro", "Segoe UI", sans-serif;
            }}
        """.encode())

    def fade_tick(self):
        self.fade_step += 1
        self.opacity_val = max(0, 1.0 - self.fade_step / FADE_STEPS)
        self._update_css(self.opacity_val)
        if self.opacity_val <= 0:
            Gtk.main_quit()
            return False
        return True

    def close_now(self):
        Gtk.main_quit()


if __name__ == "__main__":
    text = sys.argv[1] if len(sys.argv) > 1 else "Hello"
    icon = sys.argv[2] if len(sys.argv) > 2 else None
    PopupOverlay(text, icon)
    Gtk.main()
