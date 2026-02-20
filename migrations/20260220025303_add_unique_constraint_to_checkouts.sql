-- Add migration script here
ALTER TABLE checkouts
ADD CONSTRAINT unique_student_month_year UNIQUE (student_id, month, year);