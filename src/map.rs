use crate::clamper::Clamper;
use crate::rect::Rect;
use crate::{Position, Viewshed};
use rltk::{Algorithm2D, BTerm as Rltk, BaseMap, Point, RGB};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tile {
  Empty,
  Floor,
  Wall,
}

pub struct Map {
  tiles: Vec<Tile>,
  width: i32,
  height: i32,
  start_position: (i32, i32),
  vcw: Clamper<i32>,
  vch: Clamper<i32>,
  revealed_tiles: Vec<bool>,
  visible_tiles: Vec<bool>,
  centers: Vec<(i32, i32)>,
}

impl Map {
  fn xy_index(&self, x: i32, y: i32) -> usize {
    (y * self.width) as usize + x as usize
  }

  fn index_xy(&self, i: usize) -> (i32, i32) {
    (
      (i % self.width as usize) as i32,
      (i / self.width as usize) as i32,
    )
  }

  pub fn new(width: i32, height: i32) -> Self {
    let tile_count = (width * height) as usize;
    let tiles = vec![Tile::Wall; tile_count];
    let mut rng = rltk::RandomNumberGenerator::new();
    let mut centers = Vec::new();
    let roomsize = 5;
    let mut map = Self {
      tiles,
      width,
      height,
      start_position: (0, 0),
      vch: Clamper::new(0, height - 1),
      vcw: Clamper::new(0, width - 1),
      visible_tiles: vec![false; tile_count],
      revealed_tiles: vec![false; tile_count],
      centers,
    };

    for _ in 0..8 {
      let rx1 = rng.roll_dice(1, width - roomsize - 1);
      let ry1 = rng.roll_dice(1, height - roomsize - 1);

      let rect = Rect::new(rx1, ry1, roomsize - 1, roomsize - 1);
      map.centers.push(rect.center());

      for (x, y) in rect.iter() {
        let i = map.xy_index(x, y);
        map.tiles[i] = Tile::Floor;
      }
    }

    map.centers.sort();
    map.start_position = map.centers[0];

    for i in 0..map.centers.len() - 1 {
      let (ax, ay) = map.centers[i];
      let (bx, by) = map.centers[i + 1];
      for x in ax..=bx {
        let i = map.xy_index(x, ay);
        map.tiles[i] = Tile::Floor;
      }
      for y in if by < ay { by..=ay } else { ay..=by } {
        let i = map.xy_index(bx, y);
        map.tiles[i] = Tile::Floor;
      }
    }

    map
  }

  pub fn render(&self, context: &mut Rltk) {
    let grey = RGB::from_f32(0.5, 0.5, 0.5);
    let black = RGB::from_f32(0., 0., 0.);
    let green = RGB::from_f32(0.0, 1.0, 0.0);
    let tiles = &self.tiles;
    for i in 0..tiles.len() {
      if !self.revealed_tiles[i] {
        continue;
      }
      let (x, y) = self.index_xy(i);
      let glyph = match tiles[i] {
        Tile::Empty => ' ',
        Tile::Floor => '.',
        Tile::Wall => '#',
      };
      let fg = if self.visible_tiles[i] { green } else { grey };
      context.set(x, y, fg, black, rltk::to_cp437(glyph));
    }
  }

  pub fn at(&self, x: i32, y: i32) -> Tile {
    self.tiles[self.xy_index(x, y)]
  }

  pub fn clamp(&self, x: i32, y: i32) -> (i32, i32) {
    (self.vcw.clamp(x), self.vch.clamp(y))
  }

  pub fn start_position(&self) -> (i32, i32) {
    self.start_position
  }

  pub fn width(&self) -> i32 {
    self.width
  }

  pub fn height(&self) -> i32 {
    self.height
  }

  pub fn visible_tiles_update(&mut self, viewshed: &Viewshed) {
    for t in self.visible_tiles.iter_mut() {
      *t = false
    }
    for vis in viewshed.visible_tiles.iter() {
      let idx = self.xy_index(vis.x, vis.y);
      self.revealed_tiles[idx] = true;
      self.visible_tiles[idx] = true;
    }
  }

  pub fn centers(&self) -> impl Iterator<Item = &(i32, i32)> {
    self.centers.iter()
  }

  pub fn is_visible(&self, pos: &Position) -> bool {
    self.visible_tiles[self.xy_index(pos.x, pos.y)]
  }
}

impl BaseMap for Map {
  fn is_opaque(&self, idx: usize) -> bool {
    self.tiles[idx as usize] == Tile::Wall
  }
}

impl Algorithm2D for Map {
  fn dimensions(&self) -> Point {
    Point::new(self.width, self.height)
  }
}
