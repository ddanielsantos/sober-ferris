use postgis::ewkb::Point;
use postgis_diesel::PointC;
use rocket::serde::{Serialize, ser::SerializeStruct};

pub struct MyPoint (pub Point);

pub struct MyPointC<T> (pub PointC<T>);

impl Serialize for MyPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: rocket::serde::Serializer {
        let mut point = serializer.serialize_struct("MyPoint", 3)?;
        point.serialize_field("srid", &self.0.srid);
        point.serialize_field("x", &self.0.x)?;
        point.serialize_field("y", &self.0.y)?;
        point.end()
    }
}

impl Serialize for MyPointC<MyPoint> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: rocket::serde::Serializer {
        let mut mp = serializer.serialize_struct("MyPointC", 1)?;
        mp.serialize_field("0", &self.0.v)?;
        mp.end()
    }
}
