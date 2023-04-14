fn unique(mut a: Vec<i32>, s: usize, e: usize, dup: bool) -> Vec<i32> {
     
    if a[s] == a[e] {
        if dup {
           a.remove(s);
           println!("removed");
        }

        return vec![] 
    }
    else{
        let m = if e<s {
            ((e-s)/2) as usize
        }
        else{
            s
        };

        let v1 = unique(a.clone(), s, m, dup);
        let v2 = unique(a.clone(), m+1, e, a[m+1] == a[e]);
        
        [v1,v2].concat()
    }
}

fn unique_v2(mut a: Vec<i32>) -> Vec<i32> {
    for (c, v) in a.iter().enumerate() {
        for j in c..a.len(){
            if a[c] == a[j as usize] {
                a.remove(j as usize);
            }
        }
    }

    a
}

fn main(){
    let d = vec![1, 2, 4, 5, 6, 6, 8];
    
    println!("{:?}", d);

    println!("{:?}", unique(d, 0, 6, false));
}
