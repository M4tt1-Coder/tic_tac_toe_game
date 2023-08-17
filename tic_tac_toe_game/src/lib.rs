//using statements
use eframe::{egui::{self, CentralPanel}, App};

//constants


//struct app
#[derive(Default)]
pub struct Game{
    round: i8,
    finished: bool,
    field: Vec<Vec<String>>,
}

impl Game{
    //for settig the default values
    pub fn new() -> Game{
        Game{
            round: 0,
            finished: false,
            //that looks like this:
            //_ _ _ 
            //_ _ _
            //_ _ _
            field: vec![vec![
                " ".to_string(),
                " ".to_string(),
                " ".to_string(),
            ],
            vec![
                " ".to_string(),
                " ".to_string(),
                " ".to_string(),
            ],
            vec![
                " ".to_string(),
                " ".to_string(),
                " ".to_string(),
            ]]
        }
    }
    //check if the game is finished
    fn check(&mut self){

    }

}

//checking functions
fn vertical(game: &mut Game, symbol: &str){
    
}

impl App for Game{
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            //check if game is finished
            //buttons which contain game symbols
            //-> store game symbols in field property
            //-> check if game is finished
            //-> when a plauer already has a symbol in the field in cant be removed
            if self.finished{
                ui.label("Game over!");
                return;
            }

            ui.horizontal(|ui| {
                
                if ui.button(&self.field[0][0]).clicked(){
                    
                }

            });
        });
    }
}


