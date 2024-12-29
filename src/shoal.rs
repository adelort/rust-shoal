use crate::{
    fish::{Fish, FishKind},
    grid::Grid,
    PREDATOR_COUNT, SCHOOL_FISH_COUNT,
};

pub struct Shoal {
    pub fishes: Vec<Fish>,
}

impl Shoal {
    pub fn new() -> Self {
        let school_fishes =
            (0..SCHOOL_FISH_COUNT).map(|_| Fish::create_initial_state(FishKind::SchoolFish));

        let predators = (0..PREDATOR_COUNT).map(|_| Fish::create_initial_state(FishKind::Predator));

        Self {
            fishes: school_fishes.chain(predators).collect(),
        }
    }

    pub fn swim(&mut self, t: f64) {
        let fishes = self.fishes.clone();

        self.fishes
            .iter_mut()
            .for_each(|fish| fish.swim(t, &fishes));
    }

    pub fn draw(&self, grid: &mut Grid) {
        self.fishes.iter().for_each(|fish| fish.draw(grid));
    }
}
