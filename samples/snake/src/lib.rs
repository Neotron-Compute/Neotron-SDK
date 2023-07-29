//! Game logic for Snake

#![no_std]

use core::fmt::Write;

use neotron_sdk::console;

#[derive(Debug)]
pub enum Error {
    ScreenTooTall,
    ScreenTooWide,
}

pub struct App {
    game: Game,
    width: u8,
    height: u8,
    stdout: neotron_sdk::File,
    stdin: neotron_sdk::File,
}

impl App {
    pub const fn new(width: u8, height: u8) -> App {
        App {
            game: Game::new(width - 2, height - 2, console::Position { row: 1, col: 1 }),
            width,
            height,
            stdout: neotron_sdk::stdout(),
            stdin: neotron_sdk::stdin(),
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        console::cursor_off(&mut self.stdout);
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

            neotron_sdk::srand(seed);

            let score = self.game.play(&mut self.stdin, &mut self.stdout);

            self.winning_message(score);
        }

        // show cursor
        console::cursor_on(&mut self.stdout);
        self.clear_screen();
        Ok(())
    }

    fn clear_screen(&mut self) {
        console::clear_screen(&mut self.stdout);
        console::move_cursor(&mut self.stdout, console::Position::origin());
        let _ = self.stdout.write_char('+');
        for _ in 1..self.width - 1 {
            let _ = self.stdout.write_char('-');
        }
        let _ = self.stdout.write_char('+');
        console::move_cursor(
            &mut self.stdout,
            console::Position {
                row: self.height - 1,
                col: 0,
            },
        );
        let _ = self.stdout.write_char('+');
        for _ in 1..self.width - 1 {
            let _ = self.stdout.write_char('-');
        }
        let _ = self.stdout.write_char('+');
        for row in 1..self.height - 1 {
            console::move_cursor(&mut self.stdout, console::Position { row, col: 0 });
            let _ = self.stdout.write_char('|');
            console::move_cursor(
                &mut self.stdout,
                console::Position {
                    row,
                    col: self.width - 1,
                },
            );
            let _ = self.stdout.write_char('|');
        }
    }

    fn title_screen(&mut self) {
        let message = "ANSI Snake";
        let pos = console::Position {
            row: self.height / 2,
            col: (self.width - message.chars().count() as u8) / 2,
        };
        console::move_cursor(&mut self.stdout, pos);
        let _ = self.stdout.write_str(message);
        let message = "Q to Quit | 'P' to Play";
        let pos = console::Position {
            row: pos.row + 1,
            col: (self.width - message.chars().count() as u8) / 2,
        };
        console::move_cursor(&mut self.stdout, pos);
        let _ = self.stdout.write_str(message);
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
        let pos = console::Position {
            row: self.height / 2,
            col: (self.width - 13 as u8) / 2,
        };
        console::move_cursor(&mut self.stdout, pos);
        let _ = writeln!(self.stdout, "Score: {:06}", score);
        let message = "Q to Quit | 'P' to Play";
        let pos = console::Position {
            row: pos.row + 1,
            col: (self.width - message.chars().count() as u8) / 2,
        };
        console::move_cursor(&mut self.stdout, pos);
        let _ = self.stdout.write_str(message);
    }
}

pub struct Game {
    board: Board<{ Self::MAX_WIDTH }, { Self::MAX_HEIGHT }>,
    width: u8,
    height: u8,
    offset: console::Position,
    head: console::Position,
    tail: console::Position,
    direction: Direction,
    score: u32,
    digesting: u32,
    tick_interval_ms: u16,
}

impl Game {
    pub const MAX_WIDTH: usize = 80;
    pub const MAX_HEIGHT: usize = 25;

    const fn new(width: u8, height: u8, offset: console::Position) -> Game {
        Game {
            board: Board::new(),
            width,
            height,
            offset,
            head: console::Position { row: 0, col: 0 },
            tail: console::Position { row: 0, col: 0 },
            direction: Direction::Up,
            score: 0,
            digesting: 0,
            tick_interval_ms: 150,
        }
    }

    fn play(&mut self, stdin: &mut neotron_sdk::File, stdout: &mut neotron_sdk::File) -> u32 {
        // Reset score
        self.score = 0;
        // Wipe board
        self.board.reset();
        // Add offset snake
        self.head = console::Position {
            row: self.height / 4,
            col: self.width / 4,
        };
        self.tail = self.head;
        self.board.set_dir(self.head, self.direction);
        self.write_at(stdout, self.head, 'U');
        // Add random food
        let pos = self.random_empty_position();
        self.board.set_food(pos);
        self.write_at(stdout, pos, 'F');

        'game: loop {
            // Wait for frame tick
            neotron_sdk::delay(core::time::Duration::from_millis(
                self.tick_interval_ms as u64,
            ));

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
            self.board.set_dir(self.head, self.direction);

            // Update head position
            match self.direction {
                Direction::Up => {
                    if self.head.row == 0 {
                        break 'game;
                    }
                    self.head.row -= 1;
                }
                Direction::Down => {
                    if self.head.row == self.height - 1 {
                        break 'game;
                    }
                    self.head.row += 1;
                }
                Direction::Left => {
                    if self.head.col == 0 {
                        break 'game;
                    }
                    self.head.col -= 1;
                }
                Direction::Right => {
                    if self.head.col == self.width - 1 {
                        break 'game;
                    }
                    self.head.col += 1;
                }
            }

            // Check what we just ate
            //   - Food => get longer
            //   - Ourselves => die
            if self.board.is_food(self.head) {
                // yum
                self.score += 10;
                self.digesting = 2;
                // Drop 10% on the tick interval
                self.tick_interval_ms *= 9;
                self.tick_interval_ms /= 10;
                if self.tick_interval_ms < 5 {
                    // Maximum speed
                    self.tick_interval_ms = 5;
                }
                // Add random food
                let pos = self.random_empty_position();
                self.board.set_food(pos);
                self.write_at(stdout, pos, 'F');
            } else if self.board.is_body(self.head) {
                // oh no
                break 'game;
            }

            // Write the new head
            self.board.set_dir(self.head, self.direction);
            self.write_at(stdout, self.head, 'U');

            if self.digesting == 0 {
                let old_tail = self.tail;
                match self.board.get_tail_dir(self.tail) {
                    Some(Direction::Up) => {
                        self.tail.row -= 1;
                    }
                    Some(Direction::Down) => {
                        self.tail.row += 1;
                    }
                    Some(Direction::Left) => {
                        self.tail.col -= 1;
                    }
                    Some(Direction::Right) => {
                        self.tail.col += 1;
                    }
                    None => {
                        panic!("Bad game state");
                    }
                }
                self.board.clear(old_tail);
                self.write_at(stdout, old_tail, ' ');
            } else {
                self.digesting -= 1;
            }
        }

        self.score
    }

    fn write_at(&self, console: &mut neotron_sdk::File, position: console::Position, ch: char) {
        let adjusted_position = console::Position {
            row: position.row + self.offset.row,
            col: position.col + self.offset.col,
        };
        console::move_cursor(console, adjusted_position);
        let _ = console.write_char(ch);
    }

    fn random_empty_position(&mut self) -> console::Position {
        loop {
            // This isn't equally distributed. I don't really care.
            let pos = console::Position {
                row: (neotron_sdk::rand() % self.height as u16) as u8,
                col: (neotron_sdk::rand() % self.width as u16) as u8,
            };
            if self.board.is_empty(pos) {
                return pos;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum CellContents {
    Empty,
    Up,
    Down,
    Left,
    Right,
    Food,
}

struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[CellContents; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    const fn new() -> Board<WIDTH, HEIGHT> {
        Board {
            cells: [[CellContents::Empty; WIDTH]; HEIGHT],
        }
    }

    fn reset(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.cells[y][x] = CellContents::Empty;
            }
        }
    }

    fn set_dir(&mut self, position: console::Position, direction: Direction) {
        self.cells[usize::from(position.row)][usize::from(position.col)] = match direction {
            Direction::Up => CellContents::Up,
            Direction::Down => CellContents::Down,
            Direction::Left => CellContents::Left,
            Direction::Right => CellContents::Right,
        }
    }

    fn get_tail_dir(&self, position: console::Position) -> Option<Direction> {
        match self.cells[usize::from(position.row)][usize::from(position.col)] {
            CellContents::Up => Some(Direction::Up),
            CellContents::Down => Some(Direction::Down),
            CellContents::Left => Some(Direction::Left),
            CellContents::Right => Some(Direction::Right),
            _ => None,
        }
    }

    fn set_food(&mut self, position: console::Position) {
        self.cells[usize::from(position.row)][usize::from(position.col)] = CellContents::Food;
    }

    fn is_food(&mut self, position: console::Position) -> bool {
        self.cells[usize::from(position.row)][usize::from(position.col)] == CellContents::Food
    }

    fn is_body(&mut self, position: console::Position) -> bool {
        let cell = self.cells[usize::from(position.row)][usize::from(position.col)];
        cell == CellContents::Up
            || cell == CellContents::Down
            || cell == CellContents::Left
            || cell == CellContents::Right
    }

    fn is_empty(&mut self, position: console::Position) -> bool {
        self.cells[usize::from(position.row)][usize::from(position.col)] == CellContents::Empty
    }

    fn clear(&mut self, position: console::Position) {
        self.cells[usize::from(position.row)][usize::from(position.col)] = CellContents::Empty;
    }
}
