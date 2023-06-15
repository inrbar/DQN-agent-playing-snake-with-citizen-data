import numpy as np
import gym as gym
from gymnasium import spaces

from typing import Tuple,Optional
import python_wrapper


class SnakeEnvTurn(gym.Env):

    def __init__(self,size : Tuple[int,int], starting_pos : Tuple[int,int], starting_direction : int):
        
        self.engine = python_wrapper.EngineWrapper(size,starting_pos,starting_direction)
        self.action_space = 3

        self.direction = starting_direction
        
        self.food_pos = self.get_food_pos()
        self.head_pos = self.get_head_pos()
        self.last_head_pos = self.head_pos

        self.state_space = 12

    def get_food_pos(self):

        return np.array(self.engine.py_get_food_pos())
    
    def get_head_pos(self):

        return np.array(self.engine.py_get_snake_head())
    
    def get_info(self,reward,food):

        dist = np.linalg.norm(self.food_pos - self.head_pos)


        return {"dist" : dist,
                "reward" : reward,
                "steps without food" : self.steps_without_food,
                "food_eaten" : food}

    def get_obs_shape(self):
        obs = self.get_obs()
        
        return obs.shape

    def get_obs(self):

        world = np.array(self.engine.py_get_world())

        x,y = world.shape

        i,j = self.head_pos[0], self.head_pos[1]
        
        if j + 1 < y and (i >=0 and i < x): 
            obstacle_right = world[i,j+1] == 2 and (self.direction != 0)
        elif j + 1 < y:
            obstacle_right = False
        else:
            obstacle_right = True
 
        if i + 1 < x and (j >=0 and j < y):
            obstacle_up = world[i+1,j] == 2 and (self.direction != 1)
        elif i + 1 < x:
            obstacle_up = False
        else:
            obstacle_up = True
 
        if j - 1 >= 0 and (i >=0 and i < x):
            obstacle_left = world[i,j-1] == 2 and (self.direction != 2)
        elif j - 1 >= 0:
            obstacle_left = False
        else:
            obstacle_left = True
 
        if i - 1 >= 0 and (j >=0 and j < y):
            obstacle_down = world[i-1,j] == 2 and (self.direction != 3)
        elif i - 1 >= 0:
            obstacle_down = False
        else:
            obstacle_down = True

        
        d1 = int(self.direction == 0)
        d2 = int(self.direction == 1)
        d3 = int(self.direction == 2)
        d4 = int(self.direction == 3)

        obs = np.array([
            int(self.head_pos[0] > self.food_pos[0]),
            int(self.head_pos[0] < self.food_pos[0]),
            int(self.head_pos[1] > self.food_pos[1]),
            int(self.head_pos[1] < self.food_pos[1]),

            int(obstacle_up),
            int(obstacle_down),
            int(obstacle_right),
            int(obstacle_left),

            d4,
            d2,
            d1,
            d3
        ])
        return obs

    def reward(self,done,food_eaten,msg):

        if done:
            if msg == 'wall' or msg == 'body':
                reward = -100
            elif msg == 'victory':
                reward = 100
            return reward

        if food_eaten:
            reward = 10
            return reward

        dis = np.linalg.norm(self.food_pos - self.head_pos)
        last_dis = np.linalg.norm(self.food_pos - self.last_head_pos)

        reward = 1 if dis < last_dis else -1 

        return reward
    
    def update_information(self):
        
        self.last_head_pos = self.head_pos
        self.food_pos = self.get_food_pos()
        self.head_pos = self.get_head_pos()

    def translate_motion(self, action):
        
        self.direction = (self.direction + (action - 1))%4
        return action
       
        


    def step(self, action):

        self.steps_without_food += 1
        action_tran = self.translate_motion(action)
        terminated,food_eaten,msg = self.engine.py_step(action_tran)
        self.update_information()

        
        reward = self.reward(terminated, food_eaten, msg)
            
        observation = self.get_obs()
        info = self.get_info(reward, food_eaten)

        return observation,reward,terminated, info
    
    def view_game(self):
        return np.array(self.engine.py_get_world())


    def reset(self, seed=None, options=None,manual_seeding = False):
        self.steps_without_food = 0
        if not manual_seeding:
            super().reset(seed=seed)
            seed = np.random.randint(0, 100000)
        print(seed)
        self.engine.py_reset(seed)
        
        self.update_information()        
        observation = self.get_obs()
        self.direction = 1
        return observation