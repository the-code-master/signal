# Module: services/plm-ops/src/plm_ops/repository.py — Tenant-Scoped PLM Store (BOM, ECO, MCN, PCN, Deviation & 8D)
# Invariants: Strict (org_slug, proj_slug) scoping; tracks live Deviation consumed_qty and expiry for SFTP gate.
# Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
from __future__ import annotations

from plm_ops.schemas import (
    BomLineDto,
    BomViewKind,
    DeviationWaiverView,
    EcoRecordView,
    McnRecordView,
    NcrCapaView,
    PcnImpactView,
    PlmChangeNoticeSummary,
)


class PlmTenantStore:
    def __init__(self) -> None:
        self._boms: dict[tuple[str, str, BomViewKind], list[BomLineDto]] = {}
        self._ecos: dict[tuple[str, str, str], EcoRecordView] = {}
        self._mcns: dict[tuple[str, str], list[McnRecordView]] = {}
        self._pcns: dict[tuple[str, str], list[PcnImpactView]] = {}
        self._deviations: dict[tuple[str, str], list[DeviationWaiverView]] = {}
        self._ncrs: dict[tuple[str, str], list[NcrCapaView]] = {}
        self._seed_defaults()

    def _seed_defaults(self) -> None:
        key_base = ("signal-design-inc", "aurora-ev-inverter")
        ebom_lines = [
            BomLineDto(
                line_id="LN-SIC-01",
                upid="urn:signal:part:wolfspeed:C3M0016120K",
                orderable_mpn="C3M0016120K",
                quantity_per_unit=6,
                on_hand_inventory_qty=3600,
                lead_time_days=28,
                unit_cost_micros_usd=18_500_000,
                embodied_co2e_mg=145_000,
                workstation_step_id="SMT-TOP-01",
            ),
            BomLineDto(
                line_id="LN-DRV-02",
                upid="urn:signal:part:ti:UCC21750QDWR",
                orderable_mpn="UCC21750QDWR",
                quantity_per_unit=6,
                on_hand_inventory_qty=2400,
                lead_time_days=21,
                unit_cost_micros_usd=4_200_000,
                embodied_co2e_mg=42_000,
                workstation_step_id="SMT-TOP-02",
            ),
            BomLineDto(
                line_id="LN-HSG-03",
                upid="urn:signal:mcad:aurora:COLDPLATE_ALSI10MG",
                orderable_mpn="AUR-HSG-AL-REV-A",
                quantity_per_unit=1,
                on_hand_inventory_qty=480,
                lead_time_days=14,
                unit_cost_micros_usd=34_000_000,
                embodied_co2e_mg=620_000,
                workstation_step_id="CELL-MECH-FINAL",
            ),
        ]
        for view in ("EBOM", "MBOM", "SBOM"):
            self._boms[(key_base[0], key_base[1], view)] = list(ebom_lines)

    def get_bom_lines(self, org_slug: str, proj_slug: str, view: BomViewKind) -> list[BomLineDto]:
        return list(self._boms.get((org_slug, proj_slug, view), []))

    def append_bom_line(self, org_slug: str, proj_slug: str, view: BomViewKind, line: BomLineDto) -> list[BomLineDto]:
        bucket = self._boms.setdefault((org_slug, proj_slug, view), [])
        bucket.append(line)
        return list(bucket)

    def save_eco(self, org_slug: str, proj_slug: str, record: EcoRecordView) -> EcoRecordView:
        self._ecos[(org_slug, proj_slug, record.change_id)] = record
        return record

    def get_eco(self, org_slug: str, proj_slug: str, change_id: str) -> EcoRecordView | None:
        return self._ecos.get((org_slug, proj_slug, change_id))

    def append_mcn(self, org_slug: str, proj_slug: str, rec: McnRecordView) -> McnRecordView:
        self._mcns.setdefault((org_slug, proj_slug), []).append(rec)
        return rec

    def append_pcn(self, org_slug: str, proj_slug: str, rec: PcnImpactView) -> PcnImpactView:
        self._pcns.setdefault((org_slug, proj_slug), []).append(rec)
        return rec

    def add_deviation(self, org_slug: str, proj_slug: str, dev: DeviationWaiverView) -> DeviationWaiverView:
        self._deviations.setdefault((org_slug, proj_slug), []).append(dev)
        return dev

    def list_deviations(self, org_slug: str, proj_slug: str) -> list[DeviationWaiverView]:
        return list(self._deviations.get((org_slug, proj_slug), []))

    def replace_deviations(self, org_slug: str, proj_slug: str, items: list[DeviationWaiverView]) -> None:
        self._deviations[(org_slug, proj_slug)] = items

    def append_ncr(self, org_slug: str, proj_slug: str, rec: NcrCapaView) -> NcrCapaView:
        self._ncrs.setdefault((org_slug, proj_slug), []).append(rec)
        return rec


PLM_STORE = PlmTenantStore()


async def fetch_plm_ops_record(org_slug: str) -> PlmChangeNoticeSummary:
    _ = org_slug
    return PlmChangeNoticeSummary(
        change_id="ECN-2026-104",
        project_slug="aurora-ev-inverter",
        kind="ECN",
        status="CCB_APPROVED",
        ctb_readiness_bps=9850,
    )
