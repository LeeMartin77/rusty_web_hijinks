use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn img_generate() -> Result<(), JsValue> {

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("img")?;
    val.set_id("image-holder");
    val.set_attribute("src", "https://placekitten.com/200/300")?;

    body.append_child(&val)?;

    Ok(())
}