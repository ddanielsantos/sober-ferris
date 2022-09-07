// use postgis::ewkb::Point;
// use postgis_diesel::PointC;
use rocket::serde::Serialize;
use diesel::{prelude::*, deserialize::FromSqlRow, pg::Pg};
use crate::{schema::partner::dsl, types::{MyPoint, MyPointC}};

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Partner {
    pub id: String,
    pub trading_name: String,
    pub owner_name: String,
    pub document: String,
    pub address: MyPointC<MyPoint>
}

impl FromSqlRow<Partner, Pg> for Partner {
    fn build_from_row<'a>(row: &impl diesel::row::Row<'a, Pg>) -> diesel::deserialize::Result<Self> {
        todo!()
    }
}

impl Partner {
    pub fn find_one(id: &str, conn: &mut PgConnection) -> Partner {
        let res = dsl::partner
            .find(id)
            .first::<Partner>(conn)
            .unwrap();

        res


        // this works

        // let o = Point { x: 2., y: 4., srid: Some(4236) };
        // let point: MyPointC<MyPoint> = MyPointC(PointC { v: MyPoint(o) });
        //
        // let p = Partner {
        //     id: "2".into(),
        //     owner_name: "aaaaaa".into(),
        //     document: "23435".into(),
        //     trading_name: "dadinho".into(),
        //     address: point
        // };
        //
        // p
    }
}
