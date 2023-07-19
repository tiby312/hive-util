use std::ops::Deref;

// pub const OFFSETS: [[i16; 3]; 6] = [
//     [0, 1, -1],
//     [1, 0, -1],
//     [1, -1, 0],
//     [0, -1, 1],
//     [-1, 0, 1],
//     [-1, 1, 0],
// ];

// pub const OFFSETS: [[i16; 3]; 6] = [
//     [1, 0, -1],
//     [1, -1, 0],
//     [0, -1, 1],
//     [-1, 0, 1],
//     [-1, 1, 0],
//     [0, 1, -1],
// ];

pub const OFFSETS: [Cube; 6] = [
    Cube::from_arr([1, 0, -1]),
    Cube::from_arr([1, -1, 0]),
    Cube::from_arr([0, -1, 1]),
    Cube::from_arr([-1, 0, 1]),
    Cube::from_arr([-1, 1, 0]),
    Cube::from_arr([0, 1, -1]),
];

pub(crate) const SQRT_3: f32 = 1.73205080757;

// https://www.redblobgames.com/grids/hexagons/#hex-to-pixel

// pub const HEX_PROJ_POINTY: cgmath::Matrix2<f32> =
//     cgmath::Matrix2::new(SQRT_3, 0.0, SQRT_3 / 2.0, 3.0 / 2.0);

pub const HEX_PROJ_POINTY: [[f32; 2]; 2] = [[SQRT_3, 0.0], [SQRT_3 / 2.0, 3.0 / 2.0]];

// pub const HEX_PROJ_FLAT: cgmath::Matrix2<f32> =
//     cgmath::Matrix2::new(3.0 / 2.0, SQRT_3 / 2.0, 0.0, SQRT_3);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord {
    pub q: i16,
    pub r: i16,
}

//q r s
#[derive(Copy, Clone, Debug)]
pub struct Cube {
    coord: Coord,
    s: i16,
}
impl Deref for Cube {
    type Target = Coord;
    fn deref(&self) -> &Self::Target {
        &self.coord
    }
}

impl From<Cube> for Coord {
    fn from(a: Cube) -> Self {
        a.coord
    }
}

impl From<Coord> for Cube {
    fn from(a: Coord) -> Self {
        Cube::from_axial(a)
    }
}

impl From<[i16; 3]> for Cube {
    fn from([q, r, s]: [i16; 3]) -> Self {
        Cube {
            coord: Coord { q, r },
            s,
        }
    }
}

impl From<Cube> for [i16; 3] {
    fn from(a: Cube) -> Self {
        let Cube {
            coord: Coord { q, r },
            s,
        } = a;
        [q, r, s]
    }
}

impl Cube {
    pub const fn from_arr([q, r, s]: [i16; 3]) -> Self {
        Cube {
            coord: Coord { q, r },
            s,
        }
    }
    pub fn into_arr(self) -> [i16; 3] {
        self.into()
    }
    // pub fn new(q: i16, r: i16) -> Self {
    //     Cube::from([q, r, -q - r])
    // }
    pub fn adjacent(&self) -> impl Iterator<Item = Cube> {
        let k = *self;
        OFFSETS.iter().map(move |&a| k.add(a))
    }
    pub fn with(&self, a: Dir) -> Cube {
        let k = a as u8;
        self.add(OFFSETS[k as usize])
    }

    pub fn from_axial(a: Coord) -> Self {
        let q = a.q;
        let r = a.r;
        Cube::from([q, r, -q - r])
    }
    pub fn round(frac: [f32; 3]) -> Cube {
        let mut q = frac[0].round() as i16;
        let mut r = frac[1].round() as i16;
        let mut s = frac[2].round() as i16;

        let q_diff = (q as f32 - frac[0]).abs();
        let r_diff = (r as f32 - frac[1]).abs();
        let s_diff = (s as f32 - frac[2]).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s
        } else if r_diff > s_diff {
            r = -q - s
        } else {
            s = -q - r
        }
        return Cube::from([q, r, s]);
    }

    pub fn to_axial(&self) -> Coord {
        *self.deref()
    }

    pub fn neighbour(&self, dir: i16) -> Cube {
        self.add(Cube::direction(dir))
    }
    pub fn direction(dir: i16) -> Cube {
        OFFSETS[dir as usize]
    }
    pub fn add(self, other: Cube) -> Cube {
        let q = self.coord.q + other.coord.q;
        let r = self.coord.r + other.coord.r;
        let s = self.s + other.s;
        Cube::from([q, r, s])
    }
    pub fn sub(self, other: Cube) -> Cube {
        let q = self.coord.q - other.coord.q;
        let r = self.coord.r + -other.coord.r;
        let s = self.s - other.s;
        Cube::from([q, r, s])
    }

    // pub fn rays(&self, start: i16, end: i16, ff: impl Filter + Copy) -> impl Iterator<Item = Cube> {
    //     let o = *self;
    //     OFFSETS.iter().flat_map(move |&i| {
    //         (1..end)
    //             .map(move |a| (a, o.add(Cube(i).scale(a))))
    //             .take_while(move |(_, o)| ff.filter(&o.to_axial()) == FilterRes::Accept)
    //             .filter(move |(a, _)| *a >= start)
    //             .map(|(_, a)| a)
    //     })
    // }
    pub fn ring(&self, n: i16) -> impl Iterator<Item = Cube> {
        let mut hex = self.add(Cube::direction(4).scale(n));

        (0..6)
            .flat_map(move |i| std::iter::repeat(i).take(n as usize))
            .map(move |i| {
                let h = hex;
                hex = hex.neighbour(i);
                h
            })
    }

    pub fn rotate_60_right(self) -> Cube {
        let [q, r, s] = self.into_arr();
        Cube::from([s, q, r])
    }
    pub fn rotate_60_left(self) -> Cube {
        let [q, r, s] = self.into_arr();
        Cube::from([r, s, q])
    }

    pub fn scale(self, n: i16) -> Cube {
        let a = self.into_arr();
        Cube::from(a.map(|a| a * n))
    }

    pub fn range(&self, n: i16) -> impl Iterator<Item = Cube> {
        let o = *self;
        (-n..n + 1)
            .flat_map(move |q| ((-n).max(-q - n)..n.min(-q + n) + 1).map(move |r| (q, r)))
            .map(move |(q, r)| {
                let s = -q - r;
                o.add(Cube::from([q, r, s]))
            })
    }
    pub fn neighbours(&self) -> impl Iterator<Item = Cube> {
        let k = self.into_arr().clone();
        OFFSETS.iter().map(move |a| {
            let a = a.clone();
            for (a, b) in a.into_arr().iter_mut().zip(k.iter()) {
                *a += b;
            }
            Cube::from(a)
        })
    }

    pub fn dist(&self, other: &Cube) -> i16 {
        let b = other.into_arr();
        let a = self.into_arr();
        // https://www.redblobgames.com/grids/hexagons/#distances-cube
        ((b[0] - a[0]).abs() + (b[1] - a[1]).abs() + (b[2] - a[2]).abs()) / 2
    }
}

//TODO make sure this matches OFFETS
#[derive(Copy, Clone)]
pub enum Dir {
    LeftUp = 0,
    RightUp = 1,
    Left = 2,
    Right = 3,
    LeftDown = 4,
    RightDown = 5,
}

impl From<u8> for Dir {
    fn from(a: u8) -> Dir {
        use Dir::*;
        match a {
            0 => LeftUp,
            1 => RightUp,
            2 => Left,
            3 => Right,
            4 => LeftDown,
            5 => RightDown,
            _ => panic!("Failed to parse {} to a Dir", a),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Wiggle {
    Left = 0,
    Right = 1,
}

#[derive(Copy, Clone)]
pub struct HexMoveVector {
    pub dir: Dir,
    pub wiggle: Wiggle,
}

impl HexMoveVector {
    pub fn new(b: Cube, wiggle: Wiggle) -> Self {
        let offset = (0..)
            .zip(OFFSETS.iter())
            .find(|(_, a)| a.eq(&b))
            .expect("vector was not a unit hex vector");

        let dir = Dir::from(offset.0);

        HexMoveVector { dir, wiggle }
    }
}
