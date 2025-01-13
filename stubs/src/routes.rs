use actix_web::{get, web, Error, HttpRequest};
use actix_files as fs;
use actix_session::SessionExt;
use crate::http::controllers::{admin_controller, home_controller, auth_controller};
use crate::http::middlewares::auth_middleware::AuthMiddleware;

#[get("/public/{filename:.*}")]
pub async fn static_file(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = fs::NamedFile::open(std::path::Path::new("./dist").join(path.clone()))?;

    Ok(file)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // GET /home
        .service(home_controller::home)
        // GET /signin
        .service(auth_controller::signin)
        // POST /signin
        .service(auth_controller::signin_post)
        // GET /signout
        .service(auth_controller::signout)
        // GET /admin
        .service(
            web::scope("/admin")
                .wrap(AuthMiddleware)
                .service(admin_controller::dashboard)
                .service(admin_controller::settings)
        )
        .service(static_file);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use actix_web::{http, test, App};
    use super::*;

    #[actix_web::test]
    async fn test_static_file_ok() {
        let app = test::init_service(
            App::new().service(static_file)
        ).await;

        fs::create_dir_all("./dist").unwrap();
        fs::write("./dist/styles.css", "").unwrap();

        let req = test::TestRequest::get().uri("/public/styles.css").to_request();
        let resp = test::call_service(&app, req).await;

        fs::remove_file("./dist/styles.css").unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_static_file_not_found() {
        let app = test::init_service(
            App::new().service(static_file)
        ).await;

        let req = test::TestRequest::get().uri("/public/not-existing-styles.css").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }
}
