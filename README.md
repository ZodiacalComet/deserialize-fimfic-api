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

## License

Distributed under the [Unlicense License](LICENSE).

[fimfiction]: https://www.fimfiction.net/
