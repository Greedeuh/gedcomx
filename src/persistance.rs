use crate::*;

pub trait Persistable: Identifiable + Sized {
    fn save(&self) -> Result<(), PersistanceError>;

    fn get() -> Result<Self, PersistanceError>;
}

pub enum PersistanceError {
    Unimplemented,
}

impl Persistable for Person {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for Relationship {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for Event {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for Document {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for PlaceDescription {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for Group {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for Gender {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for Name {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}

impl Persistable for Fact {
    fn save(&self) -> Result<(), PersistanceError> {
        unimplemented!()
    }

    fn get() -> Result<Self, PersistanceError> {
        unimplemented!()
    }
}
