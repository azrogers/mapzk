use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::Deserialize;

use crate::parse_error::{ParseCompleteResult, ParseError, ParseErrorType};
use crate::reader::Reader;
use crate::token::TokenType;
use crate::types::{CollectionType, RealType};

type Result<T> = ParseCompleteResult<T>;

pub struct Deserializer<'de> {
    reader: Reader<'de>,
    started_base_struct: bool,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer {
            reader: Reader::new(&input),
            started_base_struct: false,
        }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let result = T::deserialize(&mut deserializer);
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(e.with_context(&deserializer.reader, deserializer.reader.current_position())),
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = ParseError;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let next_type = self.reader.peek_next_type()?;

        if next_type.is_none() {
            return visitor.visit_none();
        }

        let next_type = next_type.unwrap();

        match next_type {
            RealType::Boolean => self.deserialize_bool(visitor),
            RealType::Number => {
                let number = self.reader.read_number_as_str()?;
                match number.contains(".") {
                    true => visitor.visit_f64(self.reader.parse_number(number)?),
                    false => visitor.visit_i64(self.reader.parse_number(number)?),
                }
            }
            RealType::String => self.deserialize_string(visitor),
            RealType::Identifier => self.deserialize_identifier(visitor),
            RealType::ObjectOrArray => match self.reader.try_discern_array_or_map()? {
                Some(collection_type) => match collection_type {
                    CollectionType::Array => self.deserialize_seq(visitor),
                    CollectionType::Object => self.deserialize_map(visitor),
                },
                None => visitor.visit_none(),
            },
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.reader.read_boolean()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.reader.read_number()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.reader.read_number()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.reader.read_number()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.reader.read_number()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.reader.read_number()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.reader.read_number()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.reader.read_number()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.reader.read_number()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.reader.read_number()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.reader.read_number()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.reader.read_stringlike()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.reader.is_next_value_empty()? {
            true => visitor.visit_none(),
            false => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.reader.is_next_value_empty()? {
            true => visitor.visit_none(),
            false => Err(self
                .reader
                .parse_error(ParseErrorType::InvalidType, "expected unit, found value")),
        }
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.reader.begin_collection()?;
        let value = visitor.visit_seq(ArrayValue::new(self))?;
        self.reader.end_collection()?;

        Ok(value)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let had_started = self.started_base_struct;
        if self.started_base_struct {
            self.reader.begin_collection()?;
        } else {
            self.started_base_struct = true;
        }

        let value = visitor.visit_map(MapValue::new(self))?;

        if had_started {
            self.reader.end_collection()?;
        }

        Ok(value)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let next_type = self.reader.peek_next_type_expect()?;
        match next_type {
            RealType::Identifier => {
                let str = self.reader.read_identifier()?;
                visitor.visit_enum(str.into_deserializer())
            }
            RealType::String => {
                let str = self.reader.peek_expected_string()?;

                if variants.contains(&str) {
                    visitor.visit_enum(self.reader.read_string()?.into_deserializer())
                } else {
                    visitor.visit_enum(Enum::new(self))
                }
            }
            RealType::ObjectOrArray => {
                let collection_type = self.reader.try_discern_array_or_map()?;

                match collection_type {
                    Some(_) => {
                        let value = visitor.visit_enum(Enum::new(self))?;
                        Ok(value)
                    }
                    None => Err(self.reader.parse_error(
                        ParseErrorType::InvalidValue,
                        "expected enum value, found {}",
                    )),
                }
            }
            _ => visitor.visit_enum(Enum::new(self)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.reader.read_identifier()?)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct ArrayValue<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> ArrayValue<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        ArrayValue { de }
    }
}
struct MapValue<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapValue<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        MapValue { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for ArrayValue<'a, 'de> {
    type Error = ParseError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let result = self.de.reader.next_array_value()?;
        if result.is_none() {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de, 'a> MapAccess<'de> for MapValue<'a, 'de> {
    type Error = ParseError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.de.reader.is_collection_ended()? {
            true => Ok(None),
            false => {
                let identifier = seed.deserialize(&mut *self.de)?;
                self.de.reader.expect_token(TokenType::Equals)?;
                Ok(Some(identifier))
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = ParseError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = ParseError;

    fn unit_variant(self) -> Result<()> {
        Err(self.de.reader.parse_error(
            ParseErrorType::UnexpectedTokenError,
            String::from("expected string"),
        ))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}
