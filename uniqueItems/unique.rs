//doesnt work but 
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

//<n^2 i32 function
fn unique_v2(mut a: Vec<i32>) -> Vec<i32> {
    for c  in 0..(a.len()-1) {
        for j in c+1..(a.len()-1){
            if a[c as usize] == a[j as usize] {
                println!("removed {}", a[j as usize]);
                a.remove(j as usize);
            }
        }
    }

    a
}

//<n^2 generic data function 
fn unique_v3<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    for c  in 0..(a.len()-1) {
        for j in c+1..(a.len()-1){
            if a[c as usize] == a[j as usize] {
                a.remove(j as usize);
            }
        }
    }

    a
}

//using premade functions
fn unique_v4<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    a.sort();
    a.dedup();
    a
}

fn main(){
    let d = vec!['a', 'b', 'c', 'd', 'e', 'f', 'b'];
    let f = vec![3,6,8,9,1,8,3,2];

    println!("{:?}", f);

    println!("{:?}", unique_v4(f));
}
