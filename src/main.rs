use std::env;
use std::fs;
use std::path::PathBuf;


fn kill_dir(p:&PathBuf){
    match fs::read_dir(p) {
        Ok(paths) => {
        for path in paths {
            if path.is_err() { continue; }
            let pathl = path.unwrap().path();
            println!("Processing \"{}\"", pathl.display());
            if pathl.is_dir() {
                let _ = fs::remove_dir_all(pathl);
            }else {
                let _ = fs::remove_file(pathl);
            }
        }},
        Err(e) => println!("Small dir listen error: {e}"),
    };
}

fn main() {
    println!("Rust \".cargo\\registry\" cleanup\n<c> Sharkbyteprojects");
    match env::home_dir() {
        Some(path) => {
            let cargoCache = path.join(".cargo").join("registry");
            println!("Your cargo cache directory, probably: \"{}\"", cargoCache.display());
            if cargoCache.exists() {
                println!("¡FOUND!");
                kill_dir(&cargoCache);
            }else{
                println!("Cargo cache dir didn't exist!");
            }},  
        None => println!("Impossible to get your home dir!"),
    }
}
