use fltk::{app, prelude::*, window};

fn main() {
    let app = app::App::default();
    // let mut win = window::Window::default()
    //     .with_size(350, 350)
    //     .with_label("Clock");
    let mut wv_win = window::Window::default()
        .with_size(340, 340)
        .center_of_parent();
    // win.end();
    // win.make_resizable(true);
    // win.show();
    wv_win.end();
    wv_win.make_resizable(true);
    wv_win.show();

    let wv = fltk_webview::Webview::create(false, &mut wv_win);
    // wv.navigate("https://sriram23.github.io/css-clock");
    wv.navigate("https://google.com");
    
    app.run().unwrap();
}