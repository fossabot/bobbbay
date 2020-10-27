use std::fs;
use rand::Rng;

fn main() {
    let filename = "data/data.json";

    let contents = "{
  \"interests\": [
    \"🍕 Pizza\",
    \"🖥 Programming\",
    \"🧠 Configuring Linux\",
    \"📚 Reading\"
  ]
}
"; //::read_to_string(filename)
        //.expect("Something went wrong reading the file");

    let json: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    let mut rng = rand::thread_rng().gen_range(0, 10);
    println!("{}", rng);

    
    println!("{}", json["interests"][rng]);
}
