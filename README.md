# Marki

Generate Anki cards from Markdown notes.

## How it Works

A card consists of a specially-formatted comment followed by a prompt and
response (front and back of a card):

```markdown
<!-- marki[card] -->

Q. What is Marki?

A. A tool to generate Anki cards from Markdown notes.
```

<!--

TODO: https://github.com/yannickfunk/genanki-rs/issues/12

The content of the comment after `:` becomes the unique ID associated with this
card, so that you can modify the wording later without creating a new card.

-->

See [test.md](test.md) for an example.

The syntax is inspired by [Andy's notes](https://notes.andymatuschak.org/z4mAF1uBV96r72e4NjLcDaujEyTPGiUQJEj8C).

## Non-Features

Marki doesn't support:

- Media files
- Other note types (including cloze)
- Rewording cards without creating duplicates
  (see [genanki-rs#12](https://github.com/yannickfunk/genanki-rs/issues/12))

## Install

Download a binary from the [releases page][releases], or build with
[Cargo][cargo]:

```sh
cargo install marki
```

## Usage

```
Usage: marki [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Markdown files

Options:
  -d, --deck <DECK>      Deck name [default: Marki]
  -o, --output <OUTPUT>  Output file [default: marki.apkg]
  -v, --verbose
  -h, --help             Print help information
```

## Motivation and Alternatives

This tool exists because most of the alternatives use syntax that isn't
suitable for embedding in larger documents (e.g., using headers for questions)
or are written in interpreted languages like Python or JavaScript. On the other
hand, many of these alternatives are more featureful than Marki.

- <https://github.com/lukesmurray/markdown-anki-decks>
- <https://github.com/ashlinchak/mdanki>
- <https://github.com/search?q=markdown+anki>

[cargo]: https://doc.rust-lang.org/cargo/
[releases]: https://github.com/langston-barrett/marki/releases
