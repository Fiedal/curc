use std::path::Path;
use reqwest::blocking::Client;
use serde_json::Value;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Command {
    /// List all of the available currencies
    List,
}

#[derive(Parser, Debug)]
#[command(version, about="Simple currency converter", long_about = None)]
struct Args {
    /// Amount of a currency you want to convert
    value: Option<f64>,

    /// Base currency (from)
    /// input currency code
    #[arg(short, long)]
    from: Option<String>,

    /// Convert currency (to)
    /// input currency code
    #[arg(short, long)]
    to: Option<String>,

    /// Short output - shows only converted value
    #[arg(short, long, default_value_t = false)]
    soutput: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

fn get_convert(currency: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/{}.min.json", currency); 
    let response = client
        .get(&url)
        .send()?;
    let data: Value = response.json()?;
    Ok(data)
}

fn get_currencies() -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies.min.json";
    let response = client
        .get(url)
        .send()?;
    let data: Value = response.json()?;
    Ok(data)
}

fn run_list_currencies() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_currencies().map_err(|_err| {
        "Failed to get currencies. Check your internet connection.".to_string()
    })?;

    if let Value::Object(currencies_map) = data {
        let max_code_length = currencies_map.keys()
            .map(|key| key.len())
            .max()
            .unwrap_or(3);
        println!("\nAvailable currencies:");
        println!("{:<width$} : {}", "CODE", "NAME", width = max_code_length);
        println!("{:-<width$}---{:-<30}", "","", width = max_code_length);

        for (code, name_val) in currencies_map {
            if let Value::String(name) = name_val {
                println!("{:<width$} : {}",
                    code.to_uppercase(),
                    name,
                    width = max_code_length
                );
            }
        }
    } else {
        return Err("Unexpected data format from API.".into());
    }
    Ok(())
}

fn run_app(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let value = args.value.ok_or("Value is required for conversion")?;
    let from = args.from.ok_or("--from is required for conversion")?;
    let to = args.to.ok_or("--to is required for conversion")?;
    
    if value < 0.0 {
        return Err("Value must be a positive number.".into());
    }

    let currency = from.to_lowercase();
    let to_currency = to.to_lowercase();

    let data = get_convert(&currency).map_err(|_err| {
        "Failed to connect with API server. Check your internet connection.".to_string()
    })?;

    let base_price = data
        .get(&currency)
        .ok_or(format!("API error with base currency: {}", currency))?;
    
    let to_price = base_price
        .get(&to_currency)
        .ok_or(format!("API error with convert to currency: {}", to_currency))?;

    let price_f64 = to_price
        .as_f64()
        .ok_or("Error while formating data from API: price is not a number?")?;

    let result: f64 = value * price_f64;

    if args.soutput {
        println!("{result:.2}");
    } else {
        println!("{:.2} {}: {:.2} {}", value, currency.to_uppercase(), result, to_currency.to_uppercase());
    }
    Ok(())
}

fn main() {
    let program_name = std::env::args()
        .next()
        .as_deref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("currency_converter")
        .to_string();
    
    let args = Args::parse();
    
    let result = match args.command {
        Some(Command::List) => {
            run_list_currencies()
        }

        None => {
            run_app(args)
        }
    };

    if let Err(e) = result {
        eprintln!("{program_name}: Error: {e}");
        std::process::exit(1);
    }
}
