pub mod mojang;
pub mod render;
pub mod routes;

use actix_web::{web, App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Running on 127.0.0.1:26738");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index))
            .route("/3d/{id}", web::get().to(routes::make_3d_head))
            .route("/2d/{id}", web::get().to(routes::make_2d_head))
    })
    .bind(("0.0.0.0", 26738))?
    .run()
    .await
}

// fn main() {
//     let skin = image::open("skin.png").unwrap();

//     for _ in 0..1000 {
//         let skin_image = render::to_3d_head(&skin);
//         let mut buf = Cursor::new(Vec::new());
//         skin_image
//             .write_to(&mut buf, image::ImageOutputFormat::Png)
//             .unwrap();
//     }
//     // skin_image.save("head.png").unwrap();
// }
