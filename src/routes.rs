use crate::{mojang, render};
use actix_web::{web, HttpResponse, Responder};
use std::io::Cursor;

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(
            "<h1>mat's super duper simple minecraft head renderer</h1>\
            <h2>Usage:</h2>\
            <ul>
            <li>/2d/&lt;id&gt; - returns an 8x8 image of the front of the player's Minecraft head</li>\
            <li>/3d/&lt;id&gt; - returns a 128x128 image of the player's Minecraft head, the same way it'd look in an inventory</li>\
            </ul>
            <p>You can use either an undashed player UUID or a resource ID.</p>\
            <p><a href=\"https://github.com/mat-1/msdsmchr\">View source</a></p>"
        )
}

pub async fn make_2d_head(id: web::Path<String>) -> impl Responder {
    let skin_bytes = mojang::download_from_id(&id).await.unwrap();
    let skin_image = render::to_2d_head(&image::load_from_memory(&skin_bytes).unwrap());

    let mut buf = Cursor::new(Vec::new());
    skin_image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();

    HttpResponse::Ok()
        .append_header(("Content-Type", "image/png"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Cache-Control", "public, max-age=14400"))
        .body(buf.into_inner())
}

pub async fn make_3d_head(id: web::Path<String>) -> impl Responder {
    let skin_bytes = mojang::download_from_id(&id).await.unwrap();
    let skin_image = render::to_3d_head(&image::load_from_memory(&skin_bytes).unwrap());
    let mut buf = Cursor::new(Vec::new());
    skin_image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/png"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Cache-Control", "public, max-age=14400"))
        .body(buf.into_inner())
}
