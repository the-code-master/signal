# Module: ai/llm-gateway/src/llm_gateway/gbnf.py — Deterministic JSON-Schema to GBNF Grammar Compiler
# Invariants: Compiles Pydantic v2 JSON Schemas into strict Backus-Naur Form rules preventing malformed 7B JSON (F-054).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md
from __future__ import annotations

import re
from typing import Any, cast

_RULE_SANITIZE_RE = re.compile(r"[^a-zA-Z0-9-]")


def _rule_name(raw: str) -> str:
    cleaned = _RULE_SANITIZE_RE.sub("-", raw).strip("-").lower()
    return cleaned or "field"


def _as_str_dict(val: object) -> dict[str, object] | None:
    if isinstance(val, dict):
        return cast(dict[str, object], val)
    return None


def _compile_property_expr(prop_name: str, prop_schema: dict[str, object], extra_rules: list[str]) -> str:
    enum_raw = prop_schema.get("enum")
    if isinstance(enum_raw, list):
        enum_vals = cast(list[object], enum_raw)
        if enum_vals:
            alts = " | ".join(f'"\\"{v}\\""' for v in enum_vals)
            rname = f"{_rule_name(prop_name)}-enum"
            extra_rules.append(f"{rname} ::= {alts}")
            return rname

    any_of_raw = prop_schema.get("anyOf")
    if isinstance(any_of_raw, list):
        any_of_items = cast(list[object], any_of_raw)
        non_null: list[dict[str, object]] = []
        has_null = False
        for item in any_of_items:
            sub = _as_str_dict(item)
            if sub is not None:
                if sub.get("type") == "null":
                    has_null = True
                else:
                    non_null.append(sub)
        if non_null:
            base_expr = _compile_property_expr(prop_name, non_null[0], extra_rules)
            return f'({base_expr} | "null")' if has_null else base_expr

    ptype = prop_schema.get("type")
    if ptype == "string":
        return "json-string"
    if ptype == "integer":
        return "json-int"
    if ptype == "number":
        return "json-number"
    if ptype == "boolean":
        return "json-bool"
    if ptype == "array":
        items_dict = _as_str_dict(prop_schema.get("items"))
        item_expr = (
            _compile_property_expr(f"{prop_name}-item", items_dict, extra_rules)
            if items_dict is not None
            else "json-string"
        )
        rname = f"{_rule_name(prop_name)}-arr"
        extra_rules.append(f'{rname} ::= "[" ws ({item_expr} (ws "," ws {item_expr})*)? ws "]"')
        return rname
    return "json-string"


def compile_json_schema_to_gbnf(
    schema: dict[str, Any],
    allowed_tool_enum: tuple[str, ...] | None = None,
) -> str:
    props_dict = _as_str_dict(schema.get("properties")) or {}
    extra_rules: list[str] = []
    pair_segments: list[str] = []

    for key, raw_sub in props_dict.items():
        sub = _as_str_dict(raw_sub) or {}
        effective: dict[str, object] = dict(sub)
        if key == "selected_tool" and allowed_tool_enum:
            effective["enum"] = list(allowed_tool_enum)
        val_expr = _compile_property_expr(key, effective, extra_rules)
        pair_segments.append(f'"\\"{key}\\"" ws ":" ws {val_expr}')

    if pair_segments:
        joined = ' ws "," ws '.join(pair_segments)
        root_rule = f'root ::= "{{" ws {joined} ws "}}"'
    else:
        root_rule = 'root ::= "{" ws "}"'

    primitives = [
        root_rule,
        *extra_rules,
        'json-string ::= "\\"" ([^"\\\\\\x00-\\x1f] | "\\\\" ["\\\\/bfnrt])* "\\""',
        'json-int ::= "-"? ("0" | [1-9] [0-9]*)',
        'json-number ::= json-int ("." [0-9]+)? ([eE] [+-]? [0-9]+)?',
        'json-bool ::= "true" | "false"',
        "ws ::= [ \\t\\n]*",
    ]
    return "\n".join(primitives)
