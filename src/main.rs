use core::panic;
use std::{io::Read, path::PathBuf};

fn do_the_thing(path: PathBuf) -> String {
    let mut file_contents = String::new(); // haystack
        
    std::fs::File::open(path)
        .expect("Expected to be able to open file")
        .read_to_string(&mut file_contents)
        .expect("Expected to be able to read file contents to string");

    let mut links: Vec<String> = vec![];

    let re = regex::Regex::new(r#"<a href=".*id=(\d*)" data-type="Link">"#).expect("Expected a valid regex value");

    for cap in re.captures_iter(&file_contents) {
        links.push(cap.get(1).unwrap().as_str().to_string());
    }
    
    let mut final_string = String::from("-mod=\"");
    
    for x in links {
        final_string.push_str(format!("mods/{x};").as_str());
    }
    
    final_string.push('"');
    
    return final_string;
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 0 {

        let mut exe_dir = std::env::current_exe().expect("Expected to be able to get exe path");
        
        if !exe_dir.pop() {
            panic!("Could not get exe home directory.");
        }
        
        println!("{}", do_the_thing(exe_dir.join("mods/mods.html")));

    } else if args.len() == 1 {

        println!("{}", do_the_thing(args[0].clone().into()));

    }
}
