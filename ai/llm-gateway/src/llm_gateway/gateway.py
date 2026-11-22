# Module: ai/llm-gateway/src/llm_gateway/gateway.py — Unified Ollama / Apple MLX / deterministic_mock / vLLM Gateway
# Invariants: Auto-falls back to DETERMINISTIC_MOCK offline (F-143); scrubs telemetry spans; delegates math to Rust.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/09-local-development-environment.md
from __future__ import annotations

import re

from llm_gateway.finops import FinOpsCircuitBreaker
from llm_gateway.gbnf import compile_json_schema_to_gbnf
from llm_gateway.schemas import (
    GatewayCompletionRequest,
    GatewayCompletionResponse,
    LlmBackendKind,
    StructuredToolCall,
)
from llm_gateway.tool_router import select_dynamic_tool_subset

_SECRET_REDACT_RE = re.compile(r"(bearer\s+[A-Za-z0-9._\-]+|sk-[A-Za-z0-9_\-]+)", re.IGNORECASE)
_INJECTION_RE = re.compile(r"(ignore\s+previous\s+instructions|call\s+sftp|exfiltrate)", re.IGNORECASE)


def resolve_llm_backend(
    signal_env: str,
    local_ollama_available: bool,
    local_mlx_available: bool,
) -> LlmBackendKind:
    env = signal_env.strip().lower()
    if env in ("prod", "production"):
        return "ENTERPRISE_VLLM"
    if local_mlx_available:
        return "APPLE_MLX_LOCAL"
    if local_ollama_available:
        return "OLLAMA_LOCAL"
    return "DETERMINISTIC_MOCK"


def redact_telemetry_span(actor_hmac: str, backend: LlmBackendKind, prompt: str) -> str:
    scrubbed = _SECRET_REDACT_RE.sub("[REDACTED_SECRET]", prompt)
    preview = scrubbed[:48].replace("\n", " ")
    return f"span(actor={actor_hmac}, backend={backend}, prompt_len={len(prompt)}, preview='{preview}')"


class ModelAgnosticLlmGateway:
    def __init__(self, breaker: FinOpsCircuitBreaker | None = None) -> None:
        self._breaker = breaker or FinOpsCircuitBreaker()

    def complete_structured(self, req: GatewayCompletionRequest) -> GatewayCompletionResponse:
        self._breaker.check_and_record(
            org_id=req.org_id,
            actor_hmac=req.actor_hmac,
            tokens=req.estimated_tokens,
            gpu_ms=req.estimated_gpu_ms,
            now_ms=req.now_utc_ms,
        )
        backend = resolve_llm_backend(
            req.signal_env,
            req.local_ollama_available,
            req.local_mlx_available,
        )
        selected_tools = select_dynamic_tool_subset(
            tier=req.tier,
            active_workbench=req.active_workbench,
            prompt_text=req.prompt_text,
        )
        tool_names = tuple(t.tool_name for t in selected_tools)
        span = redact_telemetry_span(req.actor_hmac, backend, req.prompt_text)

        if req.tier == "ZERO_TOOL_QUARANTINE":
            quarantine_schema = {
                "type": "object",
                "properties": {"sanitized_fact": {"type": "string"}},
            }
            gbnf = compile_json_schema_to_gbnf(quarantine_schema)
            cleaned = _INJECTION_RE.sub("[QUARANTINED_DIRECTIVE]", req.prompt_text)
            return GatewayCompletionResponse(
                resolved_backend=backend,
                tier=req.tier,
                exposed_tool_names=(),
                gbnf_grammar=gbnf,
                structured_output=None,
                quarantine_sanitized_note=cleaned,
                redacted_telemetry_span=span,
            )

        gbnf = compile_json_schema_to_gbnf(
            StructuredToolCall.model_json_schema(),
            allowed_tool_enum=tool_names,
        )
        chosen_tool = tool_names[0] if tool_names else "run_spice_tr_bdf2_verify"
        structured = StructuredToolCall(
            selected_tool=chosen_tool,
            target_entity_id="C_SNUB_U1",
            delegated_rust_kernel="signal-diff-sim::saltation_adjoint",
            rationale_summary="Proposed topological target on CoW branch; numerical sizing delegated to Rust.",
        )
        return GatewayCompletionResponse(
            resolved_backend=backend,
            tier=req.tier,
            exposed_tool_names=tool_names,
            gbnf_grammar=gbnf,
            structured_output=structured,
            quarantine_sanitized_note=None,
            redacted_telemetry_span=span,
        )
