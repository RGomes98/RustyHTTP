# RustyHTTP

RustyHTTP is a personal project aimed at learning Rust and building an HTTP server. The goal is to create a fast, efficient, and feature-rich server with a focus on gaining hands-on experience with Rust and web server concepts.

## Goals

- Learn the fundamentals of Rust programming.
- Build a simple yet extensible HTTP server.
- Add features such as routing, request handling, middleware, and more as the project evolves.
- Gain experience in writing performant, safe, and scalable code with Rust.

## Features (Planned)

- Request/Response handling
- Basic routing
- Middleware support
- Logging
- Error handling
- Async capabilities for handling concurrent requests

## Getting Started

### Install Rust

Before you can build and run the project, you'll need to have the Rust compiler and Cargo (Rust's package manager and build tool) installed on your system. If you don't have Rust installed, follow these steps:

1. Visit [https://www.rust-lang.org/](https://www.rust-lang.org/) and follow the installation instructions for your operating system.
2. Alternatively, you can install Rust using the following command:

   For Linux/macOS:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   For Windows, use the [Rust installer](https://www.rust-lang.org/tools/install).

3. After installation, verify that Rust is installed by running:

   ```bash
   rustc --version
   ```

   This should output the installed version of the Rust compiler.

4. Clone the repository:

```bash
git clone https://github.com/RGomes98/RustyHTTP.git
cd RustyHTTP
```

### Configure environment variables

RustyHTTP uses two environment variables, `PORT` and `HOST`, for server configuration. These are pre-defined in the `./cargo/config.toml` file, but you can customize them based on your preference.

1. Open the `config.toml` file located in the `./cargo` folder.
2. Set the `PORT` and `HOST` variables according to your preferred configuration. By default, they are set to:

```toml
[env]
HOST = "127.0.0.1"
PORT = "8080"
```

You can change these values to suit your needs. For example, if you want the server to run on a different port or host, simply modify the `PORT` and `HOST` variables.

### Build and run the server

Once the environment variables are configured, build and run the server using:

```bash
cargo run
```

The server will start on the specified host and port (default: `127.0.0.1:8080`).

## Contributing

Feel free to open issues or contribute with improvements. Pull requests are welcome!
