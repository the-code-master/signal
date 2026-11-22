# Module: services/common/db/__init__.py — PostgreSQL 19+ NOBYPASSRLS Transaction Helpers
# Invariants: Every tenant query executes inside with_rls_tx setting app.org_id & app.actor_hmac locally.
# Spec: architecture/15-application-security-and-project-isolation.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
