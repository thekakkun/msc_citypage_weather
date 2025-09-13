use std::{default, fmt::Debug};

use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use xsd_parser::quick_xml::{Deserializer, WithDeserializer};

// include!(concat!(env!("OUT_DIR"), "/general.rs"));

#[derive(Debug)]
pub struct DateStampType<Tz: TimeZone> {
    pub datetime: DateTime<Tz>,
    // pub type_: DateStampNameType,
}

impl<Tz: TimeZone> WithDeserializer for DateStampType<Tz>
where
    for<'de> DateStampTypeDeserializer: Deserializer<'de, DateStampType<Tz>>,
{
    type Deserializer = DateStampTypeDeserializer;
}

#[derive(Debug, Default)]
pub struct DateStampTypeDeserializer {
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    hour: Option<u32>,
    minute: Option<u32>,
    timezone: Option<Tz>,
    state: Box<DateStampTypeDeserializerState>,
}

#[derive(Debug, Default)]
enum DateStampTypeDeserializerState {
    #[default]
    Init__,
    Done__,
    Unknown__,
}

impl<'de> Deserializer<'de, DateStampType<Tz>> for DateStampTypeDeserializer
where
    Tz: TimeZone,
{
    fn init<R>(
        reader: &R,
        event: quick_xml::events::Event<'de>,
    ) -> xsd_parser::quick_xml::DeserializerResult<'de, DateStampType<Tz>>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        Self::default().next(reader, event)
    }

    fn next<R>(
        self,
        reader: &R,
        event: quick_xml::events::Event<'de>,
    ) -> xsd_parser::quick_xml::DeserializerResult<'de, DateStampType<Tz>>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        todo!()
    }

    fn finish<R>(self, reader: &R) -> Result<DateStampType<Tz>, xsd_parser::quick_xml::Error>
    where
        R: xsd_parser::quick_xml::XmlReader,
    {
        todo!()
    }
    // add code here
}

// impl DeserializeBytesFromStr for DateStampType<Tz: TimeZone> {}
//
// pub(crate) struct DateStampTypeDeserializer<Tz: TimeZone> {
//     datetime: Option<DateTime<Tz>>,
// }
// impl<'de, Tz: TimeZone> Deserializer<'de, DateStampType<Tz>> for DateStampTypeDeserializer<Tz> {
//     fn init<R>(
//         reader: &R,
//         event: quick_xml::events::Event<'de>,
//     ) -> xsd_parser::quick_xml::DeserializerResult<'de, DateStampType<Tz>>
//     where
//         R: xsd_parser::quick_xml::XmlReader,
//     {
//         todo!()
//     }
//
//     fn next<R>(
//         self,
//         reader: &R,
//         event: quick_xml::events::Event<'de>,
//     ) -> xsd_parser::quick_xml::DeserializerResult<'de, DateStampType<Tz>>
//     where
//         R: xsd_parser::quick_xml::XmlReader,
//     {
//         todo!()
//     }
//
//     fn finish<R>(self, reader: &R) -> Result<DateStampType<Tz>, xsd_parser::quick_xml::Error>
//     where
//         R: xsd_parser::quick_xml::XmlReader,
//     {
//         todo!()
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct TimeStampType(pub DateTime<Utc>);
//
// impl FromStr for TimeStampType {
//     type Err = ParseError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S")
//             .map(|datetime| Self(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc)))
//     }
// }
//
// impl<'de> Deserialize<'de> for TimeStampType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         Ok(Self(s.parse().map_err(|_| {
//             DeError::custom("DateTime. Invalid value!")
//         })?))
//     }
// }
// impl DeserializeBytesFromStr for TimeStampType {}
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct DateTimeUtcType(pub DateTime<Utc>);
//
// impl FromStr for DateTimeUtcType {
//     type Err = ParseError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M")
//             .map(|datetime| Self(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc)))
//     }
// }
//
// impl<'de> Deserialize<'de> for DateTimeUtcType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         Ok(Self(s.parse().map_err(|_| {
//             DeError::custom("DateTime. Invalid value!")
//         })?))
//     }
// }
// impl DeserializeBytesFromStr for DateTimeUtcType {}
