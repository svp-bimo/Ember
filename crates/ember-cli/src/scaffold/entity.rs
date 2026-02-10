#![forbid(unsafe_code)]

#[derive(Debug, Clone)]
pub struct EntitySpec {
    pub name: String,
    pub fields: Vec<FieldSpec>,
}

#[derive(Debug, Clone)]
pub struct FieldSpec {
    pub name: String,
    pub ty: String,
}

impl EntitySpec {
    pub fn id_field(&self) -> &FieldSpec {
        self.fields
            .iter()
            .find(|field| field.name == "id")
            .unwrap_or_else(|| &self.fields[0])
    }

    pub fn snake_name(&self) -> String {
        to_snake_case(&self.name)
    }

    pub fn plural_snake_name(&self) -> String {
        let base = self.snake_name();
        if base.ends_with('s') {
            format!("{base}es")
        } else {
            format!("{base}s")
        }
    }
}

pub fn parse_entity_spec(input: &str) -> Result<EntitySpec, String> {
    let mut parts = input.splitn(2, ':');
    let name = parts
        .next()
        .ok_or_else(|| "missing entity name".to_string())?
        .trim();
    let fields_part = parts
        .next()
        .ok_or_else(|| "missing fields (expected Name:field:type,...)".to_string())?
        .trim();

    if name.is_empty() {
        return Err("entity name cannot be empty".to_string());
    }

    let mut fields = Vec::new();
    for field in fields_part.split(',') {
        let field = field.trim();
        if field.is_empty() {
            continue;
        }
        let mut field_parts = field.splitn(2, ':');
        let field_name = field_parts
            .next()
            .ok_or_else(|| "field name missing".to_string())?
            .trim();
        let field_ty = field_parts
            .next()
            .ok_or_else(|| "field type missing".to_string())?
            .trim();
        if field_name.is_empty() || field_ty.is_empty() {
            return Err(format!("invalid field definition: {field}"));
        }
        fields.push(FieldSpec {
            name: field_name.to_string(),
            ty: field_ty.to_string(),
        });
    }

    if fields.is_empty() {
        return Err("at least one field is required".to_string());
    }

    if !fields.iter().any(|field| field.name == "id") {
        fields.insert(
            0,
            FieldSpec {
                name: "id".to_string(),
                ty: "i64".to_string(),
            },
        );
    }

    Ok(EntitySpec {
        name: name.to_string(),
        fields,
    })
}

pub fn to_snake_case(input: &str) -> String {
    let mut out = String::new();
    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}
