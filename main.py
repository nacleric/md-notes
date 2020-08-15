from http.server import HTTPServer, BaseHTTPRequestHandler
import os


# TODO: convert current_directory into instance variable
# TODO: fix this fking bullshit
class RouteHandler(BaseHTTPRequestHandler):

    def __init__(self, current_directory=os.curdir):
        self.current_directory = current_directory

    def do_GET(self):
        self.send_response(200)
        self.send_header("content-type", "text/html")
        self.end_headers()
        if self.path == "/":
            self.wfile.write(self.render_table_of_contents().encode())
        else:
            print(self.update_directory())
            self.wfile.write(self.render_md_file_list().encode())

    # Entry point
    def render_table_of_contents(self) -> str:
        table_of_contents = ""
        for directory in os.listdir(self.current_directory):
            if os.path.isdir(directory):
                table_of_contents += f"<h1><a href='{directory}'> {directory} </a></h1>"
        return table_of_contents

    def render_md_file_list(self) -> str:
        # TODO: Handle errors for directories/routes that don't exist
        md_file_list = ""
        for item in os.listdir(self.current_directory):
            if item[-3:] == ".md":
                md_file_list += f"<h1> {item} </h1>"
            elif os.path.isdir(item):
                md_file_list += f"<h1><a href='{item}'> {item} </a></h1>"
        return md_file_list

    # If something gets clicked, recursively find all the other directories
    def update_directory(self) -> str:
        self.current_directory = f".{self.path}"
        return self.current_directory


def run_server(handler_class=RouteHandler) -> None:
    server_address = ("", 8000)
    http_server = HTTPServer(server_address, handler_class)
    print(f"Running on Port: {server_address[1]}")
    http_server.serve_forever()


def main():
    run_server()


if __name__ == "__main__":
    main()
