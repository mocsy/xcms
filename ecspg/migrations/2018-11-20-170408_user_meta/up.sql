CREATE TABLE user_meta (
  user_id INT8 NOT NULL REFERENCES users(id),
  display TEXT NOT NULL,
  fname TEXT NOT NULL,
  lname TEXT NOT NULL,
  email TEXT NOT NULL PRIMARY KEY,
  phone TEXT NOT NULL,
  frozen TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
