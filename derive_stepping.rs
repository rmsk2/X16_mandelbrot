// ***********************************
// Change here for new values
// ***********************************

// Coordinate of upper left corner in complex plane
const X_UPPER_LEFT: f64 = -2.0;
const Y_UPPER_LEFT: f64 = 1.0 + 1.0/8.0 + 1.0/64.0 + 1.0/256.0;
// Coordinate of lower right corner in complex plane
const X_LOWER_RIGHT: f64 = 1.0;
const Y_LOWER_RIGHT: f64 = -Y_UPPER_LEFT;
// Resolution in X and Y direction
const PIXELS_X: f64 = 320.0;
const PIXELS_Y: f64 = 240.0;
// Accuracy in bytes after the comma
const NUM_FIXED_BYTES: u16 = 4;

fn print_mandel_fixed_point(f_in: f64) {
    let mut f: f64 = f_in;
    let num_bits = (NUM_FIXED_BYTES - 1) * 8;
    let mut bit_count = 0;
    let mut res: u64 = 0;

    while bit_count < num_bits {
        f = f.fract();
        res <<= 1;
        if f >= 0.5 {
            res = res | 1;
        }

        f = f * 2.0;
        bit_count += 1;
    }

    println!("{:06x}", res);
}

fn main() {
    let stepping_x: f64 = (X_LOWER_RIGHT - X_UPPER_LEFT) / PIXELS_X;
    let stepping_y: f64 = (Y_UPPER_LEFT - Y_LOWER_RIGHT) / PIXELS_Y;

    println!("Stepping X:");
    print_mandel_fixed_point(stepping_x);
    println!("Stepping Y:");
    print_mandel_fixed_point(stepping_y);
}