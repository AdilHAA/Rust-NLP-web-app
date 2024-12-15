use actix_web::{web, App, HttpServer, HttpResponse, Responder, post};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use url::Url;
use std::error::Error;
use scraper::{Html, Selector};

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

fn is_valid_url(text: &str) -> bool {
    if let Ok(parsed_url) = Url::parse(text) {
        // Проверяем, что URL содержит mail.ru
        if let Some(host) = parsed_url.host_str() {
            // Если домен не содержит mail.ru, возвращаем false
            return host.contains("mail.ru");
        }
    }
    false
}

async fn parse(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let script_selector = Selector::parse("script").unwrap();

        for script in document.select(&script_selector) {
            if let Some(text) = script.text().next() {
                if text.contains("articleBody") {
                    if let Some(pos) = text.find("\"description\":") {
                        let desc = &text[pos..];
                        let end_pos = desc.find("genre").unwrap_or(desc.len());
                        let description = &desc[0..end_pos];
                        
                        if let Some(pos_text) = text.find("articleBody") {
                            let text_body = &text[pos_text..];
                            let end_text_pos = text_body.find("@").unwrap_or(text_body.len());
                            let extracted_text = &text_body[0..end_text_pos];

                            return Ok(format!("{}{}", description, extracted_text));
                        }
                    }
                }
            }
        }
        Ok("До связи".to_string())
    } else {
        Ok("До связи".to_string())
    }
}

async fn send_to_python(text: &String) -> Result<ResultFromAnalyze, Box<dyn Error>> {
    let client = Client::new();
    let url = "http://localhost:8000/analyze/";  // URL вашего FastAPI сервера

    let response = client
        .post(url)
        .json(&text)
        .send()
        .await?;

    if response.status().is_success() {
        let result: ResultFromAnalyze = response.json().await?;
        Ok(result)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to communicate with Python server")))
    }
}

#[post("/analyze")]
async fn analyze(data: web::Json<InputText>) -> impl Responder {
    let text = &data.text;

    // Отправляем текст на Python сервер для обработки
    match send_to_python(text).await {
        Ok(response) => HttpResponse::Ok().json(response),  // Возвращаем результат от Python
        Err(err) => HttpResponse::InternalServerError().body(format!("Ошибка: {}", err)),
    }
}

#[post("/analyze_from_url")]
async fn analyze_from_url(data: web::Json<InputText>) -> impl Responder {
    let url = &data.text;

    // Сначала валидация URL
    if !is_valid_url(url) {
        return HttpResponse::BadRequest().body("Неверный URL или URL не содержит 'mail.ru'.");
    }

    // Извлекаем текст с URL
    match parse(url).await {
        Ok(extracted_text) => {
            // Отправляем текст на сервер Python
            match send_to_python(&extracted_text).await {
                Ok(result) => HttpResponse::Ok().json(result),
                Err(e) => HttpResponse::InternalServerError().body(format!("Ошибка от сервера Python: {}", e)),
            }
        }
        Err(e) => {
            // В случае ошибки при извлечении текста из URL
            HttpResponse::InternalServerError().body(format!("Ошибка при извлечении текста: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(analyze)
            .service(analyze_from_url)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}