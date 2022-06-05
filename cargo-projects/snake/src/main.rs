use std::collections::LinkedList;
use rand::Rng;

const NUM_FOODS: usize = 5;
const INIT_FOOD_LIFE: f32 = 1.0;


#[derive(Debug, Copy, Clone)]
struct Food {
  time: f32,
  position: Position
}

struct Game {
  foods: Vec<Food>
}

impl Game {
  fn update_food_expired(&mut self) {
    let _ = &self.foods.retain(|food| food.time >= 0.1);
  }

  fn update_food_life(&mut self) {
    const FOOD_DECAY_SPEED: f32 = 0.1;
  
    for i in self.foods.iter_mut() {
      i.time = ((i.time - FOOD_DECAY_SPEED) * 100.0).round() / 100.0;
    }
  }

  fn check_eating(&mut self, snake: &mut Snake, food: &Food) {
    if snake.head_position() == &food.position {
      let mut to_remove: Option<usize> = None;

      for (i, value) in &mut self.foods.iter_mut().enumerate() {
        if value.position.0 == food.position.0 && value.position.1 == food.position.1 {          
          to_remove = Some(i);
        }
      }

      match to_remove {
        None => println!("No result"),
        Some(i) => {
          let _ = &self.foods.remove(i);
        }
      }
      
    }
  }

  fn check_snake_alive(&self, snake: &Snake, dir: Directions) -> bool {
    let next_head: (i32, i32) = match dir {
      Directions::Left => (-1, 0),
      Directions::Up => (0, 1),
      Directions::Right => (1, 0),
      Directions::Down => (0, -1)
    };

    let (x, y) = snake.head_position();
    let (x2, y2) = next_head;
    let next_pos: Position = (x + x2, y + y2);

    if snake.body.contains(&next_pos) {
      return false;
    }

    return true;
  }

  fn update_food(&mut self) {
    while &self.foods.len() < &NUM_FOODS {
      let mut rng = rand::thread_rng();
      let x = rng.gen_range(0..10);
      let y = rng.gen_range(0..10);
      let food = Food { time: INIT_FOOD_LIFE, position: (x, y) };
      let _ = &self.foods.push(food);
    }
  }
}

// Snake

#[derive(Debug)]
enum Directions {
  Up,
  Down,
  Left,
  Right
}

type Position = (i32, i32);

#[derive(Debug)]
struct Snake {
  position: Position,
  direction: Directions,
  body: LinkedList<Position>
}

impl Snake {
  fn head_position(&self) -> &Position {
    return &self.position;
  }

  fn head_direction(&self) -> &Directions {
    return &self.direction;
  }
}

fn main() {
  let food1 = Food {
    time: 0.1,
    position: (1,2)
  };

  let food2 = Food {
    time: 0.2,
    position: (7,8)
  };

  let food3 = Food {
    time: 0.3,
    position: (3,4)
  };

  let food4 = Food {
    time: 0.4,
    position: (5,6)
  };

  let mut snake: Snake = Snake { position: (3, 4), direction: Directions::Right, body: LinkedList::new() };
  let mut game = Game {
    foods: vec![]
  };

  game.foods.push(food1);
  game.foods.push(food2);
  game.foods.push(food3);
  game.foods.push(food4);

  game.update_food(); // food, auto update
  game.update_food_life();
  game.update_food_expired();
  game.check_eating(&mut snake, &food3);
  let is_snake_alive = game.check_snake_alive(&snake, Directions::Right);

  println!("{:?}", snake.head_direction());
  println!("{}", is_snake_alive);
  println!("{:?}", game.foods);
}
