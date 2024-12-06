use std::fs;

#[derive(Clone, Debug)]
enum Block {
    CurPos,
    Block,
    Empty,
    Visited,
}

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn turn_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}

struct PuzzleMap {
    pub rows: Vec<Vec<Block>>,
}

impl PuzzleMap {
    fn travel(&mut self, guard: &mut Guard) {
        let mut new_visits = 0;
        let max_row_idx = self.rows[0].len() - 1;
        let max_vert_idx = self.rows.len() - 1;
        loop {
            let (new_hor, new_vert) = match guard.direction {
                Direction::Left => (guard.pos_hor - 1, guard.pos_vert),
                Direction::Up => (guard.pos_hor, guard.pos_vert - 1),
                Direction::Right => (guard.pos_hor + 1, guard.pos_vert),
                Direction::Down => (guard.pos_hor, guard.pos_vert + 1),
            };
            match &self.rows[new_vert][new_hor] {
                Block::CurPos => panic!("IMPOSSIBLE!"),
                Block::Block => {
                    guard.direction = turn_direction(guard.direction.clone());
                }
                Block::Empty => {
                    new_visits += 1;
                    self.rows[new_vert][new_hor] = Block::CurPos;
                    self.rows[guard.pos_vert][guard.pos_hor] = Block::Visited;
                    guard.pos_hor = new_hor;
                    guard.pos_vert = new_vert;
                }
                Block::Visited => {
                    self.rows[new_vert][new_hor] = Block::CurPos;
                    self.rows[guard.pos_vert][guard.pos_hor] = Block::Visited;
                    guard.pos_hor = new_hor;
                    guard.pos_vert = new_vert;
                }
            };
            if guard.pos_hor == max_row_idx
                || guard.pos_vert == max_vert_idx
                || guard.pos_vert == 0
                || guard.pos_hor == 0
            {
                new_visits += 1;
                println!("STEPS: {}", new_visits);
                println!(
                    "STOPPED AT X,Y coords: {} , {}",
                    guard.pos_hor, guard.pos_vert
                );
                break;
            };
        }
    }
}

#[derive(Clone, Debug)]
struct Guard {
    pub pos_hor: usize,
    pub pos_vert: usize,
    pub direction: Direction,
}

fn get_direction(x: &char) -> Direction {
    match x {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => {
            panic!("Invalid input");
        }
    }
}

fn get_block(x: &char) -> Block {
    match x {
        '.' => Block::Empty,
        '#' => Block::Block,
        '^' => Block::CurPos,
        '>' => Block::CurPos,
        'v' => Block::CurPos,
        '<' => Block::CurPos,
        'X' => Block::Visited,
        _ => {
            panic!("Invalid input");
        }
    }
}

fn main() {
    let current_dir = Direction::Up;
    let mut guard = Guard {
        pos_hor: 0,
        pos_vert: 0,
        direction: current_dir,
    };
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let rows: Vec<Vec<Block>> = lines
        .enumerate()
        .map(|(idx_y, y)| {
            y.chars()
                .enumerate()
                .map(|(idx_x, x)| match x {
                    '^' | '>' | 'v' | '<' => {
                        guard.pos_hor = idx_x;
                        guard.pos_vert = idx_y;
                        guard.direction = get_direction(&x);
                        get_block(&x)
                    }
                    '.' => Block::Empty,
                    '#' => Block::Block,
                    'X' => Block::Visited,
                    _ => {
                        panic!("Invalid input");
                    }
                })
                .collect()
        })
        .collect();
    let mut puzzle_map = PuzzleMap { rows };
    puzzle_map.travel(&mut guard);
}
