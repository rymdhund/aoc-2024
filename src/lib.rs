pub mod coord {
    use std::ops::{Add, Sub, Mul};
    use std::slice::Iter;

    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Coord {
        pub x: i64,
        pub y: i64,
    }

    impl Coord {
        pub fn new(x: i64, y: i64) -> Self {
            Coord {x, y}
        }

        pub fn new_u(x: usize, y: usize) -> Self {
            Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap()}
        }

        pub fn up() -> Coord {
            Coord::new(0, -1)
        }

        pub fn right() -> Coord {
            Coord::new(1, 0)
        }

        pub fn down() -> Coord {
            Coord::new(0, 1)
        }

        pub fn left() -> Coord {
            Coord::new(-1, 0)
        }

        pub fn dir(d: Dir) -> Coord {
            match d {
                Dir::Right => Self::right(),
                Dir::Down => Self::down(),
                Dir::Left => Self::left(),
                Dir::Up => Self::up(),
            }
        }

        pub fn x_u(&self) -> usize {
            self.x.try_into().unwrap()
        }

        pub fn y_u(&self) -> usize {
            self.y.try_into().unwrap()
        }

        pub fn manhattan(&self) -> usize {
            (self.x.abs() + self.y.abs()).try_into().unwrap()
        }

        pub fn neighbours(&self) -> Vec<Coord> {
            (0..4).map(|d| {
                *self + Coord::dir(Dir::of_id(d))
            }).collect_vec()
        }
        
        pub fn go(&self, d: Dir) -> Coord {
            *self + Coord::dir(d)
        }
    }

    impl Add for Coord {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Coord::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub for Coord {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Coord::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Mul<i64> for Coord {
        type Output = Self;

        fn mul(self, rhs: i64) -> Self::Output {
            Coord::new(self.x * rhs, self.y * rhs)
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    pub enum Dir {
        Right,
        Down,
        Left,
        Up,
    }

    impl Dir {
        pub fn of_id(id: usize) -> Dir {
            match id {
                0 => Dir::Right,
                1 => Dir::Down,
                2 => Dir::Left,
                3 => Dir::Up,
                _ => panic!("Invalid direction id: {id}"),
            }
        }

        pub fn iter() -> Iter<'static, Dir> {
            static DIRECTIONS: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
            DIRECTIONS.iter()
        }

        pub fn id(self) -> usize {
            match self {
                Dir::Right => 0,
                Dir::Down => 1,
                Dir::Left => 2,
                Dir::Up => 3,
            }
        }

        pub fn turn_left(self) -> Dir {
            Self::of_id((self.id() + 3) % 4)
        }

        pub fn turn_right(self) -> Dir {
            Self::of_id((self.id() + 1) % 4)
        }

    }

    pub trait CoordMap<T: Copy> {
        fn at(&self, p: Coord) -> &T;
        fn contains(&self, p: Coord) -> bool;
        fn take_in_dir(&self, pos: Coord, dir: Coord, n: usize) -> Vec<T>;
    }

    impl<T: Copy> CoordMap<T> for Vec<Vec<T>> {
        fn at(&self, p: Coord) -> &T {
            let y: usize = p.y.try_into().unwrap();
            let x: usize = p.x.try_into().unwrap();
            &self[y][x]
        }

        fn contains(&self, p: Coord) -> bool {
            p.x >= 0 && p.y >= 0 && p.x < self[0].len().try_into().unwrap() && p.y < self.len().try_into().unwrap()
        }

        fn take_in_dir(&self, pos: Coord, dir: Coord, n: usize) -> Vec<T> {
            let mut acc = vec![];
            for i in 0..n {
                let p = pos + dir * i.try_into().unwrap();
                if !self.contains(p) {
                    break;
                }
                acc.push(*self.at(p));
            }
            acc
        }
    }

    pub fn print_char_map(map: &Vec<Vec<char>>) {
        for row in map {
            row.iter().for_each(|x| print!("{x}"));
            println!("");
        }
    }

    pub fn coord_iter<T>(x: &Vec<Vec<T>>) -> impl Iterator<Item = Coord> + '_ {
        let x = (0..x.len()).flat_map(|r| (0..x[0].len()).map(move |c| Coord::new_u(c, r)));
        return x;
    }

    pub struct Bitmask {
        masks: Vec<u64>
    }

    impl Bitmask {
        pub fn new(size: usize) -> Bitmask {
            Bitmask { masks: vec![0; size / 64 + 1] }
        }

        pub fn contains(&self, idx: usize) -> bool {
            self.masks[idx / 64] & (1 << (idx % 64)) != 0
        }

        pub fn add(&mut self, idx: usize) {
            let i = idx / 64;
            let v = 1 << (idx % 64);
            self.masks[i] |= v;
        }

        pub fn rm(&mut self, idx: usize) {
            let i = idx / 64;
            let v = 1 << (idx % 64);
            self.masks[i] &= !v;
        }
    }
}