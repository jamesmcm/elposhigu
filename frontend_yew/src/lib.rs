use core::convert::From;
use core::convert::TryInto;
use wasm_bindgen::prelude::*;
use web_sys::FormData;
use yew::prelude::*;
use yew::services::ConsoleService;

struct Model {
    link: ComponentLink<Self>,
}

enum Msg {
    Submit(FormData),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(e) => {
                ConsoleService::info(&format!("Submit: {:?}", e.get("usrform")));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let submit_callback = self.link.callback(|f: FocusEvent| {
            let as_js: JsValue = f.target().unwrap().try_into().unwrap();
            Msg::Submit(FormData::from(as_js))
        });
        html! {
            <div>
                <form id="usrform" onsubmit=submit_callback>
                <textarea rows="16" cols="50" name="paste_input" form="usrform" />
                <br/>
                  <input type="submit"/>
                </form>
            </div>
        }
    }
}

// onsubmit=|e| {
//        e.prevent_default();
//        let form_element: Element = e.target()
//            .unwrap().try_into().unwrap();
//                Msg::Submit(
//                    FormData::from_element(&form_element).unwrap()) }
#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
