use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use sqlite::State;
use serde::Serialize;


#[derive(Serialize)]
struct Result {
    site: String,
    avg_response_time: String,
    uptime: String,
}

#[get("/tracker")]
async fn targets() -> impl Responder {
    let connection = sqlite::open("/usr/shared/sites.db").unwrap();

    let mut statement = connection
        .prepare("SELECT avg(elapsed), site, (count(IIF(code = '200 OK', 1, null))*100)/count(*) as 'uptime' FROM sites GROUP BY site;")
        .unwrap();

    let mut res: Vec<Result> = Vec::new();

    while let State::Row = statement.next().unwrap() {
        let r = Result {site: statement.read::<String>(1).unwrap(), avg_response_time: statement.read::<String>(0).unwrap(), uptime: statement.read::<String>(2).unwrap()};
        res.push(r);
    }

    web::Json(res)
}

#[get("/hr_tracker")]
async fn hr_targets() -> impl Responder {
    let connection = sqlite::open("/usr/shared/sites.db").unwrap();

    let mut statement = connection
        .prepare("SELECT avg(elapsed), site, (count(IIF(code = '200 OK', 1, null))*100)/count(*) as 'uptime' FROM sites GROUP BY site;")
        .unwrap();

    let mut res = String::new();

    while let State::Row = statement.next().unwrap() {
        res.push_str(format!("{}, avg resp time: {}sec, uptime 30 min: {}%\n",
                             statement.read::<String>(1).unwrap(),
                             statement.read::<String>(0).unwrap(),
                             statement.read::<String>(2).unwrap()
        ).as_str());
    }

    HttpResponse::Ok().body(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(targets)
            .service(hr_targets)
    })
        .bind(("127.0.0.1", 8184))?
        .run()
        .await
}
