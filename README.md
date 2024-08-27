# Cork - Key-Value Store

Cork is a simple key-value store built using Rust. It provides a RESTful API for setting, retrieving, and removing key-value pairs. This README covers the basics of getting started with Cork, including how to run the server, use the API, and perform load testing.

## Table of Contents

- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [Running Load Tests](#running-load-tests)

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine. You can download them from [rustup](https://rustup.rs/).
- Python 3.x and `pip` for running the load tests.

### Building and Running the Server

1. Clone the repository:

   ```sh
   git clone <repository-url>
   cd cork
   ```

2. Build the project:

   ```sh
   cargo build
   ```

3. Run the server:

   ```sh
   cargo run -- --port 3030 --verbose
   ```

   This will start the server on port 3030 with verbose logging enabled. You can specify a different port by changing the `--port` argument.

### Logging

The application uses `env_logger` for logging. You can enable verbose logging by passing the `--verbose` flag when starting the server. The default logging level is set to `Info`.

## API Endpoints

### Set Value

- **Endpoint:** `POST /set`
- **Request Body:** JSON object with `key` and `value` fields.
- **Response:** `200 OK` if the value was set successfully.

Example:

```sh
curl -X POST http://127.0.0.1:3030/set -H "Content-Type: application/json" -d '{"key": "example_key", "value": "example_value"}'
```

### Get Value

- **Endpoint:** `GET /get`
- **Query Parameter:** `key` (string) - The key to retrieve.
- **Response:** JSON object with `value` if found, or an error message with `404 Not Found` if the key does not exist.

Example:

```sh
curl -X GET "http://127.0.0.1:3030/get?key=example_key"
```

### Remove Value

- **Endpoint:** `DELETE /remove`
- **Query Parameter:** `key` (string) - The key to remove.
- **Response:** `200 OK` if the value was removed successfully.

Example:

```sh
curl -X DELETE "http://127.0.0.1:3030/remove?key=example_key"
```

## Running Load Tests

To perform load tests on the key-value store, you can use the provided Python script.

1. Install the required Python packages:

   ```sh
   pip install aiohttp
   ```

2. Run the load test script:

   ```sh
   python test/load_test.py <number-of-requests>
   ```

   Replace `<number-of-requests>` with the number of requests you want to perform.

   Example:

   ```sh
   python test/load_test.py 100
   ```

   This script will perform a series of set, get, and delete operations and output the performance metrics.