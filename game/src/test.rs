use std::fs::File;
use std::io::{self, BufRead};

mod utils;
use crate::utils::Brick;

fn open_file() -> Vec<Brick> {
    // Spécifie le chemin du fichier
    let path = "levels/test.txt";

    // Ouvre le fichier
    let file = File::open(path).unwrap();

    // Crée un buffer pour lire le fichier ligne par ligne
    let reader = io::BufReader::new(file);

    let mut bricks:Vec<Brick> = Vec::new();
    // Lit et affiche chaque ligne
    let mut ligne= 0;
    for line in reader.lines() {
        match line {
            Ok(content) => {                
                let tmp : Vec<&str> =  content.split(" ").collect();
                for i in 0..tmp.len() {
                    match tmp[i].parse::<u32>() {
                        Ok(nombre) => bricks.push(Brick::new(ligne as i32 ,i as i32, nombre as i32)),
                        _ => {}, 
                    }
                }
            }
            _ => {}, 
        }
        ligne = ligne + 1;
    }
    bricks
}

fn main(){

}
