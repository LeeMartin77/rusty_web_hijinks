use image::GenericImage;
use image::GenericImageView;
use image::DynamicImage;
use web_sys::Element;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde;

#[derive(Serialize, Deserialize)]
pub struct GraphData {
    values: Vec<GraphValue>
}

#[derive(Serialize, Deserialize)]
pub struct GraphValue {
    x: u32,
    y: u32
}

static IMG_WIDTH: u32 = 800;
static IMG_HEIGHT: u32 = 800;
static BORDER_WIDTH: u32 = 25;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn generate_graph(img_id: &str, graph_data_js_obj: &JsValue) {
    let graph_data: GraphData = graph_data_js_obj.into_serde().unwrap();
    let image_element = get_dynamic_image_element(img_id);

    let image = generate_graph_image(graph_data);

    write_dynimage_to_img(image, &image_element);
}

pub fn get_dynamic_image_element(img_id: &str) -> Element {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let image_element = document.get_element_by_id(img_id).expect("should have a dynamic-graph image");

    image_element.set_attribute("width", IMG_WIDTH.to_string().as_str()).expect("Unable to set dynamic image width");
    image_element.set_attribute("height", IMG_HEIGHT.to_string().as_str()).expect("Unable to set dynamic image height");

    image_element
}

pub fn write_dynimage_to_img(dynimg: DynamicImage, img_element: &Element) {
    let mut buf = Vec::new();
    dynimg.write_to(&mut buf, image::ImageOutputFormat::Png).expect("Failed to fill image buffer");
    let base64_string = base64::encode(buf);
    img_element.set_attribute("src", format!("data:image/png;base64,{}", base64_string).as_str()).expect("Unable to set dynamic image data");
}

pub fn generate_graph_image(values: GraphData) -> image::DynamicImage {
    let mut dynimage = image::DynamicImage::new_rgb8(IMG_WIDTH, IMG_HEIGHT);
    dynimage = fill_background_colour(dynimage);
    dynimage = draw_axes(dynimage, BORDER_WIDTH);
    dynimage = draw_dots(dynimage, BORDER_WIDTH, values);
    return dynimage;
}

pub fn fill_background_colour(mut dynimage: image::DynamicImage) -> image::DynamicImage {
    let background_color = image::Rgba([255, 255, 255, 255]);
    for x in 0..dynimage.width() {
        for y in 0..dynimage.height() {
            dynimage.put_pixel(x, y, background_color);
        }
    }
    dynimage
}

pub fn draw_axes(mut dynimage: image::DynamicImage, border: u32) -> image::DynamicImage {
    let axes_colour = image::Rgba([0, 0, 0, 255]);
    for x in border..(dynimage.width() - border){
        dynimage.put_pixel(x, dynimage.height()-border, axes_colour);
    }
    for y in border..(dynimage.width() - border){
        dynimage.put_pixel(border, y, axes_colour);
    }
    dynimage
}

pub fn draw_dots(mut dynimage: image::DynamicImage, border: u32, values: GraphData) -> image::DynamicImage {
    let dot_colour = image::Rgba([0, 0, 255, 255]);
    let dot_expansion = 2; 
    let max_x = values.values.iter().map(|val| val.x).max().unwrap();
    let max_y = values.values.iter().map(|val| val.y).max().unwrap();
    let x_division = (dynimage.width() - border * 2) / max_x;
    let y_division = (dynimage.height() - border * 2) / max_y;
    for point in values.values {
        let x_center = (x_division * point.x) + border;
        let y_center = dynimage.height() - (y_division * point.y) - border;
        for x in (x_center - dot_expansion)..(x_center + dot_expansion) {
            for y in (y_center - dot_expansion)..(y_center + dot_expansion) {
                dynimage.put_pixel(x, y, dot_colour);
            } 
        } 
    }
    dynimage
}
