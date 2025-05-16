# Bevy pong tutorial

This is the companion code to my tutorial for recreating pong with a minimum set
of dependencies for the purpose of learning Bevy.

You can read the full thing at: https://taintedcoders.com/bevy/tutorials/pong-tutorial

This guide is up to date with Bevy `0.16`.

## Usage

Clone this repo

```
git clone https://github.com/nolantait/pong-tutorial
```

Then you can run the game

```
cargo run
```

## Starting from scratch

You can create your own repo and follow along starting with the command:

```
cargo add bevy@0.16 --no-default-features --features "bevy_core_pipeline,bevy_render,bevy_text,bevy_ui,bevy_winit,default_font,multi_threaded,x11,wayland,webgl2"
```

We only use the plugins we need for this project and remove the rest to reduce
compilation time.

## Contributing

Bug reports and pull requests are welcome on GitHub at
https://github.com/nolantait/pong-tutorial.

## License

The gem is available as open source under the terms of the
[MIT License](https://opensource.org/licenses/MIT).
