//using rust cargo to use chrono trait
//tells us the date and time
use chrono::{Date, Local, NaiveDate};

//important even data structure
struct ImportantEvent {
    name: String,
    date: NaiveDate
}

//deadline trait to see if the date passed
trait Deadline {
   fn is_passed(&self) -> bool; 
}

//functions for the important even data struct
impl ImportantEvent {
    fn new(n: String, d: NaiveDate) -> ImportantEvent{
        ImportantEvent { name: n, date: d }
    }
}

//deadline function for important event
impl Deadline for ImportantEvent {
    //returns if the current day passed the deadline
    fn is_passed(&self) -> bool {
        self.date < Local::now().date_naive()    
    }
}

fn main() {

    let i = ImportantEvent::new("Cool".to_string(), NaiveDate::from_ymd(2023, 5, 29));

    println!("{}", i.is_passed());
}
