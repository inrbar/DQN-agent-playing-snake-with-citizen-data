use crate::snake::data_types::*;


/// a basic struct to hold some data from the GameEngine struct
/// exists only for structural reasons
pub struct Snake{
    pub snake_body : Vec<Vec2>,
    pub snake_head : Vec2,
    pub direction : usize,
}

impl Snake {
    
    pub fn new(snake_head : Vec2, direction : usize) -> Self{

        let delta_x = match direction {
            1 => -1,
            3 => 1,
            _ => 0
        };
        let delta_y = match direction {
            0 => -1,
            2 => 1,
            _ => 0
            
        };
        let offset = Vec2{x : -delta_x, y: -delta_y };

        let snake_body = vec![
            snake_head.clone(),
            snake_head.clone() + offset.clone(),
            snake_head.clone() + offset.clone().mul_by_i32(2)
        ];
        
        Snake { snake_body : snake_body, snake_head: snake_head, direction: direction }

    }

}