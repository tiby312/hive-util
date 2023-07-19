pub mod hex;
pub mod substrate;
use hex::Coord;

use crate::hex::Cube;
use substrate::Substrate;

pub struct Team {
    ants: [Coord; 3],
    spiders: [Coord; 2],
}

impl substrate::Substrate for &'_ Team {
    fn contains(&self, coord: &Coord) -> bool {
        self.ants.contains(coord) || self.spiders.contains(coord)
    }
}

impl Team {
    pub fn get_used_grids(&self) -> impl Iterator<Item = Coord> + '_ {
        self.ants
            .iter()
            .copied()
            .chain(self.spiders.iter().copied())
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
        stack.push((Cube::from(c), smallvec::smallvec![]));

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
            for a in substrate.slideable_adjacent().build(coord) {
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
    path: smallvec::SmallVec<[hex::HexMoveVector; 16]>,
}
impl Dests {
    pub fn get_dest(&self) -> Coord {
        self.coord
    }
    pub fn get_path(&self) -> impl Iterator<Item = hex::HexMoveVector> + '_ {
        self.path.iter().copied()
    }
}

// pub struct Path{
//     inner:[u64;3],
//     length:u8
// }
// impl Path{
//     pub fn add(&mut self,h:hex::HexMoveVector){
//         let dir=h.dir as u8;
//         let wiggle=h.wiggle as u8;

//     }
// }
