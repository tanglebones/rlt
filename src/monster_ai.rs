use super::{Monster, Position, Viewshed};
use crate::PlayerPosition;
use rltk::Point;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    ReadExpect<'a, PlayerPosition>,
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Monster>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (player, viewshed, pos, monster) = data;
    let p = &*player;
    let x = p.x;
    let y = p.y;
    let ppos = Point { x, y };
    for (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join() {
      eprintln!("{} @ {:?} player @ {:?}", viewshed.tag, pos, p);
      if viewshed.visible_tiles.contains(&ppos) {
        eprintln!("{} Monster considers killing the player", viewshed.tag);
      }
    }
  }
}
