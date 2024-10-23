CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    github_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS user_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS packages (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    readme TEXT NOT NULL,
    description TEXT NOT NULL,
    -- A bool is technically a number, but 0 == false and 1 == true
    supports_forge BOOLEAN NOT NULL DEFAULT FALSE,
    supports_fabric BOOLEAN NOT NULL DEFAULT FALSE,
    supports_quilt BOOLEAN NOT NULL DEFAULT FALSE,
    supports_neoforge BOOLEAN NOT NULL DEFAULT FALSE,
    views INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS package_versions (
    id SERIAL PRIMARY KEY,
    package INTEGER NOT NULL REFERENCES packages(id),
    name TEXT NOT NULL,
    version_number TEXT NOT NULL,
    file_id TEXT NOT NULL, -- This is the UUID of the version's file.
    changelog TEXT,
    -- A comma-separated list of KubeJS versions that this version supports
    kubejs_versions TEXT NOT NULL,
    downloads INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_package_versions_updated_at BEFORE UPDATE
ON package_versions FOR EACH ROW EXECUTE PROCEDURE 
update_updated_at_column();

CREATE TRIGGER update_packages_updated_at BEFORE UPDATE
ON packages FOR EACH ROW EXECUTE PROCEDURE 
update_updated_at_column();

-- How to deal with diesel.rs shenanigains, part 1
CREATE TABLE IF NOT EXISTS package_version_refs (
    value INTEGER NOT NULL REFERENCES package_versions(id),
    PRIMARY KEY(value)
);

CREATE TABLE IF NOT EXISTS package_relations (
    package INTEGER NOT NULL REFERENCES package_versions(id),
    dependency INTEGER NOT NULL REFERENCES package_version_refs(value),
    -- The relation kind. 0 = dependency, 1 = incompatibility
    kind INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY(package, dependency, kind)
);

CREATE TABLE IF NOT EXISTS package_authors (
    package INTEGER NOT NULL REFERENCES packages(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    PRIMARY KEY(package, user_id)
);
