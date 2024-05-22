use sqlx;

#[derive(sqlx::FromRow)]
struct PropertyPhysicalType {
    uuid: String,
    name: String,
    unit: Vec<String>,
}

#[derive(sqlx::FromRow)]
struct PropertyDefinition {
    uuid: String,
    name: String,
    value_type: String,
    xml_name: Option<String>,
    is_array: bool,
}

#[derive(sqlx::FromRow)]
struct GenomeProperty {
    uuid: String,
    property_definition_uuid: String,
    unit: Option<String>,
    #[sqlx(rename="type")]
    _type: String,
    string_values: Vec<String>,
}

#[derive(sqlx::FromRow)]
struct Genome {
    uuid: String,
    gxbd_id: Option<String>,
    label: String,
    genome_type_uuid: String,
    state: String,
    create_date: u64,
    update_date: u64,
    manufacturer_uuid: String,
    source: String,
    is_real: Option<bool>,
    real_genome_uuid: Option<String>,
}

#[derive(sqlx::FromRow)]
struct Manufacturer {
    uuid: String,
    name: String,
}

#[derive(sqlx::FromRow)]
struct GenomeType {
    uuid: String,
    name: String,
}

#[derive(sqlx::FromRow)]
struct GenomeTypePropertyDefinition {
    genome_type_uuid: String,
    property_definition_uuid: String,
    is_required: bool,
}
