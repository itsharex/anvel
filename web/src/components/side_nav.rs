use leptos::*;
use wasm_bindgen::prelude::*;
use js_sys::{
    Array,
};
use web_sys::{
 window,
 Node,
 Event,
};

#[path="../lib/functions.rs"]
mod functions;
use functions::{
    fetch_data,
};

#[component]
pub fn Sidenav()->impl IntoView{
    let data=create_resource(|| (), |_| async move { 
        match fetch_data("http://localhost:8000/api/directory_content").await {
            Ok(data) => {
                // web_sys::console::log_1(&data.clone().into());
                let dom_elem=web_sys::window().unwrap().document().unwrap().get_element_by_id("folders").unwrap();
                let object_content = js_sys::Reflect::get(&data, &JsValue::from_str("contents"))
                .map_err(|_| JsValue::from_str("Failed to access contents property")).unwrap();

                let contents=Array::from(&object_content);
                for content in contents {
                    let name = js_sys::Reflect::get(&content, &JsValue::from_str("name"))
                    .map_err(|_| JsValue::from_str("Failed to access name property")).unwrap();
                    let path = js_sys::Reflect::get(&content, &JsValue::from_str("path"))
                    .map_err(|_| JsValue::from_str("Failed to access path property")).unwrap();
                    // metadata content
                    let metadata = js_sys::Reflect::get(&content, &JsValue::from_str("metadata"))
                    .map_err(|_| JsValue::from_str("Failed to access metadata property")).unwrap();
                    let is_file = js_sys::Reflect::get(&metadata, &JsValue::from_str("is_file"))
                    .map_err(|_| JsValue::from_str("Failed to access is_file property")).unwrap();
                    let file_extension=js_sys::Reflect::get(&metadata, &JsValue::from_str("file_extension"))
                    .map_err(|_| JsValue::from_str("Failed to access file_extension property")).unwrap();
    
                    // Convert the filename to a Rust String
                    let name_str = name.as_string().unwrap_or_default();
                    let path_str = path.as_string().unwrap_or_default();
            
                    let item=format!("
                        <details class='flex flex-col' id='folder_{name_str}'>
                            <summary class='text-[#e5e5e5] mx-[1px] px-3 text-[11px] uppercase py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>{name_str}</summary>
                            <details>
                                <summary class='hover:bg-[#3c3c3c]/35 pl-6 pr-3 mx-[1px] cursor-pointer text-[#999999] active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300'>Music</summary>
                                <div class='mt-[1px] flex flex-col text-[13px] text-[#999999]'>
                                    <p class='hover:bg-[#3c3c3c]/35 pl-[32px] pr-[2px] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300'>Don Toliver</p>
                                    <p class='hover:bg-[#3c3c3c]/35 pl-[32px] pr-[2px] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300'>New playlist</p>
                                </div>
                            </details>
                        </details>
                    "); 
                 
                    let item_element =web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
                    // item_element.set_class_name("flex");
                    item_element.set_inner_html(&item);
                    dom_elem.append_child(&Node::from(item_element)).unwrap();
    
                    let path_str_copy=path_str.clone();
                    let open_file: Closure<dyn FnMut()> = Closure::new(move|| {
                        web_sys::window().unwrap().location().set_href(path_str_copy.as_str()).unwrap();
                    });

                    let image=web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("img_{}",name_str)).unwrap();
                    if is_file.clone().as_bool().unwrap() {
                        image.set_attribute("src", "/assets/icons/file.png").unwrap();
                        image.set_attribute("alt", "File").unwrap();
                    } else {
                        image.set_attribute("src", "/assets/icons/folder.png").unwrap();
                        image.set_attribute("alt", "Folder").unwrap();    
                    }
                    
                    let btn=web_sys::window().unwrap().document().unwrap().get_element_by_id(&name_str.as_str()).unwrap();
                    btn.add_event_listener_with_callback("dblclick", &open_file.as_ref().unchecked_ref()).unwrap();
                    open_file.forget();
                }
            }
            Err(e) => { 
                web_sys::console::error_1(&e.into());
            }
        }
    });

    view!{
        <div class="min-h-[100vh] pt-[45px] pb-[12px] left-0 w-[200px] top-12 text-[13px] text-[#999999] bg-[#151515]">
            <div class="flex flex-col mb-3">
                <div class="h-[33px] flex items-center text-[#999999] uppercase pl-[12px] pr-[8px]">
                    <button class="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">draft</button>
                    <button class="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">search</button>
                    <button class="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">refresh</button>
                </div>
                //folders
                <div id="folders" class="pb-[12px]">
                    <div class="flex items-center text-[#e5e5e5] text-[11px] uppercase px-[8px] h-[35px]">
                        <p class="pl-[12px]">EXPLORER</p>
                        <span class="material-symbols-outlined md-16 text-[#999999] w-[30px] ml-auto h-[25px] active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">more_horiz</span>
                    </div>
                    {move || match data.get() {
                        None => view! { <p>"Loading..."</p> }.into_view(),
                        Some(_) =>view! { 
                        }.into_view()
                    }}
                </div>
                //search
                <div id="search"></div>
                
            </div>
        </div>
    }
} 