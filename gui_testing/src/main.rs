#![windows_subsystem = "windows"]

use web_view::*;

const HTML: &str = r#"
<html>
    <head>
        <title>Hello WebView</title>
    </head>

    <body>
        <div style="margin-bottom:10px">
            File: <input type="file">
        </div>

        <div style="margin-left:30px">
            <button value="Execute">Execute</button>
        </div>
    </body>
</html>
"#;

fn main() {
    web_view::builder().
        title("Task Runner").
        content(Content::Html(HTML)).
        size(350, 100).
        resizable(false).
        user_data(()).
        invoke_handler(|web_view, arg| Ok(())).
        run().unwrap();
}