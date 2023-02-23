use std::io::{self, Read, Write};
use std::{thread, time};

fn main() {
    let mut score = 0;
    let mut high_score = 0;
    loop {
        match menu() {
            Some(1) => {
                let mut game = Game::new();
                let result = game.play();
                score += result;
                if score > high_score {
                    high_score = score;
                }
                println!("Game Over. Score: {}, High Score: {}", result, high_score);
                score = 0;
                thread::sleep(time::Duration::from_secs(2));
            }
            Some(2) => {
                println!("High Score: {}", high_score);
                thread::sleep(time::Duration::from_secs(2));
            }
            Some(3) => {
                let name = get_name();
                println!("Hello, {}!", name);
                thread::sleep(time::Duration::from_secs(2));
            }
            Some(4) => break,
            _ => (),
        }
    }
}

fn menu() -> Option<u32> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = [0; 1];
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear the screen
        println!("Menu:");
        println!("1. Play");
        println!("2. High Score");
        println!("3. Enter Name");
        println!("4. Quit");
        stdout.flush().unwrap();
        if stdin.read(&mut buf).is_ok() {
            match buf[0] {
                b'1' => return Some(1),
                b'2' => return Some(2),
                b'3' => return Some(3),
                b'4' => return Some(4),
                _ => (),
            }
        }
    }
}

fn get_name() -> String {
    let mut name = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("Enter your name: ");
    stdout.flush().unwrap();
    stdin.read_line(&mut name).unwrap();
    name.trim().to_string()
}

struct Game {
    cactus_position: u32,
    dino_position: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            cactus_position: 100,
            dino_position: 0,
        }
    }

    fn play(&mut self) -> u32 {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut buf = [0; 1];

        let mut score = 0;
        loop {
            // move cactus and dino
            self.cactus_position -= 1;
            self.dino_position += 1;
            // draw game
            print!("\x1B[2J\x1B[1;1H"); // clear the screen
            for _ in 0..self.cactus_position {
                print!(" ");
            }
            print!("X");
            for _ in self.cactus_position..100 {
                print!(" ");
            }
            println!("");
            for _ in 0..self.dino_position {
                println!("");
            }
            println!("  O");
            for _ in self.dino_position + 1..20 {
                println!("");
            }
            print!("Score: {}\r", score);
            stdout.flush().unwrap();
            // check for collision
            if self.cactus_position == 0 {
                break;
  }
            // check for input
            if stdin.read(&mut buf).is_ok() {
                if buf[0] == b' ' {
                    self.jump();
                }
            }
            // increase score
            score += 1;
            thread::sleep(time::Duration::from_millis(50));
       “ }
        score
    }

    fn jump(&mut self) {
        if self.dino_position > 0 {
            self.dino_position -= 1;
        }
    }
}
