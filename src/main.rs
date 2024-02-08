use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs, thread, time::Duration};

const ADDRESS: &str = "0.0.0.0";
const PORT: &str = "7878";

fn main() {
    let listener = TcpListener::bind(format!("{ADDRESS}:{PORT}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => (get_status_line("200 OK"), "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (get_status_line("200 OK"), "hello.html")
        }
        _ => (get_status_line("404 NOT FOUND"), "404.html"),
    };
    
    let contents = fs::read_to_string(format!("src/{filename}")).unwrap();
    let response = create_response(status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
}



fn get_status_line(code: &str) -> String {
    return format!("HTTP/1.1 {code}");
}

fn create_response(status_line: String, contents: String) -> String {
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    return response;
}
