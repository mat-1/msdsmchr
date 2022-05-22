pub mod render;

fn main() {
    let img = image::open("py5.png").unwrap();

    // for i in 0..1000 {
    //     render::to_3d_head(&img);
    // }

    let rendered = render::to_3d_head(&img);

    rendered.save("head.png").unwrap();
}
