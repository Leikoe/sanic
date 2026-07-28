#!/usr/bin/env python3
"""Mirror Apple Developer documentation into a tree of markdown files.

The docs site is a JS app backed by a JSON API — the same DocC render JSON the
page itself consumes:

    https://developer.apple.com/tutorials/data/documentation/<path>.json

So `documentation/metal/mtldevice` is one GET away from structured content, and
its `topicSections` name its children. This walks that tree breadth-first and
writes one markdown file per page, mirroring the URL hierarchy: a page with
children becomes a directory holding `index.md`.

    ./appledoc_md.py metal --out ../metal

Re-running skips pages already written, so an interrupted crawl resumes. Pass
`--refresh` to rewrite them.
"""

import argparse
import json
import os
import re
import sys
import time
import urllib.error
import urllib.request
from collections import deque
from concurrent.futures import ThreadPoolExecutor

API = "https://developer.apple.com/tutorials/data/documentation/{}.json"
UA = "sanic-applegpu-docs (github.com/Leikoe/sanic)"

# ── fetching ────────────────────────────────────────────────────────────────


def fetch(path, tries=4, delay=0.3):
    """The render JSON for a documentation path, or None if it has none.

    Not every reference resolves to a page: some point at other frameworks,
    some at anchors. A 404 is an ordinary outcome, not an error.
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
                raise
        except Exception:
            if attempt == tries - 1:
                raise
        time.sleep(delay * (2**attempt))
    return None


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
            out.append(f"[{title}]({link_target(url)})" if url else title)
        elif kind == "link":
            out.append(f"[{n.get('title','')}]({n.get('destination','')})")
        elif kind == "image":
            ref = refs.get(n.get("identifier"), {})
            variants = ref.get("variants") or []
            src = variants[0].get("url", "") if variants else ""
            out.append(f"![image]({src})" if src else "")
        else:
            # unknown inline kind: keep whatever text it carries
            if "inlineContent" in n:
                out.append(inline(n["inlineContent"], refs))
            elif "text" in n:
                out.append(n["text"])
    return "".join(out)


def link_target(url):
    """Doc URLs stay as absolute developer.apple.com links.

    A relative link into the mirrored tree would be wrong as often as right —
    references cross frameworks, and only a fraction of them get crawled.
    """
    return f"https://developer.apple.com{url}" if url.startswith("/") else url


def blocks(nodes, refs, depth=0):
    """DocC block content → markdown."""
    out = []
    for n in nodes or []:
        kind = n.get("type")
        if kind == "paragraph":
            out.append(inline(n.get("inlineContent"), refs))
        elif kind == "heading":
            level = min(int(n.get("level", 2)), 6)
            out.append(f"{'#' * level} {n.get('text','')}")
        elif kind == "codeListing":
            syntax = n.get("syntax") or ""
            code = "\n".join(n.get("code") or [])
            out.append(f"```{syntax}\n{code}\n```")
        elif kind in ("unorderedList", "orderedList"):
            for index, item in enumerate(n.get("items") or [], 1):
                bullet = f"{index}." if kind == "orderedList" else "-"
                body = blocks(item.get("content"), refs, depth + 1)
                body = indent_after_first(body, " " * (len(bullet) + 1))
                out.append(f"{bullet} {body}")
        elif kind == "aside":
            style = (n.get("style") or n.get("name") or "note").capitalize()
            body = blocks(n.get("content"), refs, depth + 1)
            out.append("\n".join(f"> {line}" for line in [f"**{style}:**"] + body.split("\n")))
        elif kind == "termList":
            for item in n.get("items") or []:
                term = inline((item.get("term") or {}).get("inlineContent"), refs)
                definition = blocks((item.get("definition") or {}).get("content"), refs, depth + 1)
                out.append(f"- **{term}** — {definition}")
        elif kind == "table":
            out.append(table(n, refs))
        elif kind in ("row", "column", "small", "tabNavigator", "dictionaryExample"):
            for key in ("columns", "content", "tabs"):
                if key in n:
                    out.append(blocks(n[key], refs, depth))
        elif "content" in n:
            out.append(blocks(n["content"], refs, depth))
    return "\n\n".join(part for part in out if part.strip())


def indent_after_first(text, pad):
    lines = text.split("\n")
    return "\n".join([lines[0]] + [pad + line if line else line for line in lines[1:]])


def table(node, refs):
    rows = node.get("rows") or []
    if not rows:
        return ""
    header = node.get("header")
    cells = [[blocks(cell, refs).replace("\n", " ") for cell in row] for row in rows]
    width = max(len(row) for row in cells)
    cells = [row + [""] * (width - len(row)) for row in cells]
    if header == "row":
        head, body = cells[0], cells[1:]
    else:
        head, body = [""] * width, cells
    lines = ["| " + " | ".join(head) + " |", "|" + "|".join(["---"] * width) + "|"]
    lines += ["| " + " | ".join(row) + " |" for row in body]
    return "\n".join(lines)


def declaration(section, refs):
    out = []
    for decl in section.get("declarations") or []:
        tokens = "".join(t.get("text", "") for t in decl.get("tokens") or [])
        languages = ", ".join(decl.get("languages") or [])
        out.append(f"```{languages.split(',')[0].strip() or ''}\n{tokens}\n```")
    return "\n\n".join(out)


def parameters(section, refs):
    out = []
    for parameter in section.get("parameters") or []:
        name = parameter.get("name", "")
        body = blocks(parameter.get("content"), refs).replace("\n\n", " ")
        out.append(f"- **{name}** — {body}")
    return "\n".join(out)


def render(doc, path):
    """One page's render JSON → markdown."""
    refs = doc.get("references") or {}
    meta = doc.get("metadata") or {}
    title = meta.get("title") or path.split("/")[-1]

    out = [f"# {title}"]

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
            body = declaration(section, refs)
            if body:
                out.append("## Declaration\n\n" + body)
        elif kind == "parameters":
            body = parameters(section, refs)
            if body:
                out.append("## Parameters\n\n" + body)
        elif kind == "content":
            body = blocks(section.get("content"), refs)
            if body:
                out.append(body)
        elif kind == "possibleValues":
            items = []
            for value in section.get("values") or []:
                items.append(f"- `{value.get('name','')}` — {blocks(value.get('content'), refs)}")
            if items:
                out.append("## Possible values\n\n" + "\n".join(items))

    for key, heading in (("topicSections", "Topics"), ("seeAlsoSections", "See also")):
        chunks = []
        for section in doc.get(key) or []:
            lines = []
            if section.get("title"):
                lines.append(f"### {section['title']}")
            for identifier in section.get("identifiers") or []:
                ref = refs.get(identifier, {})
                name = ref.get("title") or identifier
                url = link_target(ref.get("url", ""))
                summary = inline(ref.get("abstract"), refs)
                entry = f"- [{name}]({url})" if url else f"- {name}"
                lines.append(f"{entry} — {summary}" if summary else entry)
            if len(lines) > (1 if section.get("title") else 0):
                chunks.append("\n".join(lines))
        if chunks:
            out.append(f"## {heading}\n\n" + "\n\n".join(chunks))

    return "\n\n".join(out).rstrip() + "\n"


# ── tree layout ─────────────────────────────────────────────────────────────

SAFE = re.compile(r"[^a-z0-9._-]+")


def slug(segment):
    """A URL segment → a filesystem-safe, lowercase name.

    Symbol paths carry argument lists (`newbufferwithlength(_:options:)`), and
    `:` is a bad idea in a macOS filename. The real title and canonical URL are
    written into the file, so the name only has to be stable and readable.
    """
    out = SAFE.sub("-", segment.lower()).strip("-")
    return out or "index"


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


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("root", help="documentation path to mirror, e.g. `metal`")
    parser.add_argument("--out", required=True, help="output directory")
    parser.add_argument("--max-pages", type=int, default=100000)
    parser.add_argument("--max-depth", type=int, default=64)
    parser.add_argument("--delay", type=float, default=0.05, help="seconds a worker waits between requests")
    parser.add_argument("--jobs", type=int, default=8, help="concurrent fetches")
    parser.add_argument("--refresh", action="store_true", help="rewrite pages already on disk")
    args = parser.parse_args()

    root = args.root.strip("/")
    prefix = root.split("/")[0]
    queue = deque([(root, 0)])
    seen = {root}
    written = skipped = missing = 0

    def load(item):
        """Fetch one page. Round-trip latency dominates, so these overlap."""
        path, depth = item
        try:
            doc = fetch(path)
        except Exception as error:
            print(f"  ! {path}: {error}", file=sys.stderr, flush=True)
            doc = None
        time.sleep(args.delay)
        return path, depth, doc

    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        while queue and written + skipped < args.max_pages:
            batch = [queue.popleft() for _ in range(min(args.jobs * 4, len(queue)))]
            for path, depth, doc in pool.map(load, batch):
                if doc is None:
                    missing += 1
                    continue

                segments = [slug(s) for s in path.split("/")]
                kids = children_of(doc)
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
                        print(
                            f"  {written} written, {skipped} skipped, {len(queue)} queued",
                            file=sys.stderr,
                            flush=True,
                        )

                if depth >= args.max_depth:
                    continue
                for child in kids:
                    if child in seen or not child.startswith(prefix + "/"):
                        continue
                    seen.add(child)
                    queue.append((child, depth + 1))

    print(f"written={written} skipped={skipped} missing={missing} queued={len(queue)}", file=sys.stderr)


if __name__ == "__main__":
    main()
