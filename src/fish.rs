use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use uuid::Uuid;

use crate::{
    grid::Grid, vector::Vector, ALIGNMENT_FORCE_FACTOR, ATTRACTION_FORCE_FACTOR, HEIGHT,
    PREDATOR_ATTRACTION_FACTOR, PREDATOR_REPULSION_FACTOR, REPULSION_FORCE_FACTOR,
    VISIBILITY_DISTANCE, WIDTH,
};

#[derive(Clone, PartialEq)]
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
            velocity: Vector { x: 100., y: 0. },
            t: 0.,
            kind: fish_kind,
        }
    }

    fn alpha(&self) -> f64 {
        self.velocity.y.atan2(self.velocity.x)
    }

    pub fn compute_attraction(&self, other_fish: &Fish) -> Vector {
        let delta_x = other_fish.position.x - self.position.x;
        let delta_y = other_fish.position.y - self.position.y;

        let distance: f64 = (delta_x * delta_x + delta_y * delta_y).sqrt();

        if distance > VISIBILITY_DISTANCE || distance < 1e-10 {
            // Prevents errors when the distance is too small
            return Vector { x: 0., y: 0. };
        }

        let attraction_force = Vector {
            x: delta_x / distance,
            y: delta_y / distance,
        } * ATTRACTION_FORCE_FACTOR
            * distance;

        let alignment_force =
            (other_fish.velocity.clone() - self.velocity.clone()) * ALIGNMENT_FORCE_FACTOR;

        let repulsion_force = Vector {
            x: delta_x / distance,
            y: delta_y / distance,
        } * (-REPULSION_FORCE_FACTOR / distance);

        attraction_force + alignment_force + repulsion_force
    }

    pub fn compute_predator_repulsion(&self, predator: &Fish) -> Vector {
        let delta_x = predator.position.x - self.position.x;
        let delta_y = predator.position.y - self.position.y;

        let distance: f64 = (delta_x * delta_x + delta_y * delta_y).sqrt();

        Vector {
            x: delta_x / distance,
            y: delta_y / distance,
        } * (-PREDATOR_REPULSION_FACTOR / distance / distance)
    }

    pub fn swim(&mut self, t: f64, all_fishes: &Vec<Fish>) {
        let dt = t - self.t;

        let school_fishes: Vec<&Fish> = all_fishes
            .iter()
            .filter(|fish| fish.kind == FishKind::SchoolFish)
            .collect();

        let predators: Vec<&Fish> = all_fishes
            .iter()
            .filter(|fish| fish.kind == FishKind::Predator)
            .collect();

        let school_fish_count = school_fishes.len();

        let predator_count = predators.len();

        let acceleration = match self.kind {
            FishKind::Predator => {
                ((school_fishes
                    .iter()
                    .map(|fish| fish.position.clone())
                    .sum::<Vector>()
                    * (1. / school_fish_count as f64))
                    - self.position.clone())
                    * PREDATOR_ATTRACTION_FACTOR
            }
            FishKind::SchoolFish => {
                let predator_repulsion = match predator_count {
                    0 => Vector { x: 0., y: 0. },
                    _ => {
                        predators
                            .iter()
                            .map(|predator| self.compute_predator_repulsion(predator))
                            .sum::<Vector>()
                            * (1. / predator_count as f64)
                    }
                };

                let other_fishes_interation = match school_fish_count {
                    0 => Vector { x: 0., y: 0. },
                    _ => {
                        school_fishes
                            .iter()
                            .filter(|fish| fish.id != self.id)
                            .map(|fish| self.compute_attraction(fish))
                            .sum::<Vector>()
                            * (1. / school_fish_count as f64)
                    }
                };

                predator_repulsion + other_fishes_interation
            }
        };

        self.position += self.velocity.clone() * dt;

        self.velocity += acceleration * dt;

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
