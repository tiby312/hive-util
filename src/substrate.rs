use crate::hex;
use crate::Coord;
pub struct Slidable<'a, S: ?Sized> {
    inner: &'a S,
}
impl<'a, S: Substrate + ?Sized> Slidable<'a, S> {
    pub fn build(self, curr: hex::Cube) -> impl Iterator<Item = hex::HexMoveVector> + 'a {
        let sub = self.inner;
        //TODO dont return Vec
        curr.adjacent().filter_map(move |this| {
            if sub.contains(&this) {
                return None;
            }

            let offset = this.sub(curr);

            let rr_vec = offset.rotate_60_right();
            let ll_vec = offset.rotate_60_left();
            let rr = this.add(rr_vec);
            let ll = this.add(ll_vec);

            let wiggle = if !sub.contains(&ll) {
                hex::Wiggle::Right
            } else if !sub.contains(&rr) {
                hex::Wiggle::Left
            } else {
                //There is a gate we can't slide through.
                return None;
            };

            Some(hex::HexMoveVector::new(offset, wiggle))
        })
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

    fn slideable_adjacent(&self) -> Slidable<Self> {
        Slidable { inner: self }
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
