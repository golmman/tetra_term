pub mod constants;
pub mod init;
pub mod random;
pub mod tetromino;
pub mod well;

use term2d::model::rgba::Rgba;
use term2d::App;

use self::constants::WELL_LEFT;
use self::constants::WELL_TOP;
use self::random::Random;
use self::tetromino::Tetromino;
use self::well::Well;

pub struct Model {
    pub clear_at_frame_count: u64,
    pub debug: i32,
    pub game_over: bool,
    pub gravity: u64,
    pub help: bool,
    pub pause: bool,
    pub random: Random,
    pub score: u32,
    pub tetromino: Tetromino,
    pub tetromino_next: Tetromino,
    pub well: Well,
}

impl Model {
    pub fn new(_app: &App) -> Self {
        let well = Well::new(10, 20);

        let mut random = Random::new();
        let tetromino = Tetromino::new(&mut random);
        let tetromino_next = Tetromino::new(&mut random);

        Self {
            clear_at_frame_count: 0,
            debug: 0,
            game_over: false,
            gravity: 10,
            help: false,
            pause: false,
            random,
            score: 0,
            tetromino,
            tetromino_next,
            well,
        }
    }

    pub fn reset(&mut self, app: &App) {
        *self = Self::new(app);
    }

    pub fn set_clear_at_frame_count(&mut self, frame_count: u64) {
        self.clear_at_frame_count = frame_count;
    }

    pub fn elapse(&mut self, frame_count: u64) {
        if frame_count % self.gravity == 0 {
            self.move_tetromino_down();
        }
    }

    pub fn is_paused(&self) -> bool {
        self.game_over || self.help || self.pause
    }

    pub fn toggle_help(&mut self) {
        if self.game_over || self.pause {
            return;
        }

        self.help = !self.help;
    }

    pub fn toggle_pause(&mut self) {
        if self.game_over || self.help {
            return;
        }

        self.pause = !self.pause;
    }

    pub fn drop_tetromino(&mut self) {
        if self.is_paused() {
            return;
        }

        while !self.tetromino.move_down(&self.well) {}
        self.move_tetromino_down();
    }

    pub fn rotate_tetromino(&mut self) {
        if self.is_paused() {
            return;
        }

        self.tetromino.rotate(&self.well);
    }

    pub fn move_tetromino_left(&mut self) {
        if self.is_paused() {
            return;
        }

        self.tetromino.move_left(&self.well);
    }

    pub fn move_tetromino_right(&mut self) {
        if self.is_paused() {
            return;
        }

        self.tetromino.move_right(&self.well);
    }

    pub fn move_tetromino_down(&mut self) {
        if self.is_paused() {
            return;
        }

        if !self.tetromino.move_down(&self.well) {
            return;
        }

        // TODO: clean code, outsource?
        // place tetromino in well
        let ps = self.tetromino.get_tetromino_points();
        for pi in 0..4 {
            let p = &ps[pi];
            if p.y < WELL_TOP {
                self.game_over = true;
                return;
            }
            let ci = (self.well.width * (p.y - WELL_TOP) + (p.x - WELL_LEFT)) as usize;

            let well_color = Rgba {
                r: (self.tetromino.colors[pi].r as f32 * 0.66) as u8,
                g: (self.tetromino.colors[pi].g as f32 * 0.66) as u8,
                b: (self.tetromino.colors[pi].b as f32 * 0.66) as u8,
                a: 255,
            };
            self.well.colors[ci] = Some(well_color);
        }

        // delete full rows
        self.score += self.well.delete_full_rows();

        // TODO: replace '10' with fps
        // update gravity
        self.gravity = (10 - self.score as i32 / 10).max(1) as u64;

        // set new tetromino
        self.tetromino = self.tetromino_next.clone();
        self.tetromino_next = Tetromino::new(&mut self.random);
    }
}
