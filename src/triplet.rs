use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

pub struct Triplet<T>(pub T, pub T, pub T)
where
    T: PartialOrd;

impl<T> Triplet<T>
where
    T: PartialOrd,
{
    pub fn max(&self) -> Prong {
        match self {
            Triplet(fl, f, fr) if f > fl && f > fr => Prong::Middle,
            Triplet(fl, f, fr) if f < fl && f < fr => thread_rng().gen::<Prong>(),
            Triplet(fl, _, fr) if fl < fr => Prong::Right,
            Triplet(fl, _, fr) if fr < fl => Prong::Left,
            _ => Prong::Middle,
        }
    }
}

/// Consider it an analogy of a three-pronged fork.
pub enum Prong {
    Left,
    Middle,
    Right,
}

impl Distribution<Prong> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Prong {
        match rng.gen::<bool>() {
            false => Prong::Left,
            true => Prong::Right,
        }
    }
}
