# Visual thesis — cassette-era zine

Knowledge Handoff Bundle treats a transfer like a carefully labelled mixtape: finite, ordered, copied for a specific person, and honest about dropouts. The visual language combines photocopied project notes with cassette inlay cards. It feels archival and human without becoming nostalgic decoration; labels, track numbers, check marks, and warning stamps map directly to owners, artifacts, verification, and gaps.

## Palette

The site is intentionally single-mode, like ink on warm stock, with every background painted explicitly.

| Token | Hex | Use |
| --- | --- | --- |
| Paper | `#F2E8CF` | Base stock |
| Paper lift | `#FFF8E8` | Reading surfaces |
| Ink | `#1D1B19` | Text and hard rules |
| Graphite | `#5E584F` | Secondary copy |
| Tape red | `#B22E2A` | Primary actions and required marks |
| Dub blue | `#174A5B` | Links and focus rings |
| Oxide | `#7A4D21` | Warnings and ageing |
| Good | `#24603B` | Verified/reachable |
| Bad | `#8D2424` | Broken/missing |

Ink/paper combinations exceed 4.5:1. Status always includes a word and symbol, never color alone.

## Type and spacing

Headlines use `Arial Black`, `Franklin Gothic Heavy`, and system sans fallbacks: blunt lettering that resembles a hand-cut zine masthead. Reading copy and controls use `Courier New`, `Liberation Mono`, and monospace fallbacks, echoing typed tape labels while remaining locally available. No font files or CDN requests are shipped.

The scale is 14 / 16 / 20 / 26 / 40 / 64 px. Body text never drops below 16 px. Spacing follows a 4 px base with 8, 12, 16, 24, 32, 48, 64, and 96 px intervals. Reading measures stay below 72 characters.

## Composition and interaction grammar

- A red vertical recording stripe and slightly imperfect double rules create the inlay-card frame.
- Sections are “sides”; artifacts are “tracks.” Track numbers and status stamps make scanning immediate.
- Rectangular controls have 2 px ink borders, a 4 px offset shadow, and a one-step pressed state. There are no pill buttons or generic floating cards.
- The generated handoff page leads with transfer health, then required artifacts, known gaps, ownership, and recipient acknowledgement.
- On phones, two-column spreads become one continuous tape log; ornamental metadata is reduced, never core status.

## Motion policy

Only state changes move. On load, the hero cassette settles by 8 px over 240 ms; buttons depress by 2 px over 120 ms; filtering fades rows over 160 ms. Nothing loops. Under `prefers-reduced-motion: reduce`, transforms and transitions are removed and state changes are immediate.

## Original asset plan and provenance

- `site/public/cassette-handoff.webp`: generated specifically for this product using the factory image generator (`/opt/fleet/lib/gen-image.sh`, factory-image deployment). Prompt: “Editorial still-life illustration for a CLI product landing page: a well-used transparent compact cassette on warm cream photocopy paper, its two reels connected by a dark tape path that becomes neatly labelled file tabs and check marks; late-1980s independent zine cut-paper collage, chunky black ink outlines, visible halftone and misregistration, limited warm cream / soot black / brick red / deep teal palette, landscape composition with subject on right and quiet negative space on left, tactile but legible, no words, no letters, no logos, no watermark.” Generated output is converted to WebP and kept below 300 KB. License: project-original generated asset, 2026.
- All interface marks (status squares, tape holes, arrows) are CSS shapes or Unicode glyphs authored in-repository; no third-party icon set.

## Accessibility and performance intent

Focus is a 3 px blue outline plus offset. Targets are at least 44 px. Paper textures use lightweight CSS gradients at low contrast and never sit behind dense reading copy. The hero has fixed dimensions and responsive sizing. Initial JS stays under 200 KB, CSS under 50 KB, and hero under 300 KB.
