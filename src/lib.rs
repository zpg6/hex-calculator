use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

mod header;
mod macros;

use header::wrap_html_with_header;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once(); // Log rust panics to console.error in JS
    Ok(router().call(req).await?)
}

fn router() -> Router {
    Router::new().route("/", get(root)).fallback(get(not_found))
}

/// Handler for the `/` route.
async fn root() -> axum::http::Response<axum::body::Body> {
    wrap_html_with_header(
        r#"
    <div>
        <h2>Hex to Decimal</h2>
        
        0x<input type="text" id="hex" placeholder="Enter a hex value" />
        <button onclick="calculateDecimal()">Calculate</button>
        <h3 id="result"></h3>
        <script>
            function calculateDecimal() {
                // Convert the hex value to a decimal value
                const hex = document.getElementById("hex").value;
                if (!hex.length) {
                    return;
                }
                try {
                    const decimal = parseInt(hex.replaceAll("0x","").replaceAll(" ",""), 16);
                    document.getElementById("result").innerText = decimal;
                }
                catch (e) {
                    document.getElementById("result").innerText = "Invalid hex value";
                }
            }
        </script>
    </div>
    <hr />
    <div>
        <h2>Decimal to Hex</h2>

        <input type="text" id="decimal" placeholder="Enter a decimal value" />
        <button onclick="calculateHex()">Calculate</button>
        <h3 id="result2"></h3>
        <script>
            function calculateHex() {
                // Convert the decimal value to a hex value
                const decimal = document.getElementById("decimal").value;
                if (!decimal.length) {
                    return;
                }
                try {
                    const hex = Number(decimal).toString(16);
                    document.getElementById("result2").innerText = "0x" + hex;
                } catch (e) {
                    document.getElementById("result2").innerText = "Invalid decimal value";
                }
            }
        </script>
    </div>
    "#
        .to_string(),
    )
}

async fn not_found() -> axum::http::Response<axum::body::Body> {
    wrap_html_with_header(include_markdown!("../static/pages/404.md"))
}
