use std::fmt;

struct nums(Vec<i32>);

impl fmt::Display for nums {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        
        let vec = &self.0;

        //? for combining multiple Result type to be returned at the end
        write!(f, "[")?;
        
        //state variables being used for loopm, use enumerate function to set the conditions of the
        //loop
        for(count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, " , ")?;
            }

            write!(f, "{0}: {1}", count, v)?;
        }
        
        //no semicolon so the proper state of Result can be returned
        write!(f, "]")
    }
}

fn main(){
    let l = nums(vec![1, 2, 4]);

    println!("{}", l);
}

