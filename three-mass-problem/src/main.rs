struct Vec{
    x : f64,
    y : f64,
}

struct Mass {
    pos : Vec,
    mass : f64,
}

impl Mass{
    fn update_velocity(&mut self, force : Vec, time_constant : f64 ){
        //Get the acceleration from the force
        let acc_x = force.x / self.mass;
        let acc_y = force.y / self.mass;

        self.pos.x += acc_x * f64::powf( time_constant, 2. ) / 2.;
        self.pos.y += acc_y * f64::powf( time_constant, 2. ) / 2.;

    }
}

fn main() {
    //Earth-mass 5.97219 × 10**24 kg
    let planet_1 = &mut Mass {
        pos : Vec { x : 0., y : 0. },
        mass : 5.97219 * f64::powf( 10., 24. ),
    };

    //Moon-mass 7.34767309 × 10**22
    let planet_2 = &mut Mass {
        pos : Vec { x : 384400000., y : 0. },
        mass : 7.34767309 * f64::powf( 10., 22. ),
    };
    update_masses( planet_1, planet_2, 1. );
}

fn update_masses( m1 : &mut Mass, m2 : &mut Mass, time_constant : f64 ){
    let d12 = get_euclidian_distance(&m1.pos, &m2.pos );
    let f12 = calculate_force( m1.mass, m2.mass, d12 );

    let f12_xy = get_xy( &m1.pos, &m2.pos, f12 );
    let f12_xy_negative = turn_vector( &f12_xy );

    m1.update_velocity( f12_xy, time_constant );
    m2.update_velocity( f12_xy_negative, time_constant );
}

fn turn_vector( v : &Vec ) -> Vec{
    Vec{
        x : -v.x,
        y : -v.y
    }
}

fn get_xy( p1 : &Vec, p2 : &Vec, f : f64 ) -> Vec{
    let distance_x = p2.x - p1.x;
    let distance_y = p2.y - p1.y;

    let angle = distance_y.atan2( distance_x );

    let f_x = angle.cos() * f;
    let f_y = angle.sin() * f;

    Vec{
        x : f_x,
        y : f_y
    }
}

fn calculate_force( m1 : f64, m2 : f64, d : f64 ) -> f64{
    //G = 6.67 x 10-11 N m2 / kg2
    let g = 6.67 * f64::powf( 10., 11. );

    let f = g * m1 * m2 / f64::powf( d, 2. );
    f
}

fn get_euclidian_distance( p1 : &Vec, p2 : &Vec ) -> f64{
    f64::powf( f64::powf( p1.x - p2.x, 2. ) + f64::powf( p1.y - p2.y, 2. ), 0.5 )
}