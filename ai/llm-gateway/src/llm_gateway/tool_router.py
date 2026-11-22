# Module: ai/llm-gateway/src/llm_gateway/tool_router.py — Dynamic 5-8 Tool Subset Router & Quarantine Enforcer
# Invariants: ZERO_TOOL_QUARANTINE receives 0 tools; PRIVILEGED_PLANNER receives 5..8 safe EDA tools, zero SFTP/admin tools.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/15-*.md
from __future__ import annotations

from llm_gateway.schemas import GatewayTierKind, ToolDefinition

FORBIDDEN_TOOL_PREFIXES: tuple[str, ...] = ("sftp_", "vcs_credential_", "admin_break_glass_")

DEFAULT_AGENT_TOOL_CATALOG: tuple[ToolDefinition, ...] = (
    ToolDefinition(
        tool_name="create_cow_speculative_branch",
        workbench_domain="schematic",
        keywords=("branch", "speculative", "propose", "cow"),
        description="Creates an isolated Copy-on-Write speculative branch in signal-graph.",
        parameter_types={"branch_name": "string"},
    ),
    ToolDefinition(
        tool_name="query_sharded_datasheet_rag",
        workbench_domain="library-studio",
        keywords=("datasheet", "derating", "errata", "mpn", "curve", "snubber"),
        description="Queries 3-tier sharded pgvector RAG with bbox provenance.",
        parameter_types={"mpn": "string", "query": "string"},
    ),
    ToolDefinition(
        tool_name="run_spice_tr_bdf2_verify",
        workbench_domain="sim-viewer",
        keywords=("spice", "transient", "ripple", "simulate", "overshoot", "ringing"),
        description="Delegates circuit operating-point and transient verification to signal-sim.",
        parameter_types={"branch_id": "string"},
    ),
    ToolDefinition(
        tool_name="run_saltation_adjoint_sizing",
        workbench_domain="sim-viewer",
        keywords=("optimize", "snubber", "inductor", "compensate", "gradient", "pareto"),
        description="Delegates exact component sizing to Rust signal-diff-sim Saltation-Adjoint.",
        parameter_types={"entity_id": "string", "target_metric": "string"},
    ),
    ToolDefinition(
        tool_name="verify_drc_creepage_constraints",
        workbench_domain="pcb",
        keywords=("drc", "creepage", "clearance", "layout", "trace", "via"),
        description="Runs exact i128 DRC and IEC 60664-1 3D creepage verification.",
        parameter_types={"branch_id": "string"},
    ),
    ToolDefinition(
        tool_name="prove_formal_ic3_invariants",
        workbench_domain="review",
        keywords=("formal", "deadlock", "cdc", "equivalence", "prove", "audit"),
        description="Runs SAT/SMT/IC3 formal property verification in signal-formal.",
        parameter_types={"branch_id": "string"},
    ),
    ToolDefinition(
        tool_name="explain_schematic_stages",
        workbench_domain="schematic",
        keywords=("explain", "transfer", "stage", "filter", "gain"),
        description="Extracts VF2 functional blocks and symbolic H(s) transfer cards.",
        parameter_types={"sheet_id": "string"},
    ),
    ToolDefinition(
        tool_name="consolidate_bom_feeder_slots",
        workbench_domain="specs",
        keywords=("bom", "feeder", "consolidate", "passive", "smt"),
        description="Minimizes SMT feeder slots with Saltation zero-shift verification.",
        parameter_types={"branch_id": "string"},
    ),
    ToolDefinition(
        tool_name="sculpt_parametric_padstack",
        workbench_domain="library-studio",
        keywords=("padstack", "ipc7351", "step", "backdrill", "footprint"),
        description="Generates parametric PadstackDef and verifies 3D STEP overlap.",
        parameter_types={"package_family": "string"},
    ),
    ToolDefinition(
        tool_name="compile_nl_design_constraint",
        workbench_domain="pcb",
        keywords=("constraint", "rule", "impedance", "differential", "upf"),
        description="Compiles natural-language intent into deterministic DRC/UPF/SDC AST.",
        parameter_types={"prompt": "string"},
    ),
    ToolDefinition(
        tool_name="sftp_factory_egress_forbidden",
        workbench_domain="sftp-release",
        keywords=("sftp", "release", "factory"),
        description="Forbidden in AI tier: requires direct human Step-Up 2FA.",
        parameter_types={"release_id": "string"},
        forbidden_in_agent=True,
    ),
)


def is_tool_permitted_for_agent(tool: ToolDefinition) -> bool:
    if tool.forbidden_in_agent:
        return False
    return not any(tool.tool_name.startswith(pref) for pref in FORBIDDEN_TOOL_PREFIXES)


def select_dynamic_tool_subset(
    tier: GatewayTierKind,
    active_workbench: str,
    prompt_text: str,
    catalog: tuple[ToolDefinition, ...] = DEFAULT_AGENT_TOOL_CATALOG,
    min_tools: int = 5,
    max_tools: int = 8,
) -> tuple[ToolDefinition, ...]:
    if tier == "ZERO_TOOL_QUARANTINE":
        return ()

    safe_tools = [t for t in catalog if is_tool_permitted_for_agent(t)]
    lowered_prompt = prompt_text.lower()
    wb = active_workbench.strip().lower()

    def _score(tool: ToolDefinition) -> tuple[int, str]:
        pts = 0
        if tool.workbench_domain == wb:
            pts += 20
        for kw in tool.keywords:
            if kw in lowered_prompt:
                pts += 10
        if tool.tool_name in ("create_cow_speculative_branch", "run_spice_tr_bdf2_verify"):
            pts += 5
        return (pts, tool.tool_name)

    ranked = sorted(safe_tools, key=lambda t: (-_score(t)[0], t.tool_name))
    cap = max(min_tools, min(max_tools, len(ranked)))
    return tuple(ranked[:cap])
