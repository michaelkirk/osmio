use *;

macro_rules! func_call_inner_get {
    ($slf:ident, $name:ident) => {
        match $slf {
            StringOSMObj::Node(x) => x.$name(),
            StringOSMObj::Way(x) => x.$name(),
            StringOSMObj::Relation(x) => x.$name(),
        }
    };
}

macro_rules! func_call_inner_set {
    ($slf:ident, $name:ident, $val:ident) => {
        match $slf {
            StringOSMObj::Node(x) => x.$name($val),
            StringOSMObj::Way(x) => x.$name($val),
            StringOSMObj::Relation(x) => x.$name($val),
        };
    };
}

#[builder(setter(strip_option))]
#[derive(PartialEq, Debug, Builder, Clone)]
pub struct StringNode {
    pub(crate) _id: ObjId,

    #[builder(default = "None")]
    pub(crate) _version: Option<u32>,

    #[builder(default = "false")]
    pub(crate) _deleted: bool,
    #[builder(default = "None")]
    pub(crate) _changeset_id: Option<u32>,
    #[builder(default = "None")]
    pub(crate) _timestamp: Option<TimestampFormat>,
    #[builder(default = "None")]
    pub(crate) _uid: Option<u32>,
    #[builder(default = "None")]
    pub(crate) _user: Option<String>,

    #[builder(default = "HashMap::new()")]
    pub(crate) _tags: HashMap<String, String>,

    #[builder(default = "None")]
    pub(crate) _lat_lon: Option<(Lat, Lon)>,
}

#[derive(PartialEq, Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct StringWay {
    pub(crate) _id: ObjId,
    #[builder(default = "None")]
    pub(crate) _version: Option<u32>,
    #[builder(default = "false")]
    pub(crate) _deleted: bool,
    #[builder(default = "None")]
    pub(crate) _changeset_id: Option<u32>,
    #[builder(default = "None")]
    pub(crate) _timestamp: Option<TimestampFormat>,
    #[builder(default = "None")]
    pub(crate) _uid: Option<u32>,
    #[builder(default = "None")]
    pub(crate) _user: Option<String>,

    #[builder(default = "HashMap::new()")]
    pub(crate) _tags: HashMap<String, String>,

    #[builder(default = "Vec::new()")]
    pub(crate) _nodes: Vec<ObjId>,
}

#[derive(PartialEq, Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct StringRelation {
    pub(crate) _id: ObjId,
    #[builder(default = "None")]
    pub(crate) _version: Option<u32>,
    #[builder(default = "false")]
    pub(crate) _deleted: bool,
    #[builder(default = "None")]
    pub(crate) _changeset_id: Option<u32>,
    #[builder(default = "None")]
    pub(crate) _timestamp: Option<TimestampFormat>,
    #[builder(default = "None")]
    pub(crate) _uid: Option<u32>,
    #[builder(default = "None")]
    pub(crate) _user: Option<String>,

    #[builder(default = "HashMap::new()")]
    pub(crate) _tags: HashMap<String, String>,

    #[builder(default = "Vec::new()")]
    pub(crate) _members: Vec<(OSMObjectType, ObjId, String)>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum StringOSMObj {
    Node(StringNode),
    Way(StringWay),
    Relation(StringRelation),
}

impl From<StringNode> for StringOSMObj {
    fn from(n: StringNode) -> Self {
        StringOSMObj::Node(n)
    }
}
impl From<StringWay> for StringOSMObj {
    fn from(w: StringWay) -> Self {
        StringOSMObj::Way(w)
    }
}
impl From<StringRelation> for StringOSMObj {
    fn from(r: StringRelation) -> Self {
        StringOSMObj::Relation(r)
    }
}

impl OSMObjBase for StringOSMObj {
    fn id(&self) -> ObjId {
        func_call_inner_get!(self, id)
    }
    fn version(&self) -> Option<u32> {
        func_call_inner_get!(self, version)
    }
    fn deleted(&self) -> bool {
        func_call_inner_get!(self, deleted)
    }
    fn changeset_id(&self) -> Option<u32> {
        func_call_inner_get!(self, changeset_id)
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        func_call_inner_get!(self, timestamp)
    }
    fn uid(&self) -> Option<u32> {
        func_call_inner_get!(self, uid)
    }
    fn user(&self) -> Option<&str> {
        func_call_inner_get!(self, user)
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        func_call_inner_set!(self, set_id, val);
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        func_call_inner_set!(self, set_version, val);
    }
    fn set_deleted(&mut self, val: bool) {
        func_call_inner_set!(self, set_deleted, val);
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        func_call_inner_set!(self, set_changeset_id, val);
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        func_call_inner_set!(self, set_timestamp, val);
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        func_call_inner_set!(self, set_uid, val);
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        func_call_inner_set!(self, set_user, val);
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        func_call_inner_get!(self, tags)
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        match self {
            StringOSMObj::Node(x) => x.tag(key),
            StringOSMObj::Way(x) => x.tag(key),
            StringOSMObj::Relation(x) => x.tag(key),
        }
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        match self {
            StringOSMObj::Node(x) => x.set_tag(key, value),
            StringOSMObj::Way(x) => x.set_tag(key, value),
            StringOSMObj::Relation(x) => x.set_tag(key, value),
        }
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        match self {
            StringOSMObj::Node(x) => x.unset_tag(key),
            StringOSMObj::Way(x) => x.unset_tag(key),
            StringOSMObj::Relation(x) => x.unset_tag(key),
        }
    }
}

impl OSMObj for StringOSMObj {
    type Node = StringNode;
    type Way = StringWay;
    type Relation = StringRelation;

    fn object_type(&self) -> OSMObjectType {
        match self {
            StringOSMObj::Node(_) => OSMObjectType::Node,
            StringOSMObj::Way(_) => OSMObjectType::Way,
            StringOSMObj::Relation(_) => OSMObjectType::Relation,
        }
    }

    fn into_node(self) -> Option<StringNode> {
        if let StringOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn into_way(self) -> Option<StringWay> {
        if let StringOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn into_relation(self) -> Option<StringRelation> {
        if let StringOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }

    fn as_node(&self) -> Option<&StringNode> {
        if let StringOSMObj::Node(n) = self {
            Some(&n)
        } else {
            None
        }
    }

    fn as_way(&self) -> Option<&StringWay> {
        if let StringOSMObj::Way(w) = self {
            Some(&w)
        } else {
            None
        }
    }

    fn as_relation(&self) -> Option<&StringRelation> {
        if let StringOSMObj::Relation(r) = self {
            Some(&r)
        } else {
            None
        }
    }

    fn as_node_mut(&mut self) -> Option<&mut StringNode> {
        if let StringOSMObj::Node(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn as_way_mut(&mut self) -> Option<&mut StringWay> {
        if let StringOSMObj::Way(w) = self {
            Some(w)
        } else {
            None
        }
    }

    fn as_relation_mut(&mut self) -> Option<&mut StringRelation> {
        if let StringOSMObj::Relation(r) = self {
            Some(r)
        } else {
            None
        }
    }
}

impl OSMObjBase for StringNode {
    fn id(&self) -> ObjId {
        self._id
    }
    fn version(&self) -> Option<u32> {
        self._version
    }
    fn deleted(&self) -> bool {
        self._deleted
    }
    fn changeset_id(&self) -> Option<u32> {
        self._changeset_id
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        &self._timestamp
    }
    fn uid(&self) -> Option<u32> {
        self._uid
    }
    fn user(&self) -> Option<&str> {
        match self._user {
            None => None,
            Some(ref s) => Some(&s),
        }
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        self._id = val.into();
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        self._version = val.into();
    }
    fn set_deleted(&mut self, val: bool) {
        self._deleted = val.into();
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        self._changeset_id = val.into();
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        self._timestamp = val.into();
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        self._uid = val.into();
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        self._user = val.into().map(|s| s.to_string());
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        self._tags.get(key.as_ref()).map(|s| s.as_ref())
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self._tags.insert(key.as_ref().into(), value.into());
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        self._tags.remove(key.as_ref());
    }
}

impl Node for StringNode {
    fn lat_lon(&self) -> Option<(Lat, Lon)> {
        self._lat_lon
    }

    fn set_lat_lon(&mut self, loc: impl Into<Option<(Lat, Lon)>>) {
        self._lat_lon = loc.into();
    }
}

impl OSMObjBase for StringWay {
    fn id(&self) -> ObjId {
        self._id
    }
    fn version(&self) -> Option<u32> {
        self._version
    }
    fn deleted(&self) -> bool {
        self._deleted
    }
    fn changeset_id(&self) -> Option<u32> {
        self._changeset_id
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        &self._timestamp
    }
    fn uid(&self) -> Option<u32> {
        self._uid
    }
    fn user(&self) -> Option<&str> {
        match self._user {
            None => None,
            Some(ref s) => Some(&s),
        }
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        self._id = val.into();
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        self._version = val.into();
    }
    fn set_deleted(&mut self, val: bool) {
        self._deleted = val.into();
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        self._changeset_id = val.into();
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        self._timestamp = val.into();
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        self._uid = val.into();
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        self._user = val.into().map(|s| s.to_string());
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        self._tags.get(key.as_ref()).map(|s| s.as_ref())
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self._tags.insert(key.as_ref().into(), value.into());
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        self._tags.remove(key.as_ref());
    }
}

impl Way for StringWay {
    fn num_nodes(&self) -> usize {
        self._nodes.len()
    }
    fn nodes(&self) -> &[ObjId] {
        self._nodes.as_ref()
    }
    fn node(&self, idx: usize) -> Option<ObjId> {
        self._nodes.get(idx).cloned()
    }
    fn set_nodes(&mut self, nodes: impl IntoIterator<Item = impl Into<ObjId>>) {
        self._nodes.truncate(0);
        self._nodes.extend(nodes.into_iter().map(|i| i.into()));
    }
}

impl OSMObjBase for StringRelation {
    fn id(&self) -> ObjId {
        self._id
    }
    fn version(&self) -> Option<u32> {
        self._version
    }
    fn deleted(&self) -> bool {
        self._deleted
    }
    fn changeset_id(&self) -> Option<u32> {
        self._changeset_id
    }
    fn timestamp(&self) -> &Option<TimestampFormat> {
        &self._timestamp
    }
    fn uid(&self) -> Option<u32> {
        self._uid
    }
    fn user(&self) -> Option<&str> {
        match self._user {
            None => None,
            Some(ref s) => Some(&s),
        }
    }

    fn set_id(&mut self, val: impl Into<ObjId>) {
        self._id = val.into();
    }
    fn set_version(&mut self, val: impl Into<Option<u32>>) {
        self._version = val.into();
    }
    fn set_deleted(&mut self, val: bool) {
        self._deleted = val.into();
    }
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>) {
        self._changeset_id = val.into();
    }
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>) {
        self._timestamp = val.into();
    }
    fn set_uid(&mut self, val: impl Into<Option<u32>>) {
        self._uid = val.into();
    }
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>) {
        self._user = val.into().map(|s| s.to_string());
    }

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a> {
        Box::new(self._tags.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
    }

    fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        self._tags.get(key.as_ref()).map(|s| s.as_ref())
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self._tags.insert(key.as_ref().into(), value.into());
    }

    fn unset_tag(&mut self, key: impl AsRef<str>) {
        self._tags.remove(key.as_ref());
    }
}

impl Relation for StringRelation {
    fn members<'a>(
        &'a self,
    ) -> Box<dyn ExactSizeIterator<Item = (OSMObjectType, ObjId, &'a str)> + 'a> {
        Box::new(self._members.iter().map(|(t, i, r)| (*t, *i, r.as_str())))
    }

    fn set_members(
        &mut self,
        members: impl IntoIterator<Item = (OSMObjectType, ObjId, impl Into<String>)>,
    ) {
        self._members.truncate(0);
        self._members
            .extend(members.into_iter().map(|(t, i, r)| (t, i, r.into())))
    }
}
