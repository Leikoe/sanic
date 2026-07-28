#!/usr/bin/env python3
"""Download Apple's Metal documentation into this directory as markdown.

    ./dl.py                 # mirror `metal` into ./
    ./dl.py metal/mtlbuffer # just one subtree
    ./dl.py --jobs 32       # more concurrency

**The corpus is not checked in.** It is Apple's copyrighted documentation and
not ours to redistribute, so `.gitignore` here keeps everything except this
script. Run it and you have the tree locally; that is the whole point — the
Metal reference greppable offline, without a browser and without a network
round trip per lookup.

How it works: developer.apple.com is a JS app, so fetching a documentation URL
gets you a title and nothing else. It is backed by the same DocC render JSON
the page itself consumes —

    https://developer.apple.com/tutorials/data/documentation/<path>.json

— which carries the abstract, declarations, parameters, discussion, code
listings, and a `topicSections` list naming the page's children. So the tree is
walkable, and this walks it breadth-first, mirroring the URL hierarchy: a page
with children becomes a directory holding `index.md`, a leaf becomes
`<name>.md`.

Round-trip latency dominates — fetching one page at a time manages under one
page per second, while a few dozen in flight do sixty or more, which is the
difference between a coffee break and a minute. Concurrency is capped by a
semaphore; be reasonable, it is someone else's server.

Re-running skips what is already on disk, so an interrupted run resumes.
Stdlib only, no third-party packages: the blocking fetch runs in asyncio's
thread pool, which is all this workload needs.
"""

import argparse
import asyncio
import json
import os
import re
import sys
import urllib.error
import urllib.request
from collections import deque

API = "https://developer.apple.com/tutorials/data/documentation/{}.json"
UA = "sanic-applegpu-docs (github.com/Leikoe/sanic)"
HERE = os.path.dirname(os.path.abspath(__file__))

# ── fetching ────────────────────────────────────────────────────────────────


def fetch_blocking(path, tries=4):
    """The render JSON for a documentation path, or None if it has none.

    Not every reference resolves to a page — some point at other frameworks,
    some at anchors. A 404 is an ordinary outcome here, not an error.
    """
    url = API.format(path)
    for attempt in range(tries):
        try:
            request = urllib.request.Request(url, headers={"User-Agent": UA})
            with urllib.request.urlopen(request, timeout=30) as response:
                return json.load(response)
        except urllib.error.HTTPError as error:
            if error.code in (404, 410):
                return None
            if attempt == tries - 1:
                return None
        except Exception:
            if attempt == tries - 1:
                return None
    return None


async def fetch(path, limit):
    async with limit:
        return await asyncio.to_thread(fetch_blocking, path)


# ── rendering ───────────────────────────────────────────────────────────────


def inline(nodes, refs):
    """DocC inline content → markdown."""
    out = []
    for n in nodes or []:
        kind = n.get("type")
        if kind == "text":
            out.append(n.get("text", ""))
        elif kind == "codeVoice":
            out.append(f"`{n.get('code','')}`")
        elif kind in ("emphasis", "newTerm"):
            out.append(f"*{inline(n.get('inlineContent'), refs)}*")
        elif kind in ("strong", "inlineHead"):
            out.append(f"**{inline(n.get('inlineContent'), refs)}**")
        elif kind == "strikethrough":
            out.append(f"~~{inline(n.get('inlineContent'), refs)}~~")
        elif kind == "reference":
            ref = refs.get(n.get("identifier"), {})
            title = ref.get("title") or n.get("identifier", "")
            url = ref.get("url", "")
            out.append(f"[{title}]({absolute(url)})" if url else title)
        elif kind == "link":
            out.append(f"[{n.get('title','')}]({n.get('destination','')})")
        elif kind == "image":
            variants = (refs.get(n.get("identifier"), {}) or {}).get("variants") or []
            src = variants[0].get("url", "") if variants else ""
            out.append(f"![image]({src})" if src else "")
        elif "inlineContent" in n:
            out.append(inline(n["inlineContent"], refs))
        elif "text" in n:
            out.append(n["text"])
    return "".join(out)


def absolute(url):
    """Cross-references stay absolute.

    A relative link into the mirrored tree would be wrong as often as right —
    references cross frameworks, and only a fraction of them get downloaded.
    """
    return f"https://developer.apple.com{url}" if url.startswith("/") else url


def blocks(nodes, refs):
    """DocC block content → markdown."""
    out = []
    for n in nodes or []:
        kind = n.get("type")
        if kind == "paragraph":
            out.append(inline(n.get("inlineContent"), refs))
        elif kind == "heading":
            out.append(f"{'#' * min(int(n.get('level', 2)), 6)} {n.get('text','')}")
        elif kind == "codeListing":
            out.append(f"```{n.get('syntax') or ''}\n" + "\n".join(n.get("code") or []) + "\n```")
        elif kind in ("unorderedList", "orderedList"):
            for index, item in enumerate(n.get("items") or [], 1):
                bullet = f"{index}." if kind == "orderedList" else "-"
                body = blocks(item.get("content"), refs)
                out.append(f"{bullet} " + hang(body, " " * (len(bullet) + 1)))
        elif kind == "aside":
            style = (n.get("style") or n.get("name") or "note").capitalize()
            body = blocks(n.get("content"), refs)
            out.append("\n".join(f"> {line}" for line in [f"**{style}:**"] + body.split("\n")))
        elif kind == "termList":
            for item in n.get("items") or []:
                term = inline((item.get("term") or {}).get("inlineContent"), refs)
                body = blocks((item.get("definition") or {}).get("content"), refs)
                out.append(f"- **{term}** — {body}")
        elif kind == "table":
            out.append(table(n, refs))
        elif kind in ("row", "column", "small", "tabNavigator", "dictionaryExample"):
            for key in ("columns", "content", "tabs"):
                if key in n:
                    out.append(blocks(n[key], refs))
        elif "content" in n:
            out.append(blocks(n["content"], refs))
    return "\n\n".join(part for part in out if part.strip())


def hang(text, pad):
    lines = text.split("\n")
    return "\n".join([lines[0]] + [pad + line if line else line for line in lines[1:]])


def table(node, refs):
    rows = node.get("rows") or []
    if not rows:
        return ""
    cells = [[blocks(cell, refs).replace("\n", " ") for cell in row] for row in rows]
    width = max(len(row) for row in cells)
    cells = [row + [""] * (width - len(row)) for row in cells]
    if node.get("header") == "row":
        head, body = cells[0], cells[1:]
    else:
        head, body = [""] * width, cells
    lines = ["| " + " | ".join(head) + " |", "|" + "|".join(["---"] * width) + "|"]
    return "\n".join(lines + ["| " + " | ".join(row) + " |" for row in body])


def render(doc, path):
    """One page's render JSON → markdown."""
    refs = doc.get("references") or {}
    meta = doc.get("metadata") or {}
    out = [f"# {meta.get('title') or path.split('/')[-1]}"]

    role = meta.get("roleHeading") or meta.get("symbolKind")
    platforms = ", ".join(
        f"{p.get('name','')} {p.get('introducedAt','')}".strip() for p in (meta.get("platforms") or []) if p.get("name")
    )
    tagline = " · ".join(part for part in [role, platforms] if part)
    if tagline:
        out.append(f"*{tagline}*")
    out.append(f"<https://developer.apple.com/documentation/{path}>")

    abstract = inline(doc.get("abstract"), refs)
    if abstract:
        out.append(abstract)

    for section in doc.get("primaryContentSections") or []:
        kind = section.get("kind")
        if kind == "declarations":
            decls = []
            for decl in section.get("declarations") or []:
                tokens = "".join(t.get("text", "") for t in decl.get("tokens") or [])
                language = (decl.get("languages") or [""])[0]
                decls.append(f"```{language}\n{tokens}\n```")
            if decls:
                out.append("## Declaration\n\n" + "\n\n".join(decls))
        elif kind == "parameters":
            items = [
                f"- **{p.get('name','')}** — " + blocks(p.get("content"), refs).replace("\n\n", " ")
                for p in section.get("parameters") or []
            ]
            if items:
                out.append("## Parameters\n\n" + "\n".join(items))
        elif kind == "content":
            body = blocks(section.get("content"), refs)
            if body:
                out.append(body)
        elif kind == "possibleValues":
            items = [
                f"- `{v.get('name','')}` — " + blocks(v.get("content"), refs) for v in section.get("values") or []
            ]
            if items:
                out.append("## Possible values\n\n" + "\n".join(items))

    for key, heading in (("topicSections", "Topics"), ("seeAlsoSections", "See also")):
        chunks = []
        for section in doc.get(key) or []:
            lines = [f"### {section['title']}"] if section.get("title") else []
            for identifier in section.get("identifiers") or []:
                ref = refs.get(identifier, {})
                name = ref.get("title") or identifier
                url = absolute(ref.get("url", ""))
                summary = inline(ref.get("abstract"), refs)
                entry = f"- [{name}]({url})" if url else f"- {name}"
                lines.append(f"{entry} — {summary}" if summary else entry)
            if len(lines) > (1 if section.get("title") else 0):
                chunks.append("\n".join(lines))
        if chunks:
            out.append(f"## {heading}\n\n" + "\n\n".join(chunks))

    return "\n\n".join(out).rstrip() + "\n"


# ── tree layout ─────────────────────────────────────────────────────────────

UNSAFE = re.compile(r"[^a-z0-9._-]+")


def slug(segment):
    """A URL segment → a filesystem-safe, lowercase name.

    Symbol paths carry argument lists (`newbufferwithlength(_:options:)`), and
    `:` has no business in a macOS filename. The real title and canonical URL
    are written into the file, so the name only has to be stable and readable.
    """
    return UNSAFE.sub("-", segment.lower()).strip("-") or "index"


def children_of(doc):
    """Documentation paths this page lists as its children."""
    refs = doc.get("references") or {}
    out = []
    for section in doc.get("topicSections") or []:
        for identifier in section.get("identifiers") or []:
            url = (refs.get(identifier) or {}).get("url") or ""
            if url.startswith("/documentation/"):
                out.append(url[len("/documentation/") :])
    return out


async def run(args):
    root = args.root.strip("/")
    prefix = root.split("/")[0]
    limit = asyncio.Semaphore(args.jobs)
    queue = deque([root])
    seen = {root}
    written = skipped = missing = 0

    while queue:
        batch = [queue.popleft() for _ in range(min(args.jobs * 4, len(queue)))]
        for path, doc in zip(batch, await asyncio.gather(*(fetch(p, limit) for p in batch))):
            if doc is None:
                missing += 1
                continue

            kids = children_of(doc)
            segments = [slug(s) for s in path.split("/")]
            # a page with children owns a directory holding index.md;
            # a leaf is just <name>.md
            target = (
                os.path.join(args.out, *segments, "index.md") if kids else os.path.join(args.out, *segments) + ".md"
            )
            if os.path.exists(target) and not args.refresh:
                skipped += 1
            else:
                os.makedirs(os.path.dirname(target), exist_ok=True)
                with open(target, "w") as handle:
                    handle.write(render(doc, path))
                written += 1
                if written % 250 == 0:
                    print(f"  {written} pages, {len(queue)} queued", file=sys.stderr, flush=True)

            for child in kids:
                if child not in seen and child.startswith(prefix + "/"):
                    seen.add(child)
                    queue.append(child)

    print(f"written={written} skipped={skipped} missing={missing}", file=sys.stderr)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("root", nargs="?", default="metal", help="documentation path (default: metal)")
    parser.add_argument("--out", default=os.path.dirname(HERE), help="output directory (default: alongside this file)")
    parser.add_argument("--jobs", type=int, default=24, help="concurrent fetches")
    parser.add_argument("--refresh", action="store_true", help="rewrite pages already on disk")
    asyncio.run(run(parser.parse_args()))


if __name__ == "__main__":
    main()
