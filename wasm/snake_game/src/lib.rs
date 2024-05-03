use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(name);
// }

// #[wasm_bindgen]
// extern {
//     pub fn alert(s: &str);
// }

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone, Copy)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}


#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
}


#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, 3),
            next_cell: None,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 { return; }
        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // cannot return a reference to JS because of borring rules
    // pub fn snake_cells(&self) -> &Vec<SnakeCell> {
    //     &self.snake.body
    // }

    // *const is raw pointer
    // borrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }



    // pub fn update(&mut self) {
    //     let snake_idx = self.snake_head_idx();

    //     let (row, col) = self.index_to_cell(snake_idx);
    //     let (row, col) = match self.snake.direction {
    //         Direction::Right => {
    //             (row, (col + 1) % self.width)
    //         },
    //         Direction::Left => {
    //             (row, (col - 1) % self.width)
    //         },
    //         Direction::Up => {
    //             ((row - 1) % self.width, col)
    //         },
    //         Direction::Down => {
    //             ((row +1) % self.width, col)
    //         },
    //     };

    //     let next_idx = self.cell_to_index(row, col);
    //     self.set_snake_head(next_idx);
    // }

    // fn set_snake_head(&mut self, idx: usize) {
    //     self.snake.body[0].0 = idx;
    // }

    // fn index_to_cell(&self, idx: usize) -> (usize, usize) {
    //     (idx / self.width, idx % self.width)
    // }

    // fn cell_to_index(&self, row: usize, col: usize) -> usize {
    //     (row * self.width) + col
    // }

    pub fn step(&mut self) {
        let temp = self.snake.body.clone();

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            },
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }

        // let next_cell = self.gen_next_snake_cell(&self.snake.direction);
        // self.snake.body[0] = next_cell;

        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
            Direction::Left => {
                let threshold = row * self.width;
                if snake_idx == threshold {
                    SnakeCell(threshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            },
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Direction::Down => {
                let threshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            },
        };
    }
}
