//make a pattern function and sort by that pattern 
fn sort_names<T:AsRef<str>>(names: &mut Vec<T>){
    names.sort_unstable_by(|a,b| a
                       .as_ref()
                       .to_lowercase()
                       .as_bytes()[0].cmp(&b
                                          .as_ref()
                                          .to_lowercase()
                                          .as_bytes()[0])
                       ); 
}

//make a pattern function and sort (lowercase to key)
fn sort_names2<T:AsRef<str>>(names: &mut Vec<T>){
    names.sort_unstable_by_key(|k| k.as_ref().to_lowercase())
}

fn main(){
    let mut l = vec!["Todd", "Cam", "amy", "less"];

    //sort names here
    sort_names2(&mut l);

    //print here
    println!("{:?}", l);
}
