use std::{sync::RwLock, collections::HashMap};

use hierarchical_pathfinding::prelude::*;
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;

pub type Point = (usize, usize);

const COST_MAP: [isize; 3] = [1, 10, -1];

pub struct Pathfinder{
    width: usize,
    height: usize,
    grid: Vec<Vec<usize>>,
    pathfinding: PathCache<ManhattanNeighborhood>,
}

impl Pathfinder{
    pub fn find(&self, start: Point, goal: Point) -> Result<Option<Vec<Point>>>{
        if start.0 >= self.width || start.1 >= self.height{
            return Err(anyhow!("point out of bounds. map size={}x{} start:{:?}, goal:{:?}", self.width, self.height, start, goal))
        }
        if goal.0 >= self.width || goal.1 >= self.height{
            return Err(anyhow!("point out of bounds. map size={}x{} start:{:?}, goal:{:?}", self.width, self.height, start, goal))
        }
        match self.pathfinding.find_path(start, goal, move |(x, y)| COST_MAP[self.grid[y][x]]){
            Some(path) => Ok(Some(path.collect())),
            None => Ok(None)
        }
    }
}

/// 存放设置的单张地图
static FINDER: Lazy<RwLock<Option<Pathfinder>>> = Lazy::new(|| { RwLock::new(None) });

/// HashMap中存放多个地图
static FINDERS: Lazy<RwLock<HashMap<String, Pathfinder>>> = Lazy::new(|| { RwLock::new(HashMap::new()) });


pub fn new_path_finder(mut grid: Vec<Vec<usize>>) -> Result<Pathfinder>{

    // 大于3改为2防止报错
    grid.iter_mut().for_each(|row|{
        row.iter_mut().for_each(|v|{
            if *v != 0{
                *v = 2;
            }
        });
    });

    let (height, width) = (grid.len(), grid[0].len());
    
    let grid_clone = grid.clone();

    if width == 0 || height == 0 {
        return Err(anyhow!("Error width={} height={}", width, height));
    }

    let pathfinding = PathCache::new(
        (width, height),                           // the size of the Grid
        move |(x, y)| COST_MAP[grid[y][x]],        // get the cost for walking over a Tile
        ManhattanNeighborhood::new(width, height), // the Neighborhood
        PathCacheConfig::with_chunk_size(3),       // config
    );
    Ok(Pathfinder{ width, height, grid: grid_clone, pathfinding })
}

/// 加载单张地图
pub fn load_map(grid: Vec<Vec<usize>>) -> Result<()>{
    if let Ok(mut finder) = FINDER.write(){
        finder.replace(new_path_finder(grid)?);
    }
    Ok(())
}

/// 根据key加载地图
pub fn load_map_for_key<K:Into<String>>(key: K, grid: Vec<Vec<usize>>) -> Result<()>{
    if let Ok(mut finders) = FINDERS.write(){
        finders.insert(key.into(), new_path_finder(grid)?);
    }
    Ok(())
}

/// 查找路径
pub fn find_path(start: Point, goal: Point) -> Result<Option<Vec<Point>>>{
    match FINDER.read(){
        Ok(finder) => {
            match finder.as_ref(){
                Some(finder) => Ok(finder.find(start, goal)?),
                None => Err(anyhow!("call load_map first!"))
            }
        }
        Err(err) => Err(anyhow!("{:?}", err))
    }
}

/// 在指定地图中查找路径
pub fn find_path_in_map(key: &str, start: Point, goal: Point) -> Result<Option<Vec<Point>>>{
    match FINDERS.read(){
        Ok(finders) => {
            match finders.get(key){
                Some(finder) => Ok(finder.find(start, goal)?),
                None => Err(anyhow!("call load_map_for_key first!"))
            }
        }
        Err(err) => Err(anyhow!("{:?}", err))
    }
}

#[test]
fn test_find() -> Result<()>{
    let grid = vec![
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 3, 3, 3],
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 0, 3, 0],
        vec![0, 0, 0, 3, 0],
    ];

    load_map(grid)?;

    let points = find_path((0, 0), (4, 4))?.unwrap();

    println!("{:?}", points);
    
    assert_eq!(
         points,
         vec![(0, 1),  (0, 2),  (0, 3),  (0, 4),  (1, 4),  (2, 4),
             (2, 3),  (2, 2),  (3, 2),  (4, 2),  (4, 3),  (4, 4)],
    );

    let grid = vec![
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 3, 3, 3],
        vec![0, 3, 0, 0, 0],
        vec![0, 3, 0, 3, 0],
        vec![0, 0, 0, 3, 0],
    ];
    
    load_map_for_key("map1", grid)?;
    
    let points = find_path_in_map("map1", (0, 0), (0, 0))?;

    println!("{:?}", points);

    assert!(points.is_some());

    Ok(())
}