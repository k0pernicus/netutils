use std::env;
use std::io::{stderr, stdout, Read, Write};
use std::net::TcpStream;
use std::process;
use std::str;

fn main() {
    if let Some(url) = env::args().nth(1) {
        let (scheme, reference) = url.split_at(url.find(':').unwrap_or(0));
        if scheme == "http" {
            let mut parts = reference.split('/').skip(2); //skip first two slashes
            let remote = parts.next().unwrap_or("");
            let mut path = parts.next().unwrap_or("").to_string();
            for part in parts {
                path.push('/');
                path.push_str(part);
            }

            write!(stderr(), "* Connecting to {}\n", remote).unwrap();

            let mut stream = TcpStream::connect(&remote).unwrap();

            write!(stderr(), "* Requesting {}\n", path).unwrap();

            let request = format!("GET /{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, env::args().nth(2).unwrap_or(remote.to_string()));
            stream.write(request.as_bytes()).unwrap();
            stream.flush().unwrap();

            write!(stderr(), "* Waiting for response\n").unwrap();

            let mut response = Vec::new();

            loop {
                let mut buf = [0; 65536];
                let count = stream.read(&mut buf).unwrap();
                if count == 0 {
                    break;
                }
                response.extend_from_slice(&buf[.. count]);
            }

            write!(stderr(), "* Received {} bytes\n", response.len()).unwrap();

            let mut header_end = 0;
            while header_end < response.len() {
                if response[header_end..].starts_with(b"\r\n\r\n") {
                    break;
                }
                header_end += 1;
            }

            for line in unsafe { str::from_utf8_unchecked(&response[..header_end]) }.lines() {
                write!(stderr(), "> {}\n", line).unwrap();
            }

            stdout().write(&response[header_end + 4 ..]).unwrap();
        } else {
            write!(stderr(), "wget: unknown scheme '{}'\n", scheme).unwrap();
            process::exit(1);
        }
    } else {
        write!(stderr(), "wget: http://host:port/path\n").unwrap();
        process::exit(1);
    }
}
