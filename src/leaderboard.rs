extern crate reqwest;
extern crate table_extract;

async fn download_leaderboard() -> Result<String, reqwest::Error> {
    let text = reqwest::get("https://torn.space/leaderboard/")
        .await?
        .text()
        .await?;
    Ok(text)
}

pub async fn test() {
    let table = match download_leaderboard().await {
        Ok(html) => table_extract::Table::find_first(&html),
        Err(e) => {
            eprintln!("Failed to retrieve leaderboard. {}", e);
            return ();
        }
    }
    .unwrap();

    let mut index = 1;
    for row in &table {
        println!("{}. {:#?}", index, row);
        index += 1;
    }
}
