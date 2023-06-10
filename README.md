
<div align="center"><p>
    <a href="https://github.com/michaelb/ouverture/releases/latest">
      <img alt="Latest release" src="https://img.shields.io/github/v/release/michaelb/ouverture" />
    </a>
    <a href="https://github.com/michaelb/ouverture/releases">
      <img alt="Total downloads" src="https://img.shields.io/github/downloads/michaelb/ouverture/total" />
    </a>
    <a href="https://github.com/michaelb/ouverture/pulse">
      <img alt="Last commit" src="https://github.com/michaelb/ouverture/actions/workflows/main.yml/badge.svg"/>
    </a>
    <a href="https://codecov.io/gh/michaelb/ouverture">
      <img src="https://codecov.io/gh/michaelb/ouverture/branch/main/graph/badge.svg?token=455HOJY6A7"/>
    </a>
</p></div>


# ouverture
A next-generation music player and manager

Very much at Work-In-*Planning* stage now 😄


## Planned features

 - [ ] GUI (localizable)
 - [ ] Backend (something like quodlibet would be nice for the basic features)
 - [ ] Cross-platform
 - [ ] Offline **and** online (ex: youtube-dl integration)
 - [ ] Recommandation system at least a little smart
 - [ ] Follow/sync, p2p or server tbd
 - [ ] Fuzzy search as fast as possible
 - [ ] Nice metadata handling
 - [ ] Duplicate management
 - [ ] auto playlists (auto genre, artist and 'mood' - whatever it means - detection)

## Roadmap

 - [x] working GUI framework
 - [x] cli can communicate with server
 - [x] define Song & other structs (playlist..)
 - [x] use the database to store those values
 - [x] display songs in a UI window
 - [ ] fix warnings & clean out / refactor
 - [ ] test (target > 60%)
 - [x] play a sound
 - [x] play/pause
 - [ ] interface (seek bar, crossfade)
 - [ ] logo
 - [ ] searchable library
 - [ ] playlists, sort by...
 - [ ] fix warnings & clean out / refactor
 - [ ] unit test (target > 80%)
 - [x] fully themable GUI
 - [ ] youtube-dl[p] integration
 - [ ] beats/metadata service to fix metadata
 - [ ] packaging (arch at least) & CI
 - [ ] follow / sync semantics (private for now)
 - [ ] music server
 - [ ] public follow/sync (torrent?)


## Some tech I plan to use or take inspiration from

- [QuodLibet](https://github.com/quodlibet/quodlibet)
- [iced](https://github.com/iced-rs/iced)
- [termusic](https://github.com/tramhao/termusic)
- [postgres](https://github.com/postgres/postgres)
- [beets](https://github.com/beetbox/beets)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
- [rustic](https://github.com/rustic-music-player/rustic)

## Maybe-useful ressources

- [winservice](https://lib.rs/crates/winservice)
- [Shawl](https://github.com/mtkennerly/shawl)
- [fluent-rs](https://github.com/projectfluent/fluent-rs)
- [clustering with linfa](https://github.com/rust-ml/linfa/tree/master/algorithms/linfa-clustering)
- [embbed postgres](https://crates.io/crates/pg-embed)
- [seaORM](https://www.sea-ql.org/SeaORM/docs/introduction/async)
- [diesel](https://github.com/diesel-rs/diesel)
- capnproto / bincode
- [platform-dirs](https://crates.io/crates/platform-dirs)
- [chamomille](https://github.com/cypherlink/chamomile)
- [dameonizer](https://github.com/knsd/daemonize)
- [uds on windows](https://crates.io/crates/uds_windows)
- [dim](https://github.com/Dusk-Labs/dim)


## Dependencies

 - $${\color{grey}postresql (automatically fetched from maven repos at compile time and embedded)}$$
 - libpulse (Linux)
