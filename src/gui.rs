use glib::clone;
use gtk::prelude::*;

pub struct Notebook {
    pub notebook: gtk::Notebook,
    tabs: Vec<gtk::Box>,
}

impl Notebook {
    pub fn new() -> Notebook {
        Notebook {
            notebook: gtk::Notebook::new(),
            tabs: Vec::new(),
        }
    }

    pub fn create_tab(&mut self, title: &str, widget: gtk::Widget) -> u32 {
        let label = gtk::Label::new(Some(title));
        let tab = gtk::Box::new(gtk::Orientation::Horizontal, 0);


        tab.pack_start(&label, false, false, 0);
        tab.show_all();

        let index = self.notebook.append_page(&widget, Some(&tab));

        self.tabs.push(tab);
        index
    }
}