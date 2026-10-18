# Quran Island

**An iPhone-style Dynamic Island for your Windows desktop that plays the Quran.**

> **Work in progress.** Nothing plays yet: this is the very first scaffolding (milestone M0).
> I'm also using this project to learn Rust, so you'll see that in the issues and commit history.

A small pill sits at the top of your screen showing the current surah. Hover it and it
expands with the reciter, the current ayah and the controls.

Built with [Tauri 2](https://tauri.app): the logic is in Rust and the UI is plain HTML, CSS and
JavaScript, rendered by WebView2 (already part of Windows 11). No Electron, so the app stays small.

Sister project: [F1 Island](https://github.com/AymanBAHHOU/f1-island).

## Roadmap

- [x] **M0** Scaffolding: workspace, empty pill window, first exercises
- [ ] **M1** Play Al-Fatihah from one reciter
- [ ] **M2** Choose any surah and reciter, cache audio on disk
- [ ] **M3** Show the current ayah's Arabic text and translation, in sync with the audio
- [ ] **M4** Hifz mode (repeat each ayah N times over a range), resume, sleep timer, media keys, tray, global shortcut
- [ ] **M5** Installer on GitHub Releases

## Run from source

You need [Rust](https://rustup.rs) (on Windows, accept the "Visual Studio" install it offers).

```bash
cargo run -p quran-island
```

Run the tests:

```bash
cargo test --workspace
```

## Project layout

```
crates/
  quran-core/    pure logic, no I/O: surahs, verse keys, playlists
  quran-data/    network and disk: audio URLs, quran.com API, cache
  quran-audio/   the audio player, on its own thread
app/
  src-tauri/     the Tauri app: window, tray, commands the UI can call
  ui/            HTML / CSS / JS of the island
```

## Learning Rust with this project

Issues labelled [`learn-rust`](../../issues?q=label%3Alearn-rust) and [`learn-ci`](../../issues?q=label%3Alearn-ci)
are the parts I'm writing myself to learn. In the code they're marked `LEARN(#n)`, where `n` is the
issue number, and their tests are `#[ignore]`d until I start each one.

## Data sources & credits

- Surah metadata: [quran.com API](https://api-docs.quran.foundation/)
- Recitation audio (planned): [everyayah.com](https://everyayah.com)
- Quran text (planned): [quran.com API](https://api-docs.quran.foundation/) / [Tanzil](https://tanzil.net), used unmodified

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
This covers the code only; the Quran text and audio belong to their respective sources above.
