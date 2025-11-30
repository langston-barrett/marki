use anyhow::{Context, Result};
use clap::Parser;
use genanki_rs::{Deck, Field, Model, Note, Template};
use pulldown_cmark::{Event, TagEnd};

mod cli;

#[derive(Debug)]
struct Card {
    // TODO: https://github.com/yannickfunk/genanki-rs/issues/12
    // id: String,
    front: String,
    back: String,
}

fn extract_string(md: &str, path: &str, _verbose: bool) -> Vec<Card> {
    let mut cards = Vec::new();
    // let mut id: Option<String> = None;
    let mut seen_comment = false;
    let mut front = String::new();
    let mut back = String::new();
    let mut in_front = false;
    let mut in_back = false;
    for (ev, range) in pulldown_cmark::Parser::new(md).into_offset_iter() {
        match ev {
            Event::Html(s) => {
                match s
                    .strip_prefix("<!-- marki[card] ")
                    .and_then(|t| t.strip_suffix("-->\n"))
                {
                    None => continue,
                    Some(_) => {
                        // if id.is_some() {
                        //     eprintln!("{}:{}:{}: Two IDs in a row!", path, range.start, range.end);
                        // }
                        // if i == "" {
                        //     eprintln!("{}:{}:{}: Empty id!", path, range.start, range.end);
                        //     continue;
                        // }
                        // id = Some(i.to_string());
                        if seen_comment {
                            eprintln!(
                                "{}:{}:{}: Two Marki comments in a row",
                                path, range.start, range.end
                            );
                        }
                        if in_front || in_back {
                            eprintln!(
                                "{}:{}:{}: Marki comment inside of prompt or response",
                                path, range.start, range.end
                            );
                        }
                        seen_comment = true;
                    }
                }
            }
            Event::Text(txt) => {
                if seen_comment {
                    if let Some(q) = txt.strip_prefix("Q. ") {
                        in_front = true;
                        front += q;
                    }
                    if let Some(a) = txt.strip_prefix("A. ") {
                        in_back = true;
                        back += a;
                    }
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if in_front {
                    in_front = false;
                }
                if in_back {
                    in_front = false;
                    in_back = false;
                    seen_comment = false;
                    if front.is_empty() {
                        eprintln!(
                            "{}:{}:{}: Card without a front",
                            path, range.start, range.end
                        );
                    }
                    if back.is_empty() {
                        eprintln!(
                            "{}:{}:{}: Card without a back",
                            path, range.start, range.end
                        );
                    }
                    cards.push(Card { front, back });
                    front = String::new();
                    back = String::new();
                }
            }
            _ => continue,
        }
    }
    cards
}

fn extract_file(path: &str, verbose: bool) -> Result<Vec<Card>> {
    let md = std::fs::read_to_string(path).with_context(|| format!("Couldn't read {path}"))?;
    Ok(extract_string(&md, path, verbose))
}

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let model = Model::new(
        1_608_492_319, // arbitrary
        "Marki Basic",
        vec![Field::new("Front"), Field::new("Back")],
        vec![Template::new("Card")
            .qfmt("{{Front}}")
            .afmt(r#"{{Front}}<hr id="answer">{{Back}}"#)],
    );
    let mut deck = Deck::new(
        2_050_400_110, // arbitrary
        &args.deck,
        "Marki-generated deck",
    );

    for path in args.file {
        for card in extract_file(&path, args.verbose)? {
            if args.verbose {
                println!("Card: {}, {}", card.front, card.back);
            }
            deck.add_note(Note::new(model.clone(), vec![&card.front, &card.back])?);
        }
    }

    deck.write_to_file(args.output.to_str().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_single_card() {
        let md = r#"<!-- marki[card] -->

Q. What is marki?

A. A tool to generate Anki cards from Markdown notes."#;
        let cards = extract_string(md, "test.md", false);
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].front, "What is marki?");
        assert_eq!(
            cards[0].back,
            "A tool to generate Anki cards from Markdown notes."
        );
    }

    #[test]
    fn test_extract_multiple_cards() {
        let md = r#"<!-- marki[card] -->

Q. What is marki?

A. A tool to generate Anki cards from Markdown notes.

<!-- marki[card] -->

Q. What is the syntax for beginning a marki card?

A. The following specially-formatted HTML comment: `<!-- marki[card] -->`"#;
        let cards = extract_string(md, "test.md", false);
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].front, "What is marki?");
        assert_eq!(
            cards[0].back,
            "A tool to generate Anki cards from Markdown notes."
        );
        assert_eq!(
            cards[1].front,
            "What is the syntax for beginning a marki card?"
        );
        assert!(cards[1]
            .back
            .contains("The following specially-formatted HTML comment"));
    }

    #[test]
    fn test_extract_no_cards() {
        let md = r#"This is just regular markdown content.

No cards here."#;
        let cards = extract_string(md, "test.md", false);
        assert_eq!(cards.len(), 0);
    }

    #[test]
    fn test_extract_card_with_empty_front() {
        let md = r#"<!-- marki[card] -->

A. This card has no front."#;
        let cards = extract_string(md, "test.md", false);
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].front, "");
        assert_eq!(cards[0].back, "This card has no front.");
    }

    #[test]
    fn test_extract_card_with_empty_back() {
        let md = r#"<!-- marki[card] -->

Q. This card has no back."#;
        let cards = extract_string(md, "test.md", false);
        assert_eq!(cards.len(), 0);
    }
}
