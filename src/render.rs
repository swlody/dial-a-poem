use maud::{html, Markup, DOCTYPE};

pub fn render_body(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang = "en" {
            head {
                meta charset="utf-8";
                link rel = "stylesheet" href="/static/style.css";
                script {
                    "0"
                }
            }
            body {
                (content)
            }
        }
    }
}
