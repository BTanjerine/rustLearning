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

fn main(){
    let mut l = vec!["Todd", "Cam", "amy", "less"];

    //sort names here
    sort_names(&mut l);

    //print here
    println!("{:?}", l);
}
