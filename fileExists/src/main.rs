use std::{path, fs};

trait FileMetadata{
   fn is_exists(&self) -> bool;

   fn is_writeable(&self) -> bool;

   fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path { 
    fn is_exists(&self) -> bool {
        self.try_exists().unwrap()
    }

    fn is_writeable(&self) -> bool{
        if let Ok(m) = fs::metadata(self){
            !m.permissions().readonly() 
        }
        else {
            false
        }
    }

    fn is_readable(&self) -> bool{
        fs::read(self).is_ok()
    }
}

fn main() {
   let f = path::Path::new("./foo/lol.txt"); 

   assert_eq!(true, f.is_exists());
   assert_eq!(true, f.is_readable());
   assert_eq!(true, f.is_writeable());

}
