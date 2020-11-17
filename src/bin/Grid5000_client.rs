extern crate reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // reserve_node().await;
    // delete_job("1415127").await;
    get_grid5000().await;
}

async fn reserve_node() -> Result<(), reqwest::Error> {

    let api_url = "https://api.grid5000.fr/3.0/sites/rennes/jobs/?pretty";
    let username = "batek";
    let password = "sQ}}JdrHG5ABQXz1";

    let mut map = HashMap::new();
    map.insert("resources", "nodes=2,walltime=02:00");
    map.insert("command", "sleep 7200");

    let client = reqwest::Client::new();


    let res = client.post(api_url)
                                 .json(&map)
                                 .basic_auth(username, Some(password))
                                 .send()
                                 .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Move and borrow value of `res`
    let response_body = res.text().await?;
    println!("Body:\n{}", response_body);


    Ok(())
}

async fn get_grid5000() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.grid5000.fr/3.0/?pretty")
                    .basic_auth("batek", Some("sQ}}JdrHG5ABQXz1"))
                    .send()
                    .await?;

    // Move and borrow value of `res`
    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

async fn delete_job(job_to_delete : &str) -> Result<(), reqwest::Error> {

    let api_url = "https://api.grid5000.fr/3.0/sites/rennes/jobs/";

    let client = reqwest::Client::new();
    let res = client.delete(format!("{}{}", api_url, job_to_delete).as_str())
                    .basic_auth("batek", Some("sQ}}JdrHG5ABQXz1"))
                    .send()
                    .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Move and borrow value of `res`
    let response_body = res.text().await?;
    println!("Body:\n{}", response_body);
                    
    Ok(())
}