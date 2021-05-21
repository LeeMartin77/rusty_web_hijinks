use image::GenericImage;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    // Manufacture the element we're gonna append
    let image_element = document.get_element_by_id("dynamic-image").expect("should have a dynamic-image image");
    let imgx = 800;
    let imgy = 800;
    image_element.set_attribute("width", imgx.to_string().as_str()).expect("Unable to set dynamic image width");
    image_element.set_attribute("height", imgy.to_string().as_str()).expect("Unable to set dynamic image height");

    let image = generate_base_image(imgx, imgy);
    let mut buf = Vec::new();
    image.write_to(&mut buf, image::ImageOutputFormat::Png).expect("Failed to fill image buffer");
    let base64_string = base64::encode(buf);
    image_element.set_attribute("src", format!("data:image/png;base64,{}", base64_string).as_str()).expect("Unable to set dynamic image data");
    
    Ok(())
}

pub fn generate_base_image(imgx: u32, imgy: u32) -> image::DynamicImage {
    let mut dynimage = image::DynamicImage::new_rgb8(imgx, imgy);
    let mut curx = 1;
    let mut cury = 1;

    let center_width = 20;
    let center_height = 20;
    let img_x_offset = imgx/2 - center_width/2;
    let img_y_offset = imgy/2 - center_height/2;

    while curx < imgx {
        while cury < imgy {
            let r = (0.3 * curx as f32) as u8;
            let b = (0.3 * cury as f32) as u8;
            if curx > img_x_offset && curx < img_x_offset + center_width && cury > img_y_offset && cury < img_y_offset + center_height {
                dynimage.put_pixel(curx, cury, image::Rgba([0, 0, 0, 0]));
            } else {
                dynimage.put_pixel(curx, cury, image::Rgba([r, 0, b, 255]));
            }
            cury = cury + 1;
        }
        curx = curx + 1;
        cury = 1;
    }

    return dynimage;
}