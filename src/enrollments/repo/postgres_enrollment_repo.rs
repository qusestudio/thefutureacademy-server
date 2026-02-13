use crate::enrollments::models::enrollment::{Enrollment, EnrollmentNew, NotEnrolled, StudentEnrollment};
use crate::enrollments::repo::enrollment_repo::EnrollmentRepository;
use sqlx::{Error, PgPool};

pub struct PostgresEnrollmentRepo {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl EnrollmentRepository for PostgresEnrollmentRepo {
    async fn db_get_enrollment(&self, id: &str) -> sqlx::Result<Option<Enrollment>, Error> {
        let enrollment = sqlx::query_as("select * from enrollments where id = $1")
            .bind(id)
            .fetch_optional(&self.pg_pool)
            .await?;

        Ok(enrollment)
    }

    async fn db_get_enrollments_by_student(
        &self,
        student_id: &str,
    ) -> sqlx::Result<Vec<StudentEnrollment>, Error> {
        let enrollments = sqlx::query_as(
            "SELECT
                s.cognito_id AS student_id,
                s.name AS student_name,
                sub.id AS subject_id,
                sub.title AS subject_title,
                sub.grade,
                sub.term
            FROM
                students s
                    INNER JOIN
                enrollments e ON s.cognito_id = e.student_id
                    INNER JOIN
                subjects sub ON e.subject_id = sub.id
            WHERE
                s.cognito_id=$1;
            ",
        )
        .bind(student_id)
        .fetch_all(&self.pg_pool)
        .await?;
        Ok(enrollments)
    }

    async fn db_get_non_enrolled_subjects(
        &self,
        student_id: &str,
    ) -> sqlx::Result<Vec<NotEnrolled>, Error> {
        let non_enrollments = sqlx::query_as(
            "SELECT
                    sub.id AS subject_id,
                    sub.title AS subject_title,
                    sub.grade,
                    sub.term
                FROM
                    subjects sub
                        LEFT JOIN
                        enrollments e ON sub.id = e.subject_id AND e.student_id =$1
                WHERE
                    e.id IS NULL
            ",
        )
        .bind(student_id)
        .fetch_all(&self.pg_pool)
        .await?;
        Ok(non_enrollments)
    }

    async fn db_get_enrollments_by_subject(
        &self,
        subject_id: &str,
    ) -> sqlx::Result<Vec<Enrollment>, Error> {
        let enrollments = sqlx::query_as("SELECT * FROM enrollments WHERE subject_id = $1")
            .bind(subject_id)
            .fetch_all(&self.pg_pool)
            .await?;
        Ok(enrollments)
    }

    async fn db_create_enrollment(
        &self,
        enrollment_new: EnrollmentNew,
    ) -> sqlx::Result<Enrollment, Error> {
        let enrollment = Enrollment::new(enrollment_new);
        let enrollment = sqlx::query_as(
            "
            INSERT INTO enrollments (id, student_id, subject_id) 
            VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(enrollment.id)
        .bind(enrollment.student_id)
        .bind(enrollment.subject_id)
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(enrollment)
    }

    async fn db_get_enrollment_with_subject_student(
        &self,
        subject_id: &str,
        student_id: &str,
    ) -> sqlx::Result<Enrollment, Error> {
        let enrollment =
            sqlx::query_as("select * from enrollments where subject_id = $1 AND student_id = $2")
                .bind(subject_id)
                .bind(student_id)
                .fetch_one(&self.pg_pool)
                .await?;

        Ok(enrollment)
    }
}
