
-- frozen: if not null, content is not editable, TEXT is a reason for being frozen
-- draft: if not null, connent is not viewable, but listable, TEXT is a reason for being in draft
-- created_at: creation date of the controlled content
-- last_update: when was the content last updated, redundant with version control
-- uuid: used to join to the controlled content
--  used by edity form to add resiliency against cross-saving content
--  used by permission module to grant explicit permissions
--  in case of projects it's used to join todos
-- id, created_at, uuid are semi-immutable
CREATE TABLE access_control (
  id SERIAL8 PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_by TEXT NOT NULL,
  frozen TEXT,
  draft TEXT,
  last_update TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_by TEXT NOT NULL
);

CREATE TABLE access_groups (
  id SERIAL8 PRIMARY KEY,
  name TEXT NOT NULL,
  tag JSONB,
  access_control_id INT8 NOT NULL REFERENCES access_control(id)
);

CREATE TABLE access_group_members (
  id SERIAL8 PRIMARY KEY,
  access_group_id INT8 NOT NULL REFERENCES access_groups(id),
  user_id INT8 NOT NULL REFERENCES users(id),
  access_control_id INT8 NOT NULL REFERENCES access_control(id)
);

CREATE TABLE access_rules (
  id SERIAL8 PRIMARY KEY,
  access_group_id INT8 NOT NULL REFERENCES access_groups(id),
  access_control_id INT8 NOT NULL REFERENCES access_control(id),
  access_type TEXT NOT NULL CHECK(access_type IN ('browse', 'read', 'edit', 'add', 'delete'))
);

-- Keys open vaults. They allow a user to gain per module permissions
-- A user with an "Project" key of type 'read', can read all projects regardless of team or expiry
-- Keys are issued to support, when they need to look at data they don't own
-- These automatically expire after an hour unless otherwise specified, but these can't be changed after issued
CREATE TABLE access_keys (
  id SERIAL8 PRIMARY KEY,
  key TEXT NOT NULL,
  access_type TEXT NOT NULL CHECK(access_type IN ('browse', 'read', 'edit', 'add', 'delete')),
  user_id INT8 NOT NULL REFERENCES users(id),
  reason TEXT NOT NULL,
  expiry TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '1 HOUR',
  tag JSONB,
  access_control_id INT8 NOT NULL REFERENCES access_control(id)
);

CREATE TABLE menus (
  id SERIAL8 PRIMARY KEY,
  title TEXT NOT NULL,
  links JSONB,
  tag JSONB,
  access_control_id INT8 NOT NULL REFERENCES access_control(id)
);
