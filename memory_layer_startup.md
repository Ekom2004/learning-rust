# The Memory Layer for Compute

> "Computers forget everything they compute. We give them memory."

---

## The Problem

Every industry wastes massive compute:

| Industry | What They Recompute | Waste |
|----------|-------------------|-------|
| **Finance** | Same risk models daily | 10x |
| **Pharma** | Same drug simulations | 100x |
| **Film/VFX** | Re-render for small changes | 50x |
| **Engineering** | CAD simulations | 20x |
| **ML/AI** | Same preprocessing, training | 10x |
| **Genomics** | Same DNA analysis | 100x |

**Trillions of dollars in compute, much of it redundant.**

---

## The Solution

A universal memory layer that:

1. **Remembers** — Hash(inputs + code) → if computed before, return instantly
2. **Shares** — Computed results shared across teams, orgs, even companies
3. **Versions** — Track history of all computations
4. **Incrementals** — If 1% of input changes, only recompute 1%
5. **Verifies** — Detect tampering, prove computation integrity

---

## How It Works

```
┌─────────────────────────────────────────────┐
│           Applications                      │
│   (Any industry, any workload)              │
└─────────────────────────────────────────────┘
                     ↓ compute request
┌─────────────────────────────────────────────┐
│         THE MEMORY LAYER                    │
│                                             │
│   1. Hash inputs + code                     │
│   2. Check: "Have we done this before?"     │
│   3. YES → Return cached result (instant)   │
│      NO  → Compute, store, return           │
│   4. Track version, share if authorized     │
└─────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────┐
│         Fast Storage (NVMe)                 │
└─────────────────────────────────────────────┘
```

---

## Key Differentiators

| Feature | What It Means |
|---------|---------------|
| **Content-addressed** | Same input = same result, always |
| **Language-agnostic** | Works with Python, C++, Rust, anything |
| **Incremental** | Only recompute what changed |
| **Distributed** | Share results across machines, teams, orgs |
| **Tamper-proof** | Cryptographic verification of results |
| **Industry-agnostic** | Finance, pharma, ML, VFX — all use it |

---

## Not This

| We Are NOT | Why |
|------------|-----|
| AI prompt caching | That's commodity (many do it) |
| Redis/Memcached | Those are key-value, not computation-aware |
| Build systems (Bazel) | We're runtime, not build-time |
| Database | We're a layer, not storage |

---

## Architecture

```
                    Memory Layer Core
                          │
         ┌────────────────┼────────────────┐
         ↓                ↓                ↓
   Fast I/O Layer   Compute Index    Version Store
   (NVMe streaming)  (Vector DB)     (Git-like DAG)
         │                │                │
         └────────────────┼────────────────┘
                          ↓
                  Enterprise Platform
                (Access control, audit, UI)
```

---

## Supporting Projects (MX8 Family)

| Project | Role |
|---------|------|
| **MX8 NVMe Loader** | Fast I/O for large inputs |
| **MX8 Video Loader** | Handle video data type |
| **MX8 Cache Layer** | Core caching logic |
| **MX8 Vector DB** | Index and search results |
| **MX8 Enterprise** | Access control, audit logs, UI |

---

## Market Opportunity

| Segment | TAM | Pain Level |
|---------|-----|------------|
| Enterprise compute | $100B+/year | Very High |
| Cloud compute waste | $50B+/year | Very High |
| ML/AI training | $10B+/year | High |
| Scientific computing | $10B+/year | High |

**If we save 10-50% of wasted compute, that's $10-50B value.**

---

## Go-to-Market

### Phase 1: ML/AI (Beachhead)
- ML teams understand "don't recompute"
- Clear ROI (GPU costs are visible)
- Build reputation in AI community

### Phase 2: Enterprise Expansion
- Finance (risk, compliance)
- Pharma (simulations)
- Engineering (CAD/CAE)

### Phase 3: Platform Play
- Become the "memory layer" for all compute
- Integrations with every cloud, every tool

---

## Revenue Model

| Tier | Price | Who |
|------|-------|-----|
| **Open Source** | Free | Individual devs, researchers |
| **Pro** | $99/month | Small teams |
| **Team** | $499/month | Startups |
| **Enterprise** | $5k-50k/month | Large orgs |
| **Compute savings share** | % of savings | Very large deployments |

---

## The Vision

> "Every computation done once.  
> Every result remembered forever.  
> The world's compute, deduplicated."

---

## Why Now?

1. **Compute costs exploding** (AI, simulations, etc.)
2. **Data sizes exploding** (more to recompute)
3. **NVMe is finally fast enough** (makes this architecture possible)
4. **Sustainability pressure** (wasted compute = wasted energy)

---

## Why Us?

1. Building the low-level infrastructure (Rust)
2. Deep understanding of NVMe, GPU, memory hierarchy
3. Started with specific use cases (ML), expanding out
4. Technical founders w``h``o understand the stack

---

*Created: February 2026*
