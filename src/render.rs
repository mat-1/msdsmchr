use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use imageproc::geometric_transformations::{warp_into, Interpolation, Projection};

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

#[derive(Debug)]
struct OverlaySectionOptions {
    pub size: u32,
    pub scale: f32,
    pub x: u32,
    pub y: u32,
    pub matrix: [f32; 9],
    pub translate_x: f32,
    pub translate_y: f32,
    pub flip: bool,
}

fn overlay_3d_section(
    out: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    skin: &DynamicImage,
    opts: &OverlaySectionOptions,
) {
    let section = crop_section(skin, opts.x, opts.y);
    let section_projection = Projection::from_matrix(opts.matrix).unwrap()
        * Projection::translate(opts.translate_x, opts.translate_y)
        * Projection::scale(if opts.flip { -opts.scale } else { opts.scale }, opts.scale);
    let mut section_warped = ImageBuffer::new(opts.size, opts.size);
    warp_into(
        &section.into_rgba8(),
        &section_projection,
        Interpolation::Nearest,
        Rgba([0, 0, 0, 0]),
        &mut section_warped,
    );
    imageops::overlay(out, &section_warped, 0, 0);
}

pub fn to_3d_head(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Must be a multiple of SECTION_SIZE (8)
    let size: u32 = 128;

    // transparent image of size
    let mut out = ImageBuffer::from_pixel(size, size, Rgba([255, 255, 255, 255]));
    // let mut out = ImageBuffer::new(size, size);

    // left overlay
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 6,
            y: 1,
            matrix: TRANSFORM_RIGHT_LEFT_MATRIX,
            translate_x: (size as f32) * (231.0 / 256.0) * (8.0 / 8.1),
            translate_y: (size as f32) * (-56.0 / 256.0),
            flip: true,
            scale: (size / 20) as f32 * (9.0 / 8.0),
        },
    );
    // back overlay
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 7,
            y: 1,
            matrix: TRANSFORM_FRONT_BACK_MATRIX,
            translate_x: (size as f32) * (26.0 / 256.0),
            translate_y: (size as f32) * (70.0 / 256.0),
            flip: false,
            scale: (size / 20) as f32 * (9.0 / 8.0),
        },
    );
    // bottom overlay
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 6,
            y: 0,
            matrix: TRANSFORM_TOP_BOTTOM_MATRIX,
            translate_x: (size as f32) * (-145.0 / 256.0),
            translate_y: (size as f32) * (177.0 / 256.0),
            flip: false,
            scale: (size / 20) as f32 * (9.0 / 8.0),
        },
    );

    // top
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 1,
            y: 0,
            matrix: TRANSFORM_TOP_BOTTOM_MATRIX,
            translate_x: (size as f32) * (-40.0 / 256.0),
            translate_y: (size as f32) * (83.0 / 256.0),
            flip: false,
            scale: (size / 20) as f32,
        },
    );
    // front
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 1,
            y: 1,
            matrix: TRANSFORM_FRONT_BACK_MATRIX,
            translate_x: (size as f32) * (132.5 / 256.0),
            translate_y: (size as f32) * (177.5 / 256.0),
            flip: false,
            scale: (size / 20) as f32,
        },
    );
    // right
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 2,
            y: 1,
            matrix: TRANSFORM_RIGHT_LEFT_MATRIX,
            translate_x: (size as f32) * (121.0 / 256.0),
            translate_y: (size as f32) * (52.0 / 256.0),
            flip: true,
            scale: (size / 20) as f32,
        },
    );

    // front overlay
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 5,
            y: 1,
            matrix: TRANSFORM_FRONT_BACK_MATRIX,
            translate_x: (size as f32) * (132.5 / 256.0) * (8.1 / 8.0),
            translate_y: (size as f32) * (177.5 / 256.0),
            flip: false,
            scale: (size / 20) as f32 * (9.0 / 8.0),
        },
    );
    // right overlay
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 4,
            y: 1,
            matrix: TRANSFORM_RIGHT_LEFT_MATRIX,
            translate_x: (size as f32) * (26.0 / 256.0) * (8.0 / 8.1),
            translate_y: (size as f32) * (52.0 / 256.0),
            flip: false,
            scale: (size / 20) as f32 * (9.0 / 8.0),
        },
    );
    // top overlay
    overlay_3d_section(
        &mut out,
        img,
        &OverlaySectionOptions {
            size,
            x: 5,
            y: 0,
            matrix: TRANSFORM_TOP_BOTTOM_MATRIX,
            translate_x: (size as f32) * (-40.0 / 256.0) * (8.0 / 8.1),
            translate_y: (size as f32) * (83.0 / 256.0) * (8.0 / 9.0),
            flip: false,
            scale: (size / 20) as f32 * (9.0 / 8.0),
        },
    );
    out
}

pub fn to_2d_head(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut out = ImageBuffer::new(8, 8);

    let section = crop_section(img, 1, 1);
    imageops::overlay(&mut out, &section, 0, 0);

    let section = crop_section(img, 5, 1);
    imageops::overlay(&mut out, &section, 0, 0);

    out
}
