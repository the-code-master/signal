# Module: justfile — Thin Ergonomic Aliases Over Canonical ./signal.sh
# Invariants: Zero business logic in justfile; 100% 1-line delegation to ./signal.sh (20-*).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md

default:
    ./signal.sh doctor

doctor *ARGS:
    ./signal.sh doctor {{ARGS}}

dev MODE="lite" *ARGS:
    ./signal.sh dev {{MODE}} {{ARGS}}

build *ARGS:
    ./signal.sh build {{ARGS}}

check *ARGS:
    ./signal.sh check {{ARGS}}

image *ARGS:
    ./signal.sh image {{ARGS}}

release ACTION="preview" *ARGS:
    ./signal.sh release {{ACTION}} {{ARGS}}

deploy *ARGS:
    ./signal.sh deploy {{ARGS}}

rollback *ARGS:
    ./signal.sh rollback {{ARGS}}
