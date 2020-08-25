from http.server import HTTPServer, BaseHTTPRequestHandler
import webbrowser
import markdown
import os


STYLE = '<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/kognise/water.css@latest/dist/dark.min.css">'


# TODO: convert current_directory into instance variable
class RouteHandler(BaseHTTPRequestHandler):
    current_directory = os.curdir
    STYLE = '<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/kognise/water.css@latest/dist/dark.min.css">'

    def do_GET(self):
        self.send_response(200)
        self.send_header("content-type", "text/html")
        self.end_headers()
        if self.path == "/":
            self.wfile.write(self.render_table_of_contents().encode())
        else:
            new_directory = self.update_directory()
            print(f"[GET] {new_directory}")
            if os.path.isdir(new_directory):
                self.wfile.write(self.render_file_list().encode())
            else:
                self.wfile.write(render_md(new_directory).encode())

    # Entry point
    def render_table_of_contents(self) -> str:
        table_of_contents = self.STYLE 
        for directory in os.listdir(self.current_directory):
            if os.path.isdir(directory):
                table_of_contents += f"<h1><a href='{directory}'> /{directory} </a></h1>"
        return table_of_contents

    def render_file_list(self) -> str:
        # TODO: Handle errors for directories/routes that don't exist
        md_file_list = self.STYLE
        for item in os.listdir(self.current_directory):
            if item[-3:] == ".md":
                print(self.update_directory())
                file_location = f"{self.path}/{item}"
                md_file_list += f"<h1><a href='{file_location}'> {item} </a></h1>"
            elif os.path.isdir(item):
                md_file_list += f"<h1><a href='{self.path}'> /{item} </a></h1>"
        return md_file_list

    # If something gets clicked, recursively find all the other directories
    def update_directory(self) -> str:
        self.current_directory = f".{self.path}"
        return self.current_directory


def render_md(file: str) -> str:
    html = STYLE
    with open(file, "r", encoding="utf-8") as md:
        text = md.read()
    html += markdown.markdown(text)
    return html


def run_server(handler_class=RouteHandler) -> None:
    """ Runs the server and opens in web browser """
    server_address = ("", 8000)
    http_server = HTTPServer(server_address, handler_class)
    print(f"Running on Port: {server_address[1]}")
    webbrowser.open(f"localhost:{server_address[1]}")
    http_server.serve_forever()


def main():
    run_server()


if __name__ == "__main__":
    main()
