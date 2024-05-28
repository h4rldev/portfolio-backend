use ntex::http::{Response, StatusCode};
use ntex::web::{get, Error as WebError, HttpRequest, HttpResponse};
use ntex_files::NamedFile;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub async fn fourofour() -> Result<HttpResponse, WebError> {
    let mut content = String::new();

    let fourofour_path = Path::new("./html").join("notfound.html");

    if fourofour_path.is_file() {
        let mut fourofour_file = File::open(fourofour_path)?;
        fourofour_file.read_to_string(&mut content)?;
        return Ok(HttpResponse::NotFound()
            .content_type("text/html")
            .body(content));
    }

    return Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body("<h1> 404 Not Found <h1>"));
}

#[get("/")]
pub async fn index() -> Result<HttpResponse, WebError> {
    let mut content = String::new();
    let index_path = Path::new("./html").join("index.html");

    let mut file = File::open(index_path)?;
    file.read_to_string(&mut content)?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(content));
}

#[get("/blog")]
pub async fn blog() -> Result<HttpResponse, WebError> {
    let mut content = String::new();
    let blog_path = Path::new("./html").join("underconstruction.html");

    let mut file = File::open(blog_path)?;
    file.read_to_string(&mut content)?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(content));
}

#[get("/projects")]
pub async fn projects() -> Result<HttpResponse, WebError> {
    let mut content = String::new();
    let projects_path = Path::new("./html").join("underconstruction.html");

    let mut file = File::open(projects_path)?;
    file.read_to_string(&mut content)?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(content));
}

#[get("/cv")]
pub async fn cv() -> Result<HttpResponse, WebError> {
    let cv_path = Path::new("./assets").join("Oscar Sjödin Jansson - CV.pdf");
    if cv_path.exists() {
        Ok(Response::build(StatusCode::OK)
            .header("Content-Type", "application/pdf")
            .header(
                "Content-Disposition",
                format!(
                    "attachment; filename=\"{}\"",
                    cv_path.file_name().unwrap().to_str().unwrap()
                ),
            )
            .finish())
    } else {
        Ok(Response::build(StatusCode::NOT_FOUND).body(b"File not found".to_vec()))
    }
}

pub async fn files(req: HttpRequest) -> Result<HttpResponse, WebError> {
    let path: PathBuf = req.match_info().query("filename").parse()?;
    let file = NamedFile::open(PathBuf::from("./").join(path));
    if file.is_ok() {
        return Ok(file?.into_response(&req));
    }
    fourofour().await
}
