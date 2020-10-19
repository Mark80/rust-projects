use specs::{
    Builder, Component, join::Join, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[storage(VecStorage)]
#[derive(Component)]
pub struct Renderable {
    pub path: String
}

#[storage(VecStorage)]
#[derive(Component)]
pub struct Wall {}

#[storage(VecStorage)]
#[derive(Component)]
pub struct Box {}

#[storage(VecStorage)]
#[derive(Component)]
pub struct Player {}

#[storage(VecStorage)]
#[derive(Component)]
pub struct BoxSpot {}


pub fn create_wall(world: &mut World, position: Position) {
    world.create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable { path: "/images/wall.png".to_string() })
        .with(Wall {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world.create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable { path: "/images/box_spot.png".to_string() })
        .with(BoxSpot {})
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world.create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable { path: "/images/box.png".to_string() })
        .with(Box {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world.create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable { path: "/images/floor.png".to_string() })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world.create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable { path: "/images/player.png".to_string() })
        .with(Player {})
        .build();
}


