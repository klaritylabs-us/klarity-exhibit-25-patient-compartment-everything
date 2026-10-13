# Patient compartment / `$everything` Bundle — Rust

Small library and CLI that sanity-check **Bundle** JSON meant as **`$everything`** output: **fullUrl** on entries and **patient compartment** subject links.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 25** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [Patient compartment](http://hl7.org/fhir/R4/compartmentdefinition-patient.html)
- [`Patient/$everything`](http://hl7.org/fhir/R4/operation-patient-everything.html)

## What it checks

- **Bundle** with **entry[]**
- **fullUrl** on each entry
- Warning for resources outside typical compartment
- Error when **Observation.subject** points outside bundle patient

| Problem | What breaks |
|--------|-------------|
| **Missing fullUrl** | Client cannot resolve references |
| **Foreign subject** | Patient Access scope leak |
| **Empty bundle** | App shows shell with no data |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-everything-bundle -- examples/valid-everything-bundle.json
cargo run --bin validate-everything-bundle -- examples/invalid-everything-bundle.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or input.

## Sample output

```bash
cargo run --bin validate-everything-bundle -- examples/valid-everything-bundle.json
```

**Pass** — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-everything-bundle.json`) — exit code `1`:

```json
[
  {
    "severity": "error",
    "path": "entry[0]",
    "rule_id": "..."
  }
]
```

## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken fixtures for tests and demos

## GitHub topics

Paste in repository **About** (also: `fhir`, `hl7-fhir`, `healthcare-interoperability`, `rust`):

`patient-access, patient-compartment, fhir-bundle, cms-0057-f, fhir, hl7-fhir, healthcare-interoperability, rust, fhir-r4`

## Inferno vs this repo

Inferno exercises **$everything** on live APIs. This repo is the **bundle graph** your exporter builds.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
