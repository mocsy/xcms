-- There are three roles related to ecs:
-- The super user role used during installation and setup. It should not be used later on.
-- ecs_read: The read only role, used for content serving this should be present on DB replicas as well.
-- ecs_write: This role is used for content creation and administering.
-- Tip: generate long passwords: "openssl rand -hex 32"

-- CREATE USER ecs_read WITH PASSWORD '498e8f8206c412ed5f8fade4707876705ff173240aff9075a218a059297a1fdc';
-- CREATE USER ecs_write WITH PASSWORD '8620a2fee5e4d3c542dfb101a270c0430cbe824fd2addcdabbd0687d63f0682c';

GRANT SELECT ON users TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON users TO ecs_write;

GRANT SELECT ON user_meta TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON user_meta TO ecs_write;

GRANT SELECT ON user_pwd TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON user_pwd TO ecs_write;

GRANT SELECT(id, created_at, created_by), INSERT(id, created_at, created_by) ON access_control TO ecs_write;
GRANT SELECT(frozen,draft,last_update,updated_by), INSERT(frozen,draft,last_update,updated_by), UPDATE(frozen,draft,last_update,updated_by), REFERENCES(frozen,draft,last_update,updated_by) ON access_control TO ecs_write;
GRANT SELECT ON access_control TO ecs_read;

GRANT SELECT ON access_groups TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON access_groups TO ecs_write;

GRANT SELECT ON access_group_members TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON access_group_members TO ecs_write;

GRANT SELECT ON access_rules TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON access_rules TO ecs_write;

GRANT SELECT ON access_keys TO ecs_read;
GRANT SELECT, INSERT ON access_keys TO ecs_write;

CREATE TABLE teams (
  id SERIAL8 PRIMARY KEY,
  access_control_id INT8 NOT NULL REFERENCES access_control(id),
  user_id INT8 NOT NULL REFERENCES users(id),
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  billing_name TEXT NOT NULL,
  billing_address TEXT NOT NULL,
  billing_city TEXT NOT NULL,
  billing_country TEXT NOT NULL,
  billing_zip TEXT NOT NULL
);
GRANT SELECT ON teams TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON teams TO ecs_write;

-- this is the only table with uuid id
CREATE TABLE projects (
  projectid INT8 NOT NULL,
  team_id INT8 NOT NULL,
  -- team_id INT8 NOT NULL REFERENCES teams(id),
  uuid UUID NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  start_date TIMESTAMPTZ,
  end_date TIMESTAMPTZ
);
GRANT SELECT ON projects TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON projects TO ecs_write;

CREATE TABLE todos (
  id SERIAL8 PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  email TEXT,
  phone TEXT,
  project_id UUID NOT NULL REFERENCES projects(uuid),
  completed BOOLEAN NOT NULL,
  completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
GRANT SELECT ON todos TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON todos TO ecs_write;

CREATE TABLE api_keys (
  id SERIAL8 PRIMARY KEY,
  team_id INT8 NOT NULL REFERENCES teams(id),
  api_key TEXT NOT NULL,
  access_control_id INT8 NOT NULL REFERENCES access_control(id)
);
GRANT SELECT ON api_keys TO ecs_read;
GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON api_keys TO ecs_write;

-- this grants permisison on sequences, not tables
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO ecs_write;
