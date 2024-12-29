use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use uuid::Uuid;

use crate::{
    grid::Grid, vector::Vector, ATTRACTION_DISTANCE, ATTRACTION_FORCE_FACTOR, HEIGHT,
    REPULSION_DISTANCE, REPULSION_FORCE_FACTOR, VISIBILITY_DISTANCE, WIDTH,
};

#[derive(Clone)]
pub enum FishKind {
    SchoolFish,
    Predator,
}

#[derive(Clone)]
pub struct Fish {
    id: Uuid,
    pub position: Vector,
    velocity: Vector,
    t: f64,
    kind: FishKind,
}

impl Fish {
    pub fn create_initial_state(fish_kind: FishKind) -> Self {
        let mut rng = thread_rng();

        let initial_x = Uniform::new(0, WIDTH / 2).sample(&mut rng);
        let initial_y = Uniform::new(0, HEIGHT / 2).sample(&mut rng);

        Fish {
            id: Uuid::new_v4(),
            position: Vector {
                x: initial_x as f64,
                y: initial_y as f64,
            },
            velocity: Vector { x: 0., y: 0. },
            t: 0.,
            kind: fish_kind,
        }
    }

    fn alpha(&self) -> f64 {
        self.velocity.y.atan2(self.velocity.x)
    }

    pub fn compute_attraction(&self, other_fish: &Fish) -> Vector {
        let delta_x = self.position.x - other_fish.position.x;
        let delta_y = self.position.y - other_fish.position.y;

        let distance: f64 = (delta_x * delta_x + delta_y * delta_y).sqrt();

        let angle = delta_y.atan2(delta_x);

        if distance > VISIBILITY_DISTANCE {
            return Vector { x: 0., y: 0. };
        } else if distance > ATTRACTION_DISTANCE {
            let normalized_distance =
                (distance - ATTRACTION_DISTANCE) / (VISIBILITY_DISTANCE - ATTRACTION_DISTANCE);

            let norm = ATTRACTION_FORCE_FACTOR
                * 64.
                * normalized_distance.powi(3)
                * (1. - normalized_distance.powi(3));

            return Vector {
                x: angle.cos(),
                y: angle.sin(),
            } * norm
                * -1.;
        } else if distance > REPULSION_DISTANCE {
            return Vector { x: 0., y: 0. };
        } else if distance < 1e-10 {
            // Prevents errors when the distance is too small
            return Vector { x: 0., y: 0. };
        } else {
            let normalized_distance = distance / REPULSION_DISTANCE;

            let norm = REPULSION_FORCE_FACTOR * 2. * (1. - normalized_distance).powi(2)
                / normalized_distance;

            return Vector {
                x: angle.cos(),
                y: angle.sin(),
            } * norm;
        }
    }

    pub fn swim(&mut self, t: f64, all_fishes: &Vec<Fish>) {
        let dt = t - self.t;

        //TODO: if predator, random path or goes toward close fishes
        let dv = (all_fishes
            .iter()
            .filter(|fish| fish.id != self.id)
            .map(|fish| self.compute_attraction(fish))
            .sum::<Vector>())
            * (dt / all_fishes.len() as f64);

        self.velocity += dv;

        self.position += self.velocity.clone() * dt;

        self.t = t;
    }

    pub fn draw(&self, grid: &mut Grid) {
        let color = match self.kind {
            FishKind::Predator => 0xff0000,
            FishKind::SchoolFish => 0xfffffff,
        };

        let head_radius = match self.kind {
            FishKind::Predator => 5,
            FishKind::SchoolFish => 3,
        };

        let tail_length = match self.kind {
            FishKind::Predator => 15.,
            FishKind::SchoolFish => 8.,
        };

        grid.draw_circle(self.position.x, self.position.y, head_radius, color);
        grid.draw_line(
            self.position.x,
            self.position.y,
            self.position.x - tail_length * self.alpha().cos(),
            self.position.y - tail_length * self.alpha().sin(),
            color,
        );
    }
}
