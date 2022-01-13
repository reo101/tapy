<div align="center">
    <h1> Tapy </h1>
    <img src="./logo/logo.png" alt="logo" width="256" height="256">
</div>

---

## Problem

Do you happen to have a huge collection of *favourited* (or *starred*, if you prefer) GIFs in Discord? Or maybe just hundreds of random photos in your Downloads directory?

Have you seen yourself in a situation where you need to quickly find a particular piece of media, let it be a GIF or a video, but just couldn't find it in your enormous arsenal?

Look no further - Tapy is here!

Tapy is a tool for tagging and quickly accessing all kinds of media. Just open it up, write a few keywords into the search box and voilà - everything is now filtered to only show media that matches your search tags.

## Dependencies (for now)

- [dragon-drag-and-drop](https://github.com/mwh/dragon)

## TODO

- [ ] Load configuration from file
- [ ] Create a custom MediaTile GObject (instead of hard-coded Box composition)
- [ ] Implement GridView (instead of ListView) with configurable number of columns
- [ ] Mute videos on start-up and possibly enable sound/playing on hover
- [ ] Integrate drag-and-drop functionality by translating a part of [dragon](https://github.com/mwh/dragon) (Also made in GTK) in Rust
- [ ] Keep an index of all media related to a certain tag for faster incremental filtering
