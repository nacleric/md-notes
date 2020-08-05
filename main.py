from http.server import HTTPServer, BaseHTTPRequestHandler
import os


class RouteHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("content-type", "text/html")
        self.end_headers()
        self.wfile.write(render_table_of_contents().encode())

    def md_file_list(self):
        # TODO: placeholder code
        self.send_response(200)
        self.send_header("content-type", "text/html")
        self.end_headers()
        self.wfile.write(render_md_files().encode())


def render_table_of_contents() -> str:
    table_of_contents = ""
    files = os.listdir(os.curdir)
    for file in files:
        if os.path.isdir(file):
            table_of_contents += f"<h1> {file} </h1>"
    return table_of_contents


def render_md_files() -> str:
    # TODO: placeholder code
    md_file_list = ""
    files = os.listdir(os.curdir)
    for file in files:
        if file[-3:] == ".md":
            md_file_list += f"<h1> {file} </h1>"
    return md_file_list


def run_server(handler_class=RouteHandler) -> None:
    server_address = ("", 8000)
    http_server = HTTPServer(server_address, handler_class)
    print(f"Running on Port: {server_address[1]}")
    http_server.serve_forever()


def main():
    render_table_of_contents()
    run_server()


if __name__ == "__main__":
    main()
