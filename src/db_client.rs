use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, Mem},
    sql::Thing,
    Surreal,
};

pub struct SurrealDbHelperFuncs;
impl SurrealDbHelperFuncs {
    pub async fn surrealdb_init() -> surrealdb::Result<Surreal<Db>> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;
        Self::add_fake_data(&db).await?;
        Ok(db)
    }
    async fn add_fake_data(db: &Surreal<Db>) -> surrealdb::Result<()> {
        let _: Vec<Record> = db
            .create("person")
            .content(Person {
                title: "Founder & CEO",
                name: "Tobie Morgan Hitchcock",
                marketing: true,
            })
            .await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Person<'a> {
    title: &'a str,
    name: &'a str,
    marketing: bool,
}
#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

// async fn extract_row(State(DbHandler(db)): State<DbHandler>) -> String {
//     // eww
//     let response: Vec<Record> = db.select("person").await.unwrap();
//     let a = response.into_iter().take(1).next().unwrap();

//     format!("{a:?}")
// }
