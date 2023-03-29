use std::str::FromStr;

use rusvid_lib::core::holder::likes::PathLike;
use rusvid_lib::core::pixel::Pixel;
use rusvid_lib::types::Point;
use serde::{Deserialize, Serialize};

use super::TranslateIntoRusvidGeneric;

fn parse_raw_path<T: FromStr + Default>(raw: &str) -> impl Iterator<Item = T> + '_ {
    parse_str_iterator(split_at_char(raw.trim_start_matches(&['M', 'L', 'Z']), ','))
}

fn split_at_char(og: &str, split: char) -> impl Iterator<Item = &str> {
    og.split(split)
}

fn parse_str_iterator<'a, T: FromStr + Default>(
    og: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = T> + 'a {
    og.map(|raw| raw.parse::<T>().unwrap_or_default())
}

fn parse_raw_color<T: FromStr + Default>(raw: &str) -> impl Iterator<Item = T> + '_ {
    parse_str_iterator(split_at_char(raw, ','))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Object {
    pub name: String,
    pub path: String,
    pub color: String,
}

impl TranslateIntoRusvidGeneric for Object {
    type OUTPUT = rusvid_lib::core::holder::svg_item::SvgItem;

    fn translate(&self) -> Self::OUTPUT {
        let raw_paths = self.path.split_whitespace().collect::<Vec<_>>();
        let mut paths = Vec::new();

        for raw_path in raw_paths {
            let chars = raw_path.chars().collect::<Vec<_>>();

            let path = match chars[0] {
                'M' => {
                    let mut parsed_values = parse_raw_path::<f64>(raw_path);

                    PathLike::Move(Point::new(
                        parsed_values.next().unwrap(),
                        parsed_values.next().unwrap(),
                    ))
                }
                'L' => {
                    let mut parsed_values = parse_raw_path::<f64>(raw_path);

                    PathLike::Line(Point::new(
                        parsed_values.next().unwrap(),
                        parsed_values.next().unwrap(),
                    ))
                }
                'Z' => PathLike::Close,
                item => todo!("unknown char: '{}'", item),
            };
            paths.push(path);
        }

        let mut raw_color = parse_raw_color::<u8>(&self.color);
        let color = rusvid_lib::core::holder::likes::ColorLike::Color(Pixel::new(
            raw_color.next().unwrap_or_default(),
            raw_color.next().unwrap_or_default(),
            raw_color.next().unwrap_or_default(),
            raw_color.next().unwrap_or_default(),
        ));

        rusvid_lib::core::holder::svg_item::SvgItem::new_with_id(
            self.name.clone(),
            paths,
            Some(color),
        )
    }
}
