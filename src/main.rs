use scraper::{Html, Selector};
use std::time::{SystemTime, UNIX_EPOCH};

fn get_hn() {
    let connection = sqlite::open("hnpeaks.db").unwrap();

    let response = reqwest::blocking::get("https://news.ycombinator.com/").unwrap().text().unwrap();

    // parse the HTML document
    let doc_body = Html::parse_document(&response);

    // select the elements with athing class
    let athing = Selector::parse(".athing").unwrap();
    let mut cur_rank = 1;
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    for athing in doc_body.select(&athing) {
        let id = athing.value().attr("id");

        // Check if the post has already been ranked
        let mut db = connection.prepare("SELECT peak_rank FROM posts WHERE id=?;").unwrap();
        db.bind((1, id)).unwrap();
        db.next().unwrap();
        let peak_rank = db.read::<i64, usize>(0).unwrap();

        if peak_rank == 0 {
            // Add new post to table 
            db = connection.prepare("INSERT INTO posts (id, peak_rank, peak_time) VALUES (?, ?, ?);").unwrap();
            db.bind((1, id)).unwrap(); // db.bin((1, id))?;
            db.bind((2, cur_rank)).unwrap();
            db.bind((3, time as i64)).unwrap();
            db.next().unwrap();
            println!("NEW {} at {}", id.unwrap(), cur_rank);
        } else if cur_rank < peak_rank {
            // Update peak_rank in table if cur_rank is better
            db = connection.prepare("UPDATE posts SET peak_rank=?, peak_time=? WHERE id=?;").unwrap();
            db.bind((1, cur_rank)).unwrap(); // db.bin((1, id))?;
            db.bind((2, time as i64)).unwrap();
            db.bind((3, id)).unwrap();
            db.next().unwrap();
            println!("IMPROVED {} from {} -> {}", id.unwrap(), peak_rank, cur_rank);
        }

        // Increment cur_rank counter
        cur_rank += 1;
    }
    println!("Updated at {}", time);
}

#[derive(serde::Deserialize)]
struct GetPageParams {
    pub id: String,
}

impl Default for GetPageParams {
    fn default() -> Self {
        Self { id: "-1".to_string() }
    }
}

async fn get_page(mut req: tide::Request<()>) -> tide::Result {
    let params: GetPageParams = req.query()?;
    let id = params.id;
    let page_txt = std::fs::read_to_string("static/index.html");
    Ok(page_txt);
    //Ok(format!("Hello, {}", "luke").into())
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    // Create connection to database
    let connection = sqlite::open("hnpeaks.db").unwrap();

    // Create db table if not already existing
    let query = "CREATE TABLE IF NOT EXISTS posts (
        id INTEGER,
        peak_rank INTEGER,
        peak_time INTEGER
    );";
    connection.execute(query).unwrap();

    std::thread::spawn(move || {
        // Update every minute until killed
        loop {
            // Update database with current hackernews data
            get_hn();
            // Wait 2 minutes to update again
            std::thread::sleep(std::time::Duration::from_secs(120));
        }
    });

    let mut app = tide::new();
//    app.at("/").serve_file("static/index.html")?;
    app.at("/").get(get_page);
    app.at("/static").serve_dir("static/")?;
    app.listen("127.0.0.1:5000").await?;

    Ok(())
}

