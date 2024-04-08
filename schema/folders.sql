CREATE TABLE IF NOT EXISTS folders (
  id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name text NOT NULL,
  last_modified timestamp NOT NULL,
  starred boolean NOT NULL,
  fk_owner integer REFERENCES users(id) NOT NULL,
  fk_parent integer REFERENCES folders(id)
);
