use serde::de::{Deserialize, Deserializer, Visitor, MapVisitor, Error};

pub struct Struct {
    pub fields: Vec<Field>,
    pub comment: Option<String>,
}

impl Deserialize for Struct {
    fn deserialize<D>(d: &mut D) -> Result<Struct, D::Error>
        where D: Deserializer
    {
        enum StructField {
            Fields,
            Comment,
        }

        impl Deserialize for StructField {
            fn deserialize<D>(d: &mut D) -> Result<StructField, D::Error>
                where D: Deserializer
            {
                struct StructFieldVisitor;

                impl Visitor for StructFieldVisitor {
                    type Value = StructField;

                    fn visit_str<E>(&mut self, value: &str) -> Result<StructField, E>
                        where E: Error
                    {
                        match value {
                            "fields" => Ok(StructField::Fields),
                            "comment" => Ok(StructField::Comment),
                            f => Err(E::unknown_field(f)),
                        }
                    }
                }

                d.deserialize(StructFieldVisitor)
            }
        }

        struct StructVisitor;

        impl Visitor for StructVisitor {
            type Value = Struct;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Struct, V::Error>
                where V: MapVisitor
            {
                let mut fields = None;
                let mut comment = None;

                loop {
                    match try!(visitor.visit_key()) {
                        Some(StructField::Fields) => fields = Some(try!(visitor.visit_value())),
                        Some(StructField::Comment) => {
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

                Ok(Struct {
                    fields: fields,
                    comment: comment,
                })
            }
        }

        d.deserialize(StructVisitor)
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

pub struct Enum {
    pub values: Vec<Variant>,
    pub comment: Option<String>,
}

impl Deserialize for Enum {
    fn deserialize<D>(d: &mut D) -> Result<Enum, D::Error>
        where D: Deserializer
    {
        enum EnumField {
            Values,
            Comment,
        }

        impl Deserialize for EnumField {
            fn deserialize<D>(d: &mut D) -> Result<EnumField, D::Error>
                where D: Deserializer
            {
                struct EnumFieldVisitor;

                impl Visitor for EnumFieldVisitor {
                    type Value = EnumField;

                    fn visit_str<E>(&mut self, value: &str) -> Result<EnumField, E>
                        where E: Error
                    {
                        match value {
                            "values" => Ok(EnumField::Values),
                            "comment" => Ok(EnumField::Comment),
                            f => Err(E::unknown_field(f)),
                        }
                    }
                }

                d.deserialize(EnumFieldVisitor)
            }
        }

        struct EnumVisitor;

        impl Visitor for EnumVisitor {
            type Value = Enum;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Enum, V::Error>
                where V: MapVisitor
            {
                let mut values = None;
                let mut comment = None;

                loop {
                    match try!(visitor.visit_key()) {
                        Some(EnumField::Values) => values = Some(try!(visitor.visit_value())),
                        Some(EnumField::Comment) => {
                            comment = Some(try!(visitor.visit_value()));
                        }
                        None => break,
                    }
                }

                let values = match values {
                    Some(values) => values,
                    None => try!(visitor.missing_field("values")),
                };
                let comment = match comment {
                    Some(comment) => comment,
                    None => try!(visitor.missing_field("comment")),
                };

                try!(visitor.end());

                Ok(Enum {
                    values: values,
                    comment: comment,
                })
            }
        }

        d.deserialize(EnumVisitor)
    }
}

pub struct Variant {
    pub name: Option<String>,
    pub comment: Option<String>,
}

impl Deserialize for Variant {
    fn deserialize<D>(d: &mut D) -> Result<Variant, D::Error>
        where D: Deserializer
    {
        enum VariantField {
            Name,
            Comment,
        }

        impl Deserialize for VariantField {
            fn deserialize<D>(d: &mut D) -> Result<VariantField, D::Error>
                where D: Deserializer
            {
                struct VariantFieldVisitor;

                impl Visitor for VariantFieldVisitor {
                    type Value = VariantField;

                    fn visit_str<E>(&mut self, value: &str) -> Result<VariantField, E>
                        where E: Error
                    {
                        match value {
                            "name" => Ok(VariantField::Name),
                            "comment" => Ok(VariantField::Comment),
                            f => Err(E::unknown_field(f)),
                        }
                    }
                }

                d.deserialize(VariantFieldVisitor)
            }
        }

        struct VariantVisitor;

        impl Visitor for VariantVisitor {
            type Value = Variant;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Variant, V::Error>
                where V: MapVisitor
            {
                let mut name = None;
                let mut comment = None;

                loop {
                    match try!(visitor.visit_key()) {
                        Some(VariantField::Name) => name = Some(try!(visitor.visit_value())),
                        Some(VariantField::Comment) => {
                            comment = Some(try!(visitor.visit_value()));
                        }
                        None => break,
                    }
                }

                let name = match name {
                    Some(name) => name,
                    None => try!(visitor.missing_field("name")),
                };
                let comment = match comment {
                    Some(comment) => comment,
                    None => try!(visitor.missing_field("comment")),
                };

                try!(visitor.end());

                Ok(Variant {
                    name: name,
                    comment: comment,
                })
            }
        }

        d.deserialize(VariantVisitor)
    }
}
