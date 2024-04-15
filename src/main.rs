use std::env;

use slint::ComponentHandle;

use lib::functions_lib::{
    callback_declare_dump_list_items, callback_declare_load_list_items,
    callback_declare_pop_list_item, callback_declare_put_list_item,
};
use lib::{AppConfig, AppWindow};

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    let mut args = env::args();
    args.next();

    // Allow calling from the command line to
    // load a specified TODO file.
    let cfg = app.global::<AppConfig>();
    let pth = args
        .next()
        .unwrap_or_else(|| cfg.get_data_path().into())
        .into();
    cfg.set_data_path(pth);

    callback_declare_dump_list_items(&app);
    callback_declare_load_list_items(&app);
    callback_declare_put_list_item(&app);
    callback_declare_pop_list_item(&app);

    app.run()
}

/* REFERENCES:
// GENERATE PROYECT COMMAND:
cargo generate --git ~/Descargas/slint-rust-template --name rust-slint-todolist-demo

https://github.com/WilkinsonK/todo_app
https://github.com/tyrsday29/rust_todo_app_slint
*/
