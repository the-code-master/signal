-- Module: services/common/db/migrations/20261121214000_cursor_init_rls_and_quorum_outbox.sql
-- Invariants: Expand-and-Contract zero-downtime SQL; NOBYPASSRLS runtime role; FORCE ROW LEVEL SECURITY; zero raw PII in audit table.
-- Spec: architecture/15-application-security-and-project-isolation.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'signal_app_rls') THEN
        CREATE ROLE signal_app_rls LOGIN NOBYPASSRLS NOSUPERUSER NOCREATEDB NOCREATEROLE;
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS projects (
    project_id UUID PRIMARY KEY,
    org_id UUID NOT NULL,
    slug TEXT NOT NULL,
    meta_provider_repo_id TEXT NOT NULL,
    fencing_epoch BIGINT NOT NULL DEFAULT 1,
    encryption_mode TEXT NOT NULL CHECK (encryption_mode IN ('ENCRYPTED_VAULT', 'PLAIN_CANONICAL')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (org_id, slug)
);

ALTER TABLE projects ENABLE ROW LEVEL SECURITY;
ALTER TABLE projects FORCE ROW LEVEL SECURITY;

CREATE POLICY projects_tenant_isolation ON projects
    USING (org_id::text = current_setting('app.org_id', true))
    WITH CHECK (org_id::text = current_setting('app.org_id', true));

CREATE TABLE IF NOT EXISTS tier0_audit_outbox (
    outbox_seq BIGSERIAL PRIMARY KEY,
    org_id UUID NOT NULL,
    actor_hmac TEXT NOT NULL CHECK (position('@' in actor_hmac) = 0),
    action_kind TEXT NOT NULL,
    prev_merkle_hash CHAR(64) NOT NULL,
    entry_merkle_hash CHAR(64) NOT NULL,
    quorum_acks SMALLINT NOT NULL CHECK (quorum_acks >= 3),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
