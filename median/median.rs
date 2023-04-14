fn median(mut a: Vec<f32>) -> f32{
    let mut v1 = 0.0;
    let mut v2 = 0.0;

    if a.is_empty(){
        return 0.0;
    }

    a.sort_by(|x:&f32, y:&f32| x.partial_cmp(y).unwrap());

    let i = a.len()/2;
    
    if a.len() % 2 == 0{
        v1 = a[i];
        v2 = a[i-1];
    }
    else{
        v1 = a[i];
        v2 = a[i];
    }

    (v2+v1)/2.0
}

fn main(){
    let d = vec![1.0,2.0,5.0,3.0,4.0];

    let ans: Option<f32> = Some(median(vec![]));
    
    println!("{}",median(d));    
}
