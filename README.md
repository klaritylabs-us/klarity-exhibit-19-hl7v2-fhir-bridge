# HL7 v2 ADT → FHIR Patient bridge — Rust

Small library and CLI that sanity-check **FHIR Patient** JSON produced from **ADT (A04)**-style bridges before ADT feeds land in your FHIR store: resource type and at least one **`identifier`** — the PID mapping most v2→FHIR pipelines must preserve.

Synthetic fixtures only. Not ONC certification, not a substitute for IHE or v2 conformance tools, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 19** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [HL7 FHIR R4](http://hl7.org/fhir/R4/) (Patient mapping context)

## What it checks

- `resourceType` is **`Patient`**
- **`identifier`** array present and non-empty (v2 PID → FHIR identifier)

| Problem | What breaks |
|--------|-------------|
| **Patient without identifiers** | Duplicate patients on every ADT update |
| **Wrong resource type** | Bridge drops message or writes to wrong compartment |
| **Empty identifier list** | MPI cannot match; downstream orders orphan |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-bridge -- examples/valid-adt-patient.json
cargo run --bin validate-bridge -- examples/invalid-bridge.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-bridge -- examples/valid-adt-patient.json
```

**Pass** (`examples/valid-adt-patient.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-bridge.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "identifier",
    "detail": "v2 PID segment should map to identifier",
    "rule_id": "pid-identifier"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken bridge Patient payloads for tests and demos

## Inferno vs this repo

Inferno focuses on **FHIR REST** patient access, not v2 feeds. This repo is **bridge output shape** — the Patient resource your ADT listener writes before MPI or US Core profiles apply.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
