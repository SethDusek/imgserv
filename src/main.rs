extern crate tiny_http;
extern crate ascii;
extern crate flate2;
extern crate imgserv;
#[macro_use]
extern crate lazy_static;
use tiny_http::{Server, Response};
use std::sync::Arc;
use std::fs::File;
use std::thread;
use std::str::FromStr;
use ascii::AsciiString;
use imgserv::ipfs;
use std::path::Path;
use std::io::prelude::*;
use std::cmp::Ordering::Equal;
use flate2::read::ZlibDecoder;

lazy_static! {
    static ref KEY: AsciiString = AsciiString::from_str("KEY").unwrap(); //change to your key
    static ref FILEPATH: &'static Path = Path::new("/var/www/html"); //change to your path
}

fn main() {
    let server = Arc::new(Server::http("0.0.0.0:8080").unwrap()); //change to your port
    for i in 0..4 {
        let server = server.clone();
        let th = thread::spawn(move || {
            let contentencode = AsciiString::from_str("gzip").unwrap();
            loop {
                let mut request = server.recv().unwrap();
                let mut respstr = String::new();
                let mut passed = false;
                let mut compression = false;

                {
                    let headers = request.headers();
                    let keyh = headers.iter().find(|&header| header.field.as_str() == "key");
                    if let Some(key) = keyh {
                        if key.value.cmp(&KEY) == Equal {
                            passed = true //all checks done, key exists and it is what we want
                        } else {
                            respstr.push_str("Keys do not match");
                        }
                    } else {
                        respstr.push_str("Please give a key");
                    }

                    let compressionh = headers.iter().find(|&header| {
                        header.field.as_str() == "Content-Encoding"
                    });
                    if let Some(compressionh) = compressionh {
                        if compressionh.value.cmp(&contentencode) == Equal {
                            println!("compression true!");
                            compression = true;
                        }
                    }
                }
                if passed {
                    let mut fileext = String::new();
                    let mut bytes = Vec::new();
                    if compression {
                        let mut decoded = ZlibDecoder::new(request.as_reader());
                        decoded.read_to_end(&mut bytes);
                    } else {
                        request.as_reader().read_to_end(&mut bytes);
                    }
                    let filehash = ipfs::ipfs_hash(&bytes);
                    let headers = request.headers();
                    let fileexth = headers.iter()
                                          .find(|&header| header.field.as_str() == "filename");
                    if let Some(filename) = fileexth {
                        fileext.push_str(filename.value.as_str().split(".").last().unwrap());
                    }
                    let mut path = FILEPATH.join(&filehash);
                    path.set_extension(&fileext);
                    println!("{}", path.to_str().unwrap());
                    let mut file = File::create(path).unwrap();
                    file.write_all(&bytes);
                    respstr.push_str(&("http://i.shibe.ml/".to_owned() + &filehash + "." +
                                       &fileext))
                }
                let response = Response::from_string(respstr);
                request.respond(response).unwrap();
            }
        });
        if i == 3 {
            th.join().unwrap();
        };
    }
}
