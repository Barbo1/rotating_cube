/* Program that rotate Figures.
 *
 * If you want to make your own Figure, keep in mind that the Figure will rotate along y, or the
 * same, in the two dimentional plane (z, x). The "camera" of the scream is the plane (x, y), so
 * the Figures would not intersect with it to be properly printed.
 * */

use std::thread;
use std::sync::{RwLock, Arc};
use k_board::{keyboard::Keyboard, keys::Keys};

mod tools;
pub use tools::Vector;
pub use tools::Window;
pub use tools::Figure;
pub use tools::Writemodes;

const CANT_OPT: usize = 4;
const INIT_POS_OPT: f64 = 0.1f64;
const SPACE_POS_OPT: f64 = 0.1f64;

/*
 *  functions.
 * */

fn wait(seconds: f64) {
    std::thread::sleep(std::time::Duration::from_millis((seconds * (1000 as f64)) as u64));
}


fn draw_figures(win: &mut tools::Window, figs: &mut [tools::Figure]) {
    for fig in figs {
        win.draw (fig);
    }
}

fn write_options (win: &mut tools::Window, options: &mut Vec<&str>) {
    let mut ph: f64 = INIT_POS_OPT;
    for opt in options {
        win.write(opt, ph, 0.5f64, Writemodes::Centered);
        ph  = ph + SPACE_POS_OPT;
    }
}

fn read_lock (lock: &Arc<RwLock<usize>>) -> usize {
    *lock.read().unwrap()
}

fn write_lock (lock: &Arc<RwLock<usize>>, new_val: usize) {
    *lock.write().unwrap() = new_val;
}

/*
 *  main.
 * */
fn main() {
    let modex = Arc::new(RwLock::new(0));
    let _modex = Arc::clone(&modex);
    let opti = Arc::new(RwLock::new(0));
    let _opti = Arc::clone(&opti);

    let thread1 = thread::spawn(move || {
        // create a window (must be mutable)
        let mut win: tools::Window = tools::Window::get_new_term_size();

        // create figures (the posible figures are cube, ruby and pyramid).
        let mut figs: Vec<tools::Figure> = vec! {
            tools::Figure::ruby (
                15f64, 
                Vector {
                    x : 35f64,
                    y : 10f64,
                    z : 40f64,
                }
            ),
            tools::Figure::ruby (
                12f64, 
                Vector {
                    x : -60f64,
                    y : -20f64,
                    z : 40f64,
                }
            ),
            tools::Figure::ruby (
                13f64, 
                Vector {
                    x : -40f64,
                    y : 5f64,
                    z : 20f64,
                }
            ),
            tools::Figure::pyramid(
                10f64, 
                Vector {
                    x : 49f64,
                    y : -11f64,
                    z : 20f64,
                }
            ),
            tools::Figure::cube (
                25f64, 
                Vector {
                    x : 0f64,
                    y : 0f64,
                    z : 60f64,
                }
            ),
        };
        let cant_figs: usize = figs.len();

        // cube to the mode "cube"
        let mut cube: tools::Figure = tools::Figure::cube (30f64, Vector {x : 0f64, y : 0f64, z : 60f64});

        // pyramid to the mode "pyramid"
        let mut pyramid: tools::Figure = tools::Figure::pyramid (20f64, Vector {x : 0f64, y : 0f64, z : 60f64});
        
        // ruby to the mode "ruby"
        let mut ruby: tools::Figure = tools::Figure::ruby (30f64, Vector {x : 0f64, y : 0f64, z : 60f64});

        let mut mode: usize = 0;
        while mode < 5 {
            match mode {
                0 => {
                    // write selection square. 
                    let dim: f64 = (win.get_height() * (CANT_OPT - read_lock(&opti))) as f64 * SPACE_POS_OPT;
                    let mut fig = tools::Figure::square (
                        3, 
                        20, 
                        Vector {
                            x: 0f64,
                            y: -dim,
                            z: 0f64,
                        }
                    );
                    win.draw(&mut fig);
                    
                    // write options. 
                    write_options(&mut win, &mut vec!["Things", "Cube", "Pyramid", "Ruby", "Exit"]);
                },
                1 => {
                    // draw and rotate the figures
                    figs[cant_figs - 1].rotate_in_y(0.2f64);
                    figs[cant_figs - 1].rotate_in_x(0.07f64);

                    figs[0].rotate_in_y(0.3f64);
                    figs[1].rotate_in_y(0.3f64);
                    figs[2].rotate_in_y(0.3f64);
                    figs[3].rotate_in_y(-0.2f64);
                    figs[3].rotate_in_x(0.03f64);

                    draw_figures(&mut win, figs.as_mut_slice());
                    win.write("Libertad", 0.5f64, 0.5f64, Writemodes::Centered);
                },
                2 => {
                    cube.rotate_in_y(0.2f64);
                    cube.rotate_in_x(0.07f64);
                    win.draw(&mut cube);
                },
                3 =>  {
                    pyramid.rotate_in_y(0.2f64);
                    pyramid.rotate_in_x(0.07f64);
                    win.draw(&mut pyramid);
                },
                4 =>  {
                    ruby.rotate_in_y(0.2f64);
                    ruby.rotate_in_x(0.07f64);
                    win.draw(&mut ruby);
                },
                _ => {},
            }

            //print and clear.
            win.print();
            win.clear();

            wait(0.07f64);
    
            mode = read_lock(&modex);
        }
    });
    
    let thread2 = thread::spawn(move || {
        let mut exit: bool = false;
        let mut keyboard = Keyboard::new();
        while !exit {
            if read_lock(&_modex) == 0 {
                let data = read_lock(&_opti);
                match keyboard.read_key() {
                    Keys::Enter => {
                        write_lock(&_modex, data + 1);
                        if data == 4 { exit = true; }
                    },
                    Keys::Up => {
                        let data: usize = data - if data > 0  { 1 } else { 0 };
                        write_lock(&_opti, data);
                    },
                    Keys::Down => {
                        let data: usize = data + if data < CANT_OPT  { 1 } else { 0 };
                        write_lock(&_opti, data);
                    },
                    _ => {}
                }
            } else {
                match keyboard.read_key() {
                    Keys::Enter => { write_lock(&_modex, 0); },
                    _ => {}
                }
            }
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    clearscreen::clear().expect("failed to clear screen");
}
