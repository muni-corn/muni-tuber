# muni-tuber

the vtubing app made for municorn (me), but also for you!

## configuration

muni-tuber is in rapid development right now, so everything here is very much
subject to change. lots of change. for now, here's how you can tinker with the
app:

### place images in `src/assets`

place the following .png images in the `src/assets` folder. for now, each image needs to be
the same size. eventually, we'll be able to configure individual placement for
images of different sizes, so that'll be nice!

- "head base" images:
	- `quiet.png`: for when your character isn't speaking
	- `half_speak.png`: head with half-open mouth for speaking
	- `full_speak.png`: head with fully-open mouth for speaking
- eyes:
	- `eyes_open.png`: open eyes, default state
	- `eyes_closed.png`: closed eyes, for blinking

### adjust speaking activation levels

right now, volume thresholds are configured as `const`s in the source code. in
`src/main.rs`, change the values of `HALF_SPEAK_THRESHOLD_DBFS` and
`FULL_SPEAK_THRESHOLD_DBFS` as needed.

## running

ensure you have a [Rust toolchain installed](https://rustup.rs). this app has been tested with Rust
1.71.0 (stable) on NixOS Linux and swaywm.

first, clone this repo with `git`:

```shell
git clone https://codeberg.org/municorn/muni-tuber
```

### dependencies

this app uses `eframe` and `egui`, so according to them, this app should run
just compile and run just fine on Windows and macOS.

if you're on Linux, you will need to install some extra dependencies first. if
you use `nix`, this repo provides a dev shell that you can use with `nix
develop`. otherwise, see [eframe's documentation](https://github.com/emilk/egui/tree/master/crates/eframe) for
info on needed dependencies.

### ready?

once you're all set, it should be as simple as running

```shell
cargo run
```

the app should use your default microphone as input. enjoy!!

if you run into problems, don't hesitate to [open an issue here](https://codeberg.org/municorn/muni-tuber/issues)!
