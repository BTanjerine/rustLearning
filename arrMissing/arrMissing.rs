fn sum_with_missing(arr: Vec<Option<i32>>) -> i32 {
    let mut total = 0;

    for i in arr.iter() {
        //pattern match using match (bulky)
        match i {
            Some(i) => total += i,
            None => total += 0,
        };

        /*
         * Other solution
         *
         * if let Some(i) = i {
         *      total += i;
         * }
         *
         */

        /*
         * Better solution
         * 
         * total = arr
         *  .iter()
         *  .map(|i| i.unwrap_or(0)) // mapping values using lambda function
         *  .sum()
         *
         *
         */

    }

    return total;
}


fn main() {
    let a = vec![Some(1), Some(5), None, Some(100)];

    let ans = sum_with_missing(a);

    println!("{}", ans);
}
