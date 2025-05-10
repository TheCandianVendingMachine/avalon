#[cfg(feature = "write")]
pub mod write {
    use crate::asset::{ Metadata, Unit };
    use std::borrow::Cow;
    use miniserde::{ ser, Serialize };

    struct MetadataStream<'a> {
        metadata: &'a Metadata,
        uuid_string: Option<String>,
        state: usize
    }

    impl<'a> ser::Map for MetadataStream<'a> {
        fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn Serialize)> {
            let state = self.state;
            self.state += 1;
            match state {
                0 => {
                    Some((Cow::Borrowed("tag"), &self.metadata.tag))
                },
                1 => {
                    self.uuid_string = Some(self.metadata.uuid.to_string());
                    let str_ref = self.uuid_string.as_ref().unwrap();
                    Some((Cow::Borrowed("uuid"), str_ref))
                },
                2 => {
                    Some(match &self.metadata.unit {
                        Unit::Shader(shader) => (Cow::Borrowed("shader"), shader),
                        Unit::Texture(texture) => (Cow::Borrowed("texture"), texture),
                        Unit::Model(model) => (Cow::Borrowed("model"), model),
                        Unit::Text(text) => (Cow::Borrowed("text"), text),
                    })
                },
                _ => None,
            }
        }
    }

    impl Serialize for Metadata {
        fn begin(&self) -> ser::Fragment {
            ser::Fragment::Map(Box::new(MetadataStream {
                metadata: self,
                uuid_string: None,
                state: 0
            }))
        }
    }
}

#[cfg(feature = "read")]
pub mod read {
    use crate::asset::{ Metadata, Unit };
    use crate::{ shader, texture, text, model };
    use miniserde::{ make_place, de, Deserialize, Result };
    use std::str::FromStr;

    make_place!(Place);

    struct MetadataUnit {
        shader: Option<shader::Shader>,
        texture: Option<texture::Texture>,
        model: Option<model::Model>,
        text: Option<text::Text>,
    }

    impl MetadataUnit {
        fn as_unit(&self) -> Option<Unit> {
            if let Some(shader) = self.shader {
                return Some(Unit::Shader(shader));
            }
            if let Some(texture) = self.texture {
                return Some(Unit::Texture(texture));
            }
            if let Some(model) = self.model {
                return Some(Unit::Model(model));
            }
            if let Some(text) = self.text {
                return Some(Unit::Text(text));
            }
            None
        }
    }

    struct MetadataBuilder<'a> {
        tag: Option<String>,
        uuid_string: Option<String>,
        unit: MetadataUnit,
        out: &'a mut Option<Metadata>
    }

    impl de::Visitor for Place<Metadata> {
        fn map(&mut self) -> Result<Box<dyn de::Map + '_>> {
            Ok(Box::new(MetadataBuilder {
                tag: None,
                uuid_string: None,
                unit: MetadataUnit {
                    shader: None,
                    texture: None,
                    model: None,
                    text: None
                },
                out: &mut self.out
            }))
        }
    }

    impl<'a> de::Map for MetadataBuilder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn de::Visitor> {
            match k {
                "tag" => Ok(Deserialize::begin(&mut self.tag)),
                "uuid" => Ok(Deserialize::begin(&mut self.uuid_string)),
                "shader" => Ok(Deserialize::begin(&mut self.unit.shader)),
                "texture" => Ok(Deserialize::begin(&mut self.unit.texture)),
                "model" => Ok(Deserialize::begin(&mut self.unit.model)),
                "text" => Ok(Deserialize::begin(&mut self.unit.text)),
                _ => Ok(<dyn de::Visitor>::ignore())
            }
        }

        fn finish(&mut self) -> Result<()> {
            let tag = self.tag.take().ok_or(miniserde::Error)?;
            let uuid = self.uuid_string.take().ok_or(miniserde::Error)?;
            let unit = self.unit.as_unit().ok_or(miniserde::Error)?;

            *self.out = Some(Metadata {
                tag,
                unit,
                filepath: None,
                uuid: uuid::Uuid::from_str(&uuid).map_err(|_| miniserde::Error)?
            });
            Ok(())
        }
    }

    impl Deserialize for Metadata {
        fn begin(out: &mut Option<Self>) -> &mut dyn de::Visitor {
            Place::new(out)
        }
    }
}
