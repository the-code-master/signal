# Module: ai/llm-gateway/src/llm_gateway/schemas.py — Model-Agnostic LLM Gateway, GBNF & FinOps Schemas
# Invariants: StrictSchema (extra="forbid", frozen=True); SecretStr API keys; ZERO_TOOL_QUARANTINE forbids tools (F-090).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-*.md
from __future__ import annotations

from typing import Literal

from pydantic import SecretStr

from services.common.schemas import Int64Str, StrictSchema

LlmBackendKind = Literal["OLLAMA_LOCAL", "APPLE_MLX_LOCAL", "DETERMINISTIC_MOCK", "ENTERPRISE_VLLM"]
GatewayTierKind = Literal["ZERO_TOOL_QUARANTINE", "PRIVILEGED_PLANNER"]


class ToolDefinition(StrictSchema):
    tool_name: str
    workbench_domain: str
    keywords: tuple[str, ...]
    description: str
    parameter_types: dict[str, str]
    forbidden_in_agent: bool = False


class FinOpsQuotaPolicy(StrictSchema):
    user_max_tokens_per_min: int = 32_000
    org_max_tokens_per_min: int = 250_000
    user_max_gpu_ms_per_min: int = 15_000
    org_max_gpu_ms_per_min: int = 120_000
    window_ms: int = 60_000


class GatewayCompletionRequest(StrictSchema):
    org_id: str
    actor_hmac: str
    tier: GatewayTierKind
    active_workbench: str
    prompt_text: str
    estimated_tokens: int
    estimated_gpu_ms: int
    now_utc_ms: Int64Str
    signal_env: str = "local"
    local_ollama_available: bool = False
    local_mlx_available: bool = False
    provider_api_key: SecretStr | None = None


class StructuredToolCall(StrictSchema):
    selected_tool: str
    target_entity_id: str
    delegated_rust_kernel: str
    rationale_summary: str


class GatewayCompletionResponse(StrictSchema):
    resolved_backend: LlmBackendKind
    tier: GatewayTierKind
    exposed_tool_names: tuple[str, ...]
    gbnf_grammar: str
    structured_output: StructuredToolCall | None
    quarantine_sanitized_note: str | None
    redacted_telemetry_span: str
