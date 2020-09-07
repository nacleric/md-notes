use std::path::PathBuf;

use pulldown_cmark::{html, Parser};
use structopt::StructOpt;
use tiny_http::{Response, Request, Server};
use webbrowser;

#[derive(StructOpt)]
struct Cli {
    // Add more fields for more arguments
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    port: String,
}

fn is_md_file<P>(path: P) -> bool
where
    P: Into<PathBuf>,
{
    path.into() // Into<PathBuf>::into() -> PathBuf
        .extension() // PathBuf::extension() -> Option<&OsStr>
        .is_some() // Option<&OsStr> -> bool
}

fn process_md(path: &PathBuf) -> String {
    // let args = Cli::from_args();
    let mdfile = is_md_file(path);

    if mdfile {
        let result = std::fs::read_to_string(path);
        let content = match result {
            Ok(content) => content,
            Err(error) => {
                panic!("Can't deal with {:?}, just exit here", error);
            }
        };
        let parser = Parser::new(&content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    } else {
        panic!("Not a Markdown file");
    }
}

struct WebServer<'a> {
    path: &'a PathBuf,
    port: String,
}

impl WebServer<'_> {
    fn send_response(&self, request: Request) {
        let mut html_body = String::from("<link rel='stylesheet' href='https://unpkg.com/sakura.css/css/sakura.css' type='text/css'>");
        let html_output = process_md(self.path);
        html_body.push_str(&html_output);

        let response = Response::from_data(html_body.clone().into_bytes());
        request.respond(response).unwrap(); // TODO: Get rid of unwrap()
    }

    fn run(&self) {
        let result = Server::http("0.0.0.0:8000");
        let server = match result {
            Ok(server) => server,
            Err(error) => panic!("This will be an error: {:?}", error),
        };
        println!("Port: 8000, Server is running...");
        webbrowser::open("http://localhost:8000/").unwrap(); // TODO: Get rid of unwrap()

        loop {
            let request = match server.recv() {
                Ok(request) => request,
                Err(error) => {
                    panic!("Error: {:?}", error);
                }
            };
            println!("{:?}", request);
            self.send_response(request);
        }
    }
}

fn main() {
    let args = Cli::from_args();
    
    let server = WebServer{ path: &args.path, port: String::from("8000") };
    server.run();
}
