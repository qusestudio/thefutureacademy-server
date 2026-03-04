-- Add migration script here
DO $$
    BEGIN
        IF NOT EXISTS (
            SELECT 1
            FROM pg_constraint
            WHERE conname = 'unique_student_month_year'
        ) THEN
            ALTER TABLE checkouts
                ADD CONSTRAINT unique_student_month_year
                    UNIQUE (student_id, month, year);
        END IF;
    END $$;