extern crate gtk;
extern crate gio;

use gtk::{ WidgetExt, WindowExt, ContainerExt };
use gio::{ ApplicationExt };

fn main() {
    match gtk::Application::new("com.github.nixiesquid.note_bit", gio::APPLICATION_HANDLES_OPEN) {
        Ok(app) => {
            app.connect_activate(|app| {
                let win = gtk::ApplicationWindow::new(&app);
                win.set_default_size(800, 600);
                win.set_title("NoteBit");

                let scr_win = gtk::ScrolledWindow::new(None, None);
                let txt_view = gtk::TextView::new();
                scr_win.add(&txt_view);
                win.add(&scr_win);
                win.show_all();
            });

            app.run(&[""]);
        },
        Err(_) => {
            println!("Application start up error");
        }
    };
}
