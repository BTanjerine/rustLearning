#[derive(Debug)]
enum Scale{
    FAR,
    CEL
}

#[derive(Debug)]
struct Temp {
    degrees: f32,
    scale: Scale
}

impl Temp {
    fn to_fahrenheit(&mut self){
        match self.scale {
            Scale::FAR => self.degrees = self.degrees,
            Scale::CEL => self.degrees = (self.degrees*9.0)/5.0 + 32.0
        }

        self.scale = Scale::FAR;
    }

    fn to_celcius(&mut self){
        match self.scale {
            Scale::FAR => self.degrees = ((self.degrees-32.0)*5.0)/9.0,
            Scale::CEL => self.degrees = self.degrees
        }

        self.scale = Scale::CEL;
    }

    fn get_temp(&self) -> f32{
       self.degrees 
    }

    fn new(deg: f32, sc: Scale) ->  Temp {
        Temp { degrees: deg, scale: sc }
    }
}

fn main() {

    let mut t1 = Temp::new(80.0, Scale::FAR);

    t1.to_fahrenheit();
    println!("F: {}", t1.get_temp());
    
    t1.to_celcius();
    println!("C: {:?}", t1.get_temp());
}
