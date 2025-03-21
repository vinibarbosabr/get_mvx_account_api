// Run the program and enter an address when prompted

use reqwest::Client;
use serde::{Deserialize, Serialize}; // Added Serialize
use std::io;

// The API returns a nested structure, so we need a wrapper (by Grok)
// Wrapper #1
#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    data: AccountData,
}

// Wrapper #2
#[derive(Deserialize, Serialize, Debug)]
struct AccountData {
    account: Account,
}

// The actual account data
#[derive(Deserialize, Serialize, Debug)]
struct Account {
    address: String,
    nonce: u32,
    balance: String, // Changed to String since API returns balance as string not an integer
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get user input
    println!("Enter address:");
    let mut address = String::new();
    io::stdin()
        .read_line(&mut address)
        .expect("Failed to read address");

    // Remove trailing newline and whitespace (Solution from Grok)
    let address = address.trim();

    // Get the URL with the user-provided address
    let url = format!("https://gateway.multiversx.com/address/{}", address);

    let account = get_account(&url).await?;
    println!("account = {:#?}", account);
    Ok(())
}

async fn get_account(url: &str) -> Result<Account, reqwest::Error> { 
    let client = Client::new();
    
    let response = client
        .get(url)
        .send()
        .await?
        .json::<ApiResponse>() // Deserialize into ApiResponse first (by Grok)
        .await?;

    Ok(response.data.account)
}

// This project was made following the tutorial
// "Rust | How To Make an HTTP Request in Rust? (Step-by-step)"
// by Andres Reales on Become Better Programmer
// https://www.becomebetterprogrammer.com/rust-how-to-make-an-http-request/
// Grok 3 helped me to fix the code and make it work with the API

// The API used in this project is the MultiversX API
// https://docs.multiversx.com/sdk-and-tools/rest-api/addresses

// Proudly build by Vini Barbosa (@vinibarbosabr on X)
// as part of a DOJO activity