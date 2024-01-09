use async_std::net::TcpStream;
use http_body_util::Empty;
use hyper::{client::conn::http1, Request, body::Bytes};
use hyper_futures::AsyncReadWriteCompat;

#[async_std::main]
async fn main() {
    let url = "http://www.google.com/".parse::<hyper::Uri>().unwrap();
    let host = url.host().unwrap();
    let port = url.port_u16().unwrap_or(80);

    let stream = TcpStream::connect((host, port)).await.unwrap();
    let (mut sender, connection) = http1::handshake(AsyncReadWriteCompat::new(stream)).await.unwrap(); 
    async_std::task::spawn(async move {
        if let Err(err) = connection.await {
                eprintln!("An error occurred: {}", err);
        }
    });

    let authority = url.authority().unwrap().clone();
    let request = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new()).unwrap();

    let response = sender.send_request(request).await.unwrap();
    println!("Response status: {}", response.status());
}