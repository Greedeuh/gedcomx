// + TextValue => qui a lang ca peut etre bien
// 4. Extracted Conclusion Constraints
mod components;
pub use components::*;

pub enum Uri<T> {
    Some(Box<T>),
    //    Ref(&'a T),
    Id(String),
}

pub struct Person {
    pub subject: SubjectData,
    pub private: Option<bool>,
    pub gender: Option<Gender>,
    pub names: Vec<Name>,
    pub facts: Vec<Fact>,
}

impl Conclusion for Person {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for Person {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

impl Identifiable for Person {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

pub enum Relationship {
    Unknow(RelationshipData),
    Couple(RelationshipData),
    ParentChild(RelationshipData),
    EnslavedBy(RelationshipData),
}

pub struct RelationshipData {
    pub subject: SubjectData,
    pub person1: Uri<Person>,
    pub person2: Uri<Person>,
    pub facts: Vec<Uri<Fact>>,
}

impl Conclusion for Relationship {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for Relationship {
    fn subject(&self) -> &SubjectData {
        match self {
            Self::Unknow(x) => &x.subject,
            Self::Couple(x) => &x.subject,
            Self::ParentChild(x) => &x.subject,
            Self::EnslavedBy(x) => &x.subject,
        }
    }
}

impl Identifiable for Relationship {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

pub struct SourceDescription {
    pub id: Id,
    pub resource_type: Option<ResourceType>,
    pub citations: Vec<SourceCitation>,
    pub media_type: Option<String>,
    pub about: Uri<Box<dyn Identifiable>>,
    pub mediator: Uri<Agent>,
    pub publisher: Uri<Agent>,
    pub sources: Vec<SourceReference>,
    pub analysis: Option<Uri<Document>>,
    pub component_of: Option<SourceReference>,
    pub titles: Vec<TextValue>,
    pub notes: Option<Note>,
    pub attribution: Option<Attribution>,
    pub rights: Option<String>,
    pub coverage: Option<Coverage>,
    pub descriptions: Vec<TextValue>,
    pub identifiers: Vec<Identifier>,
    pub created: Option<Timestamp>,
    pub modified: Option<Timestamp>,
    pub repository: Option<Agent>,
}

pub enum ResourceType {
    Collection,
    PhysicalArtifact,
    DigitalArtifact,
    Record,
}

pub struct Agent {
    pub id: Id,
    pub identifiers: Vec<String>,
    pub names: Vec<TextValue>,
    pub homepage: Option<String>,
    pub openid: Option<String>,
    pub accounts: Vec<OnlineAccount>,
    pub emails: Vec<String>,
    pub phones: Vec<String>,
    pub addresses: Vec<Address>,
    pub person: Option<Uri<Person>>,
}

pub enum Event {
    Adoption(EventData),
    Birth(EventData),
    Burial(EventData),
    Census(EventData),
    Christening(EventData),
    Death(EventData),
    Divorce(EventData),
    Marriage(EventData),
}

impl Conclusion for Event {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Identifiable for Event {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

impl Subject for Event {
    fn subject(&self) -> &SubjectData {
        match self {
            Self::Adoption(x) => &x.subject,
            Self::Birth(x) => &x.subject,
            Self::Burial(x) => &x.subject,
            Self::Census(x) => &x.subject,
            Self::Christening(x) => &x.subject,
            Self::Death(x) => &x.subject,
            Self::Divorce(x) => &x.subject,
            Self::Marriage(x) => &x.subject,
        }
    }
}

pub struct EventData {
    pub subject: SubjectData,
    pub date: Option<Date>,
    pub place: Option<PlaceReference>,
    pub roles: Vec<EventRole>,
}

pub enum Document {
    Analysis(DocumentData),
    Abstract(DocumentData),
    Transcription(DocumentData),
    Translation(DocumentData),
}

impl Conclusion for Document {
    fn conclusion(&self) -> &ConclusionData {
        match self {
            Self::Analysis(x) => &x.conclusion,
            Self::Abstract(x) => &x.conclusion,
            Self::Transcription(x) => &x.conclusion,
            Self::Translation(x) => &x.conclusion,
        }
    }
}

impl Identifiable for Document {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

pub struct DocumentData {
    pub conclusion: ConclusionData,
}

pub struct PlaceDescription {
    pub subject: SubjectData,
    pub names: NonEmptyVec<TextValue>,
    pub typee: Option<PlaceType>,
    pub place: Option<String>,
    pub jurisdiction: Option<Uri<Box<PlaceDescription>>>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub temporal_description: Option<Date>,
    pub spatial_description: Option<Kml>,
}

impl Conclusion for PlaceDescription {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Identifiable for PlaceDescription {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

impl Subject for PlaceDescription {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

pub type NonEmptyVec<T> = Vec<T>;

pub type PlaceType = String;

pub type Kml = String;

pub struct Group {
    pub subject: SubjectData,
    pub names: Vec<TextValue>,
    pub date: Option<Date>,
    pub place: Option<PlaceReference>,
    pub roles: Option<GroupRole>,
}

impl Conclusion for Group {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Identifiable for Group {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

impl Subject for Group {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}
