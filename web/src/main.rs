use dioxus::prelude::*;
use ui::SvgCanvas;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        SvgCanvas {}
    })
}
