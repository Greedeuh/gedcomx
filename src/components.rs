use crate::*;

pub struct Identifier {
    pub uri: String,
    pub typee: Option<IdentifierType>,
}

pub enum IdentifierType {
    None,
}

pub struct Attribution {
    pub contributor: Option<Uri<Agent>>,
    pub modified: Option<Timestamp>,
    pub change_message: Option<String>,
    pub creator: Option<Agent>,
    pub created: Option<Timestamp>,
}

pub struct Note {
    pub lang: Option<Lang>,
    pub subject: Option<String>,
    pub text: String,
    pub attribution: Option<Attribution>,
}

pub struct TextValue {
    pub lang: Option<Lang>,
    pub value: String,
}

pub struct SourceCitation {
    pub lang: String,
    pub value: String,
}

pub struct SourceReference {
    pub description: Uri<Box<SourceDescription>>,
    pub description_id: Option<Id>,
    pub attribution: Option<Attribution>,
    pub qualifiers: Vec<SourceReferenceQualifier>,
}

pub enum SourceReferenceQualifier {
    CharacterRegion,
    RectangleRegion,
    TimeRegion,
}

pub struct EvidenceReference {
    pub resource: Uri<Box<dyn Subject>>,
    pub attribution: Option<Attribution>,
}

pub struct OnlineAccount {
    pub service_homepage: String,
    pub account_name: String,
}
pub struct Address {
    pub value: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub state_or_province: Option<String>,
    pub street: Option<String>,
    pub street2: Option<String>,
    pub street3: Option<String>,
    pub street4: Option<String>,
    pub street5: Option<String>,
    pub street6: Option<String>,
}

pub trait Identifiable {
    fn id(&self) -> &Id;
}

pub type Id = String;

pub trait Conclusion: Identifiable {
    fn conclusion(&self) -> &ConclusionData;
}

pub struct ConclusionData {
    pub id: Id,
    pub lang: Option<String>,
    pub sources: Vec<SourceReference>,
    pub analysis: Option<Uri<Document>>,
    pub notes: Vec<Note>,
    pub confidence: Option<ConfidenceLevel>,
    pub attribution: Option<Attribution>,
}

pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

pub trait Subject: Conclusion {
    fn subject(&self) -> &SubjectData;
}

pub struct SubjectData {
    pub conclusion: ConclusionData,
    pub extracted: Option<bool>,
    pub evidence: Vec<EvidenceReference>,
    pub media: Vec<SourceReference>,
    pub identifiers: Vec<Identifier>,
}

pub enum Gender {
    Male(ConclusionData),
    Female(ConclusionData),
    Unknown(ConclusionData),
    Intersex(ConclusionData),
}

impl Conclusion for Gender {
    fn conclusion(&self) -> &ConclusionData {
        match self {
            Self::Male(x) => &x,
            Self::Female(x) => &x,
            Self::Unknown(x) => &x,
            Self::Intersex(x) => &x,
        }
    }
}

impl Identifiable for Gender {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

pub enum Name {
    None(NameData),
    BirthName(NameData),
    MarriedName(NameData),
    AlsoKnownAs(NameData),
    Nickname(NameData),
    AdoptiveName(NameData),
    FormalName(NameData),
    ReligiousName(NameData),
}

impl Name {
    pub fn data(&self) -> &NameData {
        match self {
            Self::None(x) => &x,
            Self::BirthName(x) => &x,
            Self::MarriedName(x) => &x,
            Self::AlsoKnownAs(x) => &x,
            Self::Nickname(x) => &x,
            Self::AdoptiveName(x) => &x,
            Self::FormalName(x) => &x,
            Self::ReligiousName(x) => &x,
        }
    }
}

pub struct NameData {
    pub conclusion: ConclusionData,
    pub name_forms: Vec<NameForm>,
    pub date: Option<Date>,
}

impl Conclusion for Name {
    fn conclusion(&self) -> &ConclusionData {
        &self.data().conclusion
    }
}

impl Identifiable for Name {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

pub enum Fact {
    Adoption(FactData),
    Birth(FactData),
    Burial(FactData),
    Christening(FactData),
    Death(FactData),
    Residence(FactData),
    Divorce(FactData),
    Marriage(FactData),
}

impl Fact {
    pub fn data(&self) -> &FactData {
        match self {
            Self::Adoption(x) => &x,
            Self::Birth(x) => &x,
            Self::Burial(x) => &x,
            Self::Christening(x) => &x,
            Self::Death(x) => &x,
            Self::Residence(x) => &x,
            Self::Divorce(x) => &x,
            Self::Marriage(x) => &x,
        }
    }
}

pub struct FactData {
    pub conclusion: ConclusionData,
    pub date: Option<Date>,
    pub place: Option<PlaceReference>,
    pub value: Option<String>,
    pub qualifiers: Vec<FactQualifier>,
}

pub enum FactQualifier {
    Age,
    Cause,
    Religion,
    Transport,
    NonConsensual,
}

impl Conclusion for Fact {
    fn conclusion(&self) -> &ConclusionData {
        &self.data().conclusion
    }
}

impl Identifiable for Fact {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

pub enum EventRole {
    None(EventRoleData),
    Principal(EventRoleData),
    Participant(EventRoleData),
    Official(EventRoleData),
    Witness(EventRoleData),
}

pub struct EventRoleData {
    pub person: Uri<Person>,
    pub details: Option<String>,
}

pub struct Date {
    pub original: Option<String>,
    pub formal: Option<DateX>,
}

pub struct DateX;

pub struct PlaceReference {
    pub original: Option<String>,
    pub description_ref: Option<Uri<PlaceDescription>>,
}

pub enum NamePart {
    None(NamePartData),
    Prefix(NamePartData),
    Suffix(NamePartData),
    Given(NamePartData),
    Surname(NamePartData),
}

pub struct NamePartData {
    pub value: String,
    pub qualifiers: Vec<NamePartQualifier>,
}

pub enum NamePartQualifier {
    Title,
    Primary,
    Secondary,
    Middle,
    Familiar,
    Religious,
    Family,
    Maiden,
    Patronymic,
    Matronymic,
    Geographic,
    Occupational,
    Characteristic,
    Postnom,
    Particle,
    RootName,
}

pub struct NameForm {
    pub lang: Option<Lang>,
    pub full_text: Option<String>,
    pub parts: Vec<NamePart>,
}

pub struct Coverage {
    pub spatial: Option<PlaceReference>,
    pub temporal: Option<Date>,
}

pub enum GroupRole {
    Unknow(GroupRoleData),
}

pub struct GroupRoleData {
    pub person: Uri<Person>,
    pub date: Option<Date>,
    pub details: Option<String>,
}

pub type Lang = String;

pub struct Timestamp {}
