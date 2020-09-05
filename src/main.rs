use std::path::PathBuf;

use pulldown_cmark::{html, Parser};
use structopt::StructOpt;
use tiny_http::{Response, Server};
use webbrowser;

#[derive(StructOpt)]
struct Cli {
    // Add more fields for more arguments
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn is_md_file<P>(path: P) -> bool
where
    P: Into<PathBuf>,
{
    path.into() // Into<PathBuf>::into() -> PathBuf
        .extension() // PathBuf::extension() -> Option<&OsStr>
        .is_some() // Option<&OsStr> -> bool
}

fn parse_arg_to_md() -> String {
    let args = Cli::from_args();
    let mdfile = is_md_file(&args.path);

    if mdfile {
        let result = std::fs::read_to_string(&args.path);
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

fn main() {
    let mut html_body = String::from("<link rel='stylesheet' href='https://unpkg.com/sakura.css/css/sakura.css' type='text/css'>");
    let html_output = parse_arg_to_md();
    html_body.push_str(&html_output);

    let server = Server::http("0.0.0.0:8000");
    println!("Port: 8000, Server is running...");
    webbrowser::open("http://localhost:8000/").unwrap();

    match server {
        Ok(server) => loop {
            // Prints requests that hit the server
            match server.recv() {
                Ok(request) => {
                    println!("{:?}", request);
                    let response = Response::from_data(html_body.clone().into_bytes());
                    request.respond(response).unwrap();
                }
                Err(error) => {
                    panic!("Error: {:?}", error);
                }
            };
        },
        Err(error) => {
            panic!("This will be an error: {:?}", error);
        }
    }
}
