mod render;

fn main() {
    let img = image::open("py5.png").unwrap();

    let rendered = render::to_3d_head(&img);

    rendered.save("head.png").unwrap();
}
