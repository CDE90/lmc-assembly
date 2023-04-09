# Little Man Computer Assembly Language

This is my implementation of the LMC assembly language.

This implementation is based on the [LMC specification (wikipedia)](https://en.wikipedia.org/wiki/Little_man_computer) and [Peter Higginson's LMC simulator](https://peterhigginson.co.uk/lmc).

See the [examples](examples) directory for example programs.

## Web App

The web app is available at [https://lmc.ethancoward.dev](https://lmc.ethancoward.dev), which calls the [lmc-api](https://github.com/CDE90/lmc-api) to run the programs (this API is publically available at [https://api.lmc.ethancoward.dev](https://api.lmc.ethancoward.dev)). The web app is built using [SolidJS](https://www.solidjs.com/) and [TailwindCSS](https://tailwindcss.com/).

## Example CLI

I have created a separate repository for the CLI, [lmc-cli](https://github.com/CDE90/lmc-cli). This is a simple CLI that can be used to run LMC programs.

You can install it using `cargo install lmc-cli`.

## Example API

I have created a separate repository for the API, [lmc-api](https://github.com/CDE90/lmc-api). This is a simple API that can be used to run LMC programs. It is written in Rust using [actix-web](https://actix.rs/).

You can call this API at [https://api.lmc.ethancoward.dev](https://api.lmc.ethancoward.dev).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
