use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct SpiralNode {
    pos: (i32, i32),
    value: usize,
}

fn move_pos(pos: (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

/*
2 -> turn, move
3 -> turn, move
4 -> ----, move
5 -> turn, move
6 -> ----, move
7 -> turn, move
8 -> ----, move
9 -> ----, move
10-> turn, move
11-> ----, move
12-> ----, move
13-> turn, move
14-> ----, move
15-> ----, move
16-> ----, move
17-> turn, move
18-> ----, move
19-> ----, move
20-> ----, move
21-> turn, move
 */

fn populate_spiral(max_num: usize) -> Vec<SpiralNode> {
    let mut spiral: Vec<SpiralNode> = Vec::with_capacity(max_num);

    let mut dir = Direction::Right;
    let mut steps_since_last_turn = 0;
    let mut turned = false;
    let mut gap = 0;

    spiral.push(SpiralNode {
        value: 1,
        pos: (0, 0),
    });

    let mut pos = (1, 0);

    for n in 1..max_num {
        let value = n + 1;

        spiral.push(SpiralNode { value, pos });
        steps_since_last_turn += 1;

        if steps_since_last_turn > gap {
            dir = dir.turn();
            steps_since_last_turn = 0;
            if turned {
                turned = false;
                gap += 1;
            } else {
                turned = true;
            }
        }

        pos = move_pos(pos, &dir);
    }

    spiral
}

fn populate_spiral_2(max_num: usize) -> HashMap<(i32, i32), usize> {
    let mut spiral = HashMap::with_capacity(max_num);

    let mut dir = Direction::Right;
    let mut steps_since_last_turn = 0;
    let mut turned = false;
    let mut gap = 0;

    spiral.insert((0, 0), 1);

    let mut pos = (1, 0);

    for _n in 1..max_num {
        let value = {
            let mut surrounding = Vec::with_capacity(8);
            for dx in -1..2 {
                for dy in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if let Some(v) = spiral.get(&(pos.0 + dx, pos.1 + dy)) {
                        surrounding.push(*v);
                    }
                }
            }

            surrounding.iter().sum()
        };

        spiral.insert(pos, value);

        steps_since_last_turn += 1;

        if steps_since_last_turn > gap {
            dir = dir.turn();
            steps_since_last_turn = 0;
            if turned {
                turned = false;
                gap += 1;
            } else {
                turned = true;
            }
        }

        pos = move_pos(pos, &dir);
    }

    spiral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_spiral() {
        assert_eq!(
            format!("{:?}", populate_spiral(1)),
            format!(
                "{:?}",
                vec![SpiralNode {
                    value: 1,
                    pos: (0, 0)
                }]
            )
        );

        assert_eq!(
            format!("{:?}", populate_spiral(3)),
            format!(
                "{:?}",
                vec![
                    SpiralNode {
                        value: 1,
                        pos: (0, 0)
                    },
                    SpiralNode {
                        value: 2,
                        pos: (1, 0)
                    },
                    SpiralNode {
                        value: 3,
                        pos: (1, 1)
                    }
                ]
            )
        );

        assert_eq!(
            format!("{:?}", populate_spiral(8)),
            format!(
                "{:?}",
                vec![
                    SpiralNode {
                        value: 1,
                        pos: (0, 0)
                    },
                    SpiralNode {
                        value: 2,
                        pos: (1, 0)
                    },
                    SpiralNode {
                        value: 3,
                        pos: (1, 1)
                    },
                    SpiralNode {
                        value: 4,
                        pos: (0, 1)
                    },
                    SpiralNode {
                        value: 5,
                        pos: (-1, 1)
                    },
                    SpiralNode {
                        value: 6,
                        pos: (-1, 0)
                    },
                    SpiralNode {
                        value: 7,
                        pos: (-1, -1)
                    },
                    SpiralNode {
                        value: 8,
                        pos: (0, -1)
                    },
                ]
            )
        );
    }
    #[test]
    fn get_first_solution() {
        let spiral = populate_spiral(277678);
        let node = spiral.last().unwrap();
        assert_eq!(475, node.pos.0.abs() + node.pos.1.abs());
    }
}
