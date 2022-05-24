pub mod mojang;
pub mod render;
pub mod routes;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    console_error_panic_hook::set_once();

    router
        .get_async("/", routes::index)
        .get_async("/2d/:id", routes::make_2d_head)
        .get_async("/3d/:id", routes::make_3d_head)
        .run(req, env)
        .await
}
