# HTTP Server and HTTP Library Documentation

**Note: This documentation and code are intended for learning purposes and not suitable for production use.**

Our Web server will have four components - Server, Router, Handler, and HTTP library. Each of these components has a specific purpose, in line with Single Responsibility Principle (SRP). The Server listens for incoming TCP byte streams. The HTTP library interprets the byte stream and converts it to HTTP Request (message). The router accepts an HTTP Request and determines which handler to invoke. The handler processes the HTTP request and constructs an HTTP response. The HTTP response message is converted back to a byte stream using the HTTP library, which is then sent back to the client.


Let’s build the code in the following sequence:

1. Build the HTTP library
2. Write the `main()` function for the project
3. Write the server module
4. Write the router module
5. Write the handler module

For convenience, Figure 2.5 shows a summary of the code design, showing the key modules, structs, and methods for the http library and httpserver project.


We’ll be writing code for the modules, structs, and methods shown in this figure. Here is a short summary of what each component in the figure does:

- **http**: Library containing types `HttpRequest` and `HttpResponse`. It implements the logic for converting between HTTP requests and responses, and corresponding Rust data structures.
- **httpserver**: Main web server that incorporates a `main()` function, socket server, handler, and router, and manages the coordinations among them. It serves as both a web server (serving html) and a web service (serving JSON).


