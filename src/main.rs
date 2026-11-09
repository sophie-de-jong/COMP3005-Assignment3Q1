use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use std::env;

// Define a Student struct to represent a record from the students table.
struct Student {
    // Integer, Primary Key, Auto-increment
    student_id: i32,
    // Text, Not Null
    first_name: String,
    // Text, Not Null
    last_name: String,
    // Text, Not Null, Unique
    email: String,
    // Date
    enrollment_date: Option<NaiveDate>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load environment variables from .env file. See .env file for the PostgreSQL database connection URL.
    dotenvy::dotenv().ok();

    // Retrieve the database connection URL from the environment, erroring if this variable is not set.
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    // Create a connection pool targeting the database.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Call get_all_students() to retrieve and display all student records
    // currently stored in the students table.
    get_all_students(&pool).await?;

    // Call add_student() to insert a new student into the database.
    let enrollment_date = NaiveDate::from_ymd_opt(2023, 9, 6).unwrap();
    add_student(&pool, "Sophie", "de Jong", "SophieDeJong@cmail.carleton.ca", enrollment_date).await?;

    // Call update_student_email() to update an existing student's email.
    update_student_email(&pool, 1, "johndoe67@gmail.com").await?;

    // Call delete_student() to remove the student with student_id 4
    // from the students table.
    delete_student(&pool, 13).await?;

    Ok(())
}

/// Retrieve and display all student records.
async fn get_all_students(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // Execute a SELECT query to retrieve every record from the students table.
    // query_as! maps each returned database row into a Student struct.
    // fetch_all() retrieves all matching records and stores them in a vector.
    let rows = sqlx::query_as!(Student, "SELECT * FROM students")
        .fetch_all(pool)
        .await?;

    // Loop through each Student record returned by the database.
    for s in rows {
        // Check whether the student has an enrollment date (since enrollment_date could be NULL).
        if let Some(date) = s.enrollment_date {
            println!(
                "[{}] {} {} ({}) enrolled in {date}",
                s.student_id, s.first_name, s.last_name, s.email
            );
        } else {
            println!(
                "[{}] {} {} ({})",
                s.student_id, s.first_name, s.last_name, s.email
            );
        }
    }
    Ok(())
}

/// Insert a new student record into the database.
async fn add_student(
    pool: &sqlx::PgPool,
    first_name: &str,
    last_name: &str,
    email: &str,
    enrollment_date: NaiveDate,
) -> Result<(), sqlx::Error> {
    // Execute an INSERT query to add a new student to the students table.
    // execute() runs the query against the database.
    sqlx::query!(
        "INSERT INTO students (first_name, last_name, email, enrollment_date) VALUES ($1, $2, $3, $4)", 
        first_name,
        last_name,
        email,
        enrollment_date
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Update the email address of an existing student.
async fn update_student_email(
    pool: &sqlx::PgPool,
    student_id: i32,
    new_email: &str,
) -> Result<(), sqlx::Error> {
    // Execute an UPDATE query to change the email address of the student matching the provided student_id.
    // The WHERE clause ensures that only the matching student's email address is modified.
    // execute() runs the query against the database.
    sqlx::query!(
        "UPDATE students SET email = $1 WHERE student_id = $2",
        new_email,
        student_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a student record from the database.
async fn delete_student(pool: &sqlx::PgPool, student_id: i32) -> Result<(), sqlx::Error> {
    // Execute a DELETE query to remove the student whose ID matches the student_id provided to this function.
    // The WHERE clause ensures that only the specified student is deleted rather than removing every record in the table.
    // execute() runs the query against the database.
    sqlx::query!("DELETE FROM students WHERE student_id = $1", student_id)
        .execute(pool)
        .await?;
    Ok(())
}
