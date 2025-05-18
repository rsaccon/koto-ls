use indexmap::IndexMap;
use koto_cli::docs;
use pulldown_cmark::HeadingLevel;
use std::{iter::Peekable, sync::Arc};

const HELP_RESULT_STR: &str = "# ➝ ";
pub const HELP_INDENT: &str = "  ";

pub struct HelpEntry {
    // The entry's user-displayed name
    pub name: Arc<str>,
    // The entry's contents
    pub help: Arc<str>,
    // Additional keywords that should be checked when searching
    pub keywords: Vec<Arc<str>>,
    // Names of related topics to show in the 'See also' section
    // pub see_also: Vec<Arc<str>>,
}

pub struct Help {
    // All help entries, keys are lower_snake_case
    help_map: IndexMap<Arc<str>, HelpEntry>,
    // The list of guide topics
    guide_topics: Vec<Arc<str>>,
    // The list of core library module names
    core_lib_names: Vec<Arc<str>>,
    // The list of extra module names
    extra_lib_names: Vec<Arc<str>>,
}

impl Help {
    pub fn new() -> Self {
        let mut result = Self {
            help_map: IndexMap::new(),
            guide_topics: Vec::new(),
            core_lib_names: Vec::new(),
            extra_lib_names: Vec::new(),
        };

        result.add_help_from_guide();

        let core_lib_files = [
            docs::core_lib::io(),
            docs::core_lib::iterator(),
            docs::core_lib::koto(),
            docs::core_lib::list(),
            docs::core_lib::map(),
            docs::core_lib::number(),
            docs::core_lib::os(),
            docs::core_lib::range(),
            docs::core_lib::string(),
            docs::core_lib::test(),
            docs::core_lib::tuple(),
        ];
        for file_contents in core_lib_files.iter() {
            let module_name = result.add_help_from_reference(file_contents);
            result.core_lib_names.push(module_name);
        }

        let extra_lib_files = [
            docs::extra_lib::color(),
            docs::extra_lib::geometry(),
            docs::extra_lib::json(),
            docs::extra_lib::random(),
            docs::extra_lib::regex(),
            docs::extra_lib::tempfile(),
            docs::extra_lib::toml(),
            docs::extra_lib::yaml(),
        ];
        for file_contents in extra_lib_files.iter() {
            let module_name = result.add_help_from_reference(file_contents);
            result.extra_lib_names.push(module_name);
        }
        result
    }

    // pub fn topics(&self) -> impl Iterator<Item = Arc<str>> {
    //     self.core_lib_names
    //         .iter()
    //         .chain(self.extra_lib_names.iter())
    //         .chain(self.guide_topics.iter())
    //         .cloned()
    // }

    // pub fn all_entries(&self) -> impl Iterator<Item = (&Arc<str>, &HelpEntry)> {
    //     self.help_map.iter()
    // }

    pub fn get_help(&self, search: &str) -> String {
        let search_key = text_to_key(search);
        match self.help_map.get(&search_key) {
            Some(entry) => {
                let help = format!(
                    "{name}\n{underline}{help}",
                    name = entry.name,
                    underline = "=".repeat(entry.name.len()),
                    help = entry.help
                );

                //                 let see_also: Vec<_> = entry
                //                     .see_also
                //                     .iter()
                //                     .chain(self.help_map.iter().filter_map(|(key, search_entry)| {
                //                         if key.contains(search_key.as_ref())
                //                             && !entry.see_also.contains(&search_entry.name)
                //                             && search_entry.name != entry.name
                //                         {
                //                             Some(&search_entry.name)
                //                         } else {
                //                             None
                //                         }
                //                     }))
                //                     .collect();

                //                 if !see_also.is_empty() {
                //                     help += "

                // --------

                // See also:";

                //                     let item_prefix = format!("\n{HELP_INDENT}- ");
                //                     for see_also_entry in see_also.iter() {
                //                         help.push_str(&item_prefix);
                //                         help.push_str(see_also_entry);
                //                     }
                //                 }

                help
            }
            None => {
                let matches = self
                    .help_map
                    .iter()
                    .filter(|(key, value)| {
                        key.contains(search_key.as_ref())
                            || value
                                .keywords
                                .iter()
                                .any(|keyword| keyword.contains(search_key.as_ref()))
                    })
                    .collect::<Vec<_>>();

                match matches.as_slice() {
                    [] => format!("No matches for '{search}' found."),
                    [(only_match, _)] => self.get_help(only_match),
                    _ => {
                        let mut help = String::new();
                        help.push_str("More than one match found: ");
                        let item_prefix = format!("\n{HELP_INDENT}- ");
                        for (_, HelpEntry { name, .. }) in matches {
                            help.push_str(&item_prefix);
                            help.push_str(name);
                        }
                        help
                    }
                }
            }
        }
    }

    fn add_help_from_guide(&mut self) {
        let mut parser = pulldown_cmark::Parser::new(docs::language_guide()).peekable();

        // Skip the guide intro
        consume_help_section(&mut parser, None, HeadingLevel::H1, false);

        while parser.peek().is_some() {
            // Consume the module overview section
            let topic = consume_help_section(&mut parser, None, HeadingLevel::H2, false);
            // We should avoid top-level topics without a body
            debug_assert!(
                !topic.contents.trim().is_empty(),
                "Missing contents for {}",
                topic.name
            );

            // Add sub-topics
            let mut sub_topics = Vec::new();
            loop {
                let sub_topic = consume_help_section(&mut parser, None, HeadingLevel::H3, true);
                if sub_topic.contents.trim().is_empty() {
                    break;
                }
                sub_topics.push(sub_topic);
            }

            // let see_also = sub_topics
            //     .iter()
            //     .flat_map(|sub_topic| {
            //         iter::once(&sub_topic.name).chain(sub_topic.sub_sections.iter())
            //     })
            //     .cloned()
            //     .collect();
            self.help_map.insert(
                text_to_key(&topic.name),
                HelpEntry {
                    name: topic.name.clone(),
                    help: topic.contents,
                    // see_also,
                    keywords: vec![],
                },
            );
            self.guide_topics.push(topic.name.clone());

            for sub_topic in sub_topics {
                self.help_map.insert(
                    text_to_key(&sub_topic.name),
                    HelpEntry {
                        name: sub_topic.name,
                        help: sub_topic.contents,
                        keywords: sub_topic
                            .sub_sections
                            .iter()
                            .map(|sub_section| text_to_key(sub_section))
                            .collect(),
                        // see_also: vec![topic.name.clone()],
                    },
                );
            }
        }
    }

    fn add_help_from_reference(&mut self, markdown: &str) -> Arc<str> {
        let mut parser = pulldown_cmark::Parser::new(markdown).peekable();

        let help_section = consume_help_section(&mut parser, None, HeadingLevel::H1, false);

        // Consume each module entry
        let mut entry_names = Vec::new();
        while parser.peek().is_some() {
            let module_entry = consume_help_section(
                &mut parser,
                Some(&help_section.name),
                HeadingLevel::H2,
                true,
            );
            self.help_map.insert(
                text_to_key(&module_entry.name),
                HelpEntry {
                    name: module_entry.name.clone(),
                    help: module_entry.contents,
                    keywords: vec![],
                },
            );
            entry_names.push(module_entry.name);
        }

        if !help_section.contents.trim().is_empty() {
            self.help_map.insert(
                text_to_key(&help_section.name),
                HelpEntry {
                    name: help_section.name.clone(),
                    help: help_section.contents,
                    keywords: vec![],
                },
            );
        }

        help_section.name
    }
}

fn text_to_key(text: &str) -> Arc<str> {
    text.trim().to_lowercase().replace(' ', "_").into()
}

struct HelpSection {
    name: Arc<str>,
    contents: Arc<str>,
    sub_sections: Vec<Arc<str>>,
}

#[derive(Debug)]
enum ParsingMode {
    WaitingForSectionStart,
    Any,
    Section,
    SubSection,
    Code,
    TypeDeclaration,
}

// Consumes a section of content between headers
//
// - If the title section is being consumed, then the function will break out at the first
//   sub-header.
// - If a sub-section is being consumed, then
fn consume_help_section(
    parser: &mut Peekable<pulldown_cmark::Parser>,
    module_name: Option<&str>,
    level_to_consume: HeadingLevel,
    include_sub_sections: bool,
) -> HelpSection {
    use pulldown_cmark::{CodeBlockKind, Event::*, Tag, TagEnd};

    let mut section_name = String::new();
    let mut sub_section_name = String::new();
    let mut sub_sections = Vec::new();
    let mut result = HELP_INDENT.to_string();

    let mut list_indent = 0;
    let mut parsing_mode = ParsingMode::WaitingForSectionStart;

    while let Some(peeked) = parser.peek() {
        match peeked {
            Start(Tag::Heading { level, .. }) => {
                use std::cmp::Ordering::*;
                let waiting_for_start = matches!(parsing_mode, ParsingMode::WaitingForSectionStart);
                match level.cmp(&level_to_consume) {
                    Less => {
                        break;
                    }
                    Equal => {
                        if waiting_for_start {
                            parsing_mode = ParsingMode::Section;
                        } else {
                            break;
                        }
                    }
                    Greater => {
                        if waiting_for_start {
                            // Continue consuming until the start of the section is found
                        } else if include_sub_sections {
                            // Start a new subsection
                            parsing_mode = ParsingMode::SubSection;
                            sub_section_name.clear();
                            result.push_str("\n\n");
                        } else {
                            break;
                        }
                    }
                }
            }
            End(TagEnd::Heading(_)) => {
                if matches!(parsing_mode, ParsingMode::SubSection) {
                    sub_sections.push(sub_section_name.as_str().into());
                    // result.push('\n');
                    // for _ in 0..sub_section_name.len() {
                    //     result.push('-');
                    // }
                }
                parsing_mode = ParsingMode::Any;
            }
            // Start(Tag::Link { title, .. }) => result.push_str(title),
            Start(Tag::Link {
                title, dest_url, ..
            }) => {
                result.push('[');
                result.push_str(title);
                result.push(']');
                result.push('(');
                result.push_str(dest_url);
                result.push(')');
            }
            End(TagEnd::Link) => {}
            Start(Tag::List(_)) => {
                if list_indent == 0 {
                    result.push('\n');
                }
                list_indent += 1;
            }
            End(TagEnd::List(_)) => list_indent -= 1,
            Start(Tag::Item) => {
                result.push('\n');
                for _ in 1..list_indent {
                    result.push_str("  ");
                }
                result.push_str("- ");
            }
            End(TagEnd::Item) => {}
            Start(Tag::Paragraph) => result.push_str("\n\n"),
            End(TagEnd::Paragraph) => {}
            Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                // result.push_str("\n\n");
                result.push_str("\n```koto\n ");
                match lang.split(',').next() {
                    Some("koto") => parsing_mode = ParsingMode::Code,
                    Some("kototype") => parsing_mode = ParsingMode::TypeDeclaration,
                    _ => {}
                }
            }
            End(TagEnd::CodeBlock) => {
                result.push_str("```\n");
                parsing_mode = ParsingMode::Any
            }
            Start(Tag::Emphasis) => result.push('_'),
            End(TagEnd::Emphasis) => result.push('_'),
            Start(Tag::Strong) => result.push('*'),
            End(TagEnd::Strong) => result.push('*'),
            Text(text) => match parsing_mode {
                ParsingMode::WaitingForSectionStart => {}
                ParsingMode::Any => result.push_str(text),
                ParsingMode::Section => section_name.push_str(text),
                ParsingMode::SubSection => {
                    sub_section_name.push_str(text);
                    result.push_str(text);
                }
                ParsingMode::Code => {
                    for (i, line) in text.split('\n').enumerate() {
                        // if i == 0 {
                        //     result.push('|');
                        // }
                        // result.push_str("\n|  ");
                        if i > 0 {
                            result.push_str("\n ");
                        }
                        let processed_line = line.trim_start_matches("print! ").replacen(
                            "check! ",
                            HELP_RESULT_STR,
                            1,
                        );
                        result.push_str(&processed_line);
                    }
                }
                ParsingMode::TypeDeclaration => {
                    // result.push('`');
                    result.push_str(text.trim_end());
                    result.push('\n');
                    // result.push('`');
                }
            },
            Code(code) => match parsing_mode {
                ParsingMode::Section => {
                    section_name.push_str(code);
                }
                ParsingMode::SubSection => {
                    sub_section_name.push_str(code);
                    result.push_str(code);
                }
                _ => {
                    result.push('`');
                    result.push_str(code);
                    result.push('`');
                }
            },
            SoftBreak => result.push(' '),
            HardBreak => result.push('\n'),
            _other => {}
        }

        parser.next();
    }

    if let Some(module_name) = module_name {
        section_name = format!("{module_name}.{section_name}");
    }

    HelpSection {
        name: section_name.into(),
        contents: result.into(),
        sub_sections,
    }
}
