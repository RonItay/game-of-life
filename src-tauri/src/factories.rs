use crate::game_state::Point;

pub fn get_glider(base_coordinates: Point) -> Vec<Point>{
    let base_row = base_coordinates.row;
    let base_col = base_coordinates.col;
    vec![
        Point{row: base_row, col:base_col},
        Point{row: base_row + 1, col:base_col + 1},
        Point{row: base_row, col:base_col + 2},
        Point{row: base_row + 1, col:base_col + 2},
        Point{row: base_row + 2, col:base_col + 1},
    ]
}

pub fn get_gun(base_coordinates: Point) -> Vec<Point> {
    let base_row = base_coordinates.row;
    let base_col = base_coordinates.col;
    vec![
        Point{row: base_row, col: base_col + 24},
        Point{row: base_row + 1, col: base_col + 22}, Point{row: base_row + 1, col: base_col + 24},
        Point{row: base_row + 2, col: base_col + 12}, Point{row: base_row + 2, col: base_col + 13}, Point{row: base_row + 2, col: base_col + 20}, Point{row: base_row + 2, col: base_col + 21}, Point{row: base_row + 2, col: base_col + 34}, Point{row: base_row + 2, col: base_col + 35},
        Point{row: base_row + 3, col: base_col + 11}, Point{row: base_row + 3, col: base_col + 15}, Point{row: base_row + 3, col: base_col + 20}, Point{row: base_row + 3, col: base_col + 21}, Point{row: base_row + 3, col: base_col + 34}, Point{row: base_row + 3, col: base_col + 35},
        Point{row: base_row + 4, col: base_col + 0}, Point{row: base_row + 4, col: base_col + 1}, Point{row: base_row + 4, col: base_col + 10}, Point{row: base_row + 4, col: base_col + 16}, Point{row: base_row + 4, col: base_col + 20}, Point{row: base_row + 4, col: base_col + 21},
        Point{row: base_row + 5, col: base_col + 0}, Point{row: base_row + 5, col: base_col + 1}, Point{row: base_row + 5, col: base_col + 10}, Point{row: base_row + 5, col: base_col + 14}, Point{row: base_row + 5, col: base_col + 16}, Point{row: base_row + 5, col: base_col + 17}, Point{row: base_row + 5, col: base_col + 22}, Point{row: base_row + 5, col: base_col + 24},
        Point{row: base_row + 6, col: base_col + 10}, Point{row: base_row + 6, col: base_col + 16}, Point{row: base_row + 6, col: base_col + 24},
        Point{row: base_row + 7, col: base_col + 11}, Point{row: base_row + 7, col: base_col + 15},
        Point{row: base_row + 8, col: base_col + 12}, Point{row: base_row + 8, col: base_col + 13},
    ]
}


pub fn get_eater(base_coordinates: Point) -> Vec<Point> {
    let base_row = base_coordinates.row;
    let base_col = base_coordinates.col;

    vec![
        Point{row: base_row, col: base_col}, Point{row: base_row, col: base_col + 1},
        Point{row: base_row + 1, col: base_col}, Point{row: base_row + 1, col: base_col + 2},
        Point{row: base_row + 2, col: base_col + 2},
        Point{row: base_row + 3, col: base_col + 2}, Point{row: base_row + 3, col: base_col + 3},
    ]


}