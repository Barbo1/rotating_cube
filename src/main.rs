/* Program that rotate Figures.
 *
 * If you want to make your own Figure, keep in mind that the Figure will rotate along y, or the
 * same, in the two dimentional plane (z, x). The "camera" of the scream is the plane (x, y), so
 * the Figures would not intersect with it to be properly printed.
 * */

mod tools;

pub use tools::Vector;
pub use tools::Window;
pub use tools::Figure;

/*
 *  functions.
 * */

fn wait(seconds: f64) {
    std::thread::sleep(std::time::Duration::from_millis((seconds * (1000 as f64)) as u64));
}


/*
 *  main.
 * */
fn main() {
    // create a window (must be mutable)
    let mut win: tools::Window = tools::Window::get_new_term_size();

    // create figures (the posible figures are cube, ruby and pyramid).
    let mut fig: tools::Figure = tools::Figure::cube(25f64, Vector {
            x : 0f64,
            y : 0f64,
            z : 60f64,
        }
    );

    // draw and rotate the figures
    loop {
        win.draw(&mut fig);
        fig.rotate_in_y(0.2f64);
        fig.rotate_in_x(0.07f64);
        win.print();
        win.clear();
        wait(0.085f64);
    }
}

