# fimfiction-api

Deserialization for the JSON responses of the [Fimfiction][fimfiction] story API
(`https://www.fimfiction.net/api/story.php?story={ID}`).

```rust
let story: Story = fimfiction_api::from_str(&response)?;
```

## Disclaimer

This small crate is not affiliated not endorsed in any way by [Fimfiction][fimfiction]
or its team. Do not bother them when it doesn't work as intended and instead I invite you to
open an issue in this repository.

## The `date` fields

By default, they are `i64` which if you don't need them it saves you having a dependency that you
wont be using. But if you need to do something with it maybe you would use the [`chrono`][chrono]
crate, in that case you can enable the `chrono` feature to convert them into `DateTime<Utc>`.

## License

Distributed under the [Unlicense License](LICENSE).

[fimfiction]: https://www.fimfiction.net/
[chrono]: https://docs.rs/chrono/0.4/chrono/
