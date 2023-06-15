use snake_game_rust::snake::GameEngine;
use speedy2d::Graphics2D;
use dataEngine::data_engine_mod::DataEngine;
use super::{graphics_manager::GraphicManager};


#[derive (PartialEq)]
pub enum UIState {
    StartScreen,
    InGame,
    GameOver
}

pub struct GameManager{
    pub state : UIState,
    graphics_manager : GraphicManager,
    ge : GameEngine,
    de : DataEngine,
    score : usize
}

// TODO : add options
impl GameManager{

    pub fn new(graphics_manager : GraphicManager, ge : GameEngine, de : DataEngine) -> GameManager{
        GameManager { 
            state: UIState::StartScreen, 
            graphics_manager: graphics_manager,
            ge: ge, 
            de: de,
            score : 0
        }
    }


    pub fn call_draw(&self, graphics: &mut Graphics2D){

        self.graphics_manager.clear_screen(graphics);
        match self.state {
            UIState::StartScreen => {
                self.graphics_manager.start_screen(graphics)
            }
            UIState::InGame => {
                let board = self.ge.get_world();
                self.graphics_manager.draw_game(board, graphics);
            }
            UIState::GameOver => {
                self.graphics_manager.game_over_screen(self.score, graphics);
            }
            
        }
    }

    pub fn user_action(&mut self, user_action : usize, graphics: &mut Graphics2D) {
       
        let mut is_game_done = false;
        if self.state == UIState::InGame{
            let (game_done,food_eaten,_) = self.ge.step(user_action);
            is_game_done = game_done;
            if food_eaten{
                self.score += 1;
            }
            let mut world_str = self.ge.get_flattened_world();
            world_str += &format!("{user_action}");
            world_str += &self.ge.get_info_string();
            self.de.create_statement(world_str);
        } 
        if is_game_done && self.state != UIState::GameOver{
            self.state = UIState::GameOver;

            match self.de.send_run(){
                Err(err) => {dbg!("Failed to save game data {}", err);},
                _ => ()
            }
        }
            
        self.call_draw(graphics);
        
    }

    pub fn reset_game(&mut self) {
        if self.state == UIState::GameOver{
            

            if let Err(err) = self.de.get_seed()
            {
                println!("{}", err);
            }
            self.ge.reset(self.de.seed);
            self.score = 0;
            self.state = UIState::InGame;
    
        }
    }

    pub fn start_game(&mut self){
        if self.state != UIState::GameOver{
            self.state = UIState::InGame;

            if let Err(err) = self.de.get_seed()
            {
                println!("{}", err);
            }

            self.ge.reset(self.de.seed);
        } 
    }

}