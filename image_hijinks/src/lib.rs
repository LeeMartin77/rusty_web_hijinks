use image::DynamicImage;
use web_sys::Element;
use image::GenericImage;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    x: i32,
    y: i32
}

#[wasm_bindgen]
pub struct GameInput {
    movement: Point
}

#[wasm_bindgen]
pub struct GameProperties {
    width: u32,
    height: u32,
    player_size: u32,
    player_position: Point
}



#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let _image_element = get_dynamic_image_element();
    Ok(())
}

#[wasm_bindgen]
pub fn game_init() -> GameProperties {
    let game = GameProperties {
        width: 800,
        height: 800,
        player_size: 10,
        player_position: Point {x:50, y:50}
    };    
    let image_element = get_dynamic_image_element();

    image_element.set_attribute("width", game.width.to_string().as_str()).expect("Unable to set dynamic image width");
    image_element.set_attribute("height", game.height.to_string().as_str()).expect("Unable to set dynamic image height");

    let image = generate_image(&game);

    write_dynimage_to_img(image, &image_element);
    game
}

#[wasm_bindgen]
pub fn game_loop(game: GameProperties, x: i32, y: i32) -> GameProperties {
    //"Logic" Step
    let input = GameInput {
        movement: Point {
            x: x,
            y: y
        }
    };
    let moved_game = move_player(game, input);
    //"Rendering" Step
    let image_element = get_dynamic_image_element();
    let process_image = generate_image(&moved_game);
    write_dynimage_to_img(process_image, &image_element);
    moved_game
}

pub fn move_player(game: GameProperties, input: GameInput) -> GameProperties {
    let new_position = Point {
        x: game.player_position.x + input.movement.x,
        y: game.player_position.y + input.movement.y
    };
    GameProperties {
        height: game.height,
        width: game.width,
        player_size: game.player_size,
        player_position: new_position
    }
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
    
    let img_x_offset = game.player_position.x as u32;
    let img_y_offset = game.player_position.y as u32;

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