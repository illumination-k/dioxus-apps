use dioxus::{prelude::*, events::{MouseData}};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    points: Vec<Point>
}

#[allow(non_snake_case)]
pub fn SvgCanvas(cx: Scope) -> Element {
    let lines = use_ref(&cx, || Vec::<Line>::new());
    let is_drawing = use_ref(&cx, || false);
    
    let handle_mouse_down = |evt: &MouseData| {
        *is_drawing.write() = true;
        let x = evt.client_x;
        let y = evt.client_y;
        lines.write().push(Line { points: vec![Point {x, y}] })
    };

    let handle_mouse_move = |evt: &MouseData| {
        if !(*is_drawing.read()) {
            return;
        }

        let x = evt.client_x;
        let y = evt.client_y;
        lines.write().last_mut().map(|l| l.points.push(Point {x, y}));
    };

    let handle_mouse_up = |evt: &MouseData| {
        *is_drawing.write() = false;
    };

    cx.render(rsx! {

        div {
            onmousedown: move |evt| { handle_mouse_down(evt.data.as_ref())},
            onmouseup: move |evt| { handle_mouse_up(evt.data.as_ref())},
            onmousemove: move |evt| { handle_mouse_move(evt.data.as_ref())},
            onmouseleave: move |evt| { handle_mouse_up(evt.data.as_ref())},
            height: "500",
            width: "500",
            svg {
                width: "100%",
                height: "100%",
                lines.read().iter().map(|line| {
                    let mut d = "M ".to_string();
                    let line_s = line.points.iter().map(|p| format!("{} {}", p.x, p.y)).collect::<Vec<String>>().join(" L ");
                    d += &line_s;

                    return rsx! {
                        path {
                            d: "{d}",
                            fill: "none",
                            stroke: "black",
                            stroke_width: "10"
                        }
                    };
                })
            },
        },
        div {
            lines.read().iter().enumerate().take(10).map(|(i, line)| {
                let val = line.points.iter().map(|p| format!("x: {} y: {}", p.x, p.y)).collect::<Vec<String>>().join(",");
                return rsx! {
                    "{val}",
                }
            })
        }
        
    })
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
