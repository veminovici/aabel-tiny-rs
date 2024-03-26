# Simplee > Aabel > Tiny

[![Crates.io][crates-badge]][crates-url]
[![CI][ci-badge]][ci-url]
![GitHub top language][lang-badge]
[![License:MIT][license-badge]][license-url]
![GitHub code size in bytes][size-badge]
![GitHub last commit][last-commit-badge]
![GitHub watchers][watchers-badge]

A Url shortener service. Uses axum a a web framework.

## Shorten the url
```bsh
curl \
  --header "Content-Type: application/json" \
  --request POST \
  --data '{"url":"http://whalar.com"}' \
  http://localhost:3000/api/v1/tiny
```

## Retrieve the url
```bsh
curl -v \
  --request GET \
  http://localhost:3000/api/v1/tiny\?url\="0fe5e13014"
```

## About
> Code designed and written on the beautiful island of [**Saaremaa**][url_estonia], Estonia.

[crates-badge]: https://img.shields.io/crates/v/aabel-tiny-rs.svg
[crates-url]: https://crates.io/crates/aabel-tiny-rs
[ci-badge]: https://github.com/veminovici/aabel-tiny-rs/actions/workflows/ci.yml/badge.svg?branch=main
[ci-url]: https://github.com/veminovici/aabel-tiny-rs/actions/workflows/ci.yml
[lang-badge]: https://img.shields.io/github/languages/top/veminovici/aabel-tiny-rs
[license-badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[license-url]: https://opensource.org/licenses/MIT
[size-badge]: https://img.shields.io/github/languages/code-size/veminovici/aabel-tiny-rs
[last-commit-badge]: https://img.shields.io/github/last-commit/veminovici/aabel-tiny-rs
[watchers-badge]: https://img.shields.io/github/watchers/veminovici/aabel-tiny-rs
[url_estonia]: https://goo.gl/maps/DmB9ewY2R3sPGFnTA