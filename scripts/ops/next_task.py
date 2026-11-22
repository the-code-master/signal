# Module: scripts/ops/next_task.py — Automated Multi-IDE Ledger Parser & Next-Task Coordinator
# Invariants: Enforces Phase Contract Freeze (`C-P<N>-*` before `AG-P<N>-*` / `CU-P<N>-*`); stdlib-only Python.
# Spec: architecture/12-multi-ide-collaboration.md, plan/README.md
from __future__ import annotations

import re
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output

LEDGER_MAP: dict[str, str] = {
    "contracts": "plan/TODO-LEDGER-SHARED-CONTRACTS.md",
    "ag": "plan/TODO-LEDGER-ANTIGRAVITY-LANE-A.md",
    "cursor": "plan/TODO-LEDGER-CURSOR-LANE-B.md",
}

TASK_LINE_RE = re.compile(
    r"^- \[(?P<done>[ xX])\] \*\*`(?P<id>[A-Z]+-P(?P<phase>\d+)-\d+)`\*\* \| "
    r"\*\*Branch:\*\* `(?P<branch>[^`]+)` \| "
    r"\*\*Paths:\*\* (?P<paths>[^|]+)\| "
    r"\*\*Skills:\*\* (?P<skills>[^|]+)\| "
    r"\*\*Deliverable:\*\* (?P<deliverable>.+?)\| "
    r"\*\*Verify:\*\* (?P<verify>.+)$"
)


@dataclass(frozen=True)
class LedgerTask:
    task_id: str
    phase: int
    done: bool
    branch: str
    paths: str
    skills: str
    deliverable: str
    verify: str


def parse_ledger(path: Path) -> list[LedgerTask]:
    tasks: list[LedgerTask] = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        m = TASK_LINE_RE.match(raw.strip())
        if not m:
            continue
        tasks.append(
            LedgerTask(
                task_id=m.group("id"),
                phase=int(m.group("phase")),
                done=m.group("done").lower() == "x",
                branch=m.group("branch").strip(),
                paths=m.group("paths").strip(),
                skills=m.group("skills").strip(),
                deliverable=m.group("deliverable").strip(),
                verify=m.group("verify").strip(),
            )
        )
    return tasks


def run_next_task(reg: RepoRegistry, *, lane: str, as_json: bool = False) -> int:
    if lane not in LEDGER_MAP:
        emit_output(
            {"command": "next", "status": "FAILED", "summary": f"Invalid lane `{lane}`", "details": []},
            as_json=as_json,
        )
        return 2

    contract_tasks = parse_ledger(reg.repo_root / LEDGER_MAP["contracts"])
    lane_tasks = parse_ledger(reg.repo_root / LEDGER_MAP[lane])
    pending = [t for t in lane_tasks if not t.done]

    if not pending:
        emit_output(
            {
                "command": "next",
                "lane": lane,
                "status": "COMPLETE",
                "summary": f"All tasks in `{LEDGER_MAP[lane]}` are completed!",
                "details": [],
            },
            as_json=as_json,
        )
        return 0

    nxt = pending[0]
    blocking_contracts = [c.task_id for c in contract_tasks if c.phase <= nxt.phase and not c.done and lane != "contracts"]
    blocked = len(blocking_contracts) > 0

    details = [
        f"Task ID: {nxt.task_id} (Phase {nxt.phase})",
        f"Branch: {nxt.branch}",
        f"Disjoint Paths: {nxt.paths}",
        f"Required Skills: {nxt.skills}",
        f"Verify Gate: {nxt.verify}",
    ]
    if blocked:
        details.insert(0, f"BLOCKED_ON_CONTRACTS: Merge {', '.join(blocking_contracts)} to `main` FIRST!")

    payload: dict[str, Any] = {
        "command": "next",
        "lane": lane,
        "status": "BLOCKED_ON_CONTRACTS" if blocked else "READY",
        "blocking_contracts": blocking_contracts,
        "next_task": asdict(nxt),
        "summary": (
            f"Lane `{lane}` next task `{nxt.task_id}` is blocked on {', '.join(blocking_contracts)}"
            if blocked
            else f"Lane `{lane}` ready to execute `{nxt.task_id}` on `{nxt.branch}`"
        ),
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0
