#![allow(warnings)]

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
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
struct Snake {
    body: Vec<(u32, u32)>,
    direction: Direction,
    head_pos: (u32, u32),
    alive: bool,
    i_body: usize,
    grow: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![(0, 0)],
            direction: Direction::Right,
            head_pos: (0, 0),
            alive: true,
            i_body: 0,
            grow: false,
        }
    }

    pub fn walk(&mut self) {
        self.head_pos = self.next_pos();

        if self.grow {
            self.grow = false;
            self.body.push(self.head_pos);
        } else {
            self.body[self.i_body] = self.head_pos;
            self.i_body = (self.i_body + 1) % self.body.len();
        }
    }

    pub fn next_pos(&self) -> (u32, u32) {
        let (x, y) = self.direction.value();

        let x = u32::try_from(x + self.head_pos.0 as i32).expect("overflow in x");
        let y = u32::try_from(y + self.head_pos.1 as i32).expect("overflow in y");

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
struct Food {
    pos: (u32, u32),
}

impl Food {
    pub fn new(x: u32, y: u32) -> Food {
        Food { pos: (x, y) }
    }
}

#[derive(Clone)]
struct ConfigGame {
    food_amount: u32,
}

impl Default for ConfigGame {
    fn default() -> Self {
        ConfigGame { food_amount: 1 }
    }
}

#[derive(Clone)]
pub struct Game {
    snake: Snake,
    food: Vec<Food>,
    config: ConfigGame,
    height: u32,
    width: u32,
}

impl Game {
    pub fn next(&mut self) -> bool {
        if (!self.snake.alive || !self.snake_inside() || self.snake.self_collision()) {
            self.snake.alive = false;
            return false;
        }

        self.snake_collion_food();
        self.snake.walk();

        true
    }

    pub fn snake_inside(&self) -> bool {
        self.snake.head_pos.0 < self.width
            && self.snake.head_pos.1 < self.height
            && self.snake.head_pos.0 > 0
            && self.snake.head_pos.1 > 0
    }

    pub fn snake_collion_food(&mut self) {
        let food_amount = self.food.len();

        self.food = self
            .food
            .iter()
            .filter(|f| f.pos != self.snake.head_pos)
            .cloned()
            .collect();

        let food_amount_eat = (food_amount - self.food.len()) as u32;
        for _ in 0..food_amount_eat {
            self.snake.eat();
        }
    }

    pub fn input(&mut self, direction: Direction) {
        if !self.snake.direction.is_valid_direction(direction) {
            return;
        }

        self.snake.direction = direction
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
                alive: true,
                i_body: 0,
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
        assert_eq!(game.snake.i_body, 1);
        assert_eq!(game.snake.body, vec![(4, 5), (2, 5), (3, 5)]);
        assert_eq!(game.snake.grow, false);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (5, 5));
        assert_eq!(game.snake.i_body, 2);
        assert_eq!(game.snake.body, vec![(4, 5), (5, 5), (3, 5)]);
        assert_eq!(game.snake.grow, false);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (6, 5));
        assert_eq!(game.snake.i_body, 0);
        assert_eq!(game.snake.body, vec![(4, 5), (5, 5), (6, 5)]);
        assert_eq!(game.snake.grow, false);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (7, 5));
        assert_eq!(game.snake.i_body, 1);
        assert_eq!(game.snake.body, vec![(7, 5), (5, 5), (6, 5)]);
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
                alive: true,
                i_body: 0,
                grow: false,
            },
            food: vec![Food::new(5, 5)],
            height: 11,
            width: 11,
            config: ConfigGame::default(),
        };

        game.next();
        game.next();

        assert_eq!(game.snake.head_pos, (5, 5));
        assert_eq!(game.snake.i_body, 0);
        assert_eq!(game.snake.body, vec![(5, 5)]);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (6, 5));
        assert_eq!(game.snake.i_body, 0);
        assert_eq!(game.snake.body, vec![(5, 5), (6, 5)]);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (7, 5));
        assert_eq!(game.snake.i_body, 1);
        assert_eq!(game.snake.body, vec![(7, 5), (6, 5)]);
        assert_eq!(game.snake.alive, true);

        game.next();
        assert_eq!(game.snake.head_pos, (8, 5));
        assert_eq!(game.snake.i_body, 0);
        assert_eq!(game.snake.body, vec![(7, 5), (8, 5)]);
        assert_eq!(game.snake.alive, true);
    }

    #[test]
    fn snake_should_die() {
        let mut game = Game {
            snake: Snake {
                head_pos: (5, 5),
                body: vec![(5, 5)],
                direction: Direction::Right,
                alive: true,
                i_body: 0,
                grow: false,
            },
            food: vec![],
            height: 6,
            width: 6,
            config: ConfigGame::default(),
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
                alive: true,
                i_body: 0,
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
                alive: true,
                i_body: 0,
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
}
