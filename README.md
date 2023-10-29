# Metrics Filter

Metrics Filter is a simple HTTP proxy designed to filter the content of an HTTP endpoint's body.

It is intended to be used as a SideCar container within a Kubernetes Pod of a software application that does
not allow selection of exposed metrics.

In contexts where exposing too many metrics can lead to cardinality issues, Metrics Filter helps manage and
reduce the number of exposed metrics to improve Prometheus Crawlers' performance.

## Table of Contents

- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Usage

To use Metrics Filter, follow these steps:

1. Clone the repository.
2. Build the Docker image:

    ```bash
    docker build -t metrics-filter .
    ```

3. Deploy Metrics Filter as a SideCar container within your Kubernetes Pod.

## Configuration

Metrics Filter can be configured using environment variables. Below are the available configuration options:

- `LISTEN_PORT`: The port where Metrics Filter will listen for incoming requests. (default: `9090`)
- `TARGET_URL`: The target URL of the HTTP endpoint that Metrics Filter will proxy requests to. (default: `http://localhost:8080`)
- `EXCLUDED_METRICS_REGEX`: A regular expression to match against metrics. Only lines that **do not** match this
  expression will be passed through. All other lines will be filtered out.
  (default: `$^`, meaning no line is excluded)

## License

This project is licensed under the [MIT License](LICENSE).
