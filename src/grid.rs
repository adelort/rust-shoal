use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::{fish::Fish, Vector, BACKGROUND_COLOR, GRID_COLOR, GRID_DOT_SPACING, HEIGHT, WIDTH};

pub struct Grid {
    center: Vector,
    buffer: Vec<u32>,
    start_time: Instant,
    pub window: Window,
}

impl Grid {
    pub fn new() -> Self {
        let window = Window::new(
            "Runners Simulation - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .expect("Unable to open window");

        Self {
            center: Vector { x: 0., y: 0. },
            buffer: vec![BACKGROUND_COLOR; WIDTH * HEIGHT],
            start_time: Instant::now(),
            window,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(BACKGROUND_COLOR);
    }

    pub fn print_buffer(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Failed to update buffer");
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    fn draw_dot(&mut self, x: f64, y: f64, color: u32) {
        let centered_x = (x - self.center.x as f64 + WIDTH as f64 / 2.).round() as i64;
        let centered_y = (y - self.center.y as f64 + HEIGHT as f64 / 2.).round() as i64;

        if centered_x >= 0
            && centered_x < WIDTH as i64
            && centered_y >= 0
            && centered_y < HEIGHT as i64
        {
            let index = centered_y as usize * WIDTH + centered_x as usize;
            self.buffer[index] = color;
        }
    }

    pub fn draw_circle(&mut self, x: f64, y: f64, radius: usize, color: u32) {
        let radius_isize = radius as isize;

        (-radius_isize..=radius_isize).for_each(|dx| {
            (-radius_isize..=radius_isize).for_each(|dy| {
                if (dx * dx) + (dy * dy) <= (radius_isize * radius_isize) {
                    self.draw_dot(x + dx as f64, y + dy as f64, color);
                }
            })
        });
    }

    pub fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: u32) {
        // Bresenham's line algorithm

        let mut x1 = x1 as i32;
        let mut y1 = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            self.draw_dot(x1 as f64, y1 as f64, color);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }
            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    pub fn center_screen(&mut self, fishes: &Vec<Fish>) {
        // Center the viewport on the shoal
        self.center = fishes
            .iter()
            .map(|fish| fish.position.clone())
            .sum::<Vector>()
            * (1. / fishes.len() as f64);
    }

    pub fn draw_grid(&mut self) {
        let min_x = self.center.x as f64 - WIDTH as f64 / 2.;
        let max_x = self.center.x as f64 + WIDTH as f64 / 2.;
        let min_y = self.center.y as f64 - HEIGHT as f64 / 2.;
        let max_y = self.center.y as f64 + HEIGHT as f64 / 2.;

        let x_grid_1 = max_x.div_euclid(WIDTH as f64) * WIDTH as f64;
        let y_grid_1 = max_y.div_euclid(HEIGHT as f64) * HEIGHT as f64;
        let x_grid_2 = ((max_x - WIDTH as f64 / 2.).div_euclid(WIDTH as f64) + 0.5) * WIDTH as f64;
        let y_grid_2 =
            ((max_y - HEIGHT as f64 / 2.).div_euclid(HEIGHT as f64) + 0.5) * HEIGHT as f64;

        (min_x.round() as i64..max_x.round() as i64)
            .filter(|x| x % GRID_DOT_SPACING as i64 == 0)
            .for_each(|x| {
                self.draw_dot(x as f64, y_grid_1, GRID_COLOR);
                self.draw_dot(x as f64, y_grid_2, GRID_COLOR);
            });
        (min_y.round() as i64..max_y.round() as i64)
            .filter(|y| y % GRID_DOT_SPACING as i64 == 0)
            .for_each(|y| {
                self.draw_dot(x_grid_1, y as f64, GRID_COLOR);
                self.draw_dot(x_grid_2, y as f64, GRID_COLOR);
            });
    }
}
