pub mod mojang;
pub mod render;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::io::Cursor;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .body(
            "mat's super duper simple Minecraft head renderer\n\n\
            Usage:\n\
              /2d/<id>.png - returns an 8x8 image of the front of the player's Minecraft head\n\
              /3d/<id>.png - returns a 128x128 image of the player's Minecraft head, the same way it'd look in a Minecraft inventory\n\n\
            You can use either an undashed player UUID or a resource ID.\n\n\
            https://github.com/mat-1/ TODO: add link to repo"
        )
}

#[get("/2d/{id}.png")]
async fn make_2d_head(id: web::Path<String>) -> impl Responder {
    let skin_bytes = mojang::download_from_id(&id).await.unwrap();
    let skin_image = render::to_2d_head(&image::load_from_memory(&skin_bytes).unwrap());
    let mut buf = Cursor::new(Vec::new());
    skin_image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    HttpResponse::Ok().body(buf.into_inner())
}

#[get("/3d/{id}.png")]
async fn make_3d_head(id: web::Path<String>) -> impl Responder {
    let skin_bytes = mojang::download_from_id(&id).await.unwrap();
    let skin_image = render::to_3d_head(&image::load_from_memory(&skin_bytes).unwrap());
    let mut buf = Cursor::new(Vec::new());
    skin_image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    HttpResponse::Ok().body(buf.into_inner())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Running :)");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(make_3d_head)
            .service(make_2d_head)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// fn main() {
//     let skin = image::open("skin.png").unwrap();
//     let head = render::to_3d_head(&skin);
//     head.save("head.png").unwrap();
// }
