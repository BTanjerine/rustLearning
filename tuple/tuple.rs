use std::fmt;

#[derive(Debug)]
struct Matrix (f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {0}, {1} )\n", self.0, self.1)?;
        write!(f, "( {0}, {1} )", self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    let newM = Matrix(m.0, m.2, m.1, m.3);

    newM
}

fn main(){
    let m = Matrix (0.0, 1.3, 2.5, 3.5);
    
    println!("{}", m);
    println!("{}", transpose(m))
}

