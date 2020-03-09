CREATE TABLE person
(
    id VARCHAR PRIMARY KEY REFERENCES subject(id),
    private BOOLEAN,
    gender_id VARCHAR REFERENCES gender(id)
)

CREATE TABLE relationship
(
    id SERIAL PRIMARY KEY REFERENCES subject(id),
    type VARCHAR,
    person1_id VARCHAR REFERENCES person(id),
    person2_id VARCHAR REFERENCES person(id)
)

CREATE TABLE source_description
(
    id VARCHAR PRIMARY KEY,
    resource_type VARCHAR,
    media_type VARCHAR,
    about_id VARCHAR,
    -- TODO about

    mediator_id VARCHAR REFERENCES agent(id),
    publisher_id VARCHAR REFERENCES agent(id),
    analysis_id VARCHAR,
    component_of VARCHAR,
    media_type VARCHAR,
    notes_id VARCHAR,
    attribution_id VARCHAR,
    notes_id VARCHAR,
    rights_id VARCHAR,
    coverage_spatial_id INTEGER,
    coverage_temporal DATE,
    created TIMESTAMP,
    modified TIMESTAMP,
    repository_id VARCHAR
)

CREATE TABLE agent
(
    id VARCHAR PRIMARY KEY,
    homepage VARCHAR,
    openid VARCHAR,
    person_id VARCHAR
)

CREATE TABLE event
(
    id SERIAL,
    type VARCHAR,
    date DATE,
    place_reference_original TEXT,
    place_description_ref VARCHAR
)

CREATE TABLE document
(
    id SERIAL,
    type VARCHAR
)

CREATE TABLE place_description
(
    id SERIAL,
    type VARCHAR,
    juridiction VARCHAR,
    latitute FLOAT,
    longitude FLOAT,
    temporal_description DATE,
    spatial_desicription VARCHAR
)

CREATE TABLE group
(
    id SERIAL,
    date DATE,
    place_reference_original TEXT,
    place_description_ref VARCHAR
)

CREATE TABLE identifier
(
    id SERIAL,
    uri VARCHAR,
    type VARCHAR
)

CREATE TABLE attribution
(
    id SERIAL,
    contributor_id VARCHAR,
    modified TIMESTAMP,
    change_message TEXT,
    creator_id VARCHAR,
    created TIMESTAMP
)

CREATE TABLE note
(
    id SERIAL,
    subject VARCHAR,
    text VARCHAR,
    attribution_id INTEGER
)

CREATE TABLE text_value
(
    lang VARCHAR,
    value TEXT
)

CREATE TABLE source_citation
(
    id SERIAL,
    source_description_id VARCHAR REFERENCES source_description(id),
    lang VARCHAR,
    value TEXT
)

CREATE TABLE source_reference
(
    id SERIAL,
    description_id VARCHAR,
    description_id VARCHAR,
    attribution_id INTEGER
)

CREATE TABLE source_reference_qualifier
(
    code VARCHAR
)

CREATE TABLE evidence_reference
(
    resource_id VARCHAR,
    attribution_id INTEGER
)

CREATE TABLE online_account
(
    service_homepage VARCHAR,
    account_name VARCHAR
)

CREATE TABLE online_account
(
    id SERIAL,
    value VARCHAR,
    city VARCHAR,
    country VARCHAR,
    postal_code VARCHAR,
    state_or_province VARCHAR,
    street VARCHAR,
    street2 VARCHAR,
    street3 VARCHAR,
    street4 VARCHAR,
    street5 VARCHAR,
    street6 VARCHAR
)

CREATE TABLE conclusion
(
    id VARCHAR,
    type VARCHAR,
    lang VARCHAR,
    analysis_id VARCHAR,
    confidence VARCHAR,
    attribution_id INTEGER
)

CREATE TABLE subject
(
    id SERIAL,
    extracted BOOLEAN
)

CREATE TABLE gender
(
    id VARCHAR PRIMARY KEY
)

CREATE TABLE name
(
    id SERIAL,
    type VARCHAR,
    date DATE
)

CREATE TABLE fact
(
    id SERIAL,
    type VARCHAR,
    date DATE,
    place_id INTEGER,
    value text
)

CREATE TABLE fact_qualifier
(
    code VARCHAR
)

CREATE TABLE event_role
(
    id SERIAL,
    type VARCHAR,
    person_id VARCHAR,
    details VARCHAR
)

CREATE TABLE name_part
(
    id SERIAL,
    type VARCHAR,
    value VARCHAR
)

CREATE TABLE name_part_qualifier
(
    code VARCHAR
)

CREATE TABLE name_form
(
    id SERIAL,
    lang VARCHAR,
    full_text VARCHAR
)

CREATE TABLE name_form
(
    id SERIAL,
    lang VARCHAR,
    full_text VARCHAR
)

-- coverage

CREATE TABLE group_role
(
    id SERIAL,
    person_id VARCHAR,
    date DATE,
    details text
)

CREATE TABLE group_role
(
    id SERIAL,
    person_id VARCHAR,
    date DATE,
    details text
)

CREATE TABLE person_has_names
(
    person_id VARCHAR REFERENCES person(id),
    name_id VARCHAR REFERENCES name(id)
)

CREATE TABLE person_has_fact_have_links
(
    person_id VARCHAR REFERENCES person(id),
    fact_id VARCHAR REFERENCES fact(id)
)

CREATE TABLE relationship_and_fact_have_links
(
    relationship_id VARCHAR REFERENCES relationship(id),
    fact_id VARCHAR REFERENCES fact(id)
)




