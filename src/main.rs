use std::io;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)] 
#[derive(Debug, Deserialize, Serialize)]
struct Body {
    id: String,
    name: String,
    englishName: String,
    isPlanet: bool,
    semimajorAxis: u64,
    perihelion: u64,
    aphelion: u64,
    eccentricity: f64,
    inclination: f64,
}

fn main() {
    let user_input = get_user_input();

    match get_detail(user_input) {
        Ok(_) => println!("\nSuccess"),
        Err(_) => println!("Error during operation"),
    };
}

#[tokio::main]
async fn get_detail(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let full_url = format!("https://api.le-systeme-solaire.net/rest/bodies/{}", name);

    let response = reqwest::get(full_url).await?;
    // println!("Status: {}", response.status());
    // println!("Headers:\n{:#?}", response.headers());

    assert!(response.status().is_success());
    let json_body = response.text().await?;
    let body: Body = serde_json::from_str(&json_body).unwrap();

    

    print_body_details(body);

    Ok(())
}

fn get_user_input() -> String{
    println!("Please input name of heavenly body: ");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name.to_string()
}


fn print_body_details(detail: Body) {
    println!("====================================================");
    println!("The english name of the body is {}", detail.englishName);
    println!("The scientific name of the body is {}", detail.name);
    match detail.isPlanet {
        true => println!("It is a planet"),
        false => println!("It is not a planet"),
    }
    println!("The semi-major axis of {} is around {} km", detail.englishName, detail.semimajorAxis);
    println!("Inclination to ecliptic is around {} deg", detail.inclination);
    println!("====================================================");
}
