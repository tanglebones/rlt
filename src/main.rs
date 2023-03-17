use clamper::Clamper;
use custom_derive::custom_derive;
use newtype_derive::{NewtypeDeref, NewtypeDerefMut, NewtypeFrom};
use rltk::{DiceType, GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs::{Component, VecStorage};

mod clamper;

const WIDTH: i32 = 80;
const WIDTHZ: i32 = WIDTH - 1;
const HEIGHT: i32 = 50;
const HEIGHTZ: i32 = HEIGHT - 1;
const START_X: i32 = 40;
const START_Y: i32 = 25;

const VIEW_CLAMP_WIDTH: Clamper<i32> = Clamper::new(0, WIDTHZ);
const VIEW_CLAMP_HEIGHT: Clamper<i32> = Clamper::new(0, HEIGHTZ);
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
  x: i32,
  y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
  x: i32,
  y: i32,
}

#[derive(Component)]
struct Renderable {
  glyph: rltk::FontCharType,
  fg: RGB,
  bg: RGB,
}

#[derive(Component)]
struct LeftMoverTag {}

struct LeftMoverSystem {}

impl<'a> System<'a> for LeftMoverSystem {
  type SystemData = (ReadStorage<'a, LeftMoverTag>, WriteStorage<'a, Position>);

  fn run(&mut self, (left_mover, mut position): Self::SystemData) {
    for (_left_mover, pos) in (&left_mover, &mut position).join() {
      pos.x -= 1;
      if pos.x < 0 {
        pos.x = WIDTHZ;
      }
    }
  }
}

fn run_systems(world: &mut World) {
  let mut lw = LeftMoverSystem {};
  lw.run_now(&world);
  world.maintain();
}

custom_derive! {
  #[repr(transparent)]
  #[derive(NewtypeFrom, NewtypeDeref, NewtypeDerefMut)]
  struct LocalWorld(World);
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
  Empty,
  Floor,
  Wall,
}

struct Map {
  tiles: Vec<Tile>,
}

impl Map {
  fn xy_index(x: i32, y: i32) -> usize {
    (y * WIDTH + x) as usize
  }

  fn index_xy(i: usize) -> (i32, i32) {
    (i as i32 % WIDTH, i as i32 / WIDTH)
  }

  fn new() -> Self {
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

  fn render(&self, context: &mut Rltk) {
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
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
  let mut positions = world.write_storage::<Position>();
  let mut players = world.write_storage::<Player>();
  let map = world.fetch::<Map>();

  for (_player, pos) in (&mut players, &mut positions).join() {
    let x = VIEW_CLAMP_WIDTH.clamp(pos.x + delta_x);
    let y = VIEW_CLAMP_HEIGHT.clamp(pos.y + delta_y);
    if map.tiles[Map::xy_index(x, y)] == Tile::Floor {
      pos.x = x;
      pos.y = y;
    }
  }
}

fn player_input(world: &mut World, context: &mut Rltk) {
  // Player movement
  let option_virtual_key_code = context.key;
  match option_virtual_key_code {
    None => {} // Nothing happened
    Some(virtual_key_code) => match virtual_key_code {
      VirtualKeyCode::Left | VirtualKeyCode::A => try_move_player(-1, 0, world),
      VirtualKeyCode::Right | VirtualKeyCode::E | VirtualKeyCode::D => try_move_player(1, 0, world),
      VirtualKeyCode::Up | VirtualKeyCode::Comma | VirtualKeyCode::W => {
        try_move_player(0, -1, world)
      }
      VirtualKeyCode::Down | VirtualKeyCode::O | VirtualKeyCode::S => try_move_player(0, 1, world),
      _ => {}
    },
  }
}

impl GameState for LocalWorld {
  fn tick(&mut self, context: &mut Rltk) {
    context.cls();
    player_input(self, context);
    run_systems(self);

    let positions = self.read_storage::<Position>();
    let renderables = self.read_storage::<Renderable>();
    let map = self.fetch::<Map>();
    map.render(context);

    for (pos, render) in (&positions, &renderables).join() {
      context.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
  }
}

fn main() -> rltk::BError {
  let context = RltkBuilder::simple(WIDTH, HEIGHT)?
    .with_title("Roguelike Tutorial")
    .build()?;

  let map = Map::new();
  let mut world = World::new();
  world.register::<Position>();
  world.register::<Player>();
  world.register::<Velocity>();
  world.register::<Renderable>();
  world.register::<LeftMoverTag>();
  world.insert(map);

  world
    .create_entity()
    .with(Position {
      x: START_X,
      y: START_Y,
    })
    .with(Player {})
    .with(Renderable {
      glyph: rltk::to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
    })
    .build();

  for i in 0..10 {
    world
      .create_entity()
      .with(Position { x: i * 7, y: 20 })
      .with(LeftMoverTag {})
      .with(Renderable {
        glyph: rltk::to_cp437('☺'),
        fg: RGB::named(rltk::RED),
        bg: RGB::named(rltk::BLACK),
      })
      .build();
  }

  world
    .create_entity()
    .with(Position { x: 0, y: 0 })
    .with(Velocity { x: 1, y: 0 })
    .build();

  let local_world = LocalWorld::from(world);

  rltk::main_loop(context, local_world)
}
