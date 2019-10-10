#[derive(Queryable, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: Option<String>,
    pub preparation_time: i32,
    pub main_ingredient: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diesel::dsl::*;
    use crate::diesel::prelude::*;
    use crate::diesel::Connection;
    use diesel_migrations::run_pending_migrations;

    #[test]
    fn query_ingredients() {
        use crate::schema::ingredient::dsl::*;

        let conn =
            diesel::SqliteConnection::establish(":memory:").expect("Error connecting to database");

        run_pending_migrations(&conn).expect("Error running migrations");

        insert_into(ingredient)
            .values(name.eq("Test ingredient"))
            .execute(&conn)
            .expect("Error inserting ingredient");

        let _result = ingredient
            .limit(5)
            .load::<Ingredient>(&conn)
            .expect("Error loading ingredients");
    }

    #[test]
    fn query_recipes() {
        use crate::schema::recipe::dsl::*;

        let conn =
            diesel::SqliteConnection::establish(":memory:").expect("Error connecting to database");

        run_pending_migrations(&conn).expect("Error running migrations");

        insert_into(recipe)
            .values((
                name.eq("Test ingredient"),
                preparation_time.eq(20),
                main_ingredient.eq(0),
            ))
            .execute(&conn)
            .expect("Error inserting ingredient");

        let _result = recipe
            .limit(5)
            .load::<Recipe>(&conn)
            .expect("Error loading ingredients");
    }
}
