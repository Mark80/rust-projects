
use std::result;
use serde::{Serialize, Deserialize, Serializer};
use serde::ser::SerializeSeq;

struct Error {
    cause : String
}

pub type Either<T> = result::Result<T, Error>;

struct CSerializer{

}

impl Serializer for CSerializer{
    type Ok = ();
    type Error = ();
    type SerializeSeq = ();
    type SerializeTuple = ();
    type SerializeTupleStruct = ();
    type SerializeTupleVariant = ();
    type SerializeMap = ();
    type SerializeStruct = ();
    type SerializeStructVariant = ();

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

mod custom_serializer {

    use super::*;

    fn to_custom_format(value : &T) -> Either<String>
    where value : ?Size + Serialize,
    {



    }
}

// impl<T> Serialize for Vec<T>
//     where
//         T: Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         let mut seq: <S as Serializer>::SerializeSeq = serializer.serialize_seq(Some(self.len()))?;
//         for e in self {
//             seq.serialize_element(e)?;
//         }
//         seq.end()
//     }
// }
