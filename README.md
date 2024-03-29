# WGSL 2d Outline Shader
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_simple_2d_outline)](https://crates.io/crates/bevy_simple_2d_outline)
[![docs.rs](https://docs.rs/bevy_simple_2d_outline/badge.svg)](https://docs.rs/bevy_simple_2d_outline)

2D outline materials written in wgsl for the bevy game engine.

## Shaders
- Single color outline
- Single color outline with original texture
- Rainbow animated outline
- Rainbow animated outline with original texture

## Examples
<img width="172" alt="Screenshot 2022-12-29 at 12 28 41 AM" src="https://user-images.githubusercontent.com/109775391/209907264-c4a08845-df81-4035-a5d4-13ef519e32ca.png">

```bash
$ cargo run --example outline
```

<img width="165" alt="Outline and texture" src="https://user-images.githubusercontent.com/109775391/209907338-4e3ceaee-b186-42d9-9341-1b607d5ba582.png">

```bash
$ cargo run --example outline_and_texture
```

![Rainbow Outline Gif](https://user-images.githubusercontent.com/109775391/209907583-81128432-71e3-4d86-b2e8-393fc35632db.gif)


```bash
$ cargo run --example rainbow_outline
```

![Rainbow Outline And Texture Gif](https://user-images.githubusercontent.com/109775391/209907698-8d9443a7-1be1-47f5-8285-4ed01a7b3e10.gif)


```bash
$ cargo run --example rainbow_outline_and_texture
```

## Compatibility

| Bevy | `bevy-simple-2d-outline` |
| ---- |--------------------------|
| 0.11 | 0.2.11                   |
| 0.10 | 0.2.0                    |

