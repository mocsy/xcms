
CREATE UNLOGGED TABLE session_tokens (
  token TEXT PRIMARY KEY,
  claim VARCHAR NOT NULL
)
