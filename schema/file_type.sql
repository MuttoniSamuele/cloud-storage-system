DO
$$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'file_type') THEN
        CREATE TYPE file_type AS ENUM ('Text', 'Image');
    END IF;
END
$$;
