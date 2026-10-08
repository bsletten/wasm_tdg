#!/usr/bin/env python3
"""Serve this directory with cross-origin isolation enabled.

`SharedArrayBuffer` -- and therefore WebAssembly threads -- is only available
to pages that are "cross-origin isolated". That requires two response headers.
A plain `python3 -m http.server` will not set them, and the example will fail
in the browser with `SharedArrayBuffer is not defined`.
"""
import http.server
import socketserver

PORT = 10001


class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        **http.server.SimpleHTTPRequestHandler.extensions_map,
        ".wasm": "application/wasm",
    }

    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()


if __name__ == "__main__":
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"serving with cross-origin isolation at http://localhost:{PORT}/")
        httpd.serve_forever()
