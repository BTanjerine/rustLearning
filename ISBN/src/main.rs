use std::str::FromStr;

/*
 * ISBN format -> ___ - _ - __ - ______ - _ 
 * last digit -> check digits
 *
 * Error Types:
 * tooLong, tooShort, failedCheckSum
 *
 */

#[derive(Debug)]
struct ISBN{
    raw: String,
    digits: Vec<u8>
}

#[derive(Debug)]
enum ISBNParseErrors {
    TooLong,
    TooShort,
    InvalidChar (usize, char),
    FailedCheckSum
}

fn getISBNCheckDigit(digits: &Vec<u8>) -> u8{
    
    let pre_check_digit: u32 = digits.get(0..12)
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, &x)| x * (if ((i+1) % 2) == 0 { 3 } else { 1 }))
        .map(|sub| sub as u32)
        .sum();
    
    let final_check_digit = 10 - (pre_check_digit % 10); 
    
    // final check digit cannot be 2 digits long 
    if final_check_digit == 10 {
        0
    }
    else {
        final_check_digit as u8
    }
}


//need err and from_str function to work 
impl FromStr for ISBN {
    //set err alias to the enum
    type Err = ISBNParseErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_len = s.len();
        
        //check if the length of the string is correct
        if str_len > 17{
            return Err(ISBNParseErrors::TooLong);
        }
        else if str_len < 17{
            return Err(ISBNParseErrors::TooShort);
        }
        
        //make new array for isbn numbers
        let mut nums = Vec::with_capacity(13);

        //parse string and extract numbers
        for (i, c) in s.char_indices() {
            match c {
               '-' => continue,
               '0' ..= '9' => nums.push(c.to_digit(10).unwrap() as u8),
               _ => return Err(ISBNParseErrors::InvalidChar(i, c))
            }
        }

        //check if it is a valid isbn number
        if getISBNCheckDigit(&nums) != nums[12]{
            return Err(ISBNParseErrors::FailedCheckSum);
        }


        Ok(ISBN { raw: s.to_string(), digits: nums})
    }
}

fn main() {
    //sample ISBN-13 code in string format
    let test_code = "978-3-16-148410-0";
   
    //generate new ISBN from string
    let new_isbn = ISBN::from_str(test_code);

    // print final results
    println!("{:?}", new_isbn.unwrap().digits);
}
