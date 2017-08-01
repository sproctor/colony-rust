use std::collections::HashMap;

pub struct Room {
    number: u32,
    zone: u32,
    sector_type: SectorType,
    name: String,
    description: String,
    extra: Vec<ExtraDescription>,
    directions: HashMap<Direction, RoomDirection>,
    dark: bool,
    no_mob: bool,
    indoors: bool,
}

impl Room {
    pub fn build_rooms() -> Vec<Room> {
        let mut rooms = Vec::new();
        let room1 = Room {
            number: 1,
            zone: 1,
            sector_type: SectorType::Inside,
            name: String::from("Starting room"),
            description: String::from("This is the first room created"),
            extra: Vec::new(),
            directions: HashMap::new(),
            dark: false,
            no_mob: true,
            indoors: true,
        };
        rooms.push(room1);
        rooms
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

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}
