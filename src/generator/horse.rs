#![allow(unused)]

use rand::Rng;

struct Horse {
    name: String,
    age: u8,
    height: u8,
    weight: u16,
    speed: f32,
}

impl Horse {
    fn new(name: &str, speed: u32, age: u8, height: u8, weight: u16) -> Horse {
        Horse {
            name: name.to_string(),
            age,
            height,
            weight,
            speed: speed as f32,
        }
    }

    fn race(&self) -> f32{
        let random_factor = rand::thread_rng().gen_range(1..=10);
        return self.speed + random_factor as f32;
    }
}

fn genOdds(winners: Vec<String>){
    let speed1 = winners.iter().map(|x| x == &"Winx".to_string()).count();
    println!("{}", speed1);

}

fn main(){

    let horses = vec![
        Horse::new("Winx", 53, 8, 100, 52),
        Horse::new("Black Caviar", 55, 7, 100, 52),
        Horse::new("Phar Lap", 54, 6, 100, 52),
        Horse::new("Makybe Diva", 54, 5, 100, 52),
        Horse::new("Kingston Town", 51, 4, 100, 52),
    ];

    println!("Horse Racing Simulation");
    let mut max_speed = 0.0;
    let mut winner = String::new();
    let mut winners = vec![];
    for _ in 0..100 {
        for horse in &horses {
            let result = horse.race();

            if result > max_speed {
                max_speed = result;
                winner = horse.name.clone();
            }

            winners.push(winner.clone());

        }
    }
    println!("{:?}", winners);
    genOdds(winners);
}
