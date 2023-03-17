use custom_derive::custom_derive;
use map::{Map, Tile, HEIGHT, START_X, START_Y, VIEW_CLAMP_HEIGHT, VIEW_CLAMP_WIDTH, WIDTH};
use newtype_derive::{NewtypeDeref, NewtypeDerefMut, NewtypeFrom};
use rltk::{GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs::{Component, VecStorage};

mod clamper;
mod map;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
  x: i32,
  y: i32,
}

#[derive(Component)]
struct Renderable {
  glyph: rltk::FontCharType,
  fg: RGB,
  bg: RGB,
}

fn run_systems(world: &mut World) {
  world.maintain();
}

custom_derive! {
  #[repr(transparent)]
  #[derive(NewtypeFrom, NewtypeDeref, NewtypeDerefMut)]
  struct LocalWorld(World);
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
    if map.at(x, y) == Tile::Floor {
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
  world.register::<Renderable>();
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

  let local_world = LocalWorld::from(world);

  rltk::main_loop(context, local_world)
}
