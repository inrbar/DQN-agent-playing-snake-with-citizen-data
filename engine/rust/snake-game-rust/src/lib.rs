//! A simple implementaion of the snake game in rust
//!
//! For now all functionality is kept in the snake module,
//! with only utility structs placed in the `data_types` module.
//! The additional snake file is only there because the main struct
//! was getting bloated
//! 
//!
//! # Installation
//! 
//! For rust projects simply add the following line to the cargo.toml file 
//! in the dependencies section
//! ```
//! snake-game-rust = {path = "../snake-game-rust"}
//! ```
//! this path is taken as an example and it should be changed depending on the 
//! relative location of the projects
//! 
//! For python projects refer to the [`python_wrapper`] package
//! 
//! # Basic usage
//! ```
//! use snake_game_rust::snake::*;
//! 
//! //initialize the game engine with given size, snake starting position
//! //and direction
//! let mut game = GameEngine::new((10,10), (5,5), 1);
//! 
//! ```
//! 
//! currently directions are implemented as follows
//! 
//! * 0 - left
//! * 1 - up        
//! * 2 - right
//! * 3 - down
//! 
//! The game engine is initialized with a random seed.
//! To set the seed call reset after initializing and 
//! provide the seed.
//! 
//! Then call step with either 0,1 or 2 as the action to forward 
//! the game by one iteration.
//! ```
//! game.step(1);
//! ```
//! The actions are mapped as following:
//! * 0 - turn left
//! * 1 - do not turn
//! * 2 - turn right
//! 
//! Note that step also returns information about the step,
//! so in some cases it would be recommended to assign the output
//! to some values.
//!
//! Use the reset method to reset the game to the starting state
//! based on the provided seed
//! ```
//! game.reset(seed);
//! ```
//! 
//! To view the game state call get_world
//! ```
//! game.get_world();
//! ```
//! 
//! This return the current world Matrix
//! 
//! Additional get method are present for food position and head position


///snake module
pub mod snake{
    pub mod data_types;
    pub mod snake;

    use fastrand::{self};
    use data_types::*;
    use snake::Snake;

    


    pub struct GameEngine{
        pub game_world : Matrix,
        pub world_size : (usize,usize),
        pub rng_generator : fastrand::Rng,
        pub seed : u64,
        pub snake : Snake,
        pub starting_direction : usize,
        pub food_pos : Vec2,
        pub starting_pos : (usize,usize),
        pub free_space : Vec<Vec2>
        /*
                    1-up
            0-left       2-right
                    3-down
         */

    }

    impl GameEngine {


        /// returns a new instance of gameEngine struct based on
        /// the size of the world grid and the starting position of the snake
        /// 
        /// # Panics
        /// 
        /// Will panic if one of the world dimensions is 0 or if the snake would be spawned
        /// outside of the world grid.
        /// 
        /// # Examples
        /// 
        /// ```
        /// use snake_game_rust::snake::*;
        /// 
        /// let size : (usize,usize) = (10,10);
        /// let starting_pos : (usize,usize) = (5,5); 
        /// let direction : usize = 1;
        /// //creates the engine with world size 10x10 and the snake
        /// //head spawned in cell (5,5) facing up
        /// let mut ge = GameEngine::new(size, starting_pos, direction);
        /// ```

        pub fn new(
            world_size : (usize,usize), 
            starting_pos : (usize,usize),
            starting_direction : usize,
        ) -> GameEngine{

            if world_size.0 == 0 || world_size.1 == 0{
                panic!("World size cannot be zero!")
            };

            if starting_direction > 3 {
                panic!("Direction has to be either
                    1 - up
                    2 - down
                    0 - left 
                    3 - right
                    and not {starting_direction}
                ")
            }

            let x  = starting_pos.0 as i32;
            let y  = starting_pos.1 as i32;

            let snake_head= Vec2{x : x, y : y};

            let snake  = Snake::new(snake_head, starting_direction);

            let mut free_space : Vec<Vec2> = Vec::with_capacity(world_size.0*world_size.1);

            for x in 0..world_size.0{
                for y in 0..world_size.1{
                    let pair_of_coordinates = Vec2{ x : x.try_into().unwrap(), y : y.try_into().unwrap() };
                    if !&snake.snake_body.contains(&pair_of_coordinates){
                        free_space.push(pair_of_coordinates);
                    }
                }
            }

            let mut ge = GameEngine { 
                game_world: Matrix::zeros(world_size), 
                world_size : world_size,
                rng_generator : fastrand::Rng::new(),
                seed : 0,
                snake: snake,
                starting_direction : starting_direction,
                food_pos : Vec2 { x: 0, y: 0 },
                starting_pos : starting_pos,
                free_space : free_space
            };

            ge.draw_world();
            ge.spawn_food();
            
            return ge;

        }

        /// Forwards the game by one iteration and returns infomation about it
        /// 
        /// Input action should be 0,1 or 2 : 
        /// * 0 - turn left
        /// * 1 - go forward
        /// * 2 - turn right
        /// 
        /// Returns a tuple `(done, food_eaten, msg)`
        /// * `done` - a boolean, true if the game has ended in some way
        /// * `food_eaten` - a boolean, true if food has been eaten this iteration
        /// * `msg` - a string, a short message describing the iteration, mostly 
        /// used for learning enviroment 
        /// 
        /// For now `msg` can take on 4 values:
        /// 
        /// * `"alive"` - if snake is alive, 
        /// * `"body"` - if the snake has collided with its own body
        /// * `"wall"` - if the snake has collided with on of the walls
        /// * `"victory"` - if the snake body is taking up the entire world grid
        /// 
        /// # Panics
        /// 
        /// Will panic if supplied with an action that is not mapped
        /// 
        /// # Examples
        /// 
        /// ```
        /// use snake_game_rust::snake::*;
        /// 
        /// let mut game = GameEngine::new((10,10), (5,5), 1);
        /// 
        /// (done, food_eaten, msg) = game.step(1);
        /// 
        /// if done {
        ///     //some code to run after the game ends
        /// }
        /// ``` 
        pub fn step(&mut self, action : usize) -> (bool,bool,String){

            
            self.move_snake(action);
            let (done,msg) = self.game_over();
            
            let food_eaten = if !done{
                self.snake_updates()
            } else {
                false
            };

            if self.game_world.matrix.iter().map(|x| x.iter().any(|x| *x==3)).any(|x| x) == false {
                self.spawn_food();
                println!("This should not happen");
            }
            (done, food_eaten, msg)

        }

        fn move_snake(&mut self, action : usize){

            if action > 2 {
                panic!("Expected action to be 0,1 or 2, got {action} instead");
            }

            self.snake.direction = (self.snake.direction + (action + 3)%4)%4;

            let delta_x = match self.snake.direction {
                1 => -1,
                3 => 1,
                _ => 0
            };
            let delta_y = match self.snake.direction {
                0 => -1,
                2 => 1,
                _ => 0
                
            };

            self.snake.snake_head += Vec2{x : delta_x, y : delta_y};

            self.snake.snake_body.insert(0,self.snake.snake_head.clone());


        }

        fn snake_updates(&mut self) -> bool{
            
            let snake_head_pos_x = self.snake.snake_head.x as usize;
            let snake_head_pos_y = self.snake.snake_head.y as usize;

            self.free_space.retain(|cell| *cell != self.snake.snake_head);
            

            let snake_neck = &self.snake.snake_body[1];
            self.game_world[(snake_neck.x, snake_neck.y)] = 2;
            if self.snake.snake_head  == self.food_pos{
            
                self.spawn_food();
                self.game_world[(snake_head_pos_x,snake_head_pos_y)] = 1;

                return true;
            }   
            
            let snake_tail = self.snake.snake_body.last().unwrap();
            
            self.game_world[(snake_tail.x,snake_tail.y)] = 0;
            self.free_space.push(snake_tail.clone());           
            self.game_world[(snake_head_pos_x,snake_head_pos_y)] = 1;
            self.snake.snake_body.pop();

            return false;
        }

        fn spawn_food(&mut self){

            //potential performance issues when snake length is comparable to world size
            //better change to chosing a random indecies from a list of empty tiles
            // time complexity changes to O(1) memory to O(n)
            //only need tor update a maximum of twice per step
            let upper_bound = self.free_space.len();
            let new_food_index = self.rng_generator.usize(0..upper_bound);
            let new_food_pos = self.free_space[new_food_index];

            self.food_pos = new_food_pos;
            self.game_world[(self.food_pos.x,self.food_pos.y)] = 3;
        }


        fn game_over(&self) -> (bool,String){

            let on_self_collision = "body".to_string();
            let on_wall_collision = "wall".to_string();
            let on_game_completed = "victory".to_string();
            let stil_playing = "alive".to_string();

            for i in 1..(self.snake.snake_body.len()-1){
                if self.snake.snake_body[i] == self.snake.snake_head{
                    return (true,on_self_collision);
                }
            }
            if self.snake.snake_head.x < 0 || self.snake.snake_head.x >= self.world_size.0.try_into().unwrap() {
                return (true,on_wall_collision);
            }
            if self.snake.snake_head.y < 0 || self.snake.snake_head.y >= self.world_size.1.try_into().unwrap() {
                return (true,on_wall_collision);
            }
            if self.snake.snake_body.len() == self.world_size.0*self.world_size.1{
                return (true,on_game_completed);
            }
            return (false,stil_playing);
            
        }
        
        fn draw_world(&mut self){


            for i in 0..self.snake.snake_body.len(){

                let color = match i {
                    0 => 1,
                    _ => 2
                };
                let snake_body_part = self.snake.snake_body[i];
                self.game_world[(snake_body_part.x, snake_body_part.y)] = color;
            }
        }

        ///resets the world state given a seed 
        /// 
        /// # Panics
        /// 
        /// Should only panic if the type of the input seed is incorrect
        /// 
        /// # Examples
        /// 
        /// ```
        /// use snake_game_rust::snake::*;
        /// 
        /// let mut game = GameEngine::new((10,10), (5,5), 1);
        /// 
        /// // some code that advances the game
        /// 
        /// let seed : u64 = 12;
        /// game.reset(seed);
        /// 
        /// ``` 
        pub fn reset(&mut self, seed : u64){

            self.seed = seed;

            self.rng_generator.seed(seed);
            
            let x  = self.starting_pos.0 as i32;
            let y  = self.starting_pos.1 as i32;
            
            let snake_head = Vec2{x : x, y : y};

            self.snake = Snake::new(snake_head, self.starting_direction);
            let mut free_space : Vec<Vec2> = Vec::with_capacity(self.world_size.0*self.world_size.1);

            for x in 0..self.world_size.0{
                for y in 0..self.world_size.1{
                    let pair_of_coordinates = Vec2{ x : x.try_into().unwrap(), y : y.try_into().unwrap() };
                    if !&self.snake.snake_body.contains(&pair_of_coordinates){
                        free_space.push(pair_of_coordinates);
                    }
                }
            }
            self.free_space = free_space;
            self.game_world = Matrix::zeros(self.world_size);
            let mut free_space : Vec<Vec2> = Vec::with_capacity(self.world_size.0*self.world_size.1);

            for x in 0..self.world_size.0{
                for y in 0..self.world_size.1{
                    let pair_of_coordinates = Vec2{ x : x.try_into().unwrap(), y : y.try_into().unwrap() };
                    if !&self.snake.snake_body.contains(&pair_of_coordinates){
                        free_space.push(pair_of_coordinates);
                    }
                }
            }
            self.draw_world();
            self.spawn_food();
            
        }

        ///generic getter function for the game_world matrix
        pub fn get_world(&self) -> Matrix{
            self.game_world.clone()
        }

        ///generic getter function for the position of the snake head
        pub fn get_snake_head(&self) -> &Vec2{
            &self.snake.snake_head
        }

        ///gerneric getter function for the position if the food
        pub fn get_food_pos(&self) -> &Vec2{
            &self.food_pos
        }
        
        pub fn get_score(&self) -> usize{
            let sum : i32 = self.game_world.matrix.iter().map(|x| x.iter().sum::<i32>()).sum();
            let score = (sum - 3 + 1)/2 - 3;
            score as usize
        }

        pub fn get_direction(&self) -> usize{
            self.snake.direction
        }

        pub fn get_seed(&self) -> u64{
            self.seed
        }

        pub fn get_info_string(&self) -> String{
            let direction = self.get_direction();
            let seed = self.get_seed();
            let score = self.get_score();

            format!(",{direction},{seed},{score}")
        }

        pub fn get_flattened_world(&self) -> String
        {
            let flattened = self.get_world().matrix.into_iter().flat_map(|s| s).collect::<Vec<i32>>();
            let flattened_string_vec = flattened.into_iter().map(|i| i.to_string() + ", ").collect::<Vec<String>>();
            let mut flattened_string = "".to_string();
            for part in flattened_string_vec.iter() {
                flattened_string += part;
            }
            flattened_string
        }

    }


}


#[cfg(test)]
mod tests {
    //use super::*;

    /*
    added later
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
     */
   
}
