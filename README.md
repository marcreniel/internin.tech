# internin.tech

`internin.tech` is a high-performance web scraper developed in Rust for extracting internship job postings from various popular job listing websites. It supports scraping from multiple platforms using customizable patterns to ensure accurate data retrieval.

## Supported Websites

`internin.tech` can scrape job postings from the following websites:

- **Greenhouse** (`greenhouse.io`)
- **Lever** (`lever.co`)
- **Ashby** (`ashbyhq.com`)
- **Paylocity** (`paylocity.com`)
- **Workable** (`workable.com`)
- **iCIMS** (`icims.com`)
- **Workday Jobs** (`myworkdayjobs.com`)
- **Jobvite** (`jobvite.com`)
- **Breezy HR** (`breezy.hr`)
- **SmartRecruiters** (`jobs.smartrecruiters.com`)

## Features

- **Customizable Patterns**: Each supported site uses specific patterns to identify and extract job postings.
- **Rust Performance**: Built with Rust for high performance and safety.
- **Extendable**: Easily add support for additional websites by defining new patterns.

## Setup

1. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/internin.tech.git
   cd internin.tech
   ```

2. **Install dependencies**:
   ```sh
   cargo build
   ```

3. **Configure scraping rules**: Define the patterns for each website in the `src/rules.rs` file as needed.

4. **Run the scraper**:
   ```sh
   cargo run
   ```

## Configuration

The scraper uses `JobRule` objects to specify scraping rules for each supported site. Here's an example configuration for a site:

```rust
pub const JOB_RULES: &[JobRule] = &[
    JobRule {
        site: "greenhouse.io",
        patterns: GREENHOUSE_PATTERNS,
    },
    // Add more rules here
];
```

Ensure that the `patterns` constants (e.g., `GREENHOUSE_PATTERNS`) are defined and configured to match the structure of each website.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
