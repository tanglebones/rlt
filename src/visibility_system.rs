use super::{Map, Position, Viewshed};
use crate::Player;
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
  type SystemData = (
    WriteExpect<'a, Map>,
    Entities<'a>,
    WriteStorage<'a, Viewshed>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Player>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, entities, mut viewshed, pos, player) = data;
    for (entity, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
      if !viewshed.dirty {
        return;
      }
      viewshed.visible_tiles.clear();
      viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
      let width = map.width();
      let height = map.height();
      viewshed
        .visible_tiles
        .retain(|p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height);
      let option_player: Option<&Player> = player.get(entity);
      if let Some(_player) = option_player {
        map.visible_tiles_update(viewshed);
      } else {
        eprintln!("here?");
      }
      viewshed.dirty = false;
    }
  }
}
