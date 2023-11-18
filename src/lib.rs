pub mod macros;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

async fn handle_http(mut stream: TcpStream, html_content: String) {
    let response = r#"HTTP/1.1 200 OK
Date: Thu, 18 Nov 2023 12:00:00 GMT
Server: ExampleServer/1.0
Content-Type: text/html
"#;

    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        response,
        html_content.len(),
        html_content
    );

    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}



async fn handle_connection(stream: TcpStream) {
    let html_content = r#"
<script src="https://unpkg.com/htmx.org@1.9.8"></script>
<!-- have a button POST a click via AJAX -->
<button hx-post="/clicked" hx-swap="outerHTML">
Click Me
</button>
"#;
    let mut buffer = [0; 1000];
    if let Ok(n) = stream.peek(&mut buffer).await {
        //println!("{:?}", std::str::from_utf8(&buffer));
            handle_http(stream, html_content.to_owned()).await;
        if n > 0 && buffer.starts_with(b"POST /") {
        println!("hit");

        }    
    }
}

async fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server listening on http://127.0.0.1:8000");

    while let Some(stream) = listener.incoming().next().await {
        let stream = stream?;
        task::spawn(handle_connection(stream));
    }

    Ok(())
}

pub fn start_server() {
    task::block_on(run_server()).unwrap();
}


