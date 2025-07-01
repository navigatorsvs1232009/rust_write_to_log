# HTTP Request Logger

[![Rust](https://github.com/user/repo/actions/workflows/rust.yml/badge.svg)](https://github.com/user/repo/actions)  
A simple HTTP server that listens for POST requests, logs incoming data to a file, and provides structured logging with unique request IDs.

---

## Features

- **HTTP Server:** Accepts POST requests and extracts JSON data from the request body.
- **Logging:** Writes incoming data to a log file (`hook.log`) with a unique request ID for each request.
- **Modular Design:** Separates formatting, writing, and HTTP handling logic into distinct modules for better maintainability.
- **Error Handling:** Gracefully handles errors and returns appropriate HTTP responses.

---

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Example](#example)
- [Contributing](#contributing)
- [License](#license)

---

## Installation

### Prerequisites

- Rust (>= 1.60)
- Cargo (Rust's package manager)

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/navigatorsvs1232009/rust_write_to_log.git
   cd http-request-logger
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run
   ```

The server will start listening on `http://localhost:3030`.

---

## Usage

### Sending Requests

You can send POST requests to the server using tools like `curl`, Postman, or any HTTP client.

#### Example with `curl`:

```bash
curl -X POST http://localhost:3030 \
     -H "Content-Type: application/json" \
     -d '{"key1": "value1", "key2": 123, "key3": true}'
```

### Log Format

Each request is logged in the `hook.log` file with the following format:

```
------------------------

Request ID: 550e8400-e29b-41d4-a716-446655440000
Title: incoming
{
    "key1": "value1",
    "key2": 123,
    "key3": true,
}
------------------------
```

- **Request ID:** A unique identifier for each request.
- **Title:** Optional title for the log entry (default: `DEBUG`).
- **Data:** The JSON payload sent in the request body.

---

## Project Structure

The project is organized into modular components for better maintainability:

```
src/
├── main.rs          # Main application entry point
├── log/
│   ├── mod.rs       # Main module for logging functionality
│   ├── writer.rs    # Handles writing logs to a file
│   └── formatter.rs # Formats log data into a readable string
```

---

## Example

### Sending a Request

```bash
curl -X POST http://localhost:3030 \
     -H "Content-Type: application/json" \
     -d '{"name": "John Doe", "age": 30, "active": true}'
```

### Log Output

After sending the request, the `hook.log` file will contain:

```
------------------------

Request ID: 550e8400-e29b-41d4-a716-446655440000
Title: incoming
{
    "name": "John Doe",
    "age": 30,
    "active": true,
}
------------------------
```

---

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

Please ensure your code adheres to Rust's best practices and includes tests where applicable.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Warp](https://docs.rs/warp/latest/warp/).
- Inspired by the need for a simple HTTP request logger.

---

## Future Improvements

- Add support for multiple log files with rotation.
- Integrate with external logging services (e.g., AWS CloudWatch, Elasticsearch).
- Provide configuration options via environment variables or a config file.
- Add Prometheus metrics for monitoring server performance.

---

Feel free to customize this `README.md` further based on your project's specific needs!
