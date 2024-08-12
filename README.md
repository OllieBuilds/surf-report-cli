# Surf Report CLI

Reverse engineering of your favorite surf forecasting site's API to get the surf report from the comfort of your terminal.

## Description

You've already checked the surf report 20 times today, and you're still disappointed every time you open that browser tab.

Why not just check it from your terminal? That way you can be disappointed _and_ efficient!

To be honest: this isn't even that efficient.

This project leverages services that require a subscription. IFYKYK... don't be a kook.

## Getting Started

### Installing

* Clone repo
* Use cargo build to install dependencies and compile

### Executing program

* Create `config.txt`
* Populate with necessary credentials from your favorite surf forecasting site
* Check tide data via CLI:
```sh
cargo run --bin surf-report-cli -- --spot "<Your local break>"
```

## License

This project is licensed under the MIT License - see the LICENSE.md file for details