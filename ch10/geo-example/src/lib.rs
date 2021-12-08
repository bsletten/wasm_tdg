use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn describe_location( lat : f32, lon : f32 ) {
    let i_lat = lat as i32;
    let i_lon = lon as i32;
    
    use std::cmp::Ordering::*;
    
    let relative_position = match(i_lat.cmp(&38), i_lon.cmp(&-121)) {
        (Equal, Equal) => "very close!",
        (Equal, Greater) => "east of Me",
        (Equal, Less) => "west of Me",
        (Less, Equal) => "south of Me",
        (Less, Greater) => "southeast of Me",
        (Less, Less) => "southwest of Me",
        (Greater, Equal) => "north of Me",
        (Greater, Greater) => "northeast of Me",
        (Greater, Less) => "northwest of Me"
    };

    log(&format!("You are {}!", relative_position));

    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p").unwrap();
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val).unwrap();
}
