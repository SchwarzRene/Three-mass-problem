use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Vec2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Mass {
    pos: Vec2D,
    vel: Vec2D,
    mass: f64,
}

impl Mass {
    fn update_velocity(&mut self, force: Vec2D, time_step: f64) {
        let acc_x = force.x / self.mass;
        let acc_y = force.y / self.mass;

        self.vel.x += acc_x * time_step;
        self.vel.y += acc_y * time_step;

        self.pos.x += self.vel.x * time_step;
        self.pos.y += self.vel.y * time_step;
    }
}

#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut earth = Mass {
        pos: Vec2D { x: 0., y: 0. },
        vel: Vec2D { x: 0., y: 0. },
        mass: 5.97219e24,
    };

    let mut moon = Mass {
        pos: Vec2D { x: 384400000., y: 0. },
        vel: Vec2D { x: 100., y: 1022. },
        mass: 7.34767309e22,
    };

    let mut moon2 = Mass {
        pos: Vec2D { x: -384400000., y: 10000. },
        vel: Vec2D { x: 200., y: 500. },
        mass: 7.34767309e23,
    };

    loop {
        clear_background(BLACK);

        // Update physics for each unique pair.
        update_masses(&mut earth, &mut moon, 12000.0);
        update_masses(&mut earth, &mut moon2, 12000.0);
        update_masses(&mut moon, &mut moon2, 12000.0);

        // Build an immutable slice of the objects for camera calculations.
        let objects = [&earth, &moon, &moon2];
        let center_of_mass = compute_center_of_mass(&objects);
        let scale_factor = compute_scale(&objects, screen_width() / 2.0);

        // Draw the objects relative to the computed center.
        draw_mass(&earth, scale_factor, center_of_mass);
        draw_mass(&moon, scale_factor, center_of_mass);
        draw_mass(&moon2, scale_factor, center_of_mass);

        next_frame().await;
    }
}

/// Computes the center of mass of the system.
fn compute_center_of_mass(objects: &[&Mass]) -> Vec2D {
    let mut total_mass = 0.0;
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;

    for obj in objects {
        total_mass += obj.mass;
        x_sum += obj.pos.x * obj.mass;
        y_sum += obj.pos.y * obj.mass;
    }

    Vec2D {
        x: x_sum / total_mass,
        y: y_sum / total_mass,
    }
}

/// Computes a dynamic scale so that all objects fit in the view.
fn compute_scale(objects: &[&Mass], screen_size: f32) -> f64 {
    let mut max_dist = 1.0;
    let center = compute_center_of_mass(objects);

    for obj in objects {
        let dist = ((obj.pos.x - center.x).powi(2) + (obj.pos.y - center.y).powi(2)).sqrt();
        if dist > max_dist {
            max_dist = dist;
        }
    }

    screen_size as f64 / (2.0 * max_dist)
}

/// Draws a mass relative to the center of mass.
fn draw_mass(mass: &Mass, scale: f64, center: Vec2D) {
    let screen_x = ((mass.pos.x - center.x) * scale + screen_width() as f64 / 2.0) as f32;
    let screen_y = ((mass.pos.y - center.y) * scale + screen_height() as f64 / 2.0) as f32;
    let radius = (mass.mass.log10() - 20.0) as f32; // Log scale for visibility
    draw_circle(screen_x, screen_y, radius.max(5.0), WHITE);
}

/// Updates the velocities of two masses with force capping.
fn update_masses(m1: &mut Mass, m2: &mut Mass, time_step: f64) {
    let d12 = get_euclidean_distance(&m1.pos, &m2.pos);

    // Prevent singularities by using a minimum distance.
    let d_min = 1_000_000.0; // 1000 km minimum distance
    let d_safe = d12.max(d_min);

    let f12 = calculate_force(m1.mass, m2.mass, d_safe);
    let f_max = 1e22; // Maximum force cap
    let f_clamped = f12.min(f_max);

    let f12_xy = get_xy(&m1.pos, &m2.pos, f_clamped);
    let f12_xy_negative = Vec2D { x: -f12_xy.x, y: -f12_xy.y };

    m1.update_velocity(f12_xy, time_step);
    m2.update_velocity(f12_xy_negative, time_step);
}

/// Returns the force vector components based on the positions.
fn get_xy(p1: &Vec2D, p2: &Vec2D, f: f64) -> Vec2D {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let angle = dy.atan2(dx);
    Vec2D {
        x: angle.cos() * f,
        y: angle.sin() * f,
    }
}

/// Calculates gravitational force.
fn calculate_force(m1: f64, m2: f64, d: f64) -> f64 {
    let g = 6.67430e-11;
    g * m1 * m2 / d.powi(2)
}

/// Computes the Euclidean distance between two points.
fn get_euclidean_distance(p1: &Vec2D, p2: &Vec2D) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}
