# bevy-mp-experiments

A small repo of some experiments with Bevy and Multiplayer networking. For a somewhat secret project.

## General requirements

Must have:

- Works with Bevy
- Low enough latency for FPS games
- ~8 connections "good"

Nice to have:

- Rust native
- WASM support (browsers ideally)
- P2P
- 16-32 connections "good"

## Leads

- [lightyear][lightyear]
- [renet][renet]
- [bevy_replicon][replicon]

## Research notes

### Bevy Replicon

[Repo][replicon]

> Server-authoritative networking crate for the Bevy game engine.


### Lightyear

[Repo][lightyear]


### Renet

[Repo][renet]


## Comparisons

| Name | WASM support |
| --- | --- | 
| bevy_replicon | :question: |
| lightyear | ✅ | 
| renet | :question: | 


[lightyear]: https://github.com/cBournhonesque/lightyear
[renet]: https://github.com/lucaspoffo/renet
[replicon]: https://github.com/projectharmonia/bevy_replicon

