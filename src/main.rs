use gtk::prelude::*;
use gtk::{gdk, glib, Label, TargetList, Window, WindowType};
use std::cell::Cell;
use std::env;
use std::path::PathBuf;
use std::process::{self, Command};
use std::rc::Rc;

fn main() {
    // 1. Argument Handling
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Nutzung: cargo run -- /pfad/zur/datei");
        process::exit(1);
    }

    let filepath = PathBuf::from(&args[1]);
    if !filepath.is_file() {
        eprintln!("Datei existiert nicht: {}", filepath.display());
        process::exit(1);
    }

    // 2. Initialize GTK
    if gtk::init().is_err() {
        eprintln!("Fehler bei der Initialisierung von GTK.");
        process::exit(1);
    }

    // 3. Window Setup
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Drag Datei");
    window.set_decorated(false);
    window.set_keep_above(true);
    window.set_resizable(false);
    window.set_app_paintable(true);
    window.set_default_size(100, 40);
    window.set_border_width(8);

    let label = Label::new(None);
    label.set_markup("<span foreground=\"white\" font_weight=\"bold\">📎 Datei ziehen</span>");
    window.add(&label);

    // 4. Drag & Drop Setup
    let targets = TargetList::new(&[]);
    targets.add_uri_targets(0);

    window.drag_source_set(gdk::ModifierType::BUTTON1_MASK, &[], gdk::DragAction::COPY);
    window.drag_source_set_target_list(Some(&targets));

    let drag_started = Rc::new(Cell::new(false));

    // 5. Signals
    let filepath_clone = filepath.clone();
    window.connect_drag_data_get(move |_, _, data, _, _| {
        if let Ok(abs_path) = filepath_clone.canonicalize() {
            // Safely encode the absolute path into a file:// URI
            if let Ok(uri) = glib::filename_to_uri(&abs_path, None) {
                data.set_uris(&[&uri.to_string()]);
            }
        }
    });

    let drag_started_clone = drag_started.clone();
    window.connect_drag_begin(move |win, _| {
        drag_started_clone.set(true);
        win.hide();
    });

    window.connect_drag_end(|_, _| {
        println!("Drag beendet. Schließe Anwendung.");
        gtk::main_quit();
    });

    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    // 6. Move under mouse (Idle task)
    let win_clone = window.clone();
    glib::idle_add_local(move || {
        if let Ok(output) = Command::new("xdotool")
            .args(["getmouselocation", "--shell"])
            .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut x = 0;
                let mut y = 0;

                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() == 2 {
                        match parts[0] {
                            "X" => x = parts[1].parse().unwrap_or(0),
                         "Y" => y = parts[1].parse().unwrap_or(0),
                         _ => {}
                        }
                    }
                }
                // `move_` is used instead of `move` because `move` is a reserved keyword in Rust
                win_clone.move_(x - 50, y - 20);
            } else {
                eprintln!("Fehler bei Mausposition: xdotool fehlgeschlagen/nicht installiert.");
            }

            // Return Break to ensure this only runs once
            glib::ControlFlow::Break
    });

    // 7. Safety Timeout
    let drag_started_timeout = drag_started.clone();
    glib::timeout_add_seconds_local(3, move || {
        if !drag_started_timeout.get() {
            println!("Kein Drag erkannt – beende Prozess (Safety-Timeout).");
            gtk::main_quit();
        }

        // Return Break to end the timeout
        glib::ControlFlow::Break
    });

    // 8. Start
    window.show_all();
    gtk::main();
}
