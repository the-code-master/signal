# Module: ai/llm-gateway/src/llm_gateway/__init__.py — Public Exports for Signal Model-Agnostic LLM Gateway
# Invariants: Exposes GBNF compiler, dynamic 5-8 tool selector, FinOps circuit-breaker, and unified gateway.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md
from llm_gateway.finops import FinOpsCircuitBreaker, FinOpsQuotaExceededError
from llm_gateway.gateway import ModelAgnosticLlmGateway, resolve_llm_backend
from llm_gateway.gbnf import compile_json_schema_to_gbnf
from llm_gateway.schemas import (
    FinOpsQuotaPolicy,
    GatewayCompletionRequest,
    GatewayCompletionResponse,
    LlmBackendKind,
    StructuredToolCall,
    ToolDefinition,
)
from llm_gateway.tool_router import select_dynamic_tool_subset

__all__ = [
    "FinOpsCircuitBreaker",
    "FinOpsQuotaExceededError",
    "FinOpsQuotaPolicy",
    "GatewayCompletionRequest",
    "GatewayCompletionResponse",
    "LlmBackendKind",
    "ModelAgnosticLlmGateway",
    "StructuredToolCall",
    "ToolDefinition",
    "compile_json_schema_to_gbnf",
    "resolve_llm_backend",
    "select_dynamic_tool_subset",
]
