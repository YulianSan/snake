#![allow(warnings)]

use rand::{rng, seq::IteratorRandom};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn get_valid_direction(&self, currrent: Direction) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }

    fn is_valid_direction(&self, direction: Direction) -> bool {
        self.get_valid_direction(*self).contains(&direction)
    }
}

#[derive(Clone)]
pub struct Snake {
    pub body: Vec<(u16, u16)>,
    pub direction: Direction,
    pub next_direction: Direction,
    pub head_pos: (u16, u16),
    pub alive: bool,
    grow: bool,
}

impl Snake {
    pub fn new(pos: (u16, u16)) -> Snake {
        Snake {
            body: vec![(pos.0, pos.1)],
            direction: Direction::Right,
            next_direction: Direction::Right,
            head_pos: (pos.0, pos.1),
            alive: true,
            grow: false,
        }
    }

    pub fn walk(&mut self) {
        self.head_pos = self.next_pos();

        if self.grow {
            self.grow = false;
            self.body.push(self.head_pos);
        } else {
            self.body.remove(0);
            self.body.push(self.head_pos);
        }
    }

    pub fn next_pos(&self) -> (u16, u16) {
        let (x, y) = self.direction.value();

        let x = u16::try_from(x + self.head_pos.0 as i32).expect("overflow in x");
        let y = u16::try_from(y + self.head_pos.1 as i32).expect("overflow in y");

        (x, y)
    }

    pub fn eat(&mut self) {
        self.grow = true;
    }

    pub fn self_collision(&self) -> bool {
        let next_pos = self.next_pos();

        for i in 1..self.body.len() {
            if next_pos == self.body[i] {
                return true;
            }
        }

        false
    }
}

#[derive(Clone, Copy)]
pub struct Food {
    pub pos: (u16, u16),
}

impl Food {
    pub fn new(x: u16, y: u16) -> Food {
        Food { pos: (x, y) }
    }
}

#[derive(Clone)]
struct ConfigGame {
    food_amount: u16,
}

impl Default for ConfigGame {
    fn default() -> Self {
        ConfigGame { food_amount: 1 }
    }
}

#[derive(Clone)]
pub struct Game {
    pub snake: Snake,
    pub food: Vec<Food>,
    initial_pos: (u16, u16),
    config: ConfigGame,
    height: u16,
    width: u16,
}

impl Game {
    pub fn new(pos: (u16, u16), width: u16, height: u16) -> Game {
        Game {
            snake: Snake::new(pos),
            initial_pos: pos,
            food: vec![],
            height,
            width,
            config: ConfigGame {
                food_amount: 1,
                ..Default::default()
            },
        }
    }

    pub fn reset(&mut self) {
        self.snake = Snake::new(self.initial_pos);
        self.food = vec![];
    }

    pub fn next(&mut self) -> bool {
        self.snake.direction = self.snake.next_direction;
        if (!self.snake.alive || !self.snake_inside() || self.snake.self_collision()) {
            self.snake.alive = false;
            return false;
        }

        self.snake_collion_food();
        self.generate_food();
        self.snake.walk();

        true
    }

    pub fn generate_food(&mut self) {
        let food_amount = (self.config.food_amount as usize)
            .checked_sub(self.food.len())
            .unwrap_or(0);

        if food_amount == 0 {
            return;
        }

        let mut possible_positions = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                possible_positions.push((x, y));
            }
        }

        let pos = possible_positions
            .iter()
            .choose_multiple(&mut rand::thread_rng(), self.config.food_amount as usize);

        for pos in pos {
            self.food.push(Food::new(pos.0, pos.1));
        }
    }

    pub fn snake_inside(&self) -> bool {
        let next_pos = self.snake.next_pos();

        next_pos.0 < self.width && next_pos.1 < self.height && next_pos.0 != 0 && next_pos.1 != 0
    }

    pub fn snake_collion_food(&mut self) {
        let food_amount = self.food.len();
        let next_pos = self.snake.next_pos();

        self.food = self
            .food
            .iter()
            .filter(|f| f.pos != next_pos)
            .cloned()
            .collect();

        let food_amount_eat = (food_amount - self.food.len()) as u16;
        for _ in 0..food_amount_eat {
            self.snake.eat();
        }
    }

    pub fn input(&mut self, direction: Direction) {
        if !self.snake.direction.is_valid_direction(direction) {
            return;
        }

        self.snake.next_direction = direction
    }
}

mod test {
    use crate::*;

    #[test]
    fn snake_should_walk() {
        let mut game = Game {
            snake: Snake {
                head_pos: (3, 5),
                body: vec![(1, 5), (2, 5), (3, 5)],
                direction: Direction::Right,
                next_direction: Direction::Right,
                alive: true,
                grow: false,
            },
            food: vec![],
            height: 11,
            width: 11,
            config: ConfigGame {
                food_amount: 0,
                ..Default::default()
            },
        };

        game.next();
        assert_eq!(game.snake.head_pos, (4, 5));
        assert_eq!(game.snake.body, vec![(2, 5), (3, 5), (4, 5)]);
        assert_eq!(game.snake.grow, false);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (5, 5));
        assert_eq!(game.snake.body, vec![(3, 5), (4, 5), (5, 5)]);
        assert_eq!(game.snake.grow, false);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (6, 5));
        assert_eq!(game.snake.body, vec![(4, 5), (5, 5), (6, 5)]);
        assert_eq!(game.snake.grow, false);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (7, 5));
        assert_eq!(game.snake.body, vec![(5, 5), (6, 5), (7, 5)]);
        assert_eq!(game.snake.grow, false);
        assert_eq!(game.snake.alive, true);
    }

    #[test]
    fn snake_should_eat() {
        let mut game = Game {
            snake: Snake {
                head_pos: (3, 5),
                body: vec![(3, 5)],
                direction: Direction::Right,
                next_direction: Direction::Right,
                alive: true,
                grow: false,
            },
            food: vec![Food::new(5, 5)],
            height: 11,
            width: 11,
            config: ConfigGame { food_amount: 0 },
        };

        game.next();
        assert_eq!(game.snake.head_pos, (4, 5));
        assert_eq!(game.snake.body, vec![(4, 5)]);
        assert_eq!(game.snake.alive, true);

        game.next();

        assert_eq!(game.snake.head_pos, (5, 5));
        assert_eq!(game.snake.body, vec![(4, 5), (5, 5)]);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (6, 5));
        assert_eq!(game.snake.body, vec![(5, 5), (6, 5)]);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (7, 5));
        assert_eq!(game.snake.body, vec![(6, 5), (7, 5)]);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (8, 5));
        assert_eq!(game.snake.body, vec![(7, 5), (8, 5)]);
        assert_eq!(game.snake.alive, true);

        game.food.push(Food::new(9, 5));

        game.next();
        assert_eq!(game.snake.head_pos, (9, 5));
        assert_eq!(game.snake.body, vec![(7, 5), (8, 5), (9, 5)]);
        assert_eq!(game.snake.alive, true);
    }

    #[test]
    fn snake_should_die() {
        let mut game = Game {
            snake: Snake {
                head_pos: (4, 3),
                body: vec![(4, 3)],
                direction: Direction::Right,
                next_direction: Direction::Right,
                alive: true,
                grow: false,
            },
            food: vec![],
            height: 6,
            width: 6,
            config: ConfigGame { food_amount: 0 },
        };

        game.next();
        assert!(game.snake.alive);

        game.next();
        assert!(!game.snake.alive);
    }

    #[test]
    fn snake_should_change_direction() {
        let mut game = Game {
            snake: Snake {
                head_pos: (5, 5),
                body: vec![(5, 5)],
                direction: Direction::Right,
                next_direction: Direction::Right,
                alive: true,
                grow: false,
            },
            food: vec![],
            height: 11,
            width: 11,
            config: ConfigGame::default(),
        };

        game.input(Direction::Left);
        assert_eq!(game.snake.direction, Direction::Right);

        game.input(Direction::Up);
        assert_eq!(game.snake.direction, Direction::Up);
        game.next();
        assert_eq!(game.snake.direction, Direction::Up);
        assert_eq!(game.snake.head_pos, (5, 4));
        assert!(game.snake.alive);

        game.input(Direction::Left);
        assert_eq!(game.snake.direction, Direction::Left);
        game.next();
        assert_eq!(game.snake.direction, Direction::Left);
        assert_eq!(game.snake.head_pos, (4, 4));
        assert!(game.snake.alive);

        game.input(Direction::Down);
        assert_eq!(game.snake.direction, Direction::Down);
        game.next();
        assert_eq!(game.snake.direction, Direction::Down);
        assert_eq!(game.snake.head_pos, (4, 5));
        assert!(game.snake.alive);

        game.input(Direction::Right);
        assert_eq!(game.snake.direction, Direction::Right);
        game.next();
        assert_eq!(game.snake.direction, Direction::Right);
        assert_eq!(game.snake.head_pos, (5, 5));
        assert!(game.snake.alive);
    }

    #[test]
    fn snake_should_self_collision() {
        let mut game = Game {
            snake: Snake {
                head_pos: (5, 5),
                body: vec![(6, 5), (5, 4), (4, 4), (4, 5), (5, 5)],
                direction: Direction::Right,
                next_direction: Direction::Right,
                alive: true,
                grow: false,
            },
            food: vec![],
            height: 11,
            width: 11,
            config: ConfigGame::default(),
        };

        assert!(!game.snake.self_collision());
        game.input(Direction::Up);
        assert!(game.snake.self_collision());

        game.next();
        assert!(!game.snake.alive);
    }

    #[test]
    fn should_generate_new_food() {
        let mut game = Game {
            snake: Snake {
                head_pos: (5, 5),
                body: vec![(5, 5)],
                direction: Direction::Right,
                next_direction: Direction::Right,
                alive: true,
                grow: false,
            },
            food: vec![Food::new(6, 5)],
            height: 11,
            width: 11,
            config: ConfigGame { food_amount: 3 },
        };

        assert_eq!(game.food.len(), 1);
        game.next();
        assert_eq!(game.snake.head_pos, (6, 5));
        assert_eq!(game.snake.body, vec![(5, 5), (6, 5)]);
        assert_eq!(game.snake.alive, true);
        assert_eq!(game.food.len(), 3);
    }
}
