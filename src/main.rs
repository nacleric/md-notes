use pulldown_cmark::{html, Parser};
use std::path::PathBuf;
use structopt::StructOpt;
use tiny_http::{Response, Server};

#[derive(StructOpt)]
struct Cli {
    // Add more fields for more arguments
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Debug)]
enum Mdfile {
    IsMd,
    NotMd,
}

// Checks if file is a markdown file
#[allow(dead_code)]
fn check_for_mdfile(file: &PathBuf) -> Mdfile {
    //let expected_filetype: &str = file.to_str().unwrap(); Don't be tempted to use unwrap()
    let option = file.to_str();
    let expected_filetype = match option {
        Some(s) => match &s[&s.len() - 2..] {
            "md" => Mdfile::IsMd,
            _ => Mdfile::NotMd,
        },
        None => panic!("Could not convert string"),
    };
    expected_filetype
}

fn parse_arg_to_md() -> String {
    let args = Cli::from_args();
    let mdfile = check_for_mdfile(&args.path);

    match mdfile {
        Mdfile::IsMd => {
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
        },
        Mdfile::NotMd => {
            panic!("Not a Markdown file");
        },
    }
}

fn main() {
    let html_output = parse_arg_to_md();

    let server = Server::http("0.0.0.0:8000");
    println!("Port: 8000, Server is running...");
    match server {
        Ok(server) => loop {
            // Prints requests that hit the server
            match server.recv() {
                Ok(req) => {
                    println!("{:?}", req);
                    let response = Response::from_data(html_output.clone().into_bytes());
                    req.respond(response).unwrap();
                },
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
