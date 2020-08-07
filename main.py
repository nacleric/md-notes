from http.server import HTTPServer, BaseHTTPRequestHandler
import os


class RouteHandler(BaseHTTPRequestHandler):
    current_directory = os.curdir

    def do_GET(self):
        self.send_response(200)
        self.send_header("content-type", "text/html")
        self.end_headers()
        if self.path == "/":
            self.wfile.write(self.render_table_of_contents().encode())
        else:
            self.wfile.write(self.update_directory().encode())

    # Helper functions for routes
    # Entry point
    def render_table_of_contents(self, current_directory=current_directory) -> str:
        table_of_contents = ""
        for directory in os.listdir(current_directory):
            if os.path.isdir(directory):
                table_of_contents += f"<h1><a href='{directory}'> {directory} </a></h1>"
        return table_of_contents

    def render_md_file_list(self) -> str:
        # TODO: placeholder code
        md_file_list = ""
        files = "foo"
        for file in files:
            if file[-3:] == ".md":
                md_file_list += f"<h1> {file} </h1>"
        return md_file_list

    # If something gets clicked, recursively find all the other directories
    def update_directory(self, current_directory=current_directory) -> str:
        # TODO: This might cause a bug /foo/bar -> .//foo/bar
        # current_directory = f"{os.curdir}/{self.path}"
        current_directory = f".{self.path}"
        return current_directory


def run_server(handler_class=RouteHandler) -> None:
    server_address = ("", 8000)
    http_server = HTTPServer(server_address, handler_class)
    print(f"Running on Port: {server_address[1]}")
    http_server.serve_forever()


def main():
    run_server()


if __name__ == "__main__":
    main()
