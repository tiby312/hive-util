pub mod hex;
use hex::Coord;

use crate::hex::Cube;

pub struct Team {
    ants: [Coord; 3],
    spiders: [Coord; 2],
}

impl Substrate for &'_ Team {
    fn contains(&self, coord: &Coord) -> bool {
        self.ants.contains(coord) || self.spiders.contains(coord)
    }
}

impl Team {
    pub fn get_used_grids(&self) -> impl Iterator<Item = Coord> {
        [Coord { q: 0, r: 0 }; 0].into_iter()
    }
}

pub trait Substrate {
    fn contains(&self, coord: &Coord) -> bool;
    fn chain<S: Substrate>(self, s: S) -> Chain<Self, S>
    where
        Self: Sized,
    {
        Chain { a: self, b: s }
    }

    fn slideable_adjacent(&self, c: Coord) -> std::vec::IntoIter<hex::HexMoveVector> {
        let curr = hex::Cube::from_axial(c);
        let c = Cube::from(c);
        //TODO dont return Vec
        c.adjacent()
            .filter_map(|this| {
                if self.contains(&this) {
                    return None;
                }

                let offset = this.sub(curr);

                let rr_vec = offset.rotate_60_right();
                let ll_vec = offset.rotate_60_left();
                let rr = this.add(rr_vec);
                let ll = this.add(ll_vec);

                let wiggle = if !self.contains(&ll) {
                    hex::Wiggle::Right
                } else if !self.contains(&rr) {
                    hex::Wiggle::Left
                } else {
                    //There is a gate we can't slide through.
                    return None;
                };

                Some(hex::HexMoveVector::new(offset, wiggle))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

pub struct Chain<A, B> {
    a: A,
    b: B,
}
impl<A: Substrate, B: Substrate> Substrate for Chain<A, B> {
    fn contains(&self, coord: &Coord) -> bool {
        self.a.contains(coord) || self.b.contains(coord)
    }
}

pub struct View<'a> {
    pub a_team: &'a Team,
    pub b_team: &'a Team,
}

impl<'a> View<'a> {
    //First coord must be the coord of a curr_team bug.
    pub fn get_possible_placements(&self) -> Vec<Coord> {
        let mut k: Vec<Coord> = self
            .a_team
            .get_used_grids()
            .flat_map(|a| Cube::from(a).adjacent())
            .map(|a| a.into())
            .collect();
        let substrate = self.a_team.chain(self.b_team);
        k.retain(|k| {
            let is_empty = !substrate.contains(k);
            let has_bad_neighbour = Cube::from(*k)
                .adjacent()
                .fold(false, |acc, j| acc || self.b_team.contains(&j));
            is_empty && !has_bad_neighbour
        });
        k.dedup();

        k
    }

    //TODO optimize
    pub fn ant_moves_with_path(&self, ant_index: usize) -> Vec<Dests> {
        let mut handled_paths: Vec<Dests> = vec![];
        let mut stack = vec![];

        let c = self.a_team.ants[ant_index];
        stack.push((Cube::from(c), vec![]));

        let substrate = self.a_team.chain(self.b_team);

        while let Some((coord, path)) = stack.pop() {
            if let Some(ff) = handled_paths.iter_mut().find(|x| x.coord == *coord) {
                if path.len() < ff.path.len() {
                    ff.path = path;
                }

                //Already looked at adjecents from here?
                continue;
            } else {
                handled_paths.push(Dests {
                    coord: *coord,
                    path: path.clone(),
                });
            }
            for a in substrate.slideable_adjacent(*coord) {
                let mut mm = path.clone();
                mm.push(a);
                stack.push((coord.with(a.dir), mm));
            }
        }

        handled_paths
    }
}

#[derive(Clone)]
pub struct Dests {
    coord: Coord,
    //TODO optimize this
    // 3 u64s would be able to fit 64 path moves.
    // One movement takes up 3 bits of space. just need 2^3 values.
    path: Vec<hex::HexMoveVector>,
}
impl Dests {
    pub fn get_dest(&self) -> Coord {
        self.coord
    }
    pub fn get_path(&self) -> impl Iterator<Item = hex::HexMoveVector> + '_ {
        self.path.iter().copied()
    }
}
