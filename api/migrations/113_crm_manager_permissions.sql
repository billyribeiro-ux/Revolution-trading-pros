-- CRM delegation never changes users.role or grants access to other admin APIs.
CREATE TABLE crm_manager_roles (
 id BIGSERIAL PRIMARY KEY,
 name TEXT NOT NULL CHECK(length(btrim(name)) BETWEEN 1 AND 100),
 slug TEXT NOT NULL UNIQUE CHECK(slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND length(slug)<=100),
 description TEXT CHECK(length(description)<=2000),
 permissions JSONB NOT NULL DEFAULT '[]'::jsonb CHECK(jsonb_typeof(permissions)='array' AND jsonb_array_length(permissions)<=100),
 is_default BOOLEAN NOT NULL DEFAULT false,
 version BIGINT NOT NULL DEFAULT 1 CHECK(version>0),
 created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
 updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
CREATE UNIQUE INDEX crm_manager_role_name ON crm_manager_roles(lower(btrim(name)));
CREATE UNIQUE INDEX crm_manager_one_default ON crm_manager_roles((true)) WHERE is_default;
INSERT INTO crm_manager_roles(name,slug,description,is_default)
 VALUES('Unconfigured CRM role','unconfigured-crm-role','No CRM permissions are granted until a global administrator explicitly configures this role.',true);

CREATE TABLE crm_managers (
 id BIGSERIAL PRIMARY KEY,
 user_id BIGINT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
 role_id BIGINT NOT NULL REFERENCES crm_manager_roles(id) ON DELETE RESTRICT,
 assigned_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
 assigned_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
 version BIGINT NOT NULL DEFAULT 1 CHECK(version>0),
 updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);
CREATE INDEX crm_managers_role ON crm_managers(role_id);
