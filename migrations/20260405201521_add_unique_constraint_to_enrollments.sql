-- Add migration script here
DO $$
    BEGIN
        IF NOT EXISTS (
            SELECT 1
            FROM pg_constraint
            WHERE conname = 'unique_student_subject'
        ) THEN
            ALTER TABLE enrollments
                ADD CONSTRAINT unique_student_subject
                    UNIQUE (student_id, subject_id);
        END IF;
    END $$;