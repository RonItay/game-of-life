use std::sync::Mutex;

use crate::game_state::{GameState, Point};
use crate::factories::{get_glider, get_gun};

#[tauri::command]
pub async fn step(state_locked: tauri::State<'_, Mutex<GameState>>) -> Result<String, String> {
  let mut state = state_locked.lock().unwrap();
  let changed_points = state.apply_rules();
  Ok(serde_json::to_string(&changed_points).unwrap())
}


#[tauri::command]
pub async fn clear(state_locked: tauri::State<'_, Mutex<GameState>>) -> Result<String, String> {
  let mut state = state_locked.lock().unwrap();
  let current_active = state.active_points.get_vector();
  state.active_points.clear();
  Ok(serde_json::to_string(&current_active).unwrap())
}


#[tauri::command]
pub async fn select(state_locked: tauri::State<'_, Mutex<GameState>>, object: String) -> Result<String, String> {
  let mut state = state_locked.lock().unwrap();
  let mut points: Vec<Point> = vec![];
  if (object == "gun") {
    points = get_gun(Point{row: 0, col: 0});
  } else if object == "glider" {
    points = get_glider(Point{row: 0, col: 0});
  }

  for point in &points {
    state.add_or_remove_point(point.clone());
  }
  Ok(serde_json::to_string(&points).unwrap())
}
