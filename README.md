Simple service to retrieve IP location written in Rust for learning purposes.

Only implemented one route:

- /country/\<ip>

Implementing /city and other routes are super simple. Try it.

## Running

For running, you need Rust installed and using the nightly version.
More information: https://github.com/SergioBenitez/Rocket/issues/19

You can configure nightly version as default:
```sh
rustup default nightly
```
Or only for this project running the following command in the project directory:
```sh
rustup override set nightly
```

After installing, you can run the application with the following command:
```sh
DATABASE=geoip2.mmdb cargo run
```

Calling the API:

```sh
curl -s http://localhost:8000/country/1.1.1.1 | jq
```

Return a simple json:

```json
{
  "geoname_id": 2077456,
  "is_in_european_union": false,
  "iso_code": "AU",
  "names": {
    "de": "Australien",
    "en": "Australia",
    "es": "Australia",
    "fa": "استرالیا",
    "fr": "Australie",
    "ja": "オーストラリア",
    "ko": "오스트레일리아",
    "pt-BR": "Austrália",
    "ru": "Австралия",
    "zh-CN": "澳大利亚"
  }
}
```

## Docker

Running and compiled code inside Docker containers have some tricks for caching 
and have the right dependencies. See Dockerfile for more information.

Building:
```sh
docker build -t georust .
```

Running:
```sh
docker run -d --name georust -v "$(pwd)"/geoip.mmdb:/data/geoip2.mmdb:ro -e DATABASE=/data/geoip2.mmdb -p 8000:8000 georust
```

Stop and cleanup:
```sh
docker rm -f georust
```

## Tests

Pending.

## Learning Rust?

I'm studying Rust reading the official rust book: https://doc.rust-lang.org/book/
and reading some other guides like Rocket guide: https://rocket.rs/v0.4/guide/ 

Keep Learning 📖