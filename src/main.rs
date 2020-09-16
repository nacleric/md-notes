use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use pulldown_cmark::{html, Parser};
use structopt::StructOpt;
use tiny_http::{Request, Response, Server};
use webbrowser;

extern crate ispell;
use ispell::SpellLauncher;

#[derive(Debug, StructOpt)]
enum CliArgs {
    #[structopt(name = "serve", about = "Serve subcommand")]
    Serve {
        #[structopt(help = "Input path to file", parse(from_os_str))]
        path: PathBuf,

        #[structopt(help = "Input port number", default_value = "5000")]
        port: String,
    },
    #[structopt(name = "debug", about = "Debug subcommand")]
    Debug {
        #[structopt(help = "Input path to file", parse(from_os_str))]
        path: PathBuf,
    },
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

#[derive(Debug)]
struct WebServer<'a> {
    path: &'a PathBuf,
    port: &'a String,
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
        let mut address = String::from("http://localhost:");
        address.push_str(self.port);

        let result = Server::http(&address[7..address.len()]);
        let server = match result {
            Ok(server) => server,
            Err(error) => panic!("{:?}", error),
        };
        println!("Port: {}, Server is running...", self.port);
        webbrowser::open(&address[..]).unwrap(); // TODO: Get rid of unwrap()

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

fn debug(path: &PathBuf) {
    let mut checker = SpellLauncher::new()
        .aspell()
        .dictionary("en_GB")
        .launch()
        .unwrap();
    let file = std::fs::File::open(&path).unwrap();
    let file = BufReader::new(file);
    let mut line_count = 0;
    for line in file.lines() {
        line_count += 1;
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                panic!("Error: {:?}", error);
            }
        };
        let errors = checker.check(&line).unwrap();
        for e in errors {
            println!(
                "'{}' (line: {} pos: {}) is misspelled!",
                &e.misspelled, line_count, e.position
            );
            if !e.suggestions.is_empty() {
                println!(
                    "Maybe you meant '{}, {}, {}'?",
                    &e.suggestions[0], &e.suggestions[1], &e.suggestions[2]
                );
            }
        }
    }
}

fn main() {
    let args = CliArgs::from_args();
    match args {
        CliArgs::Serve { path, port } => {
            let server = WebServer {
                path: &path,
                port: &port,
            };
            server.run();
        }
        CliArgs::Debug { path } => {
            debug(&path);
        }
    }
}
