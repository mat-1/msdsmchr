use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::geometric_transformations::{warp, warp_into, Interpolation, Projection};

const SKEW_A: f32 = 26.0 / 45.0;
const SKEW_B: f32 = SKEW_A * 2.0;

/// How big each "section" on the original flat image is.
const SECTION_SIZE: u32 = 8;

fn crop_section(image: &DynamicImage, x: u32, y: u32) -> DynamicImage {
    let (x, y) = (x * SECTION_SIZE, y * SECTION_SIZE);
    let (width, height) = (SECTION_SIZE, SECTION_SIZE);
    let (x, y, width, height) = (x, y, width, height);
    image.crop_imm(x, y, width, height)
}

#[rustfmt::skip]
const TRANSFORM_TOP_BOTTOM_MATRIX: [f32; 9] = [
    1.0,     1.0,    0.0,
    -SKEW_A, SKEW_A, 0.0,
    0.0,     0.0,    1.0
];
#[rustfmt::skip]
const TRANSFORM_FRONT_BACK_MATRIX: [f32; 9] = [
    1.0,     0.0,    0.0,
    -SKEW_A, SKEW_B, SKEW_A,
    0.0,     0.0,    1.0
];
#[rustfmt::skip]
const TRANSFORM_RIGHT_LEFT_MATRIX: [f32; 9] = [
    1.0,     0.0,    0.0,
    SKEW_A,  SKEW_B, 0.0,
    0.0,     0.0,    1.0
];

fn main() {
    // Must be a multiple of SECTION_SIZE (8)
    let size: u32 = 128;
    let scale = (size / 20) as f32;

    // transparent image of size
    let mut out = ImageBuffer::from_pixel(size, size, Rgba([255, 255, 255, 255]));
    // let mut out = ImageBuffer::new(size, size);

    let img = image::open("steve.png").unwrap();

    let head_top = crop_section(&img, 1, 0);
    let head_top_projection = Projection::from_matrix(TRANSFORM_TOP_BOTTOM_MATRIX).unwrap()
        * Projection::translate(
            (size as f32) * (-40f32 / 256f32),
            (size as f32) * (83.0 / 256.0),
        )
        * Projection::scale(scale, scale);
    let mut head_top_warped = ImageBuffer::new(size, size);
    warp_into(
        &head_top.into_rgba8(),
        &head_top_projection,
        Interpolation::Nearest,
        Rgba([0, 0, 0, 0]),
        &mut head_top_warped,
    );
    imageops::overlay(&mut out, &head_top_warped, 0, 0);

    let head_front = crop_section(&img, 1, 1);
    let head_front_projection = Projection::from_matrix(TRANSFORM_FRONT_BACK_MATRIX).unwrap()
        * Projection::translate(
            (size as f32) * (132.5 / 256.0),
            (size as f32) * (178.0 / 256.0),
        )
        * Projection::scale(scale, scale);
    let mut head_front_warped = ImageBuffer::new(size, size);
    warp_into(
        &head_front.into_rgba8(),
        &head_front_projection,
        Interpolation::Nearest,
        Rgba([0, 0, 0, 0]),
        &mut head_front_warped,
    );
    imageops::overlay(&mut out, &head_front_warped, 0, 0);

    let head_right = crop_section(&img, 2, 1);
    let head_right_projection = Projection::from_matrix(TRANSFORM_RIGHT_LEFT_MATRIX).unwrap()
        * Projection::translate(
            (size as f32) * (121.0 / 256.0),
            (size as f32) * (52.0 / 256.0),
        )
        * Projection::scale(-scale, scale);
    let mut head_right_warped = ImageBuffer::new(size, size);
    warp_into(
        &head_right.into_rgba8(),
        &head_right_projection,
        Interpolation::Nearest,
        Rgba([0, 0, 0, 0]),
        &mut head_right_warped,
    );
    imageops::overlay(&mut out, &head_right_warped, 0, 0);

    out.save("head.png").unwrap();
}
