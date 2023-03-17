use crate::clamper::Clamper;
use rltk::{BTerm as Rltk, RGB};

pub const WIDTH: i32 = 80;
pub const WIDTHZ: i32 = WIDTH - 1;
pub const HEIGHT: i32 = 50;
pub const HEIGHTZ: i32 = HEIGHT - 1;
pub const START_X: i32 = 40;
pub const START_Y: i32 = 25;

pub const VIEW_CLAMP_WIDTH: Clamper<i32> = Clamper::new(0, WIDTHZ);
pub const VIEW_CLAMP_HEIGHT: Clamper<i32> = Clamper::new(0, HEIGHTZ);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
  Empty,
  Floor,
  Wall,
}

pub struct Map {
  tiles: Vec<Tile>,
}

impl Map {
  fn xy_index(x: i32, y: i32) -> usize {
    (y * WIDTH + x) as usize
  }

  fn index_xy(i: usize) -> (i32, i32) {
    (i as i32 % WIDTH, i as i32 / WIDTH)
  }

  pub fn new() -> Self {
    let mut tiles = vec![Tile::Empty; (WIDTH * HEIGHT) as usize];
    let mut rng = rltk::RandomNumberGenerator::new();
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        let i = Self::xy_index(x, y);
        tiles[i] = if x == 0 || x == WIDTHZ || y == 0 || y == HEIGHTZ {
          Tile::Wall
        } else if rng.roll_dice(1, 10) == 1 {
          Tile::Wall
        } else {
          Tile::Floor
        }
      }
    }
    tiles[Self::xy_index(START_X, START_Y)] = Tile::Floor;
    Self { tiles }
  }

  pub fn render(&self, context: &mut Rltk) {
    let grey = RGB::from_f32(0.5, 0.5, 0.5);
    let black = RGB::from_f32(0., 0., 0.);
    let green = RGB::from_f32(0.0, 1.0, 0.0);
    let tiles = &self.tiles;
    for i in 0..tiles.len() {
      let (x, y) = Self::index_xy(i);
      let (fg, bg, glyph) = match tiles[i] {
        Tile::Empty => (grey, black, ' '),
        Tile::Floor => (grey, black, '.'),
        Tile::Wall => (green, black, '#'),
      };
      context.set(x, y, fg, bg, rltk::to_cp437(glyph));
    }
  }

  pub fn at(&self, x: i32, y: i32) -> Tile {
    self.tiles[Map::xy_index(x, y)]
  }
}
