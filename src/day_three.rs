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

impl SpiralNode {
    pub fn new(value: usize, pos: (i32, i32)) -> Self {
        SpiralNode { pos, value }
    }
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

fn populate_spiral_2(max_num: usize) -> usize {
    let mut spiral = HashMap::with_capacity(max_num);

    let mut dir = Direction::Right;
    let mut steps_since_last_turn = 0;
    let mut turned = false;
    let mut gap = 0;

    spiral.insert((0, 0), 1);

    let mut pos = (1, 0);

    loop {
        let value = {
            let mut surrounding = [0; 8];
            let mut counter = 0;
            for dx in -1..2 {
                for dy in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if let Some(v) = spiral.get(&(pos.0 + dx, pos.1 + dy)) {
                        surrounding[counter] = *v;
                    }

                    counter += 1;
                }
            }

            surrounding.iter().sum()
        };

        if value > max_num {
            return value;
        }

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
                    SpiralNode::new(1, (0, 0)),
                    SpiralNode::new(2, (1, 0)),
                    SpiralNode::new(3, (1, 1)),
                ]
            )
        );

        assert_eq!(
            format!("{:?}", populate_spiral(8)),
            format!(
                "{:?}",
                vec![
                    SpiralNode::new(1, (0, 0)),
                    SpiralNode::new(2, (1, 0)),
                    SpiralNode::new(3, (1, 1)),
                    SpiralNode::new(4, (0, 1)),
                    SpiralNode::new(5, (-1, 1)),
                    SpiralNode::new(6, (-1, 0)),
                    SpiralNode::new(7, (-1, -1)),
                    SpiralNode::new(8, (0, -1)),
                ]
            )
        );
    }

    // #[test]
    // fn test_second_spiral() {
    //     let mut one_map = HashMap::new();
    //     one_map.insert((0, 0), 1);
    //     assert_eq!(populate_spiral_2(1).get(&(0, 0)), one_map.get(&(0, 0)));
    //     one_map.insert((1, 0), 1);
    //     assert_eq!(populate_spiral_2(2).get(&(1, 0)), one_map.get(&(1, 0)));
    //     one_map.insert((1, 1), 2);
    //     assert_eq!(populate_spiral_2(3).get(&(1, 1)), one_map.get(&(1, 1)));

    //     one_map.insert((0, 1), 4);
    //     assert_eq!(populate_spiral_2(4).get(&(0, 1)), one_map.get(&(0, 1)));

    //     one_map.insert((-1, 1), 5);
    //     assert_eq!(populate_spiral_2(5).get(&(-1, 1)), one_map.get(&(-1, 1)));
    // }

    #[test]
    fn get_first_solution() {
        let spiral = populate_spiral(277678);
        let node = spiral.last().unwrap();
        assert_eq!(475, node.pos.0.abs() + node.pos.1.abs());
    }

    #[test]
    fn get_second_solution() {
        let answer = populate_spiral_2(277678);
        assert_eq!(279138, answer);
    }
}
