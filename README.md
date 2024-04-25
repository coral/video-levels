# Codec video level tool in Rust

If you've had to configure encoders of various kinds, you've probably stumbled upon [level tables](https://en.wikipedia.org/wiki/High_Efficiency_Video_Coding_tiers_and_levels). These are fun until you realize you have to copy this stuff into your code manually or "JUST SET LEVEL 5.2 BRO" which... sadly is more true than you think. Since I didn't want to to that I ended up writing this small helper tool that just has all this in code with sane checking so you can find the right level at runtime.

```rust
use video_levels::hevc::{LevelSelector, Profile, Tier};

let level = LevelSelector::new()
    .width(1920)
    .height(1080)
    .framerate(60.0)
    .tier(Tier::Main)
    .profile(Profile::Main)
    .select();

println!("Level: {:?}", level.unwrap().id());
```

But hey, your fancy MediaTek garbage dump android phone hardware decodeder doesn't support every level in the world? The TL;DR of what you want to do is clamp up to the level required by your configuration (width,height,fps etc) and then clamp up to a level that allows you headroom for your application. This is as easy as:

```rust
use video_levels::hevc::{LevelSelector, Profile, Tier};

let level = LevelSelector::new()
    .width(3840)
    .height(2160)
    .framerate(60.0)
    .tier(Tier::Main)
    .profile(Profile::Main)
    // Clamping between L4 and L5.2
    .clamp(Level::L4, Level::L5_2)
    .select();

println!("Level: {:?}", level.unwrap().id());
```

AV1 is equally easy

```rust
use video_levels::AV1::{LevelSelector};

let level = LevelSelector::new()
    .width(3840)
    .height(2160)
    .framerate(60.0)
    .select();

println!("Level: {:?}", level.unwrap().id());
```


## Why?

after going insane reading wikipedia and the ITU specs I was like... yeah ok i don't want to do this again

## Progress

- [x] HEVC
- [x] AV1
- [ ] H264 (Dreading this one lol)
- [ ] Make selector a trait

## Contributing

m8 just open a PR with some gucchimucchi code and I'll review it.

![KADSBUGGEL](https://raw.githubusercontent.com/coral/fluidsynth2/master/kadsbuggel.png)

## License

MIT -> go wild

## Acknowledgements

[ridley combs](https://github.com/rcombs) for help