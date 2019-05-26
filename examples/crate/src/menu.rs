use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Node, Element, HtmlElement, HtmlHyperlinkElementUtils};
use lazy_static::*;
use std::collections::HashMap;

pub struct Menu <'a> {
    pub label: &'a str,
    pub source: &'a str,
}

lazy_static! {
    pub static ref MENU_LOOKUP: HashMap<&'static str, Menu<'static>> = {
        let mut m = HashMap::new();
        //Tick
        m.insert("clock", Menu {label: "Clock", source: "clock"});
        //Loaders
        m.insert("loaders-image", Menu {label: "Image", source: "loaders/image"});
        //WebGl
        m.insert("webgl-simple", Menu {label: "Simple", source: "webgl/simple"});
        m.insert("webgl-texture", Menu {label: "Texture", source: "webgl/texture"});
        m.insert("webgl-instancing", Menu {label: "Instancing", source: "webgl/instancing"});
        m
    };
}

pub fn build_menu(document:&Document) -> Result<web_sys::Node, JsValue> {
    let container: Node = document.create_element("div")?.into();

    append_menu(&container, &document, "Tick", vec![
      "clock" 
    ])?;

    append_menu(&container, &document, "Loaders", vec![
        "loaders-image",
    ])?;

    append_menu(&container, &document, "WebGl", vec![
        "webgl-simple",
        "webgl-texture",
        "webgl-instancing",
    ])?;

    Ok(container)
}

fn append_menu (container:&Node, document:&Document, label:&str, menu_routes:Vec<&str>) -> Result<(), JsValue> {

    let menu_element: Element = document.create_element("div")?.into();
    menu_element.set_class_name("menu");

    let header: Element = document.create_element("div")?.into();
    header.set_class_name("menu-header");
    header.set_text_content(Some(&label));

    let menu_list: Element = document.create_element("div")?.into();
    menu_list.set_class_name("menu-list");

    for menu_route in menu_routes.into_iter() {
        if let Some(menu) = MENU_LOOKUP.get(menu_route) {
            let item = create_menu_item(&menu_route, &menu, document)?;
            menu_list.append_child(&item)?;
        }
    }

    menu_element.append_child(&header)?;
    menu_element.append_child(&menu_list)?;

    container.append_child(&menu_element)?;
    Ok(())

}

fn create_menu_item(href:&str, menu:&Menu, document:&Document) -> Result<HtmlElement, JsValue> {

    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button");
    item.set_text_content(Some(&menu.label));

    wrap_link(&href, item, &document)
}

fn wrap_link(href:&str, contents:HtmlElement, document:&Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    anchor.append_child(&contents)?;
    
    let anchor = anchor.unchecked_into::<HtmlHyperlinkElementUtils>();
    anchor.set_href(&href);

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}
