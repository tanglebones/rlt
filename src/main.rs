use custom_derive::custom_derive;
use map::{Map, Tile};
use newtype_derive::{NewtypeDeref, NewtypeDerefMut, NewtypeFrom};
use rltk::{GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs::{Component, VecStorage};
use visibility_system::VisibilitySystem;

mod clamper;
mod map;
mod rect;
mod visibility_system;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Component)]
struct Renderable {
  glyph: rltk::FontCharType,
  fg: RGB,
  bg: RGB,
}

#[derive(Component)]
pub struct Viewshed {
  pub visible_tiles: Vec<rltk::Point>,
  pub range: i32,
  pub dirty: bool,
}

fn run_systems(world: &mut World) {
  let mut vis = VisibilitySystem {};
  vis.run_now(&world);
  world.maintain();
}

custom_derive! {
  #[repr(transparent)]
  #[derive(NewtypeFrom, NewtypeDeref, NewtypeDerefMut)]
  struct LocalWorld(World);
}

#[derive(Component, Debug)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
  let mut position = world.write_storage::<Position>();
  let mut player = world.write_storage::<Player>();
  let mut viewshed = world.write_storage::<Viewshed>();
  let map = world.fetch::<Map>();

  for (_player, pos, viewshed) in (&mut player, &mut position, &mut viewshed).join() {
    let (x, y) = map.clamp(pos.x + delta_x, pos.y + delta_y);
    if map.at(x, y) == Tile::Floor {
      pos.x = x;
      pos.y = y;
      viewshed.dirty = true;
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
const WIDTH: i32 = 120;
const HEIGHT: i32 = 120;
fn main() -> rltk::BError {
  let context = RltkBuilder::simple(WIDTH, HEIGHT)?
    .with_title("Roguelike Tutorial")
    .build()?;

  let map = Map::new(WIDTH, HEIGHT);
  let (start_x, start_y) = map.start_position();
  let mut world = World::new();
  world.register::<Position>();
  world.register::<Player>();
  world.register::<Renderable>();
  world.register::<Viewshed>();
  world.insert(map);

  world
    .create_entity()
    .with(Position {
      x: start_x,
      y: start_y,
    })
    .with(Player {})
    .with(Viewshed {
      visible_tiles: Vec::new(),
      range: 8,
      dirty: true,
    })
    .with(Renderable {
      glyph: rltk::to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
    })
    .build();

  let local_world = LocalWorld::from(world);

  rltk::main_loop(context, local_world)
}
