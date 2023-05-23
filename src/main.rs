// use sqlite;
// use scraper
use scraper::{Html, Selector};
use std::time::{SystemTime, UNIX_EPOCH};

fn get_hn() {
    let connection = sqlite::open("test.db").unwrap();

    let response = reqwest::blocking::get("https://news.ycombinator.com/").unwrap().text().unwrap();

    // parse the HTML document
    let doc_body = Html::parse_document(&response);

    // select the elements with athing class
    let athing = Selector::parse(".athing").unwrap();
    let mut cur_rank = 1;
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    for athing in doc_body.select(&athing) {
        let id = athing.value().attr("id");
        println!("{}", id.unwrap());
        let mut db = connection.prepare("SELECT (peak_rank) FROM posts WHERE id=?;").unwrap();
        db.bind((1, id)).unwrap();
        db.next().unwrap();
        println!("{}", db.read::<String>(0).unwrap());


        let mut db = connection.prepare("INSERT INTO posts (id, peak_rank, peak_time) VALUES (?, ?, ?);").unwrap();
        db.bind((1, id)).unwrap(); // db.bin((1, id))?;
        db.bind((2, cur_rank)).unwrap();
        db.bind((3, time as i64)).unwrap();
        db.next().unwrap();
        cur_rank += 1;
    }
}

fn main() {
    // Create connection to database
    let connection = sqlite::open("test.db").unwrap();

    // Create db table if not already existing
    let query = "CREATE TABLE IF NOT EXISTS posts (
        id INTEGER,
        peak_rank INTEGER,
        peak_time INTEGER
    );";
    connection.execute(query).unwrap();

    // Update database with current hackernews data
    get_hn();
}

