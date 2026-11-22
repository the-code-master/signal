# Module: ai/orchestrator/src/orchestrator/__init__.py — Public Exports for Signal Neuro-Symbolic AI Orchestrator
# Invariants: Exposes CoW synthesis loop, Quiescent Design Guardian, and Merge-Time Proof Delta Re-Verifier.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md
from orchestrator.guardian import QuiescentDesignGuardian
from orchestrator.merge_verifier import verify_proof_bundle_at_merge
from orchestrator.schemas import (
    DelegatedGraphMutation,
    GuardianCheckRequest,
    GuardianCheckResponse,
    MergeProofCheckRequest,
    MergeProofCheckResponse,
    ProofBundleRecord,
    RagCitationPin,
    RustVerifierOutcome,
    SynthesisRunResponse,
    SynthesisSpecRequest,
)
from orchestrator.synthesis import ConeBudgetExceededError, NeuroSymbolicOrchestrator

__all__ = [
    "ConeBudgetExceededError",
    "DelegatedGraphMutation",
    "GuardianCheckRequest",
    "GuardianCheckResponse",
    "MergeProofCheckRequest",
    "MergeProofCheckResponse",
    "NeuroSymbolicOrchestrator",
    "ProofBundleRecord",
    "QuiescentDesignGuardian",
    "RagCitationPin",
    "RustVerifierOutcome",
    "SynthesisRunResponse",
    "SynthesisSpecRequest",
    "verify_proof_bundle_at_merge",
]
