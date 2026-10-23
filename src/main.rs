use dotenv::dotenv;
use hyper::body::Buf;
use hyper_tls::{header,body,client,request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize,Serialize};
use spinners::{Spinner,Spinners};
use std::env;
use std::io::{stdin, stdout, write};


#[derive(Deserialize, Debug)]
    struct GAIResponse{
        id:Option<String>,
        object: Option<String>,
        created: Option<u64>,
        model: Option<String>,
        choices: Vec<GAIChoices>
    }

#[derive(Deserialize, Debug)]
    struct GAIChoices{
        text: string,
        index:u8,
        logrobs: Option<u8>,
        finish_reason: String,
    }

#[derive(Serialize, Debug)]
    struct GAIRequest{
        prompt:String,
        max_tokens: u16,

    }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + send + Sync>>{
    dotenv().ok();
    let https = HttpConnector::new();
    let client = Client::builder().build(https);
    let uri = "";
    let premable = "Generate a sql code for the given statement";
    let gai_token:String = env::var("GAI_TOKEN").unwarp();
    let auth_handler_val = format!("Bearer {}", gai_token);
    println!("{esc}c",esc = 27 as char);
    loop{
        print!(">");
        stdout().flush().unwarp();
        let mut user_text = String::new();
        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
            println!("");


//Spinners to load the animation in the cli
        let sp = Spinner::new(&Spinners::Dots12, "\t\tGemini is Thinking...".into());
        let gai_request = GAIRequest{
            prompt: format!("{} {}", premable, user_text),
            max_tokens: 1000,
        };
             let body = Body::from(serde_json::to_vec(&oai_request)?);
              let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val) 
            .body(body) 
            .unwrap(); 

            let res = client.request(req).await?;
            let body = hyper::body::aggregate(res).await?;
            let json = GAIResponse = serde_json::from_reader(body.reader())?;
            sp.stop();
            println!(""); 
            println!("{}", json.choices[0].text);
    }
    Ok(())
}