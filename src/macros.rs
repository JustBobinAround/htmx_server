#[macro_export]
macro_rules! router {
    ($value:expr, $($pattern:pat => $result:expr),* $(,)?) => {
        match $value{
            $($pattern => $result),*,
            _ => {()}
        }
    };
}
#[macro_export]
macro_rules! server{
    ($addr:expr,[$($names:expr),*]) => {
        async fn send_response(mut stream: async_std::net::TcpStream, html_content: String) 
        {

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                html_content.len(),
                html_content
            );
            if let Err(err) = stream.write_all(response.as_bytes()).await {
                println!("Error writing to stream: {}", err);
            }

            if let Err(err) = stream.flush().await {
                println!("Error flushing stream: {}", err);
            }

        }

        async fn run_server() -> std::io::Result<()> {
            let listener = async_std::net::TcpListener::bind($addr).await?;
            println!("Server listening on http://{}", $addr);

            while let Some(stream) = listener.incoming().next().await {
                let stream = stream?;
                  
                let mut buffer = [0; 1000];
                if let Ok(n) = stream.peek(&mut buffer).await {
                    if n > 4 {
                        if let Ok(request_str) = std::str::from_utf8(&buffer) {
                            if let Some(mut start_of_url) = request_str.find(' ') {
                                start_of_url += 1;
                                if let Some(end_of_url) = request_str[start_of_url..n].find(' ') {
                                    //println!("s:{} e:{} n:{}", start_of_url, end_of_url, n);
                                    let url = &request_str[start_of_url..end_of_url+start_of_url]; // Assuming "GET /some_route HTTP/1.1", so skipping "GET "
                                    println!("{}", url);
                                    let mut html_content: Option<String> = None;
                                    $(
                                    if let Some(content) = $names(&url) {
                                        html_content = Some(content);
                                    }
                                    )*
                                    if let Some(content) = html_content {
                                        async_std::task::spawn(send_response(stream, content));
                                    }
                                } 
                            }
                        }
                    }
                }
            }

            Ok(())
        }
        async_std::task::block_on(run_server());
    };
}
