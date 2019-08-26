use clap::{Arg, App};
use reqwest::{Client, StatusCode, Error};
use chrono::{DateTime, Local};

#[derive(serde::Deserialize)]
struct LoginResponse {
    USER_TOKEN: String,
}

fn main() {
    let now: DateTime<Local> = Local::now();
    let date = now.format("%Y-%m-%d");
    let date_str = format!("{}", date);

    let matches = App::new("Rust-time")
        .arg(Arg::with_name("email")
            .required(true))
        .arg(Arg::with_name("password")
            .required(true))
        .arg(Arg::with_name("date")
            .long("date")
            .value_name("YYYY-MM-DD")
            .default_value(&date_str)
            .takes_value(true))
        .arg(Arg::with_name("checkin")
            .long("in")
            .value_name("HH:MM:SS | false")
            .default_value("false")
            .takes_value(true))
        .arg(Arg::with_name("break")
            .long("break")
            .value_name("HH:MM:SS | false")
            .default_value("false")
            .takes_value(true))
        .arg(Arg::with_name("return")
            .long("return")
            .value_name("HH:MM:SS | false")
            .default_value("false")
            .takes_value(true))
        .arg(Arg::with_name("checkout")
            .long("out")
            .value_name("HH:MM:SS | false")
            .default_value("false")
            .takes_value(true))
        .get_matches();

    let email = matches.value_of("email").unwrap();
    let password = matches.value_of("password").unwrap();
    let chk_in = matches.value_of("checkin").unwrap();
    let chk_break = matches.value_of("break").unwrap();
    let chk_return = matches.value_of("return").unwrap();
    let chk_out = matches.value_of("checkout").unwrap();

    let client = Client::new();
    let token = match login(&client, email, password) {
        Ok(val) => val,
        Err(err) => {
            println!("error when logging in: '{}'. Wrong credentials?", err);
            return;
        }
    };

    if chk_in != "false" {
        let timestamp = format!("{} {}", date_str, chk_in);
        let status = submit_clocking(&client, &token, 0, &timestamp).unwrap();
        if status != StatusCode::CREATED {
            println!("checkin failed. Status code: {}", status);
            return;
        }
        println!("Checked in at {}", chk_in);
    }
    if chk_break != "false" {
        let timestamp = format!("{} {}", date_str, chk_break);
        let status = submit_clocking(&client, &token, 2, &timestamp).unwrap();
        if status != StatusCode::CREATED {
            println!("break failed. Status code: {}", status);
            return;
        }
        println!("Break at {}", chk_break);
    }
    if chk_return != "false" {
        let timestamp = format!("{} {}", date_str, chk_return);
        let status = submit_clocking(&client, &token, 3, &timestamp).unwrap();
        if status != StatusCode::CREATED {
            println!("return failed. Status code: {}", status);
            return;
        }
        println!("Returned at {}", chk_return);
    }
    if chk_out != "false" {
        let timestamp = format!("{} {}", date_str, chk_out);
        let status = submit_clocking(&client, &token, 1, &timestamp).unwrap();
        if status != StatusCode::CREATED {
            println!("checkout failed. Status code: {}", status);
            return;
        }
        println!("Checked out at {}", chk_out);
    }
    println!("Done");
}

fn login(client: &Client, email: &str, password: &str) -> Result<String, reqwest::Error> {
    let res: LoginResponse = match client.post("https://newapi.intratime.es/api/user/login")
        .form(&[("user", email), ("pin", password)])
        .send()
        .unwrap()
        .json() {
        Ok(val) => val,
        Err(err) => {
            println!("error when logging in: '{}'. Wrong credentials?", err);
            return Err(err);
        }
    };

    Ok(res.USER_TOKEN)
}

fn submit_clocking(client: &Client, token: &String, action: usize, timestamp: &str) -> Result<StatusCode, Error> {
    let action_str = action.to_string();

    return client.post("https://newapi.intratime.es/api/user/clocking")
        .header("token", token)
        .form(&[
            ("user_action", action_str),
            ("user_timestamp", String::from(timestamp)),
            ("user_gps_coordinates", String::from("41.4050371,2.1926044")),
            ("user_use_server_time", String::from("false"))
        ])
        .send()
        .and_then::<StatusCode, _>(|response| {
            Result::Ok(response.status())
        });
}