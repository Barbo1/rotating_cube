use termion::color::*;

/*
 *  mode definition. 
 * */
pub enum Writemodes {
    Centered,
    ToLeft,
    ToRight
}

/*
 *  structs definition.
 * */
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    _begin: Vector,
    _end: Vector,
}

#[derive(Debug)]
pub struct Figure {
    _position : Vector,
    _lines : Vec<Line>,
    _color : &'static dyn Color
}

#[derive(Clone)]
pub struct WinElem {
    _data: char,
    _color: &'static dyn Color,
}
pub struct Window {
    // this Vectors form the plane where the Lines will be drawed.
    _plane_vec_1: Vector,
    _plane_vec_2: Vector,

    // this is the information of the Windows.
    _height: usize,
    _width: usize, 
    _Window: Vec<WinElem>,
}


/*
 *  this is the lenght that a window needs to print lines. If were too large the lines dont will be
 *  printed correctly, if were too small the program will take so much time to print a line.
 * */
static LITTLE_CONSTANT: f64 = 0.02f64;

/*
 *  plane scream vectors(i will change the manner of usage).
 * */
static VEC_1: Vector = Vector {x : 1f64, y : 0f64, z : 0f64};
static VEC_2: Vector = Vector {x : 0f64, y : 1f64, z : 0f64};

static PRINT_CHAR: char = '•';
const WHITE_COLOR: &dyn Color = &Rgb(255, 255, 255);
const EMPTY_CELL: WinElem = WinElem {_data: ' ', _color: WHITE_COLOR};


/*
 *  struct implementation.
 * */
impl Vector {
    pub fn sum(self, vect: Vector) -> Vector {
        Vector {
            x : self.x + vect.x,
            y : self.y + vect.y,
            z : self.z + vect.z,
        }
    }
    
    pub fn mul(self, coef: f64) -> Vector {
        Vector {
            x : self.x * coef,
            y : self.y * coef,
            z : self.z * coef,
        }
    }
    
    pub fn scalar_prod(&self, vect: Vector) -> f64 {
        self.x*vect.x + self.y*vect.y + self.z*vect.z 
    }
}

impl Line { 
    fn rotate_in_x(&mut self, alpha: f64) {
        let sin = alpha.sin();
        let cos = alpha.cos();
        
        let old_y = self._begin.y;
        let old_z = self._begin.z;
        self._begin.y = old_y*cos - old_z*sin;
        self._begin.z = old_y*sin + old_z*cos;
        
        let old_y = self._end.y;
        let old_z = self._end.z;
        self._end.y = old_y*cos - old_z*sin;
        self._end.z = old_y*sin + old_z*cos;
    }
    
    fn rotate_in_y(&mut self, alpha: f64) {
        let sin = alpha.sin();
        let cos = alpha.cos();
        
        let old_x = self._begin.x;
        let old_z = self._begin.z;
        self._begin.x = old_x*cos - old_z*sin;
        self._begin.z = old_x*sin + old_z*cos;
        
        let old_x = self._end.x;
        let old_z = self._end.z;
        self._end.x = old_x*cos - old_z*sin;
        self._end.z = old_x*sin + old_z*cos;
    }
    
    fn rotate_in_z(&mut self, alpha: f64) {
        let sin = alpha.sin();
        let cos = alpha.cos();
        
        let old_x = self._begin.x;
        let old_y = self._begin.y;
        self._begin.x = old_x*cos - old_y*sin;
        self._begin.y = old_x*sin + old_y*cos;
        
        let old_x = self._end.x;
        let old_y = self._end.y;
        self._end.x = old_x*cos - old_y*sin;
        self._end.y = old_x*sin + old_y*cos;
    }
}

impl Figure {
    pub fn cube(dimention: f64, _position: Vector, _color: &'static dyn Color) -> Figure {
        let dim = dimention / 2f64;
        let mut vects : Vec<Vector> = vec![
            Vector {x: dim, y: dim, z: dim}, 
            Vector {x: dim, y: -dim, z: -dim}, 
            Vector {x: -dim, y: dim, z: -dim}, 
            Vector {x: -dim, y: -dim, z: dim}
        ];

        // make all the Lines and bring them into lines.
        let mut lines : Vec<Line> = Vec::new();
        while let Some(p) = vects.pop() {
            let mut q = p;
            q.x = -q.x;
            lines.push(Line {_begin: p, _end: q});
            
            q.x = -q.x;
            q.y = -q.y;
            lines.push(Line {_begin: p, _end: q});

            q.y = -q.y;
            q.z = -q.z;
            lines.push(Line {_begin: p, _end: q});
        }

        Figure {
            _lines: lines,
            _position,
            _color
        }
    }

    pub fn ruby(dimention: f64, _position: Vector, _color: &'static dyn Color) -> Figure { 
        let dim = dimention / 2f64;
        let med =  dim * 3f64 / 4f64;
        let vect : Vec<Vector> = vec![
            Vector {x: 0f64, y: dim, z: 0f64}, 
            Vector {x: med, y: 0f64, z: med}, 
            Vector {x: -med, y: 0f64, z: med}, 
            Vector {x:  med, y: 0f64, z: -med}, 
            Vector {x: -med, y: 0f64, z: -med}, 
            Vector {x: 0f64, y: -dim/2f64, z: 0f64}, 
        ];
        
        let mut lines: Vec<Line> = Vec::new();

        lines.push(Line {_begin: vect[0], _end: vect[1]});
        lines.push(Line {_begin: vect[0], _end: vect[2]});
        lines.push(Line {_begin: vect[0], _end: vect[3]});
        lines.push(Line {_begin: vect[0], _end: vect[4]});
        lines.push(Line {_begin: vect[1], _end: vect[5]});
        lines.push(Line {_begin: vect[2], _end: vect[5]});
        lines.push(Line {_begin: vect[3], _end: vect[5]});
        lines.push(Line {_begin: vect[4], _end: vect[5]});
        lines.push(Line {_begin: vect[1], _end: vect[2]});
        lines.push(Line {_begin: vect[2], _end: vect[3]});
        lines.push(Line {_begin: vect[3], _end: vect[4]});
        lines.push(Line {_begin: vect[4], _end: vect[1]});

        Figure {
            _lines: lines,
            _position,
            _color
        }
    }

    pub fn pyramid(dimention: f64, _position: Vector, _color: &'static dyn Color) -> Figure { 
        let dcos = dimention * (std::f64::consts::PI / 3f64).cos();
        let dsen = dimention * (std::f64::consts::PI / 3f64).sin();
        let vect : Vec<Vector> = vec![
            Vector {x: 0f64, y: dimention, z: 0f64}, 
            Vector {x:  -dcos, y: 0f64, z: dsen}, 
            Vector {x:  -dcos, y: 0f64, z: -dsen}, 
            Vector {x: dimention, y: 0f64, z: 0f64}, 
        ];
        
        let mut lines: Vec<Line> = Vec::new();

        lines.push(Line {_begin: vect[0], _end: vect[1]});
        lines.push(Line {_begin: vect[0], _end: vect[2]});
        lines.push(Line {_begin: vect[0], _end: vect[3]});
        lines.push(Line {_begin: vect[1], _end: vect[2]});
        lines.push(Line {_begin: vect[1], _end: vect[3]});
        lines.push(Line {_begin: vect[2], _end: vect[3]});
        
        Figure {
            _lines: lines,
            _position,
            _color
        }
    }

    pub fn square(height: usize, width: usize, _position: Vector, _color: &'static dyn Color) -> Figure {
        let wby2: f64 = (width / 2) as f64;
        let hby2: f64 = (height / 2) as f64;
        let vect: Vec<Vector> = vec![
            Vector {x:  wby2, y:  hby2, z: 0f64}, 
            Vector {x: -wby2, y:  hby2, z: 0f64}, 
            Vector {x: -wby2, y: -hby2, z: 0f64}, 
            Vector {x:  wby2, y: -hby2, z: 0f64}, 
        ];
        
        let mut lines: Vec<Line> = Vec::new();

        lines.push(Line {_begin: vect[0], _end: vect[1]});
        lines.push(Line {_begin: vect[1], _end: vect[2]});
        lines.push(Line {_begin: vect[2], _end: vect[3]});
        lines.push(Line {_begin: vect[3], _end: vect[0]});
        
        Figure {
            _lines: lines,
            _position,
            _color
        }
    }

    pub fn rotate_in_x(&mut self, alpha: f64) {
        for vect in &mut self._lines {
            vect.rotate_in_x(alpha);
        }
    }
    
    pub fn rotate_in_y(&mut self, alpha: f64) {
        for vect in &mut self._lines {
            vect.rotate_in_y(alpha);
        }
    }
    
    pub fn rotate_in_z(&mut self, alpha: f64) {
        for vect in &mut self._lines {
            vect.rotate_in_z(alpha);
        }
    }
}

impl Window {
    pub fn get_new(_height: usize, _width: usize) -> Window {
        Window {
            _plane_vec_1: VEC_1,
            _plane_vec_2: VEC_2,
            _height,
            _width,
            _Window: vec![EMPTY_CELL; _height * _width],
        }
    }
    
    pub fn get_new_term_size() -> Window {
        let dims = match termion::terminal_size() {
            Ok(result) => result,
            Err(_) => panic!("Was imposible to get the dimentions of the termianl."),
        };
        let h = dims.1 as usize - 1;
        let w = dims.0 as usize;
        Window {
            _plane_vec_1: VEC_1,
            _plane_vec_2: VEC_2,
            _height: h,
            _width: w,
            _Window: vec![EMPTY_CELL; h * w],
        }
    }

    pub fn get_height(&self) -> usize {
        self._height
    }

    pub fn get_width(&self) -> usize {
        self._width
    }

    pub fn clear(&mut self) {
        clearscreen::clear().expect("failed to clear the screen.");
        for i in 0..self._height {
            for j in 0..self._width {
                self._Window[i*self._width + j] = EMPTY_CELL;
            }
        }
    }

    pub fn fill(&mut self, character: char) {
        for i in 0..self._height {
            for j in 0..self._width {
                self._Window[i*self._width + j] = WinElem{_data: character, _color: WHITE_COLOR};
            }
        }
    }
    
    pub fn print(&self) {
        let mut result: String = "".to_string();
        for i in 0..self._height {
            for j in 0..self._width {
                let elem = &self._Window[i*self._width + j];
                result.push_str(&format!("{}{}", Fg(elem._color), elem._data));
            }
            result.push_str("\n");
        }
        print!("{}", result);
    }

    /* To draw the Figure, first it will be moved to not intersect with the plane in which will be
     * drawed. The plane will be (x, y), so the Figure will be moved along the z axis.*/
    pub fn draw(&mut self, fig: &mut Figure) {
        let hei: i64 = self._height as i64 / 2;
        let wid: i64 = self._width as i64 / 2;
        let iter = fig._lines.iter();
        for line in iter {
            // proyect the begin Vector in the plane of the scream (this part can be used in the
            // general case).
            let vec_b = line._begin.sum(fig._position);

            let beg = self._plane_vec_1
                .mul(vec_b.scalar_prod(self._plane_vec_1))
                .sum(self._plane_vec_2.mul(vec_b.scalar_prod(self._plane_vec_2)));
            
            // proyect the end Vector in the plane of the scream (this part can be used in the
            // general case).
            let vec_e = line._end.sum(fig._position);

            let end = self._plane_vec_1
                .mul(vec_e.scalar_prod(self._plane_vec_1))
                .sum(self._plane_vec_2.mul(vec_e.scalar_prod(self._plane_vec_2)));

            // draw Line in the Window, making small steps.
            let mut coef1: f64 = beg.x;
            let mut coef2: f64 = beg.y;
            let mut acoef = 0f64;

            while acoef < 1f64 {
                let x = (coef1.round() as i64) + wid;
                let y = (coef2.round() as i64) + hei;
                if 0 <= x && x < self._width as i64 && 0 <= y && y < self._height as i64 {
                    self._Window[x as usize + self._width*y as usize] = WinElem{_data: PRINT_CHAR, _color: fig._color};
                }

                coef1 = beg.x + acoef*(end.x - beg.x);
                coef2 = beg.y + acoef*(end.y - beg.y);
                acoef += LITTLE_CONSTANT;
            }
            let x = (coef1.round() as i64) + wid;
            let y = (coef2.round() as i64) + hei;
            if 0 <= x && x < self._width as i64 && 0 <= y && y < self._height as i64 {
                self._Window[x as usize + self._width*y as usize] = WinElem{_data: PRINT_CHAR, _color: fig._color}; 
            }
        }
    }

    pub fn write (&mut self, text: &str, ph: f64, pw: f64, mode: Writemodes, color: &'static dyn Color) {
        let hegiht: usize = ((self._height as f64) * ph) as usize;
        let width: usize = ((self._width as f64) * pw) as usize;
        let length: usize = text.len();
        let textc: &[u8] = text.as_bytes();
        match mode {
            Writemodes::Centered => {
                let lenby2: usize = length / 2usize;
                if lenby2 <= width / 2 && true {
                    for i in 0..length {
                        let j = i + width - lenby2 + 1;
                        self._Window[j + self._width*hegiht] = WinElem {_data: textc[i] as char, _color: color};
                    }
                }
            },
            Writemodes::ToLeft => {

            },
            Writemodes::ToRight=> {

            },
        }
    }
}
