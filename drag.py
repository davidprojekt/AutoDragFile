import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, Gdk, GLib

import subprocess
import sys
import os

class AutoDragFile(Gtk.Window):
    def __init__(self, filepath):
        super().__init__(title="Drag Datei")
        self.set_decorated(False)
        self.set_keep_above(True)
        self.set_resizable(False)
        self.set_app_paintable(True)
        self.set_default_size(100, 40)
        self.set_border_width(8)

        self.filepath = filepath
        self.label = Gtk.Label()
        self.label.set_markup('<span foreground="white" font_weight="bold">📎 Datei ziehen</span>')
        self.add(self.label)

        # Targets für Datei (URI)
        targets = Gtk.TargetList.new([])
        targets.add_uri_targets(0)

        self.drag_source_set(Gdk.ModifierType.BUTTON1_MASK, [], Gdk.DragAction.COPY)
        self.drag_source_set_target_list(targets)
        
        # Signals
        self.connect("drag-data-get", self.on_drag_data_get)
        self.connect("drag-begin", self.on_drag_begin)
        self.connect("drag-end", self.on_drag_end) 

        # Direkt unter Maus zeigen
        GLib.idle_add(self.move_to_mouse)

        self.drag_started = False
        
        # Safety-Timeout: Beendet das Skript, falls der Nutzer nie mit dem Drag beginnt
        GLib.timeout_add_seconds(3, self.force_quit_if_idle)

    def move_to_mouse(self):
        try:
            output = subprocess.check_output(["xdotool", "getmouselocation", "--shell"]).decode()
            loc = dict(line.strip().split('=') for line in output.strip().split('\n'))
            x, y = int(loc['X']), int(loc['Y'])
            self.move(x - 50, y - 20)
        except Exception as e:
            print("Fehler bei Mausposition:", e)
        return False

    def on_drag_data_get(self, widget, drag_context, data, info, time):
        uri = f"file://{os.path.abspath(self.filepath)}"
        data.set_uris([uri])

    def on_drag_begin(self, widget, context):
        self.drag_started = True
        self.hide()

    def on_drag_end(self, widget, context):
        # Wird ausgelöst, sobald die Maus losgelassen wird (Drop oder Abbruch)
        print("Drag beendet. Schließe Anwendung.")
        Gtk.main_quit()

    def force_quit_if_idle(self):
        if not self.drag_started:
            print("Kein Drag erkannt – beende Prozess (Safety-Timeout).")
            Gtk.main_quit()
        return False


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Nutzung: python3 drag.py /pfad/zur/datei")
        sys.exit(1)

    filepath = sys.argv[1]
    if not os.path.isfile(filepath):
        print("Datei existiert nicht:", filepath)
        sys.exit(1)

    win = AutoDragFile(filepath)
    win.connect("destroy", Gtk.main_quit)
    win.show_all()
    Gtk.main()
    
