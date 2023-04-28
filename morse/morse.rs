use std::fmt;


#[derive(Debug)]
enum Pulse{
    Short,
    Long
}

type Letter = Vec<Pulse>;

type Message = Vec<Letter>;

trait MorseCode{
    fn to_morse(&self) -> Message; 
}

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "-")
        }
    }
}

impl MorseCode for String{
    fn to_morse(&self) -> Message{

        let mut res: Message = Vec::with_capacity(self.len());

        for l in self.chars() {
            match l {
                'a' => res.push(vec![Pulse::Short, Pulse::Long]),
                'b' => res.push(vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short]),
                'c' => res.push(vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short]),
                'd' => res.push(vec![Pulse::Long, Pulse::Short, Pulse::Short]),
                'e' => res.push(vec![Pulse::Short]),
                'f' => res.push(vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short]),
                'g' => res.push(vec![Pulse::Long, Pulse::Long, Pulse::Short]),
                'h' => res.push(vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short]),
                'i' => res.push(vec![Pulse::Short, Pulse::Short]),
                'j' => res.push(vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long]),
                'k' => res.push(vec![Pulse::Long, Pulse::Short, Pulse::Long]),
                'l' => res.push(vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short]),
                'm' => res.push(vec![Pulse::Long, Pulse::Long]),
                'n' => res.push(vec![Pulse::Long, Pulse::Short]),
                'o' => res.push(vec![Pulse::Long, Pulse::Long, Pulse::Long]),
                'p' => res.push(vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short]),
                'q' => res.push(vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short]),
                'r' => res.push(vec![Pulse::Short, Pulse::Long, Pulse::Short]),
                's' => res.push(vec![Pulse::Short, Pulse::Short, Pulse::Short]),
                't' => res.push(vec![Pulse::Long]),
                'u' => res.push(vec![Pulse::Short, Pulse::Short, Pulse::Long]),
                'v' => res.push(vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long]),
                'w' => res.push(vec![Pulse::Short, Pulse::Long, Pulse::Long]),
                'x' => res.push(vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long]),
                'y' => res.push(vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long]),
                'z' => res.push(vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short]),
                _ => res.push(vec![])
            }
        }

        res
    }
}

fn main() {
    //string and string in morse
    let l:String = "hello world".to_string();
    let m:Message = l.to_morse();

    //print message in morse
    for i in m.iter() {
        for p in i.iter(){
            print!("{}", p);
        }
        print!(" ");
    }
}








