pub mod render;

fn main() {
    let img = image::open("skin.png").unwrap();

    // for _ in 0..1000 {
    //     render::to_3d_head(&img);
    // }

    let rendered = render::to_3d_head(&img);

    rendered.save("head.png").unwrap();
}
