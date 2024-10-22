#!/usr/bin/python3
import http.server
import socketserver

# Define the handler to handle HTTP requests
Handler = http.server.SimpleHTTPRequestHandler

# Specify the port
PORT = 7878

# Create the server with the handler and port
with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"Serving at port {PORT}")
    # Start the server
    httpd.serve_forever()

