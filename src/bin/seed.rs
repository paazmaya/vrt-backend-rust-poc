use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use std::env;

mod schema;
use schema::{users, projects};

// Define your Diesel models (e.g., User and Project)

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let connection = establish_connection(&database_url);
    println!("Seeding default data...");

    match seed_default_data(&connection) {
        Ok(_) => println!("Seeding completed successfully."),
        Err(err) => eprintln!("Seeding failed: {:?}", err),
    }
}

fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn seed_default_data(conn: &PgConnection) -> Result<(), Error> {
    conn.transaction::<_, Error, _>(|| {
        seed_default_user(conn)?;
        seed_default_project(conn)?;
        Ok(())
    })
}

fn seed_default_user(conn: &PgConnection) -> Result<(), Error> {
    use schema::users::dsl::*;

    // Check if the user table is empty
    let user_count: i64 = users.count().get_result(conn)?;

    if user_count == 0 {
        // User table is empty, seed default user
        let default_email = "visual-regression-tracker@example.com";
        let default_password = "123456";

        let hashed_password = hash(default_password, DEFAULT_COST)?;

        diesel::insert_into(users)
            .values((
                email.eq(default_email),
                password.eq(hashed_password),
                // Add other fields here
            ))
            .execute(conn)?;

        println!("Default user seeded successfully.");
    } else {
        println!("User table is not empty. Skipping user seeding.");
    }

    Ok(())
}

fn seed_default_project(conn: &PgConnection) -> Result<(), Error> {
    use schema::projects::dsl::*;

    // Check if the project table is empty
    let project_count: i64 = projects.count().get_result(conn)?;

    if project_count == 0 {
        // Project table is empty, seed default project
        let default_project_name = "Default project";

        diesel::insert_into(projects)
            .values(name.eq(default_project_name))
            .execute(conn)?;

        println!("Default project seeded successfully.");
    } else {
        println!("Project table is not empty. Skipping project seeding.");
    }

    Ok(())
}
