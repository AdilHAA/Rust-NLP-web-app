use actix_web::{get, web,post, App, HttpServer,HttpResponse, Responder};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct InputText{
    text:String
}

#[derive(Serialize, Deserialize)]
struct ResultFromAnalyze{
    tag: String,
    summarization: String,
    sentiment_result: String,
    person: Vec<String>,
    location: String,
    organization: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, welcome to our Rust API!"
}

#[post("/analyze")]
async fn analyze(data: web::Json<InputText>) ->impl Responder {

    let _text= &data.text;
    //Обработка текста -----
    
     let response = ResultFromAnalyze {
        tag: "Example tag".to_string(),
        summarization: "Example summarization".to_string(),
        sentiment_result: "Neutral".to_string(),
        person: vec!["John Doe".to_string()],
        location: "New York".to_string(),
        organization: "Example Corp".to_string(),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(analyze)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}