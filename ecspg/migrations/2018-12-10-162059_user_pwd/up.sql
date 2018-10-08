CREATE TABLE user_pwd (
  id SERIAL8 PRIMARY KEY,
  user_id INT8 NOT NULL REFERENCES users(id),
  pw_hash TEXT NOT NULL, 
  frozen TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
