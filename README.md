# bevy_replicon_visible_sets

An alternative to `bevy_replicon`'s high-level Visibility Rules API, with sets and layers.
Works by relating entities inside of `VisibleContainer`s, which can be visible to `VisibleObserver`s. 
Clients can then utilize these `VisibleObservers` to view entities in their vicinity.

Simplified diagram of components at play:
![Diagram of Component Relations](./client_visible.png)

## Features

- [ ] Visibility Rules apply client entities
- [ ] Always Visible entities
- [ ] Visibility Layers
- [ ] Component Visibility

## Limitations

- memory cost will scale linearly with number of Replicated entities, as *ALL* entities need to be saved in `ClientVisibility`.
- no component visibility rules.
- worse overall.

## License

Dual-licensed under either ([MIT License](LICENSE-MIT) or ([Apache License, Version 2.0](LICENSE-APACHE) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Compatibility

| `bevy` | `bevy_replicon` | `bevy_replicon_visible_sets` |
|--------|-----------------|------------------------------|
| 0.19   | 0.41            | 0.0.1                        |
