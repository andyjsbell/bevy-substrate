use std::char::MAX;

use rand::Rng;
pub const GRID_SIZE: usize = 32;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, 
    Floor,
}

#[derive(Clone, Default)]
pub struct Map {
    pub rows: usize,
    pub columns: usize,
    pub tiles: Vec<TileType>,
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub struct Dimension(pub usize);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position(pub Dimension, pub Dimension);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size(pub Dimension, pub Dimension);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle(pub Position, pub Size);

impl Rectangle {
    // Rect{x1:x, y1:y, x2:x+w, y2:y+h}
    // self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    pub fn intersect(&self, other:&Rectangle) -> bool {
        self.0.0 <= other.0.0 + other.1.0 &&
        self.0.0 + self.1.0 >= other.0.0 &&
        self.0.1 <= other.0.1 + other.1.1 &&
        self.0.1 + self.1.1 >= other.0.1
    }

    pub fn center(&self) -> Position {
        Position((
            self.0.0 + self.1.0) / 2.into(), 
            (self.0.1 + self.1.1) / 2.into()
        )
    } 
}

impl std::ops::Mul for Dimension {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Dimension(self.0 * rhs.0)
    }
}

impl std::ops::Add for Dimension {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Dimension(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Dimension {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.0 > self.0 {
            Dimension(rhs.0 - self.0)
        } else {
            Dimension(self.0 - rhs.0)
        }
    }
}

impl std::ops::Div for Dimension {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Dimension(self.0 / rhs.0)
    }
}

impl From<usize> for Dimension {
    fn from(item: usize) -> Self {
        Dimension(item)
    }
}

impl From<Dimension> for usize {
    fn from(dimension: Dimension) -> Self {
        dimension.0
    }
}

/// Coordinate x, y
pub struct Coordinate(pub f32, pub f32);

impl Map { 
    pub fn grid_to_index(&self, position: Position) -> usize {
        (position.1.0 * self.columns) + position.0.0
    }
}

pub fn grid_to_coordinates(position: Position) -> Coordinate {
    Coordinate(
        (position.0.0 * GRID_SIZE) as f32,
        (position.1.0 * GRID_SIZE) as f32
    ) 
}

pub fn coordinate_to_grid(coordinate: Coordinate) -> Position {
    Position(
        Dimension(coordinate.0 as usize / GRID_SIZE),
        Dimension(coordinate.1 as usize / GRID_SIZE)
    )
}

/// Create a simple map width, height and number of random blocks
pub fn create_simple_map(width: usize, height: usize, blocks: usize, player: (usize, usize)) -> Result<Map, String> {
    if width % GRID_SIZE != 0 || height % GRID_SIZE != 0 {
        return Err("Invalid dimensions, we need to be divisable by 32".to_string());
    }

    let columns  = width / GRID_SIZE;
    let rows = height / GRID_SIZE;
    // Add borders, row 0, row N, column 0, column N
    let mut tiles = vec![TileType::Floor; columns * rows];
    // Left and right border
    for i in 0..rows {
        tiles[i * columns] = TileType::Wall;
        tiles[(i * columns) + (columns - 1)] = TileType::Wall;
    }
    // Top and bottom border
    for i in 0..columns {
        tiles[i] = TileType::Wall;
        tiles[(rows * columns) - (i + 1)] = TileType::Wall;
    }

    // Generate random blocks
    let mut rng = rand::thread_rng();
    for _ in 0..blocks {
        let column = rng.gen_range(1..columns - 1);
        let row = rng.gen_range(1..rows - 1);
        if column != player.0 || row != player.1 {
            tiles[(columns * row) + column] = TileType::Wall;
        }
    }
    
    let map = Map {
        rows,
        columns,
        tiles
    };

    Ok(map)
}

fn add_room_to_map(room: Rectangle, map: &mut Map) {
    let rows = room.1.0.0;
    let cols = room.1.1.0;
    let pos = room.0;
    for row in 0..rows {
        let start_idx = map.grid_to_index(Position(pos.0, pos.1 + Dimension(row)));
        for idx in start_idx..(start_idx + cols) {
            map.tiles[idx] = TileType::Floor;
        }
    }
}

fn add_tunnel(orientation: Orientation, position: &Position, length: Dimension, map: &mut Map) {
    match orientation {
        Orientation::Horizontal => {
            for col in 0..length.0 {
                let idx = map.grid_to_index(Position(position.0 + Dimension(col), position.1));
                map.tiles[idx] = TileType::Floor;
            }
        },
        Orientation::Vertical => {
            for row in 0..length.0 {
                let idx = map.grid_to_index(Position(position.0, position.1 + Dimension(row)));
                map.tiles[idx] = TileType::Floor;
            }
        }
    }
}

/// Return a random room 
pub fn create_room(min: Dimension, max: Dimension, size: Size) -> Rectangle {
    let mut rng = rand::thread_rng();
    let width = rng.gen_range(min.0..max.0);
    let height = rng.gen_range(min.0..max.0);
    let columns : usize = size.0.into();
    let rows : usize = size.1.into();
    let x = rng.gen_range(1..(columns - width - 1));
    let y = rng.gen_range(1..(rows - height - 1));
    
    Rectangle(
        Position(
            Dimension(x),
            Dimension(y)
        ),
        Size(width.into(), height.into())
    )
}

pub fn create_map(width: usize, height: usize, player: (usize, usize)) -> Result<Map, String> {
    if width % GRID_SIZE != 0 || height % GRID_SIZE != 0 {
        return Err("Invalid dimensions, we need to be divisable by 32".to_string());
    }

    let columns  = width / GRID_SIZE;
    let rows = height / GRID_SIZE;
    let tiles = vec![TileType::Wall; columns * rows];
    let mut map = Map {
        rows,
        columns,
        tiles
    };

    const MAX_ROOMS : i32 = 100;
    let mut rooms: Vec<Rectangle> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..MAX_ROOMS {
        let new_room = create_room(3.into(), 6.into(), Size(columns.into(), rows.into()));
        let mut ok = true;
        for existing in rooms.iter() {
            if new_room.intersect(&existing) {
                ok = false;
            }
        }
        if ok {
            println!("adding room {:?}", new_room);
            add_room_to_map(new_room, &mut map);
            
            if !rooms.is_empty() {
                let new_position = new_room.center();
                let prev_position = rooms[rooms.len()-1].center();
                // if rng.gen_range(0..2) == 1 {
                //     add_tunnel(Orientation::Horizontal, &new_position, new_position.0 - prev_position.0, &mut map);
                //     add_tunnel(Orientation::Vertical, &new_position, new_position.1 - prev_position.1, &mut map);
                // } else {
                //     add_tunnel(Orientation::Vertical, &new_position, new_position.1 - prev_position.1, &mut map);
                //     add_tunnel(Orientation::Horizontal, &new_position, new_position.0 - prev_position.0, &mut map);
                // }
            }

            rooms.push(new_room);
        }
    }
    
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_to_grid() {
        let coord = Coordinate(100.0, 100.0);
        let grid = coordinate_to_grid(coord);
        assert!(grid.0 == Dimension(3) && grid.1 == Dimension(3));
    }

    #[test]
    fn test_grid_to_coordinate() {
        let grid = Position(Dimension(7), Dimension(7));
        let coord = grid_to_coordinates(grid);
        assert!(coord.0 == 224.0 && coord.1 == 224.0);
    }

    #[test]
    fn test_new_room() {
        let room = create_room(0.into(), 4.into(), Size(10.into(), 10.into()));
        assert!(room.0.0 < 10.into());
        assert!(room.0.1 < 10.into());
        assert!(room.1.0 <= 4.into());
        assert!(room.1.1 <= 4.into());
    }

    #[test]
    fn test_intersect() {
        let r1 = Rectangle(Position(0.into(), 0.into()), Size(2.into(), 2.into()));
        let r2 = Rectangle(Position(1.into(), 1.into()), Size(2.into(), 2.into()));
        let r3 = Rectangle(Position(3.into(), 3.into()), Size(4.into(), 4.into()));
        
        assert!(r1.intersect(&r2));
        assert!(!r1.intersect(&r3));
    }
}