struct Vec{
    x : f64,
    y : f64,
}

struct Mass {
    pos : Vec,
    mass : f64,
    velocity : Vec
}

fn main() {
    println!("Hello, world!");

    //Earth-mass 5.97219 × 10**24 kg
    let planet_1 = &Mass {
        pos : Vec { x : 0., y : 0. },
        mass : 5.97219 * f64::powf( 10., 24. ),
        velocity : Vec { x : 0., y : 0. }
    };

    //Moon-mass 7.34767309 × 10**22
    let planet_2 = &Mass {
        pos : Vec { x : 384400000., y : 0. },
        mass : 7.34767309 * f64::powf( 10., 22. ),
        velocity : Vec { x : 0., y : 0. }
    };

    let f = update_masses( planet_1, planet_2 );
    println!( "{f}" );
}

fn update_masses( m1 : &Mass, m2 : &Mass ) -> f64{
    let d12 = get_euclidian_distance(&m1.pos, &m2.pos );
    let f = calculate_force( m1.mass, m2.mass, d12 );
    f
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