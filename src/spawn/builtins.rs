use std::io::ErrorKind;
use std::path::Path;
use std::env;

pub fn cd(d: Vec<&str>) {
     let new_dir;
     if d.is_empty() {
         new_dir = "/home/rayes";
     } else if d.iter().count() > 1 {
         eprintln!("yui: cd: Too many arguments");
         return;
     } else {
         new_dir = d.into_iter().peekable().peek().map_or("/", |x| *x);
     };
     let root = Path::new(new_dir);
     if let Err(e) = env::set_current_dir(&root) {
         match e.kind() {
             ErrorKind::NotFound => eprintln!("yui: cd: No such file or directory"),
             ErrorKind::PermissionDenied => eprintln!("yui: cd: Permission denied"),
             _ => eprintln!("yui: cd: {}", e),
         }
     }
}

pub fn echo(s: Vec<&str>) {
    println!("{}", s.join(" "));
}

//pub fn alias(s: Vec<&str>) {
    //let parts = s.split('X');
//}
