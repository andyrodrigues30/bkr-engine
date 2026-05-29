## 1. Overview

> Copied from [blue-mvp](https://github.com/andyrodrigues30/blue-mvp). Needs to be updated.

B.L.U.E. is a decentralised educational knowledge system built on the AT Protocol.

It separates knowledge into four independent layers:
1. Identity (who created content)
2. Integrity (is content authentic and untampered)
3. Trust (how reliable knowledge is)
4. Safety (can content be distributed or shown)

Post-MVP, Guides may include:
- text
- images
- video
- code
- mixed media instructional content

Each system operates independently to prevent any single layer from controlling truth, authenticity, or distribution.

Systems:
- [Identity Systems](#3-identity-system-at-protocol-layer)
- [Integrity Systems](#4-integrity-system-structural-authenticity-layer)
- [Trust Vector Systems](#5-trust-vector-system-epistemic-reliability-layer)
- [Review System](#6-review-system-signal-generation-layer)
- [Safety Layer](#7-safety-layer-policy--distribution-control-system)


## 5 Trust Vector System (Epistemic Reliability Layer)
AKA Multi-dimensional Trust model

The table below provide the trust outputs for each dimension:
| Trust Dimension | High Indicator | Medium Indicator | Low / Warning Indicator |
| --------------- | -------------- | ---------------- | ----------------------- |
| 1. Provenance | Provenance verified<br>Contribution history stable | Provenance partially established | Provenance weak or incomplete |
| 2. Citation Quality | Evidence strongly supported<br>High-quality citations | Citation quality mixed | Weak supporting evidence<br>Heavy reliance on secondary sources |
| 3. Reproducibility | Independently reproducible | Partial reproducibility confirmed | Reproducibility unverified |
| 4. Reviewer Reliability | Reviewer reliability strong | Reviewer reliability developing | Reviewer reliability uncertain |
|  Consensus Stability | Strong consensus stability | Moderate disagreement present | Significant unresolved disagreement |
| 6. Educational Effectiveness | Educational clarity strong | Advanced knowledge assumed | Learner comprehension inconsistent |
| 7. Cross-Network Agreement | Cross-network agreement strong | External validation partially aligned | External systems diverge |
| 8. Challenge Resistance | Survived extensive review | Some challenges remain unresolved | Criticisms remain unresolved |
| 9.Temporal Reliability | Stable over time | Topic evolving gradually | Information may become outdated rapidly |


Read about each dimension in detail [here](docs/multi-dimensional-trust-model.md)

## 6. Review System (Signal Generation Layer)

### 6.1 Purpose
Reviewers generate structured signals used by the Trust Vector.
They do NOT approve or reject Guides.

### 6.2 Reviewer Actions
- evaluate claims inside Guides
- validate reproducibility
- assess citations
- flag disagreements
- attach domain expertise context

### 6.5 System-Derived Confidence
Confidence is NOT set by reviewers.
Instead B.L.U.E. computes confidence based on observable reliability signals.

#### Inputs
- reviewer reliability history
- evidence interaction signals
- domain alignment
- peer agreement
- claim uncertainty

#### Output - Atomic Review Signal

```json
{
  "guide_id": "tcp_guide",
  "claim_id": "c12",
  "assessment": "supported",
  "system_confidence": 0.74
}
```

### 6.4 Principle
Reviewers provide observations; the system determines weight.

## 7. Safety Layer (Policy & Distribution Control System)
### 7.1 Purpose
Determines whether Guides can be:
- distributed
- recommended
- surfaced in discovery

### 7.2 Included Systems
- content classification (harmful / sensitive / dual-use)
- legal compliance filtering
- distribution rules
- safety tiering system
- warning overlays
- access friction controls

### 7.3 Safety Tiers
- Allowed
- Restricted Visibility
- Safety Flagged
- Hard Blocked

### 7.4 Principle
Safety controls distribution, Trust controls interpretation, Integrity controls authenticity.

## 8. System Flow
![Knowledge System Diagram](diagrams/Knowledge-System.png)

## 9. Separation of Concerns

| Layer     | Question Answered           | Can Block Content |
| --------- | --------------------------- | ----------------- |
| Identity  | Who created this Guide?     | No                |
| Integrity | Is this Guide authentic?    | No                |
| Review    | What do evaluators observe? | No                |
| Trust     | How reliable is it?         | No                |
| Safety    | Should it be distributed?   | Yes               |

## 10. Core Principles

- Trust is emergent, not assigned
- Confidence is system-inferred, not declared
- Reviews are signals, not decisions
- Safety is independent of epistemic truth
- Guides are multi-media by design (text, image, video, code)

## 11. Final Summary
B.L.U.E. consists of:
- Identity (AT Protocol DID system)
- Integrity (authenticity layer)
- Review System (signal generation layer)
- System-Derived Confidence (weighting mechanism)
- Trust Vector (9-dimensional epistemic model)
- Safety System (distribution enforcement layer)

B.L.U.E. separates identity, authenticity, observation, reliability, and safety into independent systems so knowledge can be evaluated transparently without centralised control.