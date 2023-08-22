use eframe::{egui, Error};
use tic_tac_toe_game::Game;

fn main() -> Result<(), Error>{
    //instance fo the game
    let game = Game::new();
    //name of the application
    let app_name = "TikTakToe";
    //set options
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
      ..eframe::NativeOptions::default()
    };
    //start the game
    eframe::run_native(
        app_name,
        native_options,
        Box::new(|_cc| Box::new(game)))
}