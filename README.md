
# Income ZKP — Halo2

**Transparent Succinct Zero Knowledge Proof of Knowledge with Universal Setup**

Privacy-preserving income verification using [Halo2](https://zcash.github.io/halo2/) in Rust.
Proves salary exceeds a threshold **without revealing the actual salary — not even once.**

---

## What It Does

| Item | Detail |
|---|---|
| **Claim proved** | Salary > ₹50,000/month |
| **Salary revealed?** | Never |
| **Proof size** | 1248 bytes |
| **Verify time** | < 0.5 seconds |
| **Setup type** | Transparent — no trusted ceremony |
| **Framework** | Halo2 (halo2_proofs v0.2.0) |
| **Language** | Rust 1.94.0 |
| **Curve** | Pallas (Pasta family, 255-bit) |

---

## The Problem

Every time someone applies for a loan or scholarship in India, they hand over months of
salary slips, bank statements, and tax documents — full of personal information the
institution does not actually need.

The bank needs to know **one thing**: does the applicant earn above a minimum amount?

This project answers: can we prove that one fact — salary above a threshold —
while keeping everything else completely private?

---

## How It Works
Private inputs (on your device):     Public input (known to verifier):
salary = 75,000  ──────────┐          threshold = 50,000
diff   = 25,000  ──────────┤
▼
┌─────────────────────┐
│    Halo2 Circuit    │
│  sel × (salary −   │
│  threshold − diff)  │
│       = 0           │
└──────────┬──────────┘
▼
ZK Proof (1248 bytes)
│
▼
verify_proof()
→ VERIFIED ✓
→ Salary NEVER revealed



---

## Quick Start

### Prerequisite
- Rust 1.70+ → install from https://rustup.rs

### Run
```bash
git clone https://github.com/YOUR_USERNAME/income-zkp-halo2.git
cd income-zkp-halo2
cargo run
```

### Expected Output
============================================== Income ZKP — Halo2 Transparent Setup
Threshold (PUBLIC)  : Rs.50000
Salary    (PRIVATE) : *** HIDDEN ***
Diff      (PRIVATE) : *** HIDDEN ***

[1] Transparent universal params (no trusted setup)...
[2] Generating proving & verification keys...
[3] Generating ZK proof (salary never exposed)...
Proof size: 1248 bytes
[4] Verifying proof...

============================================== PROOF VERIFIED SUCCESSFULLY! Salary > Rs.50000 : CONFIRMED Actual salary : NEVER REVEALED Proof system : Halo2 (Transparent)


---

## Circuit Design

PLONKish arithmetisation — single gate constraint:

| Column | Type | Visibility | Value |
|---|---|---|---|
| `salary` | Advice | **Private** | 75,000 |
| `diff` | Advice | **Private** | 25,000 |
| `threshold` | Instance | **Public** | 50,000 |
| `sel` | Selector | Fixed | 1 at row 0 |

**Gate constraint:**
sel × ( salary − threshold − diff ) = 0



---

## Security Properties

| Property | Status | Basis |
|---|---|---|
| Completeness | ✅ Satisfied | Verified empirically across all test cases |
| Soundness | ✅ Satisfied | Discrete log hardness on Pallas curve (128-bit) |
| Zero-Knowledge | ✅ Satisfied | IPA hiding property + OsRng randomness |
| Post-Quantum | ❌ Future work | IPA vulnerable to Shor's algorithm |

---

## Performance (Intel Core i5, 8GB RAM, debug build)

| Stage | Time |
|---|---|
| Parameter generation | ~0.3 sec |
| Key generation | ~0.5 sec |
| Proof creation | ~2.8 sec |
| Proof verification | ~0.4 sec |
| **Total pipeline** | **~4.0 sec** |

---

## Comparison with Other ZK Systems

| System | Setup | Proof Size | Trusted Ceremony? | Universal? |
|---|---|---|---|---|
| Groth16 | Trusted (circuit-specific) | ~200 bytes | YES ❌ | No |
| PLONK | Trusted (universal) | ~1 KB | YES ❌ | Yes |
| **Halo2 IPA ★** | **Transparent** | **~1.2 KB** | **NO ✅** | **Yes** |
| STARKs | Transparent | ~100 KB | NO ✅ | Yes |

---

## Limitations & Future Work

| Limitation | Planned Fix |
|---|---|
| No range proof for `diff` | PLOOKUP gadget to enforce diff ≥ 0 |
| Self-asserted salary | DigiLocker + employer digital signature |
| Not post-quantum | Migrate to STARK-based commitment |
| Single threshold | Multi-constraint circuit for tiered income |

---

## DPDP Act 2023 Alignment

India's **Digital Personal Data Protection Act 2023, Section 4** mandates data minimisation.
Our system collects exactly **one bit** from the applicant:
*salary exceeds threshold — yes or no.*
No name. No exact salary. No employer. No transaction history.

---

## Dependencies

```toml
[dependencies]
halo2_proofs = "0.2.0"
rand = "0.8"
```

---

## Authors

| Name 
|---|---|
| Akshitha Kommineni |
| Thatathoti Laasya |

**Guide:** Dr. Mukkoti Maruthi Venkata Chalapathi
**School of Computer Science and Engineering, VIT-AP University, Amaravati — May 2026**

---

## License
MIT License — see [LICENSE](LICENSE) for details.
