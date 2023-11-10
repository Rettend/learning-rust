use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::collections::HashSet;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 10;

struct Snake {
    x: u32,
    y: u32,
    tail: HashSet<(u32, u32)>,
}

impl Snake {
    fn new(x: u32, y: u32) -> Snake {
        Snake {
            x,
            y,
            tail: HashSet::new(),
        }
    }

    fn update(&mut self, x: u32, y: u32) {
        self.tail.insert((self.x, self.y));
        self.x = x;
        self.y = y;
    }
}

fn print_game(snake: &Snake, apple_x: u32, apple_y: u32) {
    for _ in 0..WIDTH + 2 {
        print!("#");
    }
    println!();
    
    for i in 0..HEIGHT {
        print!("#");
        for j in 0..WIDTH {
            if i == apple_y && j == apple_x {
                print!("A");
            } else if i == snake.y && j == snake.x {
                print!("O");
            } else if snake.tail.iter().any(|&(x, y)| x == j && y == i) {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!("#");
    }

    for _ in 0..WIDTH + 2 {
        print!("#");
    }
    println!();
}

fn main() {
    println!("[ Welcome to Snake Game ]\n");
    println!("- Use WASD to move the snake\n- Eat all the apples to win without dying! GO!\n");

    let mut snake = Snake::new(WIDTH / 2, HEIGHT / 2);
    let mut apple_x;
    let mut apple_y;
    let mut rng = rand::thread_rng();

    loop {
        apple_x = rng.gen_range(0..WIDTH);
        apple_y = rng.gen_range(0..HEIGHT);
        if apple_x != snake.x && apple_y != snake.y {
            break;
        }
    }

    let mut score = 0;
    let score_to_win = 10;
    let mut game_over = false;

    while !game_over && score < score_to_win {
        print_game(&snake, apple_x, apple_y);
        println!("Score: {score}");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        let (mut x, mut y) = (snake.x, snake.y);

        for ch in input.chars() {
            match ch {
                'w' => {
                    if snake.y > 0 {
                        y -= 1;
                    }
                }
                'a' => {
                    if snake.x > 0 {
                        x -= 1;
                    }
                }
                's' => {
                    if snake.y < HEIGHT - 1 {
                        y += 1;
                    }
                }
                'd' => {
                    if snake.x < WIDTH - 1 {
                        x += 1;
                    }
                }
                _ => {}
            }

            if x == apple_x && y == apple_y {
                score += 1;
                loop {
                    apple_x = rng.gen_range(0..WIDTH);
                    apple_y = rng.gen_range(0..HEIGHT);
                    if apple_x != snake.x && apple_y != snake.y {
                        break;
                    }
                }
            } else if !snake.tail.is_empty() {
                snake.tail.remove(&(snake.x, snake.y));
            }

            for _ in 0..snake.tail.len() {
                if snake.tail.contains(&(x, y)) {
                    game_over = true;
                }
            }

            Snake::update(&mut snake, x, y);
            thread::sleep(Duration::from_millis(10));
        }
    }

    print_game(&snake, apple_x, apple_y);
    println!("\nGame Over! Your score was: {score}");
}
