pub mod mojang;
pub mod render;

use std::io::Cursor;
use worker::*;

async fn index(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::from_html(
        "<h1>mat's super duper simple minecraft head renderer</h1>\
        <h2>Usage:</h2>\
        <ul>
        <li>/2d/&lt;id&gt; - returns an 8x8 image of the front of the player's Minecraft head</li>\
        <li>/3d/&lt;id&gt; - returns a 128x128 image of the player's Minecraft head, the same way it'd look in a Minecraft inventory</li>\
        </ul>
        <p>You can use either an undashed player UUID or a resource ID.</p>\
        <p><a href=\"https://github.com/mat-1/msdsmchr\">View source</a></p>"
    )
}

async fn make_2d_head(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = ctx.param("id").unwrap();
    let skin_bytes = mojang::download_from_id(&id).await.unwrap();
    let skin_image = render::to_2d_head(&image::load_from_memory(&skin_bytes).unwrap());
    let mut buf = Cursor::new(Vec::new());
    skin_image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    let mut response = Response::from_bytes(buf.into_inner())?;
    let headers = response.headers_mut();
    headers.set("Content-Type", "image/png")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Cache-Control", "public, max-age=86400")?;
    Ok(response)
}

async fn make_3d_head(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = match ctx.param("id") {
        Some(id) => id,
        None => return Response::error("Bad Request", 400),
    };
    let skin_bytes = mojang::download_from_id(&id).await.unwrap();
    let skin_image = render::to_3d_head(&image::load_from_memory(&skin_bytes).unwrap());
    let mut buf = Cursor::new(Vec::new());
    skin_image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    let mut response = Response::from_bytes(buf.into_inner())?;
    let headers = response.headers_mut();
    headers.set("Content-Type", "image/png")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Cache-Control", "public, max-age=86400")?;

    Ok(response)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    console_error_panic_hook::set_once();

    router
        .get_async("/", index)
        .get_async("/2d/:id", make_2d_head)
        .get_async("/3d/:id", make_3d_head)
        .run(req, env)
        .await
}
