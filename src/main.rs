use serde::{Deserialize, Serialize};
use std::io;

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

#[allow(unused_must_use)]
fn main() {
    loop {
        let user_input = get_user_input();
        if user_input.eq_ignore_ascii_case("EXIT") {
            println!("Program terminated !!!");
            break;
        }
        match user_input.len() {
            0 => get_all(), //TODO
            _ => get_detail(user_input),
        };
    }
}

#[tokio::main]
async fn get_detail(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let full_url = format!("https://api.le-systeme-solaire.net/rest/bodies/{}", name);

    let response = reqwest::get(full_url).await?;
    
    if response.status() != 200 {
        println!("Error Status: {}", response.status());
        return Ok(());
    }
    
    // println!("Headers:\n{:#?}", response.headers());
    let json_body = response.text().await?;
    let body: Body = serde_json::from_str(&json_body).unwrap();

    list_body_details(body);
    Ok(())
}

#[tokio::main]
async fn get_all() -> Result<(), Box<dyn std::error::Error>> {
    let full_url = format!("https://api.le-systeme-solaire.net/rest/bodies/");
    let response = reqwest::get(full_url).await?;
    // println!("Status: {}", response.status());
    // println!("Headers:\n{:#?}", response.headers());
    if response.status() != 200 {
        println!("Error Status: {}", response.status());
        return Ok(());
    }
    let json_body = response.text().await?;
    // let body = serde_json::from_str(&json_body).unwrap();

    list_all(json_body);
    Ok(())
}

fn get_user_input() -> String {
    println!("\n-> Hit enter to list all available heavenly bodies.");
    println!("-> Input name of heavenly body: ");
    println!("-> To terminate program type \"exit\" ");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name.trim().to_string()
}

fn list_all(detail: String) {
    println!("\n*-------------------------------------------------------*");
    println!("{}", detail);
    println!("*---------------------------------------------------------*");
}

fn list_body_details(detail: Body) {
    println!("\n*-------------------------------------------------------*");
    println!("| The english name of the body is {}", detail.englishName);
    println!("| The scientific name of the body is {}", detail.name);
    match detail.isPlanet {
        true => println!("| It is a planet"),
        false => println!("| It is not a planet"),
    }
    println!(
        "| The semi-major axis of {} is around {} km",
        detail.englishName, detail.semimajorAxis
    );
    println!(
        "| Inclination to ecliptic is around {} deg",
        detail.inclination
    );
    println!("*---------------------------------------------------------*");
}
