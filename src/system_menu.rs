use muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};

pub fn init_menu() -> Menu {
    let menu = Menu::new();

    let app_menu = Submenu::new("Atelier™", true);

    app_menu
        .append(&PredefinedMenuItem::about(None, None))
        .unwrap();

    app_menu.append(&PredefinedMenuItem::separator()).unwrap();

    app_menu
        .append(&MenuItem::new("Settings...", true, None))
        .unwrap();
    app_menu
        .append(&MenuItem::new("Preferences...", true, None))
        .unwrap();
    app_menu
        .append(&MenuItem::new("Plugins...", true, None))
        .unwrap();

    app_menu.append(&PredefinedMenuItem::separator()).unwrap();

    app_menu
        .append(&MenuItem::new("Check for Updates...", true, None))
        .unwrap();

    app_menu.append(&PredefinedMenuItem::separator()).unwrap();
    app_menu.append(&PredefinedMenuItem::quit(None)).unwrap();

    let file_menu = Submenu::new("File", true);

    let edit_menu = Submenu::new("Edit", true);

    let view_menu = Submenu::new("View", true);

    let library_menu = Submenu::new("Library", true);

    let workspace_menu = Submenu::new("Workspace", true);

    let window_menu = Submenu::new("Window", true);

    window_menu
        .append(&PredefinedMenuItem::minimize(None))
        .unwrap();
    window_menu
        .append(&PredefinedMenuItem::maximize(None))
        .unwrap();

    window_menu
        .append(&PredefinedMenuItem::separator())
        .unwrap();

    window_menu
        .append(&PredefinedMenuItem::bring_all_to_front(None))
        .unwrap();

    window_menu
        .append(&PredefinedMenuItem::separator())
        .unwrap();
    window_menu
        .append(&PredefinedMenuItem::close_window(None))
        .unwrap();

    let help_menu = Submenu::new("Help", true);

    help_menu
        .append(&MenuItem::new("Atelier™ Help", true, None))
        .unwrap();
    help_menu
        .append(&MenuItem::new("Send Feedback", true, None))
        .unwrap();

    menu.append(&app_menu).unwrap();

    menu.append(&file_menu).unwrap();
    menu.append(&edit_menu).unwrap();
    menu.append(&view_menu).unwrap();
    menu.append(&library_menu).unwrap();
    menu.append(&workspace_menu).unwrap();

    menu.append(&window_menu).unwrap();
    menu.append(&help_menu).unwrap();

    #[cfg(target_os = "macos")]
    {
        menu.init_for_nsapp();
    }

    menu
}
