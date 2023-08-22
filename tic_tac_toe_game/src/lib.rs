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
    //first line
    //layout:
    //X X X
    //_ _ _ 
    //_ _ _
    if game.field[0][0] == symbol && game.field[0][1] == symbol && game.field[0][2] == symbol{
        if symbol == "X"{
            game.field[0][0] = "❌".to_string();
            game.field[0][1] = "❌".to_string();
            game.field[0][2] = "❌".to_string();
        }else{
            game.field[0][0] = "⭕".to_string();
            game.field[0][1] = "⭕".to_string();
            game.field[0][2] = "⭕".to_string();
        }
    }

    //second line 
    //layout:
    //_ _ _
    //X X X
    //_ _ _
    if game.field[1][0] == symbol && game.field[1][1] == symbol && game.field[1][2] == symbol{
        if symbol == "X"{
            game.field[1][0] = "❌".to_string();
            game.field[1][1] = "❌".to_string();
            game.field[1][2] = "❌".to_string();
        }else{
            game.field[1][0] = "⭕".to_string();
            game.field[1][1] = "⭕".to_string();
            game.field[1][2] = "⭕".to_string();
        }
    }

    //third line
    //layout:
    //_ _ _
    //_ _ _
    //X X X 
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

            //first row of the game fields
            ui.horizontal_centered(|ui| {
                //button 1
                if ui.button(&self.field[0][0]).clicked(){
                    if &self.round % 2 == 0 {
                        self.field[0][0] = "O".to_string();
                    }else{
                        self.field[0][0] = "X".to_string();
                    }
                }
                //button 2
                if ui.button(&self.field[0][1]).clicked(){
                    if &self.round % 2 == 0 {
                        self.field[0][1] = "O".to_string();
                    }else{
                        self.field[0][1] = "X".to_string();
                    }
                }
                //button 3
                if ui.button(&self.field[0][2]).clicked(){
                    if &self.round % 2 == 0{
                        self.field[0][2] = "O".to_string();
                    }else{
                        self.field[0][2] = "X".to_string();
                    }
                }
            });

            //row 2 
            ui.horizontal_centered(|ui|{
                //button 1
                if ui.button(&self.field[0][0]).clicked(){
                    if &self.round % 2 == 0 {
                        self.field[0][0] = "O".to_string();
                    }else{
                        self.field[0][0] = "X".to_string();
                    }
                }
                //button 2
                if ui.button(&self.field[0][1]).clicked(){
                    if &self.round % 2 == 0 {
                        self.field[0][1] = "O".to_string();
                    }else{
                        self.field[0][1] = "X".to_string();
                    }
                }
                //button 3
                if ui.button(&self.field[0][2]).clicked(){
                    if &self.round % 2 == 0{
                        self.field[0][2] = "O".to_string();
                    }else{
                        self.field[0][2] = "X".to_string();
                    }
                }
            });

            //row 3 
            ui.horizontal_centered(|ui|{
                //button 1
                if ui.button(&self.field[0][0]).clicked(){
                    if &self.round % 2 == 0 {
                        self.field[0][0] = "O".to_string();
                    }else{
                        self.field[0][0] = "X".to_string();
                    }
                }
                //button 2
                if ui.button(&self.field[0][1]).clicked(){
                    if &self.round % 2 == 0 {
                        self.field[0][1] = "O".to_string();
                    }else{
                        self.field[0][1] = "X".to_string();
                    }
                }
                //button 3
                if ui.button(&self.field[0][2]).clicked(){
                    if &self.round % 2 == 0{
                        self.field[0][2] = "O".to_string();
                    }else{
                        self.field[0][2] = "X".to_string();
                    }
                }
            });

        });
    }
}


