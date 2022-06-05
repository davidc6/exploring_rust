mod game;
mod utils;
mod snake;
mod food;

use std::collections::LinkedList;

fn main() {
  // manually set up foods
  // let food1 = food::Food {
  //   time: 0.1,
  //   position: (1,2)
  // };

  // let food2 = food::Food {
  //   time: 0.2,
  //   position: (7,8)
  // };

  // let food3 = food::Food {
  //   time: 0.3,
  //   position: (3,4)
  // };

  // let food4 = food::Food {
  //   time: 0.4,
  //   position: (5,6)
  // };

  let snake: snake::Snake = snake::Snake { position: (3, 4), direction: utils::Directions::Right, body: LinkedList::new() };
  let mut game = game::Game {
    foods: vec![]
  };

  // manually push foods in
  // game.foods.push(food1);
  // game.foods.push(food2);
  // game.foods.push(food3);
  // game.foods.push(food4);

  game.update_food(); // food, auto update
  game.update_food_life();
  game.update_food_expired();
  let position_to_check = game.foods[0];
  game.check_eating(&snake, &position_to_check);
  let is_snake_alive = game.check_snake_alive(&snake, utils::Directions::Right);

  println!("{:?}", snake.head_direction());
  println!("{}", is_snake_alive);
  println!("{:?}", game.foods);
}
