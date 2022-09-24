use std::{collections::HashMap, fs, io};

fn main() -> io::Result<()> {
    let entries = plot_journal_entries().expect("failed to plot journal entries");

    serde_json::to_writer(&fs::File::create("static/entries.json")?, &entries).unwrap();

    Ok(())
}

fn plot_journal_entries() -> io::Result<Vec<HashMap<String, String>>> {
    let mut paths = fs::read_dir("content/journal")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    paths.sort();
    paths.reverse();

    let mut entries: Vec<HashMap<String, String>> = vec![];

    for mut path in paths {
        let string = fs::read_to_string(&path)?;
        let parts = string
            .replace("---", "===")
            .match_indices("===")
            .nth(1)
            .map(|(i, _)| string.split_at(i))
            .expect("no header found");

        let header_tokens = markdown::tokenize(&parts.0);
        let body_tokens = markdown::tokenize(&parts.1);

        let mut entry: HashMap<String, String> = HashMap::new();
        path.set_extension("json");
        let stem = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let path = path.to_str().unwrap();
        let mut preview: Option<String> = None;
        entry.insert("_path".to_owned(), format!("/{}", path));
        entry.insert("_stem".to_owned(), stem);

        for token in body_tokens {
            match token {
                markdown::Block::Paragraph(span) => {
                    for element in span {
                        match element {
                            markdown::Span::Text(text) => {
                                let mut cp = preview.unwrap_or(String::new());
                                cp = cp + "\n" + &text;
                                preview = Some(cp)
                            }
                            markdown::Span::Link(text, _, _) => {
                                let mut cp = preview.unwrap_or(String::new());
                                cp = cp + &text;
                                preview = Some(cp)
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        for token in header_tokens {
            match token {
                markdown::Block::Paragraph(span) => {
                    for element in span {
                        match element {
                            markdown::Span::Text(text) => {
                                let split: Vec<&str> = text.split(":").collect();
                                if split.len() == 2 {
                                    let key = split[0].to_lowercase();
                                    let value = split[1].trim();
                                    entry.insert(key, value.to_owned());
                                }
                            }
                            _ => {}
                        }
                    }
                }

                markdown::Block::Header(span, _size) => {
                    for element in span {
                        match element {
                            markdown::Span::Text(text) => {
                                let split: Vec<&str> = text.split(":").collect();
                                if split.len() == 2 {
                                    let key = split[0].to_lowercase();
                                    let value = split[1].trim();
                                    entry.insert(key, value.to_owned());
                                }
                            }
                            _ => {}
                        }
                    }

                    break;
                }
                _ => {}
            }
        }

        if let Some(preview) = preview {
            let preview = match preview.char_indices().nth(255) {
                None => &preview,
                Some((i, _)) => &preview[..i],
            };

            entry.insert("preview".to_owned(), preview.to_owned());
        }

        entries.push(entry.clone());

        let body = parts.1[3..].trim_start();

        entry.remove("preview");
        entry.insert("body".to_owned(), markdown::to_html(body));

        serde_json::to_writer(
            &fs::File::create(format!("static/{}", path))?,
            &entry.clone(),
        )
        .unwrap();
    }

    Ok(entries)
}
