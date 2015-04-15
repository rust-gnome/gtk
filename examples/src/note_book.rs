extern crate gtk as rgtk;

use rgtk::{Connect, Gtk, IconSize, Orientation, ReliefStyle};
use rgtk::signals::{DeleteEvent, Clicked};
use rgtk::traits::*;
use rgtk::widgets::{BoxBuilder, ButtonBuilder, ImageBuilder, LabelBuilder, NoteBookBuilder,
    WindowBuilder};

struct NoteBook {
    notebook: rgtk::NoteBook,
    tabs: Vec<rgtk::Box>
}

impl NoteBook {
    fn new(gtk: &Gtk) -> NoteBook {
        NoteBook {
            notebook: gtk.notebook().unwrap(),
            tabs: Vec::new()
        }
    }

    fn create_tab<'a, Widget: rgtk::WidgetTrait>(&mut self, gtk: &Gtk, title: &'a str,
            widget: &Widget) -> Option<u32> {
        let close_image = gtk.image_from_icon_name("window-close", IconSize::Button).unwrap();
        let mut button = gtk.button().unwrap();
        let label = gtk.label(title).unwrap();
        let mut tab = gtk._box(Orientation::Horizontal, 0).unwrap();

        button.set_relief(ReliefStyle::None);
        button.set_focus_on_click(false);
        button.add(&close_image);

        tab.pack_start(&label, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        tab.show_all();

        let index = match self.notebook.append_page(widget, Some(&tab)) {
            Some(index) => index,
            _ => return None
        };

        Connect::connect(&button, Clicked::new(&mut || self.notebook.remove_page(index as i32)));

        self.tabs.push(tab);

        Some(index)
    }
}

fn main() {
    let gtk = Gtk::new();

    let mut window = gtk.window(rgtk::WindowType::TopLevel).unwrap();

    window.set_title("Notebook");
    window.set_window_position(rgtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk.main_quit();
        true
    }));

    let mut notebook = NoteBook::new(&gtk);

    for i in 1..4 {
        let title = format!("sheet {}", i);
        let label = gtk.label(&title[..]).unwrap();
        notebook.create_tab(&gtk, &title[..], &label);
    }

    window.add(&notebook.notebook);
    window.show_all();
    gtk.main();
}

