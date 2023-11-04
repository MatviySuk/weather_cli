# **Weather CLI**

[<img alt="github" src="https://img.shields.io/badge/github-MatviySuk/weather_cli-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MatviySuk/weather_cli)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/MatviySuk/weather_cli/build.yaml?branch=main&style=for-the-badge" height="20">](https://github.com/MatviySuk/weather_cli/actions?query=branch%3Amain)

Weather CLI provides fast access to weather forecasts directly in your terminal. 

## Installation and Usage
Currently, we provides binaries for **`x86_64-pc-windows-gnu`**, **`x86_64-unknown-linux-musl`** and **`x86_64-apple-darwin`** targets. Look for them in the latest release note.

#### Linux
* Download **`weather_cli_{release_verions}_x86_64-unknown-linux-musl.tar.gz`** archive from the latest release note.

#### MacOS
* Download **`weather_cli_{release_verions}_x86_64-apple-darwin.zip`** archive from the latest release note.

#### Windows
* Download **`weather_cli_{release_verions}_x86_64-pc-windows-gnu.zip`** archive from the latest release note.

Retrive the binary **`weather`** from the downloaded archive and move it to your PATH for more comfortable usage.

Documentation for all the available Cli commands can be accessed with the following command.
 
```bash
weather --help
```

## Available functionality
Weather cli provides current weather information as well as forecasts for **`24 hours`**, **`3 days`** and **`5 days`**.

## Places
Cli tool has support for managing frequently used places, stored with **`tag`** by their geodetic coordinats.

## Configure providers
You can use any of the providers available in the tool.

* [OpenWeather](https://openweathermap.org)
* [weatherapi](https://www.weatherapi.com)

To configure the data provider you call this commands.

```bash
weather configure <provider> --key <KEY>
```

Where `<provider>` subcommand could be either `open-weather` or `weather-api`.
* A free tier subscription from any provider is sufficient for all the functionality available in the tool.



## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
