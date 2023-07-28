//! Game logic for Snake

#![no_std]

use core::fmt::Write;

#[derive(Debug)]
pub enum Error {
    ScreenTooTall,
    ScreenTooWide,
}

pub struct App {
    game: Game,
    width: usize,
    height: usize,
    stdout: neotron_sdk::File,
    stdin: neotron_sdk::File,
}

impl App {
    pub const fn new(width: usize, height: usize) -> App {
        App {
            game: Game::new(width - 2, height - 2, 1, 1),
            width,
            height,
            stdout: neotron_sdk::stdout(),
            stdin: neotron_sdk::stdin(),
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        // hide cursor
        let _ = writeln!(self.stdout, "\u{001b}[?25l");
        self.clear_screen();
        self.title_screen();

        let mut seed: u16 = 0x4f35;

        'outer: loop {
            'inner: loop {
                let key = self.wait_for_key();
                seed = seed.wrapping_add(1);
                if key == b'q' || key == b'Q' {
                    break 'outer;
                }
                if key == b'p' || key == b'P' {
                    break 'inner;
                }
            }

            self.clear_screen();

            let score = self.game.play(seed, &mut self.stdin, &mut self.stdout);

            self.winning_message(score);
        }

        // show cursor
        let _ = writeln!(self.stdout, "\u{001b}[?25h");
        self.clear_screen();
        Ok(())
    }

    fn clear_screen(&mut self) {
        let _ = self.stdout.write_str("\u{001b}[2J");
        for x in 1..=self.width {
            let _ = write!(self.stdout, "\u{001b}[{};{}H{}", 1, x, '-');
            let _ = write!(self.stdout, "\u{001b}[{};{}H{}", self.height, x, '-');
        }
        for y in 1..=self.height {
            let _ = write!(self.stdout, "\u{001b}[{};{}H{}", y, 1, '|');
            let _ = write!(self.stdout, "\u{001b}[{};{}H{}", y, self.width, '|');
        }
    }

    fn title_screen(&mut self) {
        let message = "ANSI Snake";
        let centre_x = (self.width - message.chars().count()) / 2;
        let centre_y = self.height / 2;
        let _ = writeln!(
            self.stdout,
            "\u{001b}[{};{}H{}",
            centre_y, centre_x, message
        );
        let message = "Q to Quit | 'P' to Play";
        let centre_x = (self.width - message.chars().count()) / 2;
        let centre_y = (self.height / 2) + 1;
        let _ = writeln!(
            self.stdout,
            "\u{001b}[{};{}H{}",
            centre_y, centre_x, message
        );
    }

    fn wait_for_key(&mut self) -> u8 {
        loop {
            let mut buffer = [0u8; 1];
            if let Ok(1) = self.stdin.read(&mut buffer) {
                return buffer[0];
            }
            neotron_sdk::delay(core::time::Duration::from_millis(10));
        }
    }

    fn winning_message(&mut self, score: u32) {
        let centre_x = (self.width - 13) / 2;
        let mut centre_y = self.height / 2;
        let _ = writeln!(
            self.stdout,
            "\u{001b}[{};{}HScore: {:06}",
            centre_y, centre_x, score
        );
        let message = "Q to Quit | 'P' to Play";
        let centre_x = (self.width - message.chars().count()) / 2;
        centre_y += 1;
        let _ = writeln!(
            self.stdout,
            "\u{001b}[{};{}H{}",
            centre_y, centre_x, message
        );
    }
}

pub struct Game {
    board: Board<{ Self::MAX_WIDTH }, { Self::MAX_HEIGHT }>,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    head_x: usize,
    head_y: usize,
    tail_x: usize,
    tail_y: usize,
    direction: Direction,
    seed: u16,
    score: u32,
    digesting: u32,
}

impl Game {
    pub const MAX_WIDTH: usize = 80;
    pub const MAX_HEIGHT: usize = 25;

    const fn new(width: usize, height: usize, offset_x: usize, offset_y: usize) -> Game {
        Game {
            board: Board::new(),
            width,
            height,
            offset_x,
            offset_y,
            head_x: 0,
            head_y: 0,
            tail_x: 0,
            tail_y: 0,
            direction: Direction::Up,
            seed: 0,
            score: 0,
            digesting: 0,
        }
    }

    fn random(&mut self) -> u16 {
        let lfsr = self.seed;
        let bit = ((lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5)) & 0x01;
        self.seed = (lfsr >> 1) | (bit << 15);
        self.seed
    }

    fn play(
        &mut self,
        seed: u16,
        stdin: &mut neotron_sdk::File,
        stdout: &mut neotron_sdk::File,
    ) -> u32 {
        // Reset score
        self.score = 0;
        // Init random numbers
        self.seed = seed;
        for _ in 0..100 {
            let _ = self.random();
        }
        // Wipe board
        self.board.reset();
        // Add offset snake
        self.head_x = self.width / 4;
        self.head_y = self.height / 4;
        self.tail_x = self.head_x;
        self.tail_y = self.head_y;
        self.board.set_dir(self.head_x, self.head_y, self.direction);
        self.write_at(stdout, self.head_x, self.head_y, 'U');
        // Add random food
        let (x, y) = self.random_empty_position();
        self.board.set_food(x, y);
        self.write_at(stdout, x, y, 'F');

        'game: loop {
            // Wait for frame tick
            neotron_sdk::delay(core::time::Duration::from_millis(100));

            // 1 point for not being dead
            self.score += 1;

            // Read input
            'input: loop {
                let mut buffer = [0u8; 1];
                if let Some(1) = stdin.read(&mut buffer).ok() {
                    match buffer[0] {
                        b'w' | b'W' => {
                            // Going up
                            if self.direction.is_horizontal() {
                                self.direction = Direction::Up;
                            }
                        }
                        b's' | b'S' => {
                            // Going down
                            if self.direction.is_horizontal() {
                                self.direction = Direction::Down;
                            }
                        }
                        b'a' | b'A' => {
                            // Going left
                            if self.direction.is_vertical() {
                                self.direction = Direction::Left;
                            }
                        }
                        b'd' | b'D' => {
                            // Going right
                            if self.direction.is_vertical() {
                                self.direction = Direction::Right;
                            }
                        }
                        b'q' | b'Q' => {
                            // Quit game
                            break 'game;
                        }
                        _ => {
                            // ignore
                        }
                    }
                } else {
                    break 'input;
                }
            }

            // Mark which way we're going in the old head position
            self.board.set_dir(self.head_x, self.head_y, self.direction);

            // Update head position
            match self.direction {
                Direction::Up => {
                    if self.head_y == 0 {
                        break 'game;
                    }
                    self.head_y -= 1;
                }
                Direction::Down => {
                    if self.head_y == self.height - 1 {
                        break 'game;
                    }
                    self.head_y += 1;
                }
                Direction::Left => {
                    if self.head_x == 0 {
                        break 'game;
                    }
                    self.head_x -= 1;
                }
                Direction::Right => {
                    if self.head_x == self.width - 1 {
                        break 'game;
                    }
                    self.head_x += 1;
                }
            }

            // Check what we just ate
            //   - Food => get longer
            //   - Ourselves => die
            if self.board.is_food(self.head_x, self.head_y) {
                // yum
                self.score += 10;
                self.digesting = 2;
                // Add random food
                let (x, y) = self.random_empty_position();
                self.board.set_food(x, y);
                self.write_at(stdout, x, y, 'F');
            } else if self.board.is_body(self.head_x, self.head_y) {
                // oh no
                break 'game;
            }

            // Write the new head
            self.board.set_dir(self.head_x, self.head_y, self.direction);
            self.write_at(stdout, self.head_x, self.head_y, 'U');

            if self.digesting == 0 {
                let (old_tail_x, old_tail_y) = (self.tail_x, self.tail_y);
                match self.board.get_tail_dir(self.tail_x, self.tail_y) {
                    Some(Direction::Up) => {
                        self.tail_y -= 1;
                    }
                    Some(Direction::Down) => {
                        self.tail_y += 1;
                    }
                    Some(Direction::Left) => {
                        self.tail_x -= 1;
                    }
                    Some(Direction::Right) => {
                        self.tail_x += 1;
                    }
                    None => {
                        panic!("Bad game state");
                    }
                }
                self.board.clear(old_tail_x, old_tail_y);
                self.write_at(stdout, old_tail_x, old_tail_y, ' ');
            } else {
                self.digesting -= 1;
            }
        }

        self.score
    }

    fn write_at(&self, console: &mut neotron_sdk::File, x: usize, y: usize, ch: char) {
        let _ = write!(
            console,
            "\u{001b}[{};{}H{}",
            self.offset_y + y + 1,
            self.offset_x + x + 1,
            ch
        );
    }

    fn random_empty_position(&mut self) -> (usize, usize) {
        loop {
            // This isn't equally distributed. I don't really care.
            let x = (self.random() as usize) % self.width;
            let y = (self.random() as usize) % self.height;
            if self.board.is_empty(x, y) {
                return (x, y);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_horizontal(self) -> bool {
        self == Direction::Left || self == Direction::Right
    }

    fn is_vertical(self) -> bool {
        self == Direction::Up || self == Direction::Down
    }
}

struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[u8; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    const UP: u8 = b'U';
    const DOWN: u8 = b'D';
    const LEFT: u8 = b'L';
    const RIGHT: u8 = b'R';
    const FOOD: u8 = b'F';

    const fn new() -> Board<WIDTH, HEIGHT> {
        Board {
            cells: [[0; WIDTH]; HEIGHT],
        }
    }

    fn reset(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.cells[y][x] = 0;
            }
        }
    }

    fn set_dir(&mut self, x: usize, y: usize, direction: Direction) {
        self.cells[y][x] = match direction {
            Direction::Up => Self::UP,
            Direction::Down => Self::DOWN,
            Direction::Left => Self::LEFT,
            Direction::Right => Self::RIGHT,
        }
    }

    fn get_tail_dir(&self, x: usize, y: usize) -> Option<Direction> {
        match self.cells[y][x] {
            Self::UP => Some(Direction::Up),
            Self::DOWN => Some(Direction::Down),
            Self::LEFT => Some(Direction::Left),
            Self::RIGHT => Some(Direction::Right),
            _ => None,
        }
    }

    fn set_food(&mut self, x: usize, y: usize) {
        self.cells[y][x] = Self::FOOD;
    }

    fn is_food(&mut self, x: usize, y: usize) -> bool {
        self.cells[y][x] == Self::FOOD
    }

    fn is_body(&mut self, x: usize, y: usize) -> bool {
        let cell = self.cells[y][x];
        cell == Self::UP || cell == Self::DOWN || cell == Self::LEFT || cell == Self::RIGHT
    }

    fn is_empty(&mut self, x: usize, y: usize) -> bool {
        self.cells[y][x] == 0
    }

    fn clear(&mut self, x: usize, y: usize) {
        self.cells[y][x] = 0;
    }
}
