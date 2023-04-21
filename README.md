# fimfiction-api

Deserialization for the JSON responses of the [Fimfiction][fimfiction] story API
(`https://www.fimfiction.net/api/story.php?story={ID}`).

```rust
let story: Story = fimfiction_api::from_str(&response)?.into();
```

# License

Distributed under the [Unlicense License](LICENSE).

[fimfiction]: https://www.fimfiction.net/
