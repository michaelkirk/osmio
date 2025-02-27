//! Read and write OpenStreetMap fileformats
//!
extern crate byteorder;
extern crate chrono;
extern crate flate2;
extern crate protobuf;
extern crate quick_xml;
extern crate xml as xml_rs;
#[macro_use]
extern crate derive_builder;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::iter::{ExactSizeIterator, Iterator};
use utils::{epoch_to_iso, iso_to_epoch};

#[macro_use]
pub mod utils;

pub mod nodestore;

pub mod pbf;
pub mod xml;
//pub mod opl;
pub mod osc;

pub mod obj_types;

#[cfg(test)]
mod tests;

/// OSM id of object
pub type ObjId = i64;

/// Latitude
pub type Lat = f32;

/// Longitude
pub type Lon = f32;

#[derive(Debug, Clone, Eq, Ord)]
pub enum TimestampFormat {
    ISOString(String),
    EpochNunber(i64),
}

impl TimestampFormat {
    pub fn to_iso_string(&self) -> String {
        match self {
            &TimestampFormat::ISOString(ref s) => s.clone(),
            &TimestampFormat::EpochNunber(ref t) => epoch_to_iso(*t as i32),
        }
    }

    pub fn to_epoch_number(&self) -> i64 {
        match self {
            &TimestampFormat::ISOString(ref s) => iso_to_epoch(s) as i64,
            &TimestampFormat::EpochNunber(t) => t,
        }
    }
}

impl<T> From<T> for TimestampFormat
where
    T: Into<i64>,
{
    fn from(v: T) -> Self {
        TimestampFormat::EpochNunber(v.into())
    }
}

impl std::str::FromStr for TimestampFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let date: i64 = chrono::DateTime::parse_from_rfc3339(s)
            .map_err(|_| "invalid date")?
            .timestamp();
        Ok(TimestampFormat::EpochNunber(date))
    }
}

impl fmt::Display for TimestampFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_iso_string())
    }
}

impl std::cmp::PartialOrd for TimestampFormat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (TimestampFormat::ISOString(a), TimestampFormat::ISOString(b)) => a.partial_cmp(b),
            (TimestampFormat::EpochNunber(a), TimestampFormat::EpochNunber(b)) => a.partial_cmp(b),
            (a, b) => a.to_epoch_number().partial_cmp(&b.to_epoch_number()),
        }
    }
}
impl std::cmp::PartialEq for TimestampFormat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimestampFormat::ISOString(a), TimestampFormat::ISOString(b)) => a.eq(b),
            (TimestampFormat::EpochNunber(a), TimestampFormat::EpochNunber(b)) => a.eq(b),
            (a, b) => a.to_epoch_number().eq(&b.to_epoch_number()),
        }
    }
}

/// The basic metadata fields all OSM objects share
pub trait OSMObjBase: PartialEq + Debug + Clone {
    fn id(&self) -> ObjId;
    fn set_id(&mut self, val: impl Into<ObjId>);
    fn version(&self) -> Option<u32>;
    fn set_version(&mut self, val: impl Into<Option<u32>>);
    fn deleted(&self) -> bool;
    fn set_deleted(&mut self, val: bool);
    fn changeset_id(&self) -> Option<u32>;
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>);
    fn timestamp(&self) -> &Option<TimestampFormat>;
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>);
    fn uid(&self) -> Option<u32>;
    fn set_uid(&mut self, val: impl Into<Option<u32>>);
    fn user(&self) -> Option<&str>;
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>);

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a>;
    fn tag(&self, key: impl AsRef<str>) -> Option<&str>;
    fn has_tag(&self, key: impl AsRef<str>) -> bool {
        self.tag(key).is_some()
    }
    fn num_tags(&self) -> usize {
        self.tags().count()
    }

    /// True iff this object has tags
    fn tagged(&self) -> bool {
        !self.untagged()
    }
    /// True iff this object has no tags
    fn untagged(&self) -> bool {
        self.num_tags() == 0
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>);
    fn unset_tag(&mut self, key: impl AsRef<str>);

    fn strip_metadata(&mut self) {
        self.set_uid(None);
        self.set_user(None);
        self.set_changeset_id(None);
    }
}

/// A Node
pub trait Node: OSMObjBase {
    fn lat_lon(&self) -> Option<(Lat, Lon)>;
    fn has_lat_lon(&self) -> bool {
        self.lat_lon().is_some()
    }

    fn set_lat_lon(&mut self, loc: impl Into<Option<(Lat, Lon)>>);
}

/// A Way
pub trait Way: OSMObjBase {
    fn nodes(&self) -> &[ObjId];
    fn num_nodes(&self) -> usize;
    fn node(&self, idx: usize) -> Option<ObjId>;
    fn set_nodes(&mut self, nodes: impl IntoIterator<Item = impl Into<ObjId>>);
}

/// A Relation
pub trait Relation: OSMObjBase {
    fn members<'a>(
        &'a self,
    ) -> Box<dyn ExactSizeIterator<Item = (OSMObjectType, ObjId, &'a str)> + 'a>;
    fn set_members(
        &mut self,
        members: impl IntoIterator<Item = (OSMObjectType, ObjId, impl Into<String>)>,
    );
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OSMObjectType {
    Node,
    Way,
    Relation,
}

impl std::fmt::Debug for OSMObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            OSMObjectType::Node => write!(f, "n"),
            OSMObjectType::Way => write!(f, "w"),
            OSMObjectType::Relation => write!(f, "r"),
        }
    }
}

impl std::fmt::Display for OSMObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            OSMObjectType::Node => write!(f, "node"),
            OSMObjectType::Way => write!(f, "way"),
            OSMObjectType::Relation => write!(f, "relation"),
        }
    }
}

impl TryFrom<char> for OSMObjectType {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'n' => Ok(OSMObjectType::Node),
            'w' => Ok(OSMObjectType::Way),
            'r' => Ok(OSMObjectType::Relation),
            _ => Err(format!("Cannot convert {} to OSMObjectType", c)),
        }
    }
}

impl std::str::FromStr for OSMObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" | "node" => Ok(OSMObjectType::Node),
            "w" | "way" => Ok(OSMObjectType::Way),
            "r" | "relation" | "rel" => Ok(OSMObjectType::Relation),
            _ => Err(format!("Cannot convert {} to OSMObjectType", s)),
        }
    }
}

pub trait OSMObj: OSMObjBase {
    type Node: Node;
    type Way: Way;
    type Relation: Relation;

    fn object_type(&self) -> OSMObjectType;

    fn into_node(self) -> Option<Self::Node>;
    fn into_way(self) -> Option<Self::Way>;
    fn into_relation(self) -> Option<Self::Relation>;

    fn as_node(&self) -> Option<&Self::Node>;
    fn as_way(&self) -> Option<&Self::Way>;
    fn as_relation(&self) -> Option<&Self::Relation>;

    fn as_node_mut(&mut self) -> Option<&mut Self::Node>;
    fn as_way_mut(&mut self) -> Option<&mut Self::Way>;
    fn as_relation_mut(&mut self) -> Option<&mut Self::Relation>;

    fn is_node(&self) -> bool {
        self.object_type() == OSMObjectType::Node
    }
    fn is_way(&self) -> bool {
        self.object_type() == OSMObjectType::Way
    }
    fn is_relation(&self) -> bool {
        self.object_type() == OSMObjectType::Relation
    }
}

/// A Generic reader that reads OSM objects
pub trait OSMReader {
    type R: Read;
    type Obj: OSMObj;

    fn new(Self::R) -> Self;

    #[allow(unused_variables)]
    fn set_sorted_assumption(&mut self, sorted_assumption: bool) {}
    fn get_sorted_assumption(&mut self) -> bool {
        false
    }

    fn assume_sorted(&mut self) {
        self.set_sorted_assumption(true);
    }
    fn assume_unsorted(&mut self) {
        self.set_sorted_assumption(false);
    }

    /// Conver to the underlying reader
    fn into_inner(self) -> Self::R;

    fn inner(&self) -> &Self::R;

    fn next(&mut self) -> Option<Self::Obj>;

    fn objects<'a>(&'a mut self) -> OSMObjectIterator<'a, Self>
    where
        Self: Sized,
    {
        OSMObjectIterator { inner: self }
    }

    //fn nodes<'a, N: Node>(&'a mut self) -> Box<dyn Iterator<Item=N>+'a> where Self:Sized {
    //    if self.get_sorted_assumption() {
    //        Box::new(self.objects().take_while(|o| o.is_node()).filter_map(|o| o.into_node()))
    //    } else {
    //        Box::new(self.objects().filter_map(|o| o.into_node()))
    //    }
    //}

    //fn nodes_locations<'a>(&'a mut self) -> Box<Iterator<Item=(ObjId, Lat, Lon)>+'a> where Self:Sized {
    //    Box::new(self.nodes().filter_map(|n| if n.deleted || n.lat.is_none() { None } else { Some((n.id, n.lat.unwrap(), n.lon.unwrap())) } ))
    //}

    //fn ways<'a>(&'a mut self) -> Box<Iterator<Item=Way>+'a> where Self:Sized {
    //    if self.get_sorted_assumption() {
    //        Box::new(self.objects().take_while(|o| (o.is_node() || o.is_way())).filter_map(|o| o.into_way()))
    //    } else {
    //        Box::new(self.objects().filter_map(|o| o.into_way()))
    //    }
    //}

    //fn relations<'a>(&'a mut self) -> Box<Iterator<Item=Relation>+'a> where Self:Sized {
    //    Box::new(self.objects().filter_map(|o| o.into_relation()))
    //}
}

// FIXME does this have to be public? Can I make it private?
pub struct OSMObjectIterator<'a, R>
where
    R: OSMReader + 'a,
{
    inner: &'a mut R,
}

impl<'a, R> OSMObjectIterator<'a, R>
where
    R: OSMReader,
{
    pub fn inner(&self) -> &R {
        self.inner
    }
}

impl<'a, R> Iterator for OSMObjectIterator<'a, R>
where
    R: OSMReader,
{
    type Item = R::Obj;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An error when trying to write from an OSMWriter
#[derive(Debug)]
pub enum OSMWriteError {
    FormatDoesntSupportHeaders,
    AlreadyStarted,
    AlreadyClosed,
    OPLWrite(::std::io::Error),
    XMLWriteXMLError(quick_xml::Error),
    XMLWriteIOError(::std::io::Error),
}
impl std::fmt::Display for OSMWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for OSMWriteError {}

/// A generic writer for OSM objects.
pub trait OSMWriter<W: Write> {
    /// Create a writer from an underying writer
    fn new(W) -> Self;

    /// Close this writer, cannot write any more objects.
    /// Some fileformats have certain 'end of file' things. After you write those, you cannot write
    /// any more OSM objects. e.g. an XML file format will require that you close your root XML
    /// tag.
    /// After calling this method, you cannot add any more OSM objects to this writer, and
    /// `is_open` will return `false`.
    fn close(&mut self) -> Result<(), OSMWriteError>;

    /// Return true iff this writer is closed.
    /// If open you should be able to continue to write objects to it. if closed you cannot write
    /// any more OSM objects to it.
    fn is_open(&self) -> bool;

    /// Write an OSM object to this.
    fn write_obj(&mut self, obj: &impl OSMObj) -> Result<(), OSMWriteError>;

    /// Convert back to the underlying writer object
    fn into_inner(self) -> W;

    fn set_header(&mut self, _key_value: (&str, &str)) -> Result<(), OSMWriteError> {
        todo!("set_header not done yet")
    }

    /// Create a new OSMWriter, consume all the objects from an OSMObj iterator source, and then
    /// close this source. Returns this OSMWriter.
    fn from_iter<I: Iterator<Item = impl OSMObj>>(writer: W, iter: I) -> Self
    where
        Self: Sized,
    {
        let mut writer = Self::new(writer);

        // FIXME return the results of these operations?
        for obj in iter {
            writer.write_obj(&obj).unwrap();
        }
        writer.close().unwrap();

        writer
    }
}

/// The version string of this library.
fn version<'a>() -> &'a str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("unknown-non-cargo-build")
}
