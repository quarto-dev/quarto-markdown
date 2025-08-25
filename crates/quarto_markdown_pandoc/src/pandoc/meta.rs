/*
 * meta.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::RawBlock;
use crate::pandoc::block::Blocks;
use crate::pandoc::inline::Inlines;
use std::collections::HashMap;
use yaml_rust2::parser::{Event, MarkedEventReceiver, Parser};

// Pandoc's MetaValue notably does not support numbers or nulls, so we don't either
// https://pandoc.org/lua-filters.html#type-metavalue
#[derive(Debug, Clone, PartialEq)]
pub enum MetaValue {
    MetaString(String),
    MetaBool(bool),
    MetaInlines(Inlines),
    MetaBlocks(Blocks),
    MetaList(Vec<MetaValue>),
    MetaMap(HashMap<String, MetaValue>),
}

impl Default for MetaValue {
    fn default() -> Self {
        MetaValue::MetaMap(HashMap::new())
    }
}

pub type Meta = HashMap<String, MetaValue>;

fn extract_between_delimiters(input: &str) -> Option<&str> {
    let parts: Vec<&str> = input.split("---").collect();
    if parts.len() >= 3 {
        Some(parts[1].trim())
    } else {
        None
    }
}

enum ContextFrame {
    Map(HashMap<String, MetaValue>, Option<String>),
    List(Vec<MetaValue>),
    Root,
}

struct YamlEventHandler {
    stack: Vec<ContextFrame>,
    result: Option<Meta>,
}

impl YamlEventHandler {
    fn new() -> Self {
        YamlEventHandler {
            stack: vec![ContextFrame::Root],
            result: None,
        }
    }

    fn push_value(&mut self, value: MetaValue) {
        match self.stack.last_mut() {
            Some(ContextFrame::Map(map, Some(key))) => {
                map.insert(key.clone(), value);
                if let Some(ContextFrame::Map(_, key_slot)) = self.stack.last_mut() {
                    *key_slot = None;
                }
            }
            Some(ContextFrame::Map(_, None)) => {
                panic!("Map expecting key, got value");
            }
            Some(ContextFrame::List(list)) => {
                list.push(value);
            }
            Some(ContextFrame::Root) => {
                if let MetaValue::MetaMap(map) = value {
                    self.result = Some(map);
                }
            }
            None => panic!("Empty stack"),
        }
    }

    fn parse_scalar(&self, s: &str) -> MetaValue {
        if s == "true" {
            MetaValue::MetaBool(true)
        } else if s == "false" {
            MetaValue::MetaBool(false)
        } else if s.is_empty() {
            MetaValue::MetaString(String::new())
        } else {
            MetaValue::MetaString(s.to_string())
        }
    }
}

impl MarkedEventReceiver for YamlEventHandler {
    fn on_event(&mut self, ev: Event, _mark: yaml_rust2::scanner::Marker) {
        match ev {
            Event::StreamStart | Event::DocumentStart => {}
            Event::MappingStart(..) => {
                self.stack.push(ContextFrame::Map(HashMap::new(), None));
            }
            Event::MappingEnd => {
                if let Some(ContextFrame::Map(map, _)) = self.stack.pop() {
                    self.push_value(MetaValue::MetaMap(map));
                }
            }
            Event::SequenceStart(..) => {
                self.stack.push(ContextFrame::List(Vec::new()));
            }
            Event::SequenceEnd => {
                if let Some(ContextFrame::List(list)) = self.stack.pop() {
                    self.push_value(MetaValue::MetaList(list));
                }
            }
            Event::Scalar(s, ..) => {
                match self.stack.last_mut() {
                    Some(ContextFrame::Map(_, key_slot @ None)) => {
                        *key_slot = Some(s.to_string());
                    }
                    Some(ContextFrame::Map(_, Some(_))) | Some(ContextFrame::List(_)) => {
                        let value = self.parse_scalar(&s);
                        self.push_value(value);
                    }
                    _ => {}
                }
            }
            Event::DocumentEnd | Event::StreamEnd => {}
            _ => {}
        }
    }
}

pub fn rawblock_to_meta(block: RawBlock) -> Option<Meta> {
    if block.format != "quarto_minus_metadata" {
        return None;
    }
    let content = extract_between_delimiters(&block.text)?;
    let mut parser = Parser::new_from_str(content);
    let mut handler = YamlEventHandler::new();
    let _parse_result = parser.load(&mut handler, false);

    handler.result
}
