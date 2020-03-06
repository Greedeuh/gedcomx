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
    subject: SubjectData,
    private: Option<bool>,
    gender: Option<Gender>,
    names: Vec<Name>,
    facts: Vec<Fact>,
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
    subject: SubjectData,
    person1: Uri<Person>,
    person2: Uri<Person>,
    facts: Vec<Uri<Fact>>,
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
    id: Id,
    resource_type: Option<ResourceType>,
    citations: Vec<SourceCitation>,
    media_type: Option<String>,
    about: Uri<Box<dyn Identifiable>>,
    mediator: Uri<Agent>,
    publisher: Uri<Agent>,
    sources: Vec<SourceReference>,
    analysis: Option<Uri<Document>>,
    componentOf: Option<SourceReference>,
    titles: Vec<TextValue>,
    notes: Option<Note>,
    attribution: Option<Attribution>,
    rights: Option<String>,
    coverage: Option<Coverage>,
    descriptions: Vec<TextValue>,
    identifiers: Vec<Identifier>,
    created: Option<Timestamp>,
    modified: Option<Timestamp>,
    repository: Option<Agent>,
}

pub enum ResourceType {
    Collection,
    PhysicalArtifact,
    DigitalArtifact,
    Record,
}

pub struct Agent {
    id: Id,
    identifiers: Vec<String>,
    names: Vec<TextValue>,
    homepage: Option<String>,
    openid: Option<String>,
    accounts: Vec<OnlineAccount>,
    emails: Vec<String>,
    phones: Vec<String>,
    addresses: Vec<Address>,
    person: Option<Uri<Person>>,
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
    subject: SubjectData,
    date: Option<Date>,
    place: Option<PlaceReference>,
    roles: Vec<EventRole>,
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
    conclusion: ConclusionData,
}

pub struct PlaceDescription {
    subject: SubjectData,
    names: NonEmptyVec<TextValue>,
    typee: Option<PlaceType>,
    place: Option<String>,
    jurisdiction: Option<Uri<Box<PlaceDescription>>>,
    latitude: Option<f32>,
    longitude: Option<f32>,
    temporalDescription: Option<Date>,
    spatialDescription: Option<Kml>,
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
    subject: SubjectData,
    names: Vec<TextValue>,
    date: Option<Date>,
    place: Option<PlaceReference>,
    roles: Option<GroupRole>,
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
