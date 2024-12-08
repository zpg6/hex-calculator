/// Wraps the given HTML in a full HTML document with a header and pre-included CSS.
pub fn wrap_html_with_header(body_html: String) -> axum::http::Response<axum::body::Body> {
    let html = format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
       <meta charset="UTF-8">
       <meta name="viewport" content="width=device-width, initial-scale=1.0">
       <title>Hex Calculator</title>
       <style>{}</style>
    </head>
    <body>
        <header>
            <h1>Hex Calculator</h1>
        </header>
        <main>
            {}
        </main>
        <footer>
            <p>&copy; <script>document.write(new Date().getFullYear())</script> <a href="https://zachgrimaldi.com">Zach Grimaldi</a></p>
        </footer>
    </body>
</html>
"#,
        include_str!("../static/simple.min.css"),
        body_html,
    );
    axum::http::Response::builder()
        .header("Content-Type", "text/html")
        .body(axum::body::Body::from(html))
        .unwrap()
}
