use std::thread::JoinHandle;
use crate::mpsc::Sender;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}

impl Point {
    pub fn get_adjacent_points(&self) -> [Point; 8] {
        [
            Point{row: self.row, col: self.col + 1},
            Point{row: self.row, col: self.col - 1},
            Point{row: self.row + 1, col: self.col},
            Point{row: self.row + 1, col: self.col + 1},
            Point{row: self.row + 1, col: self.col - 1},
            Point{row: self.row - 1, col: self.col},
            Point{row: self.row - 1, col: self.col + 1},
            Point{row: self.row - 1, col: self.col - 1},
        ]
    }
}


#[derive(Clone, Debug)]
enum PointState {
    Alive,
    Dead
}

#[derive(Clone, Debug)]
struct StatefulPoint {
    point: Point,
    state: PointState,
    alive_neighbors: u8
}


impl StatefulPoint {
    pub fn from_point(point: Point, state: PointState) -> Self {
        StatefulPoint {
            point: point,
            state: state,
            alive_neighbors: 0
        }
    }

    pub fn increment_neightbors(self: &mut Self) {
        self.alive_neighbors += 1;
    }
}

impl PartialEq for StatefulPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point.eq(&other.point)
    }
}

impl Eq for StatefulPoint {}

impl PartialOrd for StatefulPoint {
    fn ge(&self, other: &Self) -> bool {
        self.point.ge(&other.point)
    }

    fn le(&self, other: &Self) -> bool {
        self.point.le(&other.point)
    }

    fn gt(&self, other: &Self) -> bool {
        self.point.gt(&other.point)
    }

    fn lt(&self, other: &Self) -> bool {
        self.point.lt(&other.point)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.point.partial_cmp(&other.point)
    }
}

impl Ord for StatefulPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.point.cmp(&other.point)
    }
}

#[derive(Clone, Debug)]
pub struct MySortedSet<T: Ord + Clone> {
    vec: Vec<T>
}

impl<T: Ord + Clone> MySortedSet<T> {

    fn new() -> Self {
        Self {
            vec: vec![]
        }
    }

    // Copies Vector so ownership isnt chaned. not efficient
    pub fn get_vector(self: &Self) -> Vec<T> {
        self.vec.clone()
    }

    pub fn clear(self: &mut Self) {
        self.vec = vec![];
    }
    

    fn binary_search(self: &Self, value: &T) -> Result<usize, usize> {
        self.vec.binary_search(&value) 
    }

    /*Inserts an element to its appropriate place
      If inserted correctly, Ok returned with its index.
      If already exists, Err returned with its index
    */
    fn insert(self: &mut Self, value: T) -> Result<usize, usize> {
        match self.vec.binary_search(&value) {
            // Index already exist, we dont like
            Ok(index) => {Err(index)}

            // Actually thats good, value doesnt exist.
            Err(index) => {
                self.vec.insert(index, value);
                Ok(index)
            }

        }
    }

    fn remove_index(self: &mut Self, index: usize) {
        self.vec.remove(index);
    }

    fn iter_values(self: &Self) -> std::slice::Iter<'_, T>{
        self.vec.iter()
    }
}

impl MySortedSet<StatefulPoint> {
    fn increment_neighbors_at_index(self: &mut Self, index: usize){ 
        self.vec[index].increment_neightbors()
    }
}

pub enum ThreadCommands {
    Kill,
}

pub struct ThreadState<T, U> {
    pub thread_handler: JoinHandle<T>,
    pub tx: Sender<U>, // we send information to the thread, not the otherway around, so no receiver.

}

pub struct CalcutingThreadState<T, U> {
    pub thread_state: Option<ThreadState<T, U>>
}

#[derive(Clone, Debug)]
pub struct GameState{
    pub active_points: MySortedSet<Point>,
}

impl GameState {
    pub fn add_or_remove_point(&mut self, point: Point) {
        match self.active_points.insert(point) {
            Err(index) => {self.active_points.remove_index(index);}
            Ok(_) => {}
        }
    }

    pub fn apply_rules(&mut self) -> Vec<Point>{
        let mut candidate_points: MySortedSet<StatefulPoint> = MySortedSet::new();
        

        // Fuck this
        for active_point in self.active_points.iter_values() {
            let active_stateful_point = StatefulPoint::from_point(active_point.clone(), PointState::Alive);
            let _ = candidate_points.insert(active_stateful_point); // We dont care if this fails
            let adjacent_points = active_point.get_adjacent_points();

            for point in adjacent_points {
                let mut stateful_adjacent_point = match self.active_points.binary_search(&point) {
                    Result::Ok(_) => StatefulPoint::from_point(point, PointState::Alive),
                    Result::Err(_) => StatefulPoint::from_point(point, PointState::Dead)
                };

                stateful_adjacent_point.increment_neightbors();
                match candidate_points.insert(stateful_adjacent_point) {
                    // Point is already there, need to increment the number of neighbors
                    Err(index) => { candidate_points.increment_neighbors_at_index(index) }

                    // Nothing to do here
                    Ok(_) => {}
                }
            }
        }

        // Apply rules after finding the state of each point
        let mut new_points: MySortedSet<Point> = MySortedSet::new();
        let mut changed_points: Vec<Point> = Vec::new();
        for candidate_point in candidate_points.iter_values() {
            let mut is_state_changed: bool = false;
            let is_alive_after_rules: bool = match candidate_point.state {
                PointState::Alive => {
                    let is_alive = 2 <= candidate_point.alive_neighbors && candidate_point.alive_neighbors <= 3;
                    if !is_alive {is_state_changed = true};
                    is_alive
                }
                PointState::Dead => {
                    let is_alive = candidate_point.alive_neighbors == 3;
                    if is_alive {is_state_changed = true}
                    is_alive
                }

            };
            if is_state_changed {
                changed_points.push(candidate_point.point.clone());
            }
            if is_alive_after_rules {
                let _ = new_points.insert(candidate_point.point.clone());
            }
        }
        self.active_points = new_points;

        changed_points
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
              active_points: MySortedSet::new(),
        }
    }
}