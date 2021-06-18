use serde::{Deserialize, Serialize};
use std::io;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct Body {
    id: String,
    name: String,
    englishName: String,
    isPlanet: bool,
    semimajorAxis: f64,
    perihelion: f64,
    aphelion: f64,
    eccentricity: f64,
    inclination: f64,
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct Head {
    bodies: Vec<Body>,
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
            0 => get_all(),
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

  
    let all_info: Head = serde_json::from_str(&json_body).unwrap();
    // println!("{:?}", all_info);

    list_all(all_info);
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

fn list_all(detail: Head) {
    let mut counter = 0;
    for model in detail.bodies {
        counter = counter + 1;
        println!("\n{:-<62}|", "|");
        print!("| S.no: {}", counter);
        print!("\t| ID: {}", model.id);
        print!("\t| Name: {:<22}|", model.name);
        if model.englishName.trim().len() != 0{
            print!("\n| English name: {:<46}|", model.englishName);
        }
    }
    print!("{:-<62}|", "|");
}

fn list_body_details(detail: Body) {
    println!("\n{:-<60}|", "|");
    println!("| The english name of the body is {:-<26}|", detail.englishName);
    println!("| The scientific name of the body is {:-<23}|", detail.name);
    match detail.isPlanet {
        true => println!("| It is a planet {:-<43}|",""),
        false => println!("| It is not a planet {:-<39}|", ""),
    }
    println!(
        "| The semi-major axis is around (kilometer) {:-<16}|", detail.semimajorAxis
    );
    println!(
        "| Inclination to ecliptic is around (Degree) {:-<15}|",
        detail.inclination
    );
    println!("{:-<60}|", "|");
}
