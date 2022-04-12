use common::Person;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    window.alert_with_message("Hello Rust").unwrap();

    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = "/person";

    let request = Request::new_with_str_and_init(&url, &opts)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let person: Person = json.into_serde().unwrap();
    person.validate().unwrap();

    let val = document.create_element("p")?;
    val.set_inner_html(format!("{:?}", person).as_str());
    body.append_child(&val)?;

    Ok(())
}
