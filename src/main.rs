// import some things
mod app;
mod cell;
mod scene;
mod utility;

fn main() {
    // create the app
    let app = app::MapColoringApp::default();
    // create random stuff for running the app
    let native_options = eframe::NativeOptions::default();
    // run the app
    eframe::run_native(Box::new(app), native_options);
}
