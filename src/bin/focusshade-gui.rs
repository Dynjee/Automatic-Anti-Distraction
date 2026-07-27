#[path = "../gui.rs"]
mod gui;

use gui::FocusShadeApp;


fn main() {

    let options =
        eframe::NativeOptions::default();


    eframe::run_native(
        "FocusShade",
        options,
        Box::new(|_cc| {
           Ok(Box::new(
                FocusShadeApp::default()
            ))
        }),
    )
    .unwrap();

}
