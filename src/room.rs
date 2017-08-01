use std::collections::HashMap;

pub struct Room {
    number: u32,
    zone: u32,
    sector_type: SectorType,
    name: String,
    description: String,
    extra: ExtraDescription,
    directions: HashMap<Direction, RoomDirection>,
    dark: bool,
    no_mob: bool,
    indoors: bool,
}

impl Room {
    pub fn build_rooms() -> Vec<Room> {
        Vec::new()
    }
}

struct RoomDirection {
    description: String,        // When look DIR.
    keyword: String,            // for open/close
    is_door: bool,
    closed: bool,
    locked: bool,
    pickproof: bool,
}

struct ExtraDescription {
    keyword: String,
    description: String,
}

enum SectorType {
    Inside,
    City,
    Field,
    Forest,
    Hills,
    Mountains,
    Swimming,
    Unswimmable,
}

enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}
