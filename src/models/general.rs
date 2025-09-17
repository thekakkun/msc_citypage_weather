use std::{default, error::Error, fmt::Debug};

use chrono::{DateTime, ParseError, TimeZone};
use chrono_tz::Tz;
use quick_xml::events::{BytesStart, Event};
use xsd_parser::quick_xml::{
    DeserializeSync, Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput,
    DeserializerResult, WithDeserializer, XmlReader,
};

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

impl<Tz: TimeZone> TryFrom<DateStampTypeDeserializer> for DateStampType<Tz> {
    type Error = ParseError;

    fn try_from(value: DateStampTypeDeserializer) -> Result<Self, Self::Error> {
        let year = value.year.ok_or(ParseError)?;
        let month = value.month.ok_or(ParseError)?;
        let day = value.day.ok_or(ParseError)?;
        let hour = value.hour.ok_or(ParseError)?;
        let min = value.minute.ok_or(ParseError)?;
        let timezone = value.timezone.ok_or(ParseError)?;

        let datetime = timezone
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .single()
            .ok_or(ParseError)?;

        Ok(DateStampType { datetime })
    }
}

#[derive(Debug, Default)]
struct DateStampTypeDeserializer {
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

impl DateStampTypeDeserializer {
    fn handle_datetime(&mut self, e: &BytesStart) -> Result<(), Box<dyn Error>> {
        for attr_result in e.attributes() {
            let a = attr_result?;
            if a.key.as_ref() == b"zone" {
                let tz_name = a.decode_and_unescape_value(e.decoder())?;
                let timezone = match tz_name.as_ref() {
                    "ADT" | "AST" | "HAA" | "HNA" => Some(Tz::Canada__Atlantic),
                    "CDT" | "CST" | "HAC" | "HNC" => Some(Tz::Canada__Central),
                    "EDT" | "EST" | "HAE" | "HNE" => Some(Tz::Canada__Eastern),
                    "MDT" | "MST" | "HAR" | "HNR" => Some(Tz::Canada__Mountain),
                    "NDT" | "NST" | "HAT" | "HNT" => Some(Tz::Canada__Newfoundland),
                    "PDT" | "PST" | "HAP" | "HNP" => Some(Tz::Canada__Pacific),
                    _ => None,
                };

                self.timezone = timezone;
            }
        }

        Ok(())
    }
}

impl<'de> Deserializer<'de, DateStampType<Tz>> for DateStampTypeDeserializer
where
    Tz: TimeZone,
{
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, DateStampType<Tz>>
    where
        R: XmlReader,
    {
        Self::default().next(reader, event)
    }

    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, DateStampType<Tz>>
    where
        R: XmlReader,
    {
        let _reader = reader;

        match event {
            Event::Start(ref e) => {
                match e.name().as_ref() {
                    b"dateTime" => self.handle_datetime(&e),
                    _ => Ok(()),
                };
                return Ok(DeserializerOutput {
                    event: DeserializerEvent::Continue(event),
                    artifact: DeserializerArtifact::Deserializer(self),
                    allow_any: false,
                });
            }
            Event::End(bytes_end) => todo!(),
            Event::Empty(bytes_start) => todo!(),
            Event::Text(bytes_text) => todo!(),
            Event::CData(bytes_cdata) => todo!(),
            Event::Comment(bytes_text) => todo!(),
            Event::Decl(bytes_decl) => todo!(),
            Event::PI(bytes_pi) => todo!(),
            Event::DocType(bytes_text) => todo!(),
            Event::GeneralRef(bytes_ref) => todo!(),
            Event::Eof => {
                return Ok(DeserializerOutput {
                    event: DeserializerEvent::Break(event),
                    artifact: DeserializerArtifact::Deserializer(self),
                    allow_any: false,
                });
            }
        }
    }

    fn finish<R>(self, reader: &R) -> Result<DateStampType<Tz>, xsd_parser::quick_xml::Error>
    where
        R: XmlReader,
    {
        todo!()
    }
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
