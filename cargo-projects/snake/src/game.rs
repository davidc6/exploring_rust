use crate::food;
use crate::snake;
use crate::utils;

use rand::Rng;

const NUM_FOODS: usize = 5;
const INIT_FOOD_LIFE: f32 = 1.0;

pub struct Game {
    pub foods: Vec<food::Food>,
}

impl Game {
    pub fn update_food_expired(&mut self) {
        let _ = &self.foods.retain(|food| food.time >= 0.1);
    }

    pub fn update_food_life(&mut self) {
        const FOOD_DECAY_SPEED: f32 = 0.1;

        for i in self.foods.iter_mut() {
            i.time = ((i.time - FOOD_DECAY_SPEED) * 100.0).round() / 100.0;
        }
    }

    pub fn check_eating(&mut self, snake: &snake::Snake, food: &food::Food) {
        if snake.head_position() == &food.position {
            let mut to_remove: Option<usize> = None;

            for (i, value) in &mut self.foods.iter().enumerate() {
                if value.position.0 == food.position.0 && value.position.1 == food.position.1 {
                    to_remove = Some(i);
                }
            }

            match to_remove {
                None => println!("No result"),
                Some(i) => {
                    println!("RESULT IS HERE");
                    let _ = &self.foods.remove(i);
                }
            }
        }
    }

    pub fn check_snake_alive(&self, snake: &snake::Snake, dir: utils::Directions) -> bool {
        let next_head: (i32, i32) = match dir {
            utils::Directions::Left => (-1, 0),
            utils::Directions::Up => (0, 1),
            utils::Directions::Right => (1, 0),
            utils::Directions::Down => (0, -1),
        };

        let (x, y) = snake.head_position();
        let (x2, y2) = next_head;
        let next_pos: utils::Position = (x + x2, y + y2);

        if snake.body.contains(&next_pos) {
            return false;
        }

        return true;
    }

    pub fn update_food(&mut self) {
        while &self.foods.len() < &NUM_FOODS {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..10);
            let y = rng.gen_range(0..10);
            let food = food::Food {
                time: INIT_FOOD_LIFE,
                position: (x, y),
            };
            let _ = &self.foods.push(food);
        }
    }
}
