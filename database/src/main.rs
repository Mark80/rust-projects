use tokio_postgres::{NoTls, Error as TokioError};
use tokio_postgres::types::{FromSql, Type};
use std::error::Error;
//use create

struct Vendor {
    id : i64,
    name : String
}

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), TokioError> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=password dbname=rust-db", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT * FROM VENDORS", &[])
        .await?;

    // And then check that we got back the same string we sent over.
    for row in rows {

        let id : i32 = row.get(1);

        println!("{}",id);
        println!("{:?}",row)

    };
    //assert_eq!(value, "Microsoft");

    Ok(())
}

// impl<'a> FromSql<'a> for u64 {
//     fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
//         unimplemented!()
//     }
//
//     fn accepts(ty: &Type) -> bool {
//         unimplemented!()
//     }
// }