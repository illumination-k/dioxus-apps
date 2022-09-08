use dioxus::prelude::*;
use ui::SvgCanvas;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        SvgCanvas {}
    })
}
