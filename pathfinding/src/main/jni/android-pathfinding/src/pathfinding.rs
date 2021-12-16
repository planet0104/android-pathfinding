use anyhow::{anyhow, Result};
use bracket_pathfinding::prelude::{
    a_star_search, Algorithm2D, BaseMap, DistanceAlg, Point, SmallVec,
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::RwLock};

// Map

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,
}

impl Map {
    pub fn new(grid: Vec<Vec<u8>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        let mut tiles = vec![];

        for line in grid {
            for t in line {
                tiles.push(t);
            }
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn a_star_search(&self, start: Point, end: Point) -> Option<Vec<Point>> {
        let path = a_star_search(
            self.point2d_to_index(start),
            self.point2d_to_index(end),
            self,
        );
        if path.success {
            let steps = path.steps;
            let mut points = Vec::with_capacity(steps.len());
            for step in steps {
                let pt = self.index_to_point2d(step);
                points.push(pt);
            }
            Some(points)
        } else {
            None
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            let idx = self.point2d_to_index(destination);
            if self.tiles[idx] == 0 {
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != 0
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

/// 存放设置的单张地图
static MAP: Lazy<RwLock<Option<Map>>> = Lazy::new(|| RwLock::new(None));

/// HashMap中存放多个地图
static MAPS: Lazy<RwLock<HashMap<String, Map>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// 加载单张地图
pub fn load_map(grid: Vec<Vec<u8>>) {
    if let Ok(mut finder) = MAP.write() {
        finder.replace(Map::new(grid));
    }
}

/// 根据key加载地图
pub fn load_map_for_key<K: Into<String>>(key: K, grid: Vec<Vec<u8>>) {
    if let Ok(mut finders) = MAPS.write() {
        finders.insert(key.into(), Map::new(grid));
    }
}

/// 查找路径
pub fn find_path(start: Point, goal: Point) -> Result<Option<Vec<Point>>> {
    match MAP.read() {
        Ok(map) => match map.as_ref() {
            Some(map) => Ok(map.a_star_search(start, goal)),
            None => Err(anyhow!("call load_map first!")),
        },
        Err(err) => Err(anyhow!("{:?}", err)),
    }
}

/// 在指定地图中查找路径
pub fn find_path_in_map(key: &str, start: Point, goal: Point) -> Result<Option<Vec<Point>>> {
    match MAPS.read() {
        Ok(maps) => match maps.get(key) {
            Some(map) => Ok(map.a_star_search(start, goal)),
            None => Err(anyhow!("call load_map_for_key first!")),
        },
        Err(err) => Err(anyhow!("{:?}", err)),
    }
}

#[test]
fn test_find() -> Result<()> {
    let grid = vec![
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 3, 3, 3],
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 0, 3, 0],
        vec![0, 0, 0, 3, 0],
    ];

    load_map(grid);

    let points = find_path(Point::new(0, 0), Point::new(4, 4))?.unwrap();

    println!("{:?}", points);

    assert_eq!(
        points,
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
            Point::new(1, 4),
            Point::new(2, 3),
            Point::new(3, 2),
            Point::new(4, 3),
            Point::new(4, 4)
        ],
    );

    let grid = vec![
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 3, 3, 3],
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 0, 3, 0],
        vec![0, 0, 0, 3, 0],
    ];

    load_map_for_key("map1", grid);

    let points = find_path_in_map("map1", Point::new(0, 0), Point::new(0, 0))?;

    println!("{:?}", points);

    assert!(points.is_some());

    Ok(())
}
