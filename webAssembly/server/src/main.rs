pub mod utils;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, App, Error, HttpResponse, HttpServer, Responder};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/upload")]
async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("src/static/{}", f.file_name.unwrap());
        println!("saving to {path}");
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())
}

#[derive(serde::Deserialize, Debug)]
struct Info {
    filename: String,
    size: u8,
}

#[get("/img")]
async fn get_img(info: actix_web::web::Query<Info>) -> impl Responder {
    println!("info: {:?}", info);

    let img_data = utils::run(info.size, &info.filename);

    HttpResponse::Ok()
        .content_type("image/jpeg")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", info.filename),
        ))
        .body(img_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_img).service(save_files))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
