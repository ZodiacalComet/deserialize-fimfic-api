# fimfiction-api

A [`serde`][serde] implementation for the JSON responses of the [Fimfiction][fimfiction]
story API (`https://www.fimfiction.net/api/story.php?story={ID}`).

```rust
let story: Story = fimfiction_api::from_str(&response)?;
```

## Disclaimer

This small crate is not affiliated not endorsed in any way by [Fimfiction][fimfiction]
or its team. Do not bother them when it doesn't work as intended and instead I invite you to
[open an issue][issues].

## The `date` fields

By default, all dates are `i64` to save a dependency when unnecessary for an specific use case.
If you are going to handle them with the [`chrono`][chrono] crate, the library provides the
`chrono` feature to deserialize them as a `DateTime<Utc>`.

## License

Distributed under the [Unlicense License](LICENSE).

[fimfiction]: https://www.fimfiction.net/
[serde]: https://docs.rs/serde/1
[chrono]: https://docs.rs/chrono/0.4/chrono/

[issues]: https://github.com/ZodiacalComet/deserialize-fimfic-api/issues
