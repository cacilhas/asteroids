# Asteroids

A recreation of the classic Asteroids game.

## Build

### Debug

Use [`cargo`][]:

```sh
cargo +nightly build
```

Then run [Godot Editor][] in order to build the application.

**Note:** do NOT embed the PCK file into the binary, it’s pointless since the
[Rust][] library is necessarily shared.

In the future, when it has became possible to use Rust static library, it will
make sence to embed the PCK.

### Release

Use [`cargo`][]:

```sh
cargo +nightly build --release
```

The run [Godot Editor][] to build the application **without** debug symbols.

## Platform

This game is made using [Godot Engine][] 4.3, [Rust][] 1.81 nightly, and
[Godot-Rust][] 0.1.3 with API 4.2.

Since there isn’t support for API 4.3 until this release, [some resources][] are
set by using [GDScript][], but the target is to stuck to Rust as much as
possible.

## License

- [The 3-Clause BSD License][]
- [Copyright 2024 Rodrigo Cacilhας &lt;montegasppa@cacilhas.info&gt;][]

[`cargo`]: https://doc.rust-lang.org/cargo/
[Copyright 2024 Rodrigo Cacilhας &lt;montegasppa@cacilhas.info&gt;]: https://github.com/cacilhas/asteroids/blob/master/COPYING
[GDScript]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/
[Godot Editor]: https://editor.godotengine.org/
[Godot Engine]: https://godotengine.org/
[Godot-Rust]: https://godot-rust.github.io/
[Rust]: https://www.rust-lang.org/
[some resources]: https://github.com/cacilhas/asteroids/blob/master/parallax.gd
[The 3-Clause BSD License]: https://opensource.org/license/BSD-3-Clause
