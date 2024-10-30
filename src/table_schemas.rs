use arrow::datatypes::{DataType, Field, Schema};

pub struct TableDef {
    pub name: String,
    pub schema: Schema,
}

impl TableDef {
    pub fn new(name: impl Into<String>, schema: Schema) -> Self {
        Self {
            name: name.into(),
            schema,
        }
    }
}

pub fn movie_schemas() -> Vec<TableDef> {
    vec![
        TableDef::new(
            "movie",
            Schema::new(vec![
                Field::new("mid", DataType::Int32, false),
                Field::new("title", DataType::Utf8, false),
                Field::new("year", DataType::Int32, false),
                Field::new("runtime", DataType::Int32, false),
                Field::new("rank", DataType::Int32, false),
            ]),
        ),
        TableDef::new(
            "people",
            Schema::new(vec![
                Field::new("pid", DataType::Int32, false),
                Field::new("first_name", DataType::Utf8, false),
                Field::new("last_name", DataType::Utf8, false),
            ]),
        ),
        TableDef::new(
            "director",
            Schema::new(vec![
                Field::new("mid", DataType::Int32, false),
                Field::new("pid", DataType::Int32, false),
            ]),
        ),
        TableDef::new(
            "role",
            Schema::new(vec![
                Field::new("mid", DataType::Int32, false),
                Field::new("pid", DataType::Int32, false),
                Field::new("name", DataType::Utf8, false),
            ]),
        ),
    ]
}

pub fn department_schemas() -> Vec<TableDef> {
    vec![
        TableDef::new(
            "department",
            Schema::new(vec![
                Field::new("did", DataType::Int32, false),
                Field::new("name", DataType::Utf8, false),
                Field::new("year_started", DataType::Int32, false),
                Field::new("year_ended", DataType::Int32, true),
            ]),
        ),
        TableDef::new(
            "people",
            Schema::new(vec![
                Field::new("pid", DataType::Int32, false),
                Field::new("first_name", DataType::Utf8, false),
                Field::new("last_name", DataType::Utf8, false),
                Field::new("did", DataType::Int32, false),
            ]),
        ),
    ]
}
