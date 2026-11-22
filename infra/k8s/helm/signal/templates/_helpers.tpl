{{/*
Module: infra/k8s/helm/signal/templates/_helpers.tpl — Shared Helm Labels & Hardened Pod Security Helpers
Invariants: Enforces runAsNonRoot (65532), readOnlyRootFilesystem, seccomp RuntimeDefault, and drop ALL caps.
Spec: architecture/15-application-security-and-project-isolation.md
*/}}
{{- define "signal.labels" -}}
app.kubernetes.io/part-of: signal-platform
app.kubernetes.io/version: {{ .Values.global.platformVersion | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{- define "signal.containerSecurityContext" -}}
allowPrivilegeEscalation: false
readOnlyRootFilesystem: true
runAsNonRoot: true
runAsUser: 65532
runAsGroup: 65532
seccompProfile:
  type: RuntimeDefault
capabilities:
  drop:
    - ALL
{{- end -}}
