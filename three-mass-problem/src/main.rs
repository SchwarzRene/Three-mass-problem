use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Vec2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Mass {
    pos: Vec2D,
    vel: Vec2D, // Added velocity
    mass: f64,
}

impl Mass {
    fn update_velocity(&mut self, force: Vec2D, time_step: f64) {
        // Acceleration = Force / Mass
        let acc_x = force.x / self.mass;
        let acc_y = force.y / self.mass;

        // Update velocity
        self.vel.x += acc_x * time_step;
        self.vel.y += acc_y * time_step;

        // Update position using velocity
        self.pos.x += self.vel.x * time_step;
        self.pos.y += self.vel.y * time_step;
    }
}

#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut earth = Mass {
        pos: Vec2D { x: 0., y: 0. },
        vel: Vec2D { x: 0., y: 0. }, // Earth is stationary
        mass: 5.97219e24, // Earth mass
    };

    let mut moon = Mass {
        pos: Vec2D { x: 384400000., y: 0. }, // Moon starts at 384,400 km from Earth
        vel: Vec2D { x: 0., y: 1022. }, // Initial orbital velocity (1022 m/s for near-circular orbit)
        mass: 7.34767309e22, // Moon mass
    };

    loop {
        clear_background(BLACK);

        // Update physics
        update_masses(&mut earth, &mut moon, 24000.0);

        // Draw updated positions
        draw_circle(get_normalized_value(earth.pos.x), get_normalized_value(earth.pos.y), 30., BLUE);
        draw_circle(get_normalized_value(moon.pos.x), get_normalized_value(moon.pos.y), 10., WHITE);

        next_frame().await;
    }
}

fn get_normalized_value(v: f64) -> f32 {
    let scale_factor = 200. / 384400000.; // Normalize distance based on Moon's orbit
    (v * scale_factor + 400.) as f32 // Centering objects in window
}

fn update_masses(m1: &mut Mass, m2: &mut Mass, time_step: f64) {
    let d12 = get_euclidian_distance(&m1.pos, &m2.pos);
    
    // Avoid singularities (if too close, cap force)
    let min_distance = 1000000.; // 1000 km to prevent extreme forces
    let d12_safe = d12.max(min_distance);

    let f12 = calculate_force(m1.mass, m2.mass, d12_safe);

    let f12_xy = get_xy(&m1.pos, &m2.pos, f12);
    let f12_xy_negative = turn_vector(&f12_xy);

    m1.update_velocity(f12_xy, time_step);
    m2.update_velocity(f12_xy_negative, time_step);
}

fn turn_vector(v: &Vec2D) -> Vec2D {
    Vec2D { x: -v.x, y: -v.y }
}

fn get_xy(p1: &Vec2D, p2: &Vec2D, f: f64) -> Vec2D {
    let distance_x = p2.x - p1.x;
    let distance_y = p2.y - p1.y;
    let angle = distance_y.atan2(distance_x);

    Vec2D {
        x: angle.cos() * f,
        y: angle.sin() * f,
    }
}

fn calculate_force(m1: f64, m2: f64, d: f64) -> f64 {
    let g = 6.67430e-11; // Gravitational constant
    g * m1 * m2 / d.powi(2)
}

fn get_euclidian_distance(p1: &Vec2D, p2: &Vec2D) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}
