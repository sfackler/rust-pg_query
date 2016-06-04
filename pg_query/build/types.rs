use serde::de::{Deserialize, Deserializer, Visitor, MapVisitor, Error};

pub struct StructDef {
    pub fields: Vec<Field>,
    pub comment: Option<String>,
}

impl Deserialize for StructDef {
    fn deserialize<D>(d: &mut D) -> Result<StructDef, D::Error>
        where D: Deserializer
    {
        enum StructDefField {
            Fields,
            Comment,
        }

        impl Deserialize for StructDefField {
            fn deserialize<D>(d: &mut D) -> Result<StructDefField, D::Error>
                where D: Deserializer
            {
                struct StructDefFieldVisitor;

                impl Visitor for StructDefFieldVisitor {
                    type Value = StructDefField;

                    fn visit_str<E>(&mut self, value: &str) -> Result<StructDefField, E>
                        where E: Error
                    {
                        match value {
                            "fields" => Ok(StructDefField::Fields),
                            "comment" => Ok(StructDefField::Comment),
                            f => Err(E::unknown_field(f)),
                        }
                    }
                }

                d.deserialize(StructDefFieldVisitor)
            }
        }

        struct StructDefVisitor;

        impl Visitor for StructDefVisitor {
            type Value = StructDef;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<StructDef, V::Error>
                where V: MapVisitor
            {
                let mut fields = None;
                let mut comment = None;

                loop {
                    match try!(visitor.visit_key()) {
                        Some(StructDefField::Fields) => fields = Some(try!(visitor.visit_value())),
                        Some(StructDefField::Comment) => {
                            comment = Some(try!(visitor.visit_value()));
                        }
                        None => break,
                    }
                }

                let fields = match fields {
                    Some(fields) => fields,
                    None => try!(visitor.missing_field("fields")),
                };
                let comment = match comment {
                    Some(comment) => comment,
                    None => try!(visitor.missing_field("comment")),
                };

                try!(visitor.end());

                Ok(StructDef {
                    fields: fields,
                    comment: comment,
                })
            }
        }

        d.deserialize(StructDefVisitor)
    }
}

pub struct Field {
    pub name: Option<String>,
    pub c_type: Option<String>,
    pub comment: Option<String>,
}

impl Deserialize for Field {
    fn deserialize<D>(d: &mut D) -> Result<Field, D::Error>
        where D: Deserializer
    {
        enum FieldField {
            Name,
            Ctype,
            Comment,
        }

        impl Deserialize for FieldField {
            fn deserialize<D>(d: &mut D) -> Result<FieldField, D::Error>
                where D: Deserializer
            {
                struct FieldFieldVisitor;

                impl Visitor for FieldFieldVisitor {
                    type Value = FieldField;

                    fn visit_str<E>(&mut self, value: &str) -> Result<FieldField, E>
                        where E: Error
                    {
                        match value {
                            "name" => Ok(FieldField::Name),
                            "c_type" => Ok(FieldField::Ctype),
                            "comment" => Ok(FieldField::Comment),
                            f => Err(E::unknown_field(f)),
                        }
                    }
                }

                d.deserialize(FieldFieldVisitor)
            }
        }

        struct FieldVisitor;

        impl Visitor for FieldVisitor {
            type Value = Field;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Field, V::Error>
                where V: MapVisitor
            {
                let mut name = None;
                let mut c_type = None;
                let mut comment = None;

                loop {
                    match try!(visitor.visit_key()) {
                        Some(FieldField::Name) => name = Some(try!(visitor.visit_value())),
                        Some(FieldField::Ctype) => c_type = Some(try!(visitor.visit_value())),
                        Some(FieldField::Comment) => comment = Some(try!(visitor.visit_value())),
                        None => break,
                    }
                }

                let name = match name {
                    Some(name) => name,
                    None => try!(visitor.missing_field("name")),
                };
                let c_type = match c_type {
                    Some(c_type) => c_type,
                    None => try!(visitor.missing_field("c_type")),
                };
                let comment = match comment {
                    Some(comment) => comment,
                    None => try!(visitor.missing_field("comment")),
                };

                try!(visitor.end());

                Ok(Field {
                    name: name,
                    c_type: c_type,
                    comment: comment,
                })
            }
        }

        d.deserialize(FieldVisitor)
    }
}
