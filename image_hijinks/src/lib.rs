use image::GenericImage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("img")?;
    val.set_id("image-holder");

    let image = generate_image();
    let mut buf = Vec::new();
    image.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();
    let base64_string = base64::encode(buf);

    val.set_attribute("src", format!("data:image/png;base64,{}", base64_string).as_str())?;

    body.append_child(&val)?;

    Ok(())
}

pub fn generate_image() -> image::DynamicImage {
    let imgx = 800;
    let imgy = 800;

    let mut dynimage = image::DynamicImage::new_rgb8(imgx, imgy);
    let mut curx = 1;
    let mut cury = 1;
    while curx < imgx {
        while cury < imgy {
            let r = (0.3 * curx as f32) as u8;
            let b = (0.3 * cury as f32) as u8;
            dynimage.put_pixel(curx, cury, image::Rgba([r, 0, b, 255]));
            cury = cury + 1;
        }
        curx = curx + 1;
        cury = 1;
    }

    return dynimage;
}