use image::DynamicImage;
use web_sys::Element;
use image::GenericImage;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameProperties {
    running: bool,
    width: u32,
    height: u32,
    player_size: u32,
    player_position: (u32, u32)
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let _image_element = get_dynamic_image_element();
    Ok(())
}

#[wasm_bindgen]
pub fn game_init() -> GameProperties {
    let game = GameProperties {
        running: true,
        width: 800,
        height: 800,
        player_size: 10,
        player_position: (50, 50)
    };    
    let image_element = get_dynamic_image_element();

    image_element.set_attribute("width", game.width.to_string().as_str()).expect("Unable to set dynamic image width");
    image_element.set_attribute("height", game.height.to_string().as_str()).expect("Unable to set dynamic image height");

    let image = generate_image(&game);

    write_dynimage_to_img(image, &image_element);
    game
}


#[wasm_bindgen]
pub fn game_loop(game: GameProperties) -> GameProperties {
    let image_element = get_dynamic_image_element();
    let process_image = generate_image(&game);
    write_dynimage_to_img(process_image, &image_element);
    game
}

pub fn get_dynamic_image_element() -> Element {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    document.get_element_by_id("dynamic-image").expect("should have a dynamic-image image")
}

pub fn write_dynimage_to_img(dynimg: DynamicImage, img_element: &Element) {
    let mut buf = Vec::new();
    dynimg.write_to(&mut buf, image::ImageOutputFormat::Png).expect("Failed to fill image buffer");
    let base64_string = base64::encode(buf);
    img_element.set_attribute("src", format!("data:image/png;base64,{}", base64_string).as_str()).expect("Unable to set dynamic image data");
}

pub fn generate_image(game: &GameProperties) -> image::DynamicImage {
    let mut dynimage = image::DynamicImage::new_rgb8(game.width, game.height);
    let mut curx = 1;
    let mut cury = 1;

    let img_x_offset = game.player_position.0;
    let img_y_offset = game.player_position.1;

    while curx < game.width {
        while cury < game.height {
            if curx > img_x_offset && curx < img_x_offset + game.player_size && cury > img_y_offset && cury < img_y_offset + game.player_size {
                dynimage.put_pixel(curx, cury, image::Rgba([255, 0, 0, 255]));
            } else {
                dynimage.put_pixel(curx, cury, image::Rgba([0, 0, 0, 255]));
            }
            cury = cury + 1;
        }
        curx = curx + 1;
        cury = 1;
    }

    return dynimage;
}