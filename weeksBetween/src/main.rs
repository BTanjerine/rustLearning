use chrono::{NaiveDate, DateTime, Duration};

fn dif_weeks(w1: &str, w2: &str) -> i32 {
    // use parser to parse string into date
    let dt1 = NaiveDate::parse_from_str(w1, "%m-%d-%Y").unwrap();
    let dt2 = NaiveDate::parse_from_str(w2, "%m-%d-%Y").unwrap();
   
    // find the difference (returns duration)
    let diff = dt2 - dt1;

    diff.num_weeks() as i32
}

fn main() {

    let w1 = "03-29-2002";
    let w2 = "05-01-2002";

    let ans = dif_weeks(w1, w2);

    println!("{}", ans);
}
