use iron::middleware::Handler;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status::Status;
use staticfile::Static;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;
use std::path::PathBuf;

static FILE_FOLDER: &str = "www";

fn main() {
    let static_file_handler = Static::new(Path::new(FILE_FOLDER));

    Iron::new(move |request: &mut Request| browse(request, &static_file_handler))
        .http("0.0.0.0:3000")
        .unwrap();
}

fn browse(request: &mut Request, static_file_handler: &Static) -> IronResult<Response> {
    let file_response = static_file_handler.handle(request);
    if file_response.is_ok() {
        return file_response;
    }

    let mut path = PathBuf::new();
    path.push(FILE_FOLDER);
    for path_element in request.url.path() {
        path.push(path_element);
    }

    match fs::read_dir(&path) {
        Ok(paths) => list_paths(request, paths),
        Err(_) => Ok(Response::with((Status::NotFound, "Invalid path"))),
    }
}

fn list_paths(request: &Request, paths: ReadDir) -> IronResult<Response> {
    let mut response = String::new();
    response.push_str(&format!("<div>Content of {}</div>", request.url));
    for path in paths {
        let to_push = match path {
            Ok(file) => format!(
                r#"<a href="{0}">{0}</a>"#,
                file.path().file_name().unwrap().to_str().unwrap()
            ),
            Err(err) => format!("{}", err),
        };
        response.push_str(&format!("<div>{}\n</div>", to_push));
    }
    let mime: Mime = "text/html".parse().unwrap();
    response = format!("<html><meta></meta><body>{}</body></html>", response);
    Ok(Response::with((Status::Ok, response, mime)))
}
