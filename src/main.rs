extern crate gtk;
extern crate gio;

use gtk::{
    WidgetExt, WindowExt, ContainerExt,
    TextViewExt, TextBufferExt, GtkApplicationExt,
    DialogExt, FileChooserExt, BinExt, Cast
};
use gio::{
    ApplicationExt, SimpleActionExt, ActionMapExt,
    MenuExt, FileExt
};

fn main() {
    match gtk::Application::new("com.github.nixiesquid.note_bit", gio::APPLICATION_HANDLES_OPEN) {
        Ok(app) => {
            app.connect_activate(|app| {
                let new_window_action = gio::SimpleAction::new("new_window", None);
                {
                    let app = app.clone();
                    new_window_action.connect_activate(move |_, _| {
                        create_window(&app);
                    });
                }

                let open_action = gio::SimpleAction::new("open", None);
                {
                    let app = app.clone();
                    open_action.connect_activate(move |_, _| {
                        if let Some(file) = run_file_chooser_dialog() {
                            open(file, app.clone());
                        }
                    });
                }

                let quit_action = gio::SimpleAction::new("quit", None);
                {
                    let app = app.clone();
                    new_window_action.connect_activate(move |_, _| {
                        app.quit();
                    });
                }

                app.add_action(&new_window_action);
                app.add_action(&open_action);
                app.add_action(&quit_action);

                {
                    use gio::{ Menu, MenuItem };
                    let menubar = Menu::new();

                    // file menu
                    let submenu_file = Menu::new();
                    let create_new_window = MenuItem::new("New Window", "app.new_window");
                    let open = MenuItem::new("Open", "app.open_action");
                    let quit = MenuItem::new("Quit", "app.quit");
                    submenu_file.append_item(&create_new_window);
                    submenu_file.append_item(&open);
                    submenu_file.append_item(&quit);

                    // edit menu
                    let submenu_edit = Menu::new();
                    let copy = MenuItem::new("Copy", "win.copy");
                    let paste = MenuItem::new("Paste", "win.paste");
                    submenu_edit.append_item(&copy);
                    submenu_edit.append_item(&paste);

                    menubar.append_submenu("File", &submenu_file);
                    menubar.append_submenu("Edit", &submenu_edit);

                    app.set_menubar(&menubar);
                }
                create_window(&app);
            });
            app.run(&[""]);
        },
        Err(_) => {
            println!("Application start up error");
        }
    };
}

fn create_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let win = gtk::ApplicationWindow::new(&app);
    win.set_default_size(800, 600);
    win.set_title("NoteBit");

    let scr_win = gtk::ScrolledWindow::new(None, None);
    let txt_view = gtk::TextView::new();
    scr_win.add(&txt_view);
    win.add(&scr_win);

    let copy_action = gio::SimpleAction::new("copy", None);
    {
        let txt_view = txt_view.clone();
        copy_action.connect_activate(move |_, _| {
            let clipboard = txt_view.get_clipboard(&gdk::SELECTION_CLIPBOARD);
            txt_view.get_buffer().unwrap().copy_clipboard(&clipboard);
        });
    }

    let paste_action = gio::SimpleAction::new("paste", None);
    {
        let txt_view = txt_view.clone();
        paste_action.connect_activate(move |_, _| {
            let clipboard = txt_view.get_clipboard(&gdk::SELECTION_CLIPBOARD);
            let buf = txt_view.get_buffer().unwrap();
            buf.paste_clipboard(&clipboard, None, txt_view.get_editable());
        });
    }
    win.add_action(&copy_action);
    win.add_action(&paste_action);
    win.show_all();
    win
}

fn run_file_chooser_dialog() -> Option<gio::File> {
    let dialog = gtk::FileChooserDialog::new::<gtk::Window>
                    (
                        Some("Open File"),
                        None,
                        gtk::FileChooserAction::Open
                    );
    dialog.add_button("Calncel", gtk::ResponseType::Cancel.into());
    dialog.add_button("Open", gtk::ResponseType::Accept.into());

    let file;
    if dialog.run() == gtk::ResponseType::Accept.into() {
        if let Some(path) = dialog.get_filename() {
            file = Some(gio::File::new_for_path(path.as_path()))
        } else {
            file = None
        }
    } else {
        file = None
    }

    dialog.destroy();
}
