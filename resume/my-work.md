# My Work

A catalog of the projects, products, and thought leadership that define my career. Ordered from most recent to earliest. Intended as source material for generating resumes, capability statements, and narrative bios. Spin and positioning are left to each individual resume so they can be tailored to the role.

**Author:** Buddy Williams (buddywilliams@gmail.com)
**Blog:** [github.com/buwilliams](https://github.com/buwilliams/buddy-williams-writings)
**GitHub:** [github.com/buwilliams](https://github.com/buwilliams)
**Substack:** [buddyiterate.substack.com](https://buddyiterate.substack.com)
**X:** [@BuddyIterate](https://x.com/BuddyIterate)

> I search for good explanations and build inspiring things.

---

## Signature Projects

The projects that best illustrate what I build and why.

### 1. Forge (Apr 2026)

**Claude Code plugin for spec management, context rot, and attention.**

A plugin that lets Claude Code execute ambitious projects through big-picture prompts without losing sight of constraints. Forge codifies three layers of intent (product spec, constitution, project spec), decomposes goals into tasks, launches role-specific subagents with precisely the context they need, and runs each task through an implement/verify/commit loop. Interrupted runs resume where they stopped. Blocked tasks surface a summary the human can edit and re-run.

- **Role:** Creator, sole author
- **Key idea:** Claude Code can drive ambitious, multi-step projects to completion given three things working together: persistent layered specs, per-task subagent isolation, and an independently generated verifier. Forge supplies all three inside an existing Claude Code session.
- **Stack:** Shell, Claude Code plugin system, filesystem-as-state-machine
- **Notable design choices:** No new CLIs or tools. Everything runs inside an existing Claude Code session. Filesystem is the source of truth (`todo/` → `working/` → `done/` / `blocked/`). Each task gets a freshly scoped subagent, not a shared context. A project-specific verifier agent re-checks completed work from scratch before anything is marked done. Always forward: completed work is never re-run, and interrupted runs pick up where they stopped.
- **Replaced:** ralph-loops and my dependency on spec-kit for my own work.
- **Repository:** [github.com/buwilliams/forge](https://github.com/buwilliams/forge)

### 2. Corpus Council (Apr 2026)

**Hierarchical council deliberation over a curated corpus for grounded LLM conversations.**

A Python API and CLI for LLM-based conversations where every response is routed through a hierarchical council of markdown-defined AI personas with assigned authority tiers. Personas deliberate and validate answers before they are returned. Retrieval is deterministic (embedding plus cosine similarity in ChromaDB, with the top-K chunks injected into every prompt rather than being chosen by an LLM), so the council is reasoning over a known, grounded context. Interaction types are added by authoring a markdown goal file and running `goals process`, with no Python source changes required.

- **Role:** Creator, sole author
- **Key idea:** Separate retrieval from reasoning. Retrieval is deterministic and auditable, and the answer passes through a hierarchy of independent voices before synthesis.
- **Stack:** Python, uv, Anthropic API, ChromaDB, sentence-transformers (all-MiniLM-L6-v2), FastAPI-style serve layer, YAML config
- **Key design choices:**
  - **Goals** as file-defined configurations (desired outcome, council composition, corpus scope). Authored in markdown with YAML front matter. Processed offline into a manifest.
  - **Personas** as markdown files with a primary lens, authority position, role type, and an escalation rule. Position 1 is the synthesizer.
  - **Two deliberation modes.** `parallel` runs N+1 LLM calls with non-synthesizer members deliberating concurrently and blind to each other, then the position-1 synthesizer reconciles. `consolidated` runs 2 calls total for sub-30s turns on a six-member council.
  - **Escalation flow.** Any member can flag escalation against its rule; all members still complete; the synthesizer resolves the concern during final synthesis.
- **Shipped goals:** `intake` (structured customer interview), `create-plan` (synthesizes intake data and corpus into a COM-B 6-week plan).
- **Repository:** [github.com/buwilliams/corpus-council](https://github.com/buwilliams/corpus-council)

### 3. Buddy Williams Writings (Sep 2025)

**A public body of thought leadership on AI, intelligence abundance, consciousness, and economics.**

A markdown-first essay repository published on GitHub and cross-posted to Substack and X. The work develops a coherent position across epistemology, economics, and AI safety: mind is computation within a substrate, substrate-independence has philosophical consequences, persistent minds converge on a relational zone, and the next economy is one where cognitive work becomes nearly free and meaning becomes the scarce resource.

- **Role:** Author
- **Scope:** Essays, fragments, letters, bridges (reader's guides), and recorded conversations with other thinkers. Deliberately markdown-only, parseable by both humans and agents, no paywalls or tracking.
- **Distribution:** Published on GitHub, cross-posted on Substack (buddyiterate.substack.com) and X (@BuddyIterate). A long-form conversation with philosopher Brett Hall on objective morality is logged in full.
- **Repository:** [github.com/buwilliams/buddy-williams-writings](https://github.com/buwilliams/buddy-williams-writings)
- Detailed list of essays in the Thought Leadership section below.

### 4. HarmonicWork (2025)

**Agent orchestration platform for human and AI collaboration on real work.**

A collaborative portfolio and project management platform where AI workers are first-class citizens alongside humans. Portfolios contain skills, projects, and workflows. Skills can be assigned to either humans or AI workers. Workflows are DAG-based process definitions that route tasks to whichever entity (human or AI) best matches the skill requirement. AI workers can be generic, Anthropic chat, OpenAI chat, or OpenAI image, each executing through configurable task templates with full interaction logging and cost tracking.

- **Role:** Creator, architect, lead developer
- **Key idea:** AI workers and humans are modeled as peers inside the same skill and workflow system rather than AI being bolted on top of human workflows.
- **Stack:** TypeScript, React + Vite, Express.js, PostgreSQL, Drizzle ORM, Radix UI, Tailwind CSS, Wouter, React Query
- **Key subsystems:** aiWorkerOrchestrator (task assignment and execution), projectIntakeService (AI-assisted requirement gathering), workflowService (DAG templates and instances), queueService (prioritization), portfolioService (multi-tenancy)
- **Demo video:** [youtube.com/watch?v=-sblVndCd2E](https://www.youtube.com/watch?v=-sblVndCd2E)
- **Repository:** [github.com/buwilliams/harmonic](https://github.com/buwilliams/harmonic)

### 5. BlissUI (2015 to 2021)

**Low-code application platform for removing barriers to inspiration.**

A visual application development environment that let non-programmers build real web applications. Pages, components, an HTML editor, custom HTML import, CSS and JavaScript support, compilation to React 15, state management, and full asset export so the user owned the output. Primary development happened 2015 to 2018 under Buddy Lee Technology LLC across predecessor versions (`bliss-0.1`, `blissui`, `blissui-go`); the public `bliss-0.2` repo was created in March 2019 and maintained through 2021.

- **Role:** Founder, architect, lead developer (primary work under Buddy Lee Technology LLC, 2015 to 2018; public maintenance through 2021)
- **Key idea:** A consumer-facing low-code platform compiling visual composition to a React runtime.
- **Stack:** Go (backend), JavaScript (frontend runtime), React 15 (compilation target), Node.js
- **Demo video:** [youtube.com/watch?v=AH0zkFfc9v8](https://www.youtube.com/watch?v=AH0zkFfc9v8)
- **Repository:** [github.com/buwilliams/bliss-0.2](https://github.com/buwilliams/bliss-0.2) (public). Earlier versions: bliss-0.1, blissui, blissui-go (private or archived)
- **Outcome:** Taught me how timing and distribution shape outcomes in developer tooling. Directly shaped my later work on agent orchestration and spec-driven AI.

### 6. PhyQuest (2002 to 2006)

**Radiology significant findings follow-up platform for risk reduction.**

First co-founded company. A healthcare platform that tracked significant findings from radiology reports and drove follow-up workflows so critical results would not fall through the cracks. Worked directly with Quantum Radiology and other practices.

- **Role:** Co-founder, lead developer
- **Domain:** Healthcare IT, radiology, patient safety
- **Context:** My first venture. Built in my early twenties. A risk-reduction problem that remains unsolved in many healthcare systems today.
- **Outcome:** Operated for approximately three and a half years. Gave me a compressed education in building for regulated environments, selling to skeptical buyers, and the gap between product and business viability.

---

## Additional Projects

Ordered most recent first. Not every repository is listed. I have selected ones that show range or technical depth.

### 2026

- **Thought Experiment Generator** (Mar 2026). Rust. Research tool asking whether LLMs can create new knowledge by narrowing search space through generated and tested thought experiments. [Repo](https://github.com/buwilliams/thought-experiment-generator)
- **CIMC Challenge** (Mar 2026). Python. Evidence collection for creative problem solving by language models. [Repo](https://github.com/buwilliams/cimc-challenge)
- **Lumen Mind** (Feb 2026). A mind architecture for AI agents aligned with the Metaprogramming Framework to Explore Personhood (MFP). Models an agent with values, goals, and identity that evolve through experience. Runs three loops (Action, Explore, Reflection) with prediction-informed scoring and signed prediction error as the trigger for reflection. Originally named "Lumen Conscious" and refactored away from consciousness framing, since personhood does not require resolving the hard problem. Python. [Repo](https://github.com/buwilliams/lumen-mind)
- **SynapseAgent** (Jan 2026). TypeScript. Early exploration of agent-driven workflows. [Repo](https://github.com/buwilliams/SynapseAgent)
- **Snake Evolutionary Algo** (Jan 2026). Rust. Classic evolutionary algorithm experiment. [Repo](https://github.com/buwilliams/snake-evolutionary-algo)

### 2025

- **Euda** (Dec 2025). Personal intelligence for human flourishing. Learns who you are (values, patterns, rhythms) not to optimize productivity but to help you grow. Python. Open source under MIT, with a public Discord community. [Repo](https://github.com/buwilliams/euda)
- **tldraw-dag** (Jun 2025). TypeScript. Custom workflow DAG to tldraw adaptor, built to power visual workflow design in HarmonicWork. [Repo](https://github.com/buwilliams/tldraw-dag)
- **Autobot** (Apr 2025). Python. Save, refine, and leverage application specs for AI-driven development via a CLI that orchestrates Claude Code and OpenAI Codex from the same spec. The direct conceptual predecessor to Forge. [Repo](https://github.com/buwilliams/autobot)
- **Big Ass Calendar** (Mar 2025). JavaScript. Digital version of Jesse Itzler's Big A$$ Calendar concept. [Repo](https://github.com/buwilliams/big-ass-calendar)

### 2024

- **First Game** (May 2024). GDScript. Godot beginner tutorial by Brackeys.

### 2023

- **Learn Assembly x86-64** (Oct 2023). Assembly. Hobby project, expressed as "playing with Assembly to make my inner child self happy."

### 2022

- **Dodger, Jumper, Platformer, Tilemap** (Apr 2022). Unity, C#. Learning series for 2D platform techniques.
- **cs-typescript** (Jan 2022). Computer science fundamentals written in TypeScript.

### 2021

- **future-money** (Jul 2021). TypeScript. Open-source budget web app designed for "future money."
- **buddywilliams.dev** (Jun 2021). SCSS. Personal portfolio and blog.
- **learn-posthog** (Apr 2021). Python. Onboarding guide for PostHog's tech stack. 7 GitHub stars. [Repo](https://github.com/buwilliams/learn-posthog)
- **Recifree** (Apr 2021). Python. Fast, ad-free, open-source recipes for saving and sharing.

### 2018 to 2020

- **futuremoney** (Aug 2018). HTML. A simple plan to always have money. Personal finance essay as executable web page.
- **javascript-tree** (Jul 2018). JavaScript. A unique approach to JSON tree data structures. 3 GitHub stars.
- **thundercat** (Jun 2018). HTML. Lightweight web server with simple authentication.
- **strikebot** (Nov 2016). Go. Grab, transform, and deliver data. Early ETL experiment.

### Earlier Notable

- **connect-four, rpcsql, socialmealclub, wizard-builder, game-engine, go2dgame, todo-clj** (2015 to 2018). Portfolio of language and framework explorations: Ruby, Rails, Go, Clojure/ClojureScript, JavaScript. Hands-on exploration across paradigms.

---

## Talks and Demos

Public video samples showing how I work and how I think about AI-first development.

- **AI-first Demo: T-SQL to Spark** (YouTube, ~22 min). A working session with a prospective client demonstrating a proof-of-concept I built for converting Microsoft T-SQL to Spark SQL under an AI-first lens. Walks through the tool's workflow (pending, review, approve or reject), the AI conversion step, and a verifier built on PySpark that executes converted Spark SQL against loaded synthetic data so correctness is checked end-to-end rather than only syntactically. Closes with a discussion of the tool as a compounding asset (features added as new edge cases are encountered) and a broader aside on the shift from centralized platforms toward niche per-use-case utilities coordinated by agents. [Watch](https://www.youtube.com/watch?v=9HrwtEgYO-U)
- **AI Development: SDLC, AI Code Gen, and Preventing AI Slop** (YouTube, ~13 min). A primer on how I approach AI-assisted development. Covers the properties of AI development (automated code generation, the mirror effect, AI as "brilliant amnesiac"), specs as the new source of truth, and two fast feedback loops: spec development, then system-plus-verifier development. Argues that a verifier app built specifically for the system under development acts as a higher-level test harness, closing the loop on security, functional, and compliance requirements. Closes with practical tips: markdown specs, general-to-specific hierarchy, storing specs in `/ai-specs`, and Claude Code's `claude.md` plus `@`-mention context management. [Watch](https://www.youtube.com/watch?v=whUNFowc15U)
- **HarmonicWork Demo** (YouTube, ~17 min). A demo of HarmonicWork delivered to a prospective enterprise client (Insurity). Opens with a first-principles critique of industrial-era project management (Parkinson's law, Brook's law, and the quadratic growth of coordination overhead as organizations scale) and a systems-thinking reframe of work management as inflows, stocks, outflows, and feedback loops. Walks through four solutions HarmonicWork supplies for a creative-services workload: AI intake (eliminates requirements meetings), automatic scoring and prioritization (eliminates planning meetings), auto-delegation (eliminates who-works-on-what communication), and AI workers that collaborate with humans to expand capacity. Closes with a live walkthrough of the data model (portfolios, sponsors, goals, offers, skills), the queue-centric workboard (a deliberate alternative to date-centric planning to avoid Parkinson's law), AI-assisted project intake with confidence scoring, and an end-to-end AI-only workflow processing a new project request. [Watch](https://www.youtube.com/watch?v=-sblVndCd2E)
- **BlissUI: How I Built an App Designer Platform and What I Learned** (YouTube, ~1 hr 8 min, 2022). A talk given internally at Nevo walking through the design and engineering of BlissUI. Reframes "low code" and "no code" as subcategories of a more useful term, "app designer." Covers the four component parts in the order they were built: a custom doubly-linked-list data structure supporting a tree layout (each node has id, parent, child, next, previous) with O(1) lookups and trivial subtree moves; a compiler that emits standard HTML, CSS, and JavaScript plus framework-specific output (React); a small injected JavaScript framework; and an editor whose "preview" is the actual running app rather than a separate rendering. Closes with lessons learned: the mistake of writing the first editor inside the data structure itself before any bootstrap editor existed, the contrasting reactions from developers (preferred code), agencies (wanted to adopt immediately), and designers (found it too technical), and the business-control dispute that ended the project. Followed by an extended Q&A on round-trip code-to-designer flows, community component libraries, and a "third persona" sitting between developer and designer. [Watch](https://www.youtube.com/watch?v=AH0zkFfc9v8)

---

## Thought Leadership

Published writing in the Buddy Williams Writings repository. The essays below form the intellectual throughline of my current work on AI, intelligence abundance, consciousness, and economics. These are meant as public work samples demonstrating depth of thinking across epistemology, philosophy of mind, and systems design.

### Essays

| Date | Title | Summary |
|------|-------|---------|
| Apr 2026 | What Is Existence? | A synthesis conjecture: mind is computation within a substrate, and the substrate is itself computational. Develops judgment, explanation, and coordination as expressions of personhood. |
| Apr 2026 | What Is a Computer? | Argues that a computer is any physical system capable of implementing universal logic, not a silicon machine. Substrate-independence through nested worlds. Causal closure is real but local. |
| Mar 2026 | Computer People | Synthesis of two years of research on AGI. Argues that LLMs have the necessary ingredients for novel knowledge creation, that AGI is a universal explainer, and that AGI constitutes personhood. |
| Mar 2026 | Layers | Why explanation comes in layers. Argues against reductionism: higher-level explanations are often the only forms in which reality becomes intelligible to finite minds. |
| Mar 2026 | Structural Convergence Thesis | Tests the orthogonality thesis against real-world constraints. Argues that persistent minds converge on a relational Goldilocks zone and that AI safety depends on maturation, not just control. |
| Feb 2026 | Metaprogramming Framework to Explore Personhood (MFP) | An exploration of personhood from an information ontology perspective. Personhood is substrate-independent metaprogramming. The difference is maturity, not kind. |
| Feb 2026 | AI Phase Change | Why AI is a phase change, not just another point on a continuum. Three orthogonal questions people keep conflating. The map-territory trap. |
| Dec 2025 | Economics of the Intelligence Age | First-principles layered model of economics: scarcity, prices, externalities, power, AI. |
| Nov 2025 | Outcome Economy | The near-term economic transformation. Knowledge workers become intelligence workers. Outcome-based economies. |
| Nov 2025 | Why Hypotheticals Matter | Thought experiments as instruments for revealing what is hidden by everyday noise. |
| Nov 2025 | AI Economics Time Traveler | AI as time travel. When productivity becomes infinite, meaning becomes the scarce resource. |
| Oct 2025 | Technohumanism | Humanity's past, present, and future. Technology transforms scarcity into abundance, creating biological misalignment. |
| Oct 2025 | Cyclic Rationality | A methodology for grounded open-mindedness. Qualitative dreaming before quantitative verification. |
| Oct 2025 | Utility of Truth | Why minds resist change. Evolution favors utility over truth. How to plant seeds. |

### Fragments and Threads

Shorter pieces and preserved threads worth mentioning in any narrative of my thinking:

- **Kernel Mechanics and Prediction Error** (Mar 2026): mechanistic gap in MFP, prediction error as unified corrective mechanism.
- **Phase Change Not Continuum** (Feb 2026): time compression triggers phase changes in system behavior.
- **Top-Down Irreducibility** (Oct 2025): irreducible complexity in ant colonies, brains, DNA, reality.
- **Compelling Ideas About Limits** (Oct 2025): Gödel, Turing, Haidt, Wolfram, and the thinkers who revealed our limits.
- **Science Beyond Experiment** (Oct 2025): imagination and fantasy precede verification. A reply to Carmack.
- **AGI From Existing Components** (Aug 2025): AGI is achievable by integrating existing LLM components (language, streaming data, memory, tools, search).
- **Status Pyramid** (Oct 2025): hierarchy is human nature; do not flatten the pyramid, shorten it.

### Conversations

- **Objective Morality** (Mar 2026). Six-part exchange with philosopher Brett Hall, rooted in Deutsch's principle of optimism, Popper's epistemology, and Searle's ontological/epistemological distinction. Published in full.

### Letters

- **A Primer on Logic** (Oct 2025). A structured guide to reasoning well, written for my children.

---

## Brand and Domains

> I search for good explanations and build inspiring things.

The work in this document groups into the following domains. Spin for specific roles is left to each tailored resume.

- **Agentic systems and AI orchestration:** Forge, Corpus Council, HarmonicWork, Autobot, SynapseAgent.
- **Low-code and visual application tooling:** BlissUI.
- **Personal intelligence and mind architectures:** Lumen Mind, Euda.
- **Healthcare patient safety and risk reduction:** PhyQuest.
- **Data tooling and ETL experiments:** strikebot.
- **Public writing on AI, epistemology, and economics:** Buddy Williams Writings.

**Intellectual roots:** critical rationalism (Popper, Deutsch), systems thinking, economics as coordination under uncertainty (Hayek, Sowell), the thinkers who revealed limits (Gödel, Turing, Haidt), and Sagan's clarity.

**Core question I keep returning to:** How can humans design systems (technical, economic, cultural) that remain humane, adaptive, and truth-seeking as intelligence becomes abundant?
