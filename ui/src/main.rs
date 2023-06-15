use dataEngine::data_engine_mod::DataEngine;
use speedy2d::{Window};
use snake_game_rust::snake::*;
mod ui_core;

use ui_core::{ input_manager::InputManager, game_manager::GameManager, graphics_manager::GraphicManager};

const WINDOW_SIZE: (u32, u32) = (1000, 1000);
const WORLD_SIZE : (usize, usize) = (20, 20);
const GAME_SIZE : (f32, f32) = (900.0, 900.0);
const STARTING_POS : (usize, usize) = (WORLD_SIZE.0/2, WORLD_SIZE.1/2);
const STARTING_DIRECTION : usize = 1;

struct MyWindowHandler {
    input_manager : InputManager
}

impl MyWindowHandler {

    fn new() -> MyWindowHandler{
        
        let ge = GameEngine::new(WORLD_SIZE, STARTING_POS, STARTING_DIRECTION);

        //let game_size_0_i32 = WORLD_SIZE.0 as i32;
        //let game_size_1_i32 = WORLD_SIZE.1 as i32;
        let de = DataEngine::new();//game_size_0_i32*game_size_1_i32);
        
        let graph_m = GraphicManager::new(WINDOW_SIZE, GAME_SIZE);

        let gm = GameManager::new(graph_m, ge, de);

        let im = InputManager::new(gm);

        MyWindowHandler{
            input_manager: im
        }
    }

}



fn main() {
    
    let window = Window::new_centered("Snake",WINDOW_SIZE).unwrap();

    window.run_loop(MyWindowHandler::new());

}