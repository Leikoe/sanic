# Tools

## `appledoc_md.py` — Apple Developer documentation as a markdown tree

The docs site is a JS app, so fetching a documentation URL gets you a title and
nothing else. It is backed by a JSON API — the same DocC render JSON the page
itself consumes:

```
https://developer.apple.com/tutorials/data/documentation/<path>.json
```

That carries the real content: abstract, declarations, parameters, discussion,
code listings, and a `topicSections` list naming the page's children. So the
whole tree is walkable.

```
./appledoc_md.py metal --out ..          # writes ../metal/**.md
./appledoc_md.py metal/mtldevice --out /tmp/probe --max-pages 20
```

A page with children becomes a directory holding `index.md`; a leaf becomes
`<name>.md`. Names are lowercased and stripped to `[a-z0-9._-]`, because symbol
paths carry argument lists (`newbufferwithlength(_:options:)`) and `:` has no
business in a filename — the true title and canonical URL are written into each
file, so the name only has to be stable and readable.

Re-running skips pages already on disk, so an interrupted crawl resumes; pass
`--refresh` to rewrite. `--delay` (default 0.15s) paces the requests — be
polite, it is someone else's server.

Cross-references stay as absolute `developer.apple.com` links rather than
relative paths into the tree, because references cross frameworks and only a
fraction of them are ever crawled.

### Why this is checked in

`applegpu/metal/` is the mirror. It is here so the Metal reference is greppable
offline, without a browser and without a network round trip per lookup.

It is **Apple's documentation, not measurement**, and the rule in this directory
still applies: nothing from it belongs in `hardware.md`, `bandwidth.md`,
`execution.md`, `counters.md`, or `msl.md` without a number measured on this
machine standing beside it.
