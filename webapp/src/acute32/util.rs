use visioncortex::{BinaryImage, Color, ColorHsv, ColorImage, PointF64, PointI32};

pub(crate) fn binarize_image_util(color_image: &ColorImage) -> BinaryImage {
    color_image.to_binary_image(|c| is_black_rgb(&c))
}

/// Check Saturation and Value in HSV
pub(crate) fn is_black_hsv(color: &ColorHsv) -> bool {
    const BLACK_LIMIT: f64 = 0.125;
    //console_log_util(&format!("{:?}", color));
    if color.s != 0.0 && color.v != 0.0 {
        color.s*color.v <= BLACK_LIMIT
    } else { // Either s or v is 0.0
        (if color.s > 0.0 {color.s} else {color.v}) <= BLACK_LIMIT
    }
}

pub(crate) fn is_black_rgb(color: &Color) -> bool {
    let r = color.r as u32;
    let g = color.g as u32;
    let b = color.b as u32;

    r*r + g*g + b*b < 3*128*128
}

pub(crate) fn valid_pointi32_on_image(point: PointI32, image_width: usize, image_height: usize) -> bool {
    let w_upper = image_width as i32;
    let h_upper = image_height as i32;

    0 <= point.x && point.x < w_upper &&
    0 <= point.y && point.y < h_upper
}

pub(crate) fn valid_pointf64_on_image(point: PointF64, image_width: usize, image_height: usize) -> bool {
    let w_upper = image_width as f64;
    let h_upper = image_height as f64;

    0.0 <= point.x && point.x < w_upper &&
    0.0 <= point.y && point.y < h_upper
}

pub(crate) fn image_diff_area(img1: &BinaryImage, img2: &BinaryImage) -> u64 {
    img1.diff(img2).area()
}
