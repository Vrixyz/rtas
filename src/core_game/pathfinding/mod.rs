use bevy::prelude::*;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(system::setup.system());
    }
}

pub mod pathfinding_comp {
    use std::collections::HashMap;

    use mapgen::TileType;

    pub enum Direction {
        Up = 0,
        Right = 1,
        Down = 2,
        Left = 3,
    }

    pub type Pos = (i32, i32);
    type SearchResult = (HashMap<Pos, Option<Pos>>, HashMap<Pos, f32>);
    #[derive(Default)]
    struct PriorityQueue<T> {
        elements: Vec<(f32, T)>,
    }
    impl<T> PriorityQueue<T> {
        pub fn put(&mut self, new_element: T, priority: f32) {
            for i in 0..self.elements.len() {
                if priority >= self.elements[i].0 {
                    self.elements.insert(i, (priority, new_element));
                    return;
                }
            }
            self.elements.push((priority, new_element));
        }
        /// Returns lowest priority element
        pub fn get(&mut self) -> Option<T> {
            Some(self.elements.pop()?.1)
        }
    }

    pub struct Map {
        pub(super) tiles: Option<Vec<TileType>>,
        pub(super) width: i32,
        pub(super) height: i32,
    }
    impl Map {
        pub(super) fn new(width: u32, height: u32) -> Self {
            Map {
                tiles: Some(vec![TileType::Floor; (width * height) as usize]),
                width: width as i32,
                height: height as i32,
            }
        }
        pub fn is_ready(&self) -> bool {
            self.tiles.is_some()
        }

        fn to(pos: &Pos, dir: &Pos) -> Pos {
            (pos.0 + dir.0, pos.1 + dir.1)
        }

        fn neighbors(&self, pos: &Pos) -> [Pos; 4] {
            const directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut neighbors = [(0, 0); 4];
            let mut i = 0;
            for dir in directions.iter() {
                neighbors[i] = Self::to(pos, dir);
                i += 1;
            }
            neighbors
        }

        fn find_path_rec(&self, path: Vec<Pos>, end: Pos) -> Result<Vec<Pos>, ()> {
            let current_pos = path.last().unwrap();
            for dir in self.neighbors(current_pos).iter() {
                let next_pos = Self::to(current_pos, &dir);
                if path.contains(&next_pos) {
                    continue;
                }
                if next_pos == end {
                    return Ok(path);
                }
                let next_tile = self.get_tile(&next_pos)?;
                if next_tile == TileType::Wall {
                    continue;
                }
                let mut future_path = path.clone();
                future_path.push(next_pos.clone());
                let computed_path = self.find_path_rec(future_path, end);
                if computed_path.is_ok() {
                    return computed_path;
                }
            }
            Err(())
        }

        fn find_path_breadth(&self, start: Pos, end: Pos) -> Result<SearchResult, SearchResult> {
            let size = (self.width * self.height) as usize;

            // frontier needs to be a priorityQueue : https://www.redblobgames.com/pathfinding/a-star/implementation.html#python-dijkstra
            let mut frontier = PriorityQueue::<Pos>::default();
            frontier.put(start, 0f32);

            let mut come_from: HashMap<Pos, Option<Pos>> = HashMap::new(); //vec![None; 0];
            let mut cost_so_far: HashMap<Pos, f32> = HashMap::new();

            come_from.insert(start, None);
            cost_so_far.insert(start, 0f32);

            while let Some(current) = frontier.get() {
                if current == end {
                    return Ok((come_from, cost_so_far));
                }
                for next in self.neighbors(&current).iter() {
                    if !matches!(self.get_tile(next), Ok(TileType::Floor)) {
                        continue;
                    }
                    let new_cost = cost_so_far.get(&current).unwrap() + 1f32;
                    if !cost_so_far.contains_key(next) || new_cost < *cost_so_far.get(next).unwrap()
                    {
                        cost_so_far.insert(*next, new_cost);
                        frontier.put(*next, new_cost);
                        come_from.insert(*next, Some(current));
                    }
                }
            }
            return Err((come_from, cost_so_far));
        }

        fn reconstruct_path(
            came_from: HashMap<Pos, Option<Pos>>,
            start: Pos,
            goal: Pos,
        ) -> Vec<Pos> {
            let mut current = goal;
            let mut path = vec![];
            while current != start {
                path.push(current);
                current = came_from.get(&current).unwrap().unwrap();
            }
            path.push(start);
            path.reverse();
            path
        }

        pub fn dijkstra(&self, start: Pos, end: Pos) -> Result<Vec<Pos>, ()> {
            if let Ok(raw_res) = self.find_path_breadth(start, end) {
                let res = Self::reconstruct_path(raw_res.0, start, end);
                return Ok(res);
            }
            return Err(());
        }

        pub fn find_path(&self, start: Pos, end: Pos) -> Result<Vec<Pos>, ()> {
            // TODO: make a lot of helper functions to move
            let path: Vec<Pos>;
            let current_pos = start;
            let current_tile = self.get_tile(&current_pos);

            return self.find_path_rec(vec![start], end);
        }
        pub fn get_tile(&self, at: &Pos) -> Result<TileType, ()> {
            if at.0 < 0 || at.0 >= self.width as i32 {
                return Err(());
            }
            if at.1 < 0 || at.1 >= self.height as i32 {
                return Err(());
            }
            match self.tiles.as_ref() {
                Some(tiles) => Ok(tiles[(at.0 + at.1 * self.width) as usize]),
                None => Err(()),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn pqueue() {
            let mut pq = PriorityQueue::<i32>::default();
            pq.put(2, 1f32);
            pq.put(3, 1.5f32);
            pq.put(0, 0.5f32);
            assert_eq!(pq.get().unwrap(), 0);
            assert_eq!(pq.get().unwrap(), 2);
            assert_eq!(pq.get().unwrap(), 3);
        }

        #[test]
        fn simple_line() {
            let map = Map::new(1, 5);
            let path = map.dijkstra((0, 0), (0, 4));
            assert_eq!(path, Ok(vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4),]))
        }
        #[test]
        fn empty_map_clockwise() {
            let map = Map::new(5, 5);
            let path = map.dijkstra((1, 1), (3, 3));
            assert_eq!(path, Ok(vec![(1, 1), (1, 2), (1, 3), (2, 3), (3, 3),]))
        }
        #[test]
        fn empty_map_counter_clockwise() {
            let map = Map::new(5, 5);
            let path = map.dijkstra((3, 3), (1, 1));
            assert_eq!(path, Ok(vec![(3, 3), (3, 2), (3, 1), (2, 1), (1, 1),]))
        }
    }
}

mod system {
    use super::pathfinding_comp::Map;
    use super::*;

    pub(super) fn setup(mut commands: Commands, map: Res<crate::core_game::map::Map>) {
        let pathfinding_map = Map {
            tiles: Some(map.map.tiles.clone()),
            width: map.map.width as i32,
            height: map.map.height as i32,
        };
        commands.insert_resource(pathfinding_map);
    }
}
