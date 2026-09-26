# Buddy Williams

**Email:** buddywilliams@gmail.com
**GitHub:** [github.com/buwilliams](https://github.com/buwilliams)
**Writings:** [github.com/buwilliams/buddy-williams-writings](https://github.com/buwilliams/buddy-williams-writings)
**Substack:** [buddyiterate.substack.com](https://buddyiterate.substack.com)
**X:** [@BuddyIterate](https://x.com/BuddyIterate)

> I search for good explanations and build inspiring things.

---

## Summary

Three-time founder and long-time end-to-end operator. I own the full arc from first customer conversation through architecture, implementation, production deployment, and the internal tooling that makes the next engagement faster. 20+ years shipping software, with an active applied AI research practice focused on agent systems, spec-driven development, and mind architectures for AI. Most comfortable in ambiguous, high-ownership roles that combine engineering, product, and customer work.

---

## Experience

### Chief AI Innovation Officer, Nevo Technologies
Mar 2025 to Present · Remote (Atlanta, GA)

Three distinct roles held concurrently:

- **Sales.** Full ownership of the sales process, from sourcing leads through closing deals.
- **Implementations.** End-to-end ownership once a sale is closed: architecture, project planning, coordination, communication, billing, client relations, development leadership, hands-on software development, and production deployment through product completion.
- **AI research.** Translates directly into shipped tools like Forge (a Claude Code plugin for spec management and agentic execution) and Corpus Council (hierarchical council deliberation over a curated corpus). Research extends into theoretics, philosophy, economics, and forecasting, grounded in real customer problems.

### Principal and Account Executive, Nevo Technologies
Aug 2021 to Mar 2025 · Remote (Atlanta, GA)

Two roles held concurrently at a custom software development firm:

- **Sales.** Full ownership of the sales process, from sourcing leads through closing custom software development engagements.
- **Implementations.** End-to-end ownership once a sale closed: architecture, project planning, coordination, communication, billing, client relations, development leadership, hands-on software development, and production deployment through project delivery.

Nevo deliberately runs a simplified role structure (Principal and Developer) so that one person combines multiple disciplines and streamlines communication and decision-making across the customer engagement.

### Technical Lead, Workstorm
Mar 2018 to Mar 2021 · Remote

Led the build-out of Workstorm's UI platform across desktop, mobile, and web for a privacy- and security-focused collaboration platform.

### Founder, Buddy Lee Technology LLC
May 2015 to Feb 2018 · Atlanta, GA

Independent consulting on hardware-sales support, data integration, and no-code application platforms. Primary focus: BlissUI, a low-code app platform compiling visual composition to React 15. Owned everything from customer conversations through architecture, implementation, delivery, and platform roadmap.

### Co-founder and Lead Developer, PhyQuest
Nov 2002 to Mar 2006

First co-founded company. Healthcare platform for radiology significant-findings follow-up and risk reduction. Worked directly with Quantum Radiology and other practices. Held concurrently with my Quantum Radiology role (80+ hours per week across both; one of the PhyQuest partners was also Quantum Radiology's President).

### Earlier Roles
Senior / Lead Software Developer at PostHog, Outpace, Datatrac, Devnext, Markel Corporation, WebMD, and Turner Broadcasting; Web Developer and IT Support at Quantum Radiology. Full history available on request.

---

## Selected Projects

Public repositories. Complete catalog at [github.com/buwilliams](https://github.com/buwilliams).

### Forge (Apr 2026)
Claude Code plugin for spec management, context rot, and attention. Codifies three layers of intent (product spec, constitution, project spec), decomposes goals into tasks, launches role-specific subagents with precisely the context they need, and runs each task through an implement / verify / commit loop. Filesystem is the source of truth (`todo/` → `working/` → `done/` / `blocked/`). Always-forward execution; interrupted runs resume where they stopped; a project-specific verifier agent re-checks completed work from scratch. Everything runs inside an existing Claude Code session; no new CLIs or tools. Replaced my own dependency on ralph-loops and spec-kit. [github.com/buwilliams/forge](https://github.com/buwilliams/forge)

### Corpus Council (Apr 2026)
Hierarchical council deliberation over a curated corpus for grounded LLM conversations. Python API and CLI where every response is routed through markdown-defined personas with assigned authority tiers. Retrieval is deterministic (embedding plus cosine similarity in ChromaDB, top-K injected into every prompt rather than LLM-selected), so the council reasons over a known, grounded context. Parallel and consolidated deliberation modes. New interaction types are added by authoring a markdown goal file, with no Python source changes required. [github.com/buwilliams/corpus-council](https://github.com/buwilliams/corpus-council)

### HarmonicWork (2025)
Agent orchestration platform where AI workers are first-class citizens alongside humans. Portfolios, skills, and DAG-based workflows route tasks to whichever entity best matches the skill requirement. TypeScript, React, Express, PostgreSQL, Drizzle. [Demo](https://www.youtube.com/watch?v=-sblVndCd2E) · [github.com/buwilliams/harmonic](https://github.com/buwilliams/harmonic)

### Autobot (Apr 2025)
CLI that orchestrates Claude Code and OpenAI Codex from the same spec for AI-driven development. Direct conceptual predecessor to Forge. [github.com/buwilliams/autobot](https://github.com/buwilliams/autobot)

### BlissUI (2015 to 2021)
Low-code app platform compiling visual composition to a React 15 runtime. Custom doubly-linked-list data structure supporting tree layout, compiler, injected JS framework, and an editor where the "preview" is the actual running app. Primary development under Buddy Lee Technology LLC. [Demo and talk](https://www.youtube.com/watch?v=AH0zkFfc9v8) · [github.com/buwilliams/bliss-0.2](https://github.com/buwilliams/bliss-0.2)

### PhyQuest (2002 to 2006)
First co-founded company. Radiology significant-findings follow-up platform driving workflows so critical results would not fall through the cracks. Healthcare IT, radiology, patient safety.

---

## Talks and Public Work Samples

- **AI-first Demo: T-SQL to Spark.** Working session with a prospective enterprise client demonstrating an AI-first proof of concept for converting Microsoft T-SQL to Spark SQL, including a PySpark-based verifier that executes converted SQL against loaded synthetic data. Closes with a discussion of tool-as-compounding-asset and the shift from centralized platforms toward niche per-use-case utilities coordinated by agents. [Watch](https://www.youtube.com/watch?v=9HrwtEgYO-U)
- **AI Development: SDLC, AI Code Gen, and Preventing AI Slop.** Primer on specs as the new source of truth, two fast feedback loops (spec development, then system-plus-verifier development), and the verifier app as a higher-level test harness built specifically for the system under development. [Watch](https://www.youtube.com/watch?v=whUNFowc15U)
- **HarmonicWork Demo.** Platform demo for a prospective enterprise client, grounded in a first-principles critique of industrial-era project management (Parkinson's law, Brook's law, quadratic growth of coordination overhead). [Watch](https://www.youtube.com/watch?v=-sblVndCd2E)
- **BlissUI: How I Built an App Designer Platform and What I Learned** (2022). Internal talk walking through the custom data structure, the compiler, the editor-is-the-app design, and candid lessons learned. [Watch](https://www.youtube.com/watch?v=AH0zkFfc9v8)

---

## Selected Writings

Full catalog at [github.com/buwilliams/buddy-williams-writings](https://github.com/buwilliams/buddy-williams-writings). A markdown-first body of work deliberately parseable by both humans and agents.

- **Computer People** (Mar 2026). Synthesis of two years of research on AGI. Argues that LLMs have the necessary ingredients for novel knowledge creation and that AGI constitutes personhood.
- **Structural Convergence Thesis** (Mar 2026). Tests the orthogonality thesis against real-world constraints. Persistent minds converge on a relational Goldilocks zone; AI safety depends on maturation, not just control.
- **Metaprogramming Framework to Classify Personhood** (Feb 2026). An exploration of personhood from an information ontology perspective. Personhood as substrate-independent metaprogramming.
- **AI Phase Change** (Feb 2026). Why AI is a phase change, not just another point on a continuum.
- **Economics of the Intelligence Age** (Dec 2025). First-principles layered model of economics under AI.

---

## Skills and Domains

- **Agentic systems and AI orchestration.** Claude Code plugin development, subagent design, spec-driven development, deterministic retrieval, verifier-based correctness checking, multi-LLM orchestration (Anthropic, OpenAI).
- **Full-stack engineering.** TypeScript, React, Node, Express, Python, Go, PostgreSQL, Drizzle ORM, ChromaDB, Tailwind, Rust exposure.
- **End-to-end customer engagements.** Sales through implementation through production deployment in enterprise and regulated settings (healthcare, insurance, logistics, media, data platforms).
- **Internal tooling as a force multiplier.** Forge and Autobot are both examples of building the tool while using the tool on real customer work.
- **Compilers, DSLs, and custom data structures.** From BlissUI's tree-based app designer runtime through Corpus Council's goal-manifest processing.

---

## Additional

- Location: Atlanta, GA. Open to remote, hybrid, or on-site roles, including relocation and full-time travel.
- Intellectual roots: critical rationalism (Popper, Deutsch), systems thinking, economics as coordination under uncertainty (Hayek, Sowell), and the thinkers who revealed limits (Gödel, Turing, Haidt).
