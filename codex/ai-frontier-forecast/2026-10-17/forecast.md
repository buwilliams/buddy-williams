# AI Frontier Forecast

Public snapshot exported October 17, 2026. Source last updated 2026-10-17T06:48:17.217430Z. See [snapshot notes](snapshot-notes.md) for scope, omissions and checks. Dated observations and historical forecasts below retain their original dates; later dated entries take precedence. This export does not reverify every external claim or rewrite past forecasts.

A forecast of AI output capacity relative to a fixed humanity language-throughput reference, alongside raw Tokens/day, actual usage, and new capacity.

## Tokens/day and the SCM

Buddy's selected headline for the forecast, October 7, 2026. Two views track demand and incoming supply: **Usage** and **New capacity**. The current v1 estimate uses worldwide scope; regional breakouts must distinguish location, ownership, and accessibility.

### Evidence

**Current estimate v1 · October 7, 2026: approximately 800 trillion output tokens/day worldwide.** This is **Llama 3.3 70B reference-normalized potential serving capacity**, not measured global usage or frontier-model output. The base calculation is 825 trillion/day; analyst sensitivity scenarios span about **40 trillion to 3 quadrillion/day**, not a confidence interval. Actual global mixed-model output remains unknown.

**Adopted SCM · approximately 0.23× humanity's reference token rate.** Divide the exact v1 capacity estimate by 5 tokens/second × 86,400 seconds/day × 8.3 billion people = **3.5856 quadrillion reference tokens/day**. The exact base gives 0.23×; using the rounded 800-trillion headline gives 0.22×. Analyst low/high cases give approximately **0.01–0.71×**, not a confidence interval.

The population baseline is the rounded [UN 2026 projection](https://population.un.org/dataportal/data/indicators/49/locations/900/start/2025/end/2100/table?df=0bedacf9-361d-49c3-99bf-d8e7030811e2), WPP 2024 vintage. Hold 5 tokens/second and 8.3 billion fixed for trend comparison. Counting every person for all 24 hours, including sleep, is a generous language-throughput denominator. This is not literal human thinking, intelligence, worker replacement, or proof the capacity estimate is conservative.

**Scale milestones:** 1× = 3.5856 quadrillion/day; 10× = 35.856 quadrillion/day; 100× = 358.56 quadrillion/day. Buddy expects eventual values above 1× and considers 10× and 100× futures.

**When 1×? Conditional scenarios, October 7, 2026:** starting from exact v1 SCM 0.2302× requires 4.344× more reference capacity. At 3.4× annual supply growth, the crossing is **December 2027**; at 2.5×, **May 2028**; at a 2× stress case, **November 2028**. The provisional outlook is late 2027–2028, with December 2027 as the central scenario, not a calibrated expected date or confidence interval. [Growth source](https://epoch.ai/gradient-updates/is-a-compute-crunch-coming) · [Full assumptions](evidence-and-releases.md).

A July 2027 crossing would require roughly 6–7.5× annual growth, or additional allocation, realization, or software gains beyond the hardware trend. Initial capacity uncertainty dominates month-level precision. Keep 10×/100× SCM dates open. April 2027 Human AI 100× and July 2027 Vals RSI remain different forecasts.

- [Google Q2 2026 usage](https://blog.google/company-news/inside-google/message-ceo/alphabet-earnings-q2-2026/): approximately **22 billion API tokens processed/minute**, versus 16 billion a quarter earlier. Reported consumption; input/output and research-purpose splits unspecified. Not output-token capacity.

- **Inventory I1 · rechecked October 16:** [Epoch's October 12 inventory](https://epoch.ai/data/ai-data-centers) lists 94 sites; [current-capacity CSV](https://epoch.ai/data/data_centers/data_centers.csv) sums to 14.6196 million H100-equivalents. Its global coverage estimate is 44%. Implied stock is 33.2264 million, still **33 million** under v1 rounding. Listed sites include future entries; this is modeled dense-compute equivalence, not measured inference inventory or a commissioning count.

- **Serving T1:** [Archived NVIDIA NIM benchmark](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/performance), base row reverified October 16: Llama 3.3 70B FP8 on two H100s, 1,000 input/1,000 output tokens, concurrency 100, reports 2,895.49 system output tokens/second. V1 retains its rounded 2,895, or **1,447.5 per H100**. Input-heavy 5,000/500 tokens at concurrency 50 retains the previously checked **407 per H100** sensitivity. [Metrics](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/metrics.html).

- [IEA power cross-check, 2026](https://www.iea.org/reports/key-questions-on-energy-and-ai/executive-summary): **485 TWh global datacenter electricity in 2025; 950 TWh projected for 2030**. Includes more than inference; cannot directly convert to tokens.

**Base calculation:** 33 million H100-equivalents × 40% token-serving allocation × 50% fleet realization × 1,447.5 reference output tokens/second/H100 × 86,400 = **825 trillion/day**. The 40% and 50% factors are analyst assumptions, not reported facts. Fleet realization covers heterogeneous serving performance and sustained availability; it excludes demand utilization. Prefill is already included in the benchmark workload. [Full low/base/high assumptions and arithmetic](evidence-and-releases.md).

**Input freshness · October 17:** Epoch's dashboard/changelog still show the October 12 inventory (94 sites, 44% coverage). The exact CSV sum and archived NIM serving row were last reverified October 16: 14,619,635.626941182 H100-equivalents implies 33,226,444.6066845 before v1 rounding to 33M. Numerator **825,422,400,000,000 tokens/day**, fixed denominator **3,585,600,000,000,000**, SCM **0.2302048193×** remain unchanged. Allocation/realization are assumptions; actual global output usage and research allocation remain unknown. [Evidence and coverage gaps](evidence-and-releases.md).

### October 17 overnight update

**Another concrete discovery signal.** [Anthropic reports](https://www.anthropic.com/research/ecdsa-curve-origins) an internal model recovered long-mysterious NIST curve-seed origins in under half an hour, with reproduction code. It does not break encryption; some parameter choices remain unexplained. [Reflection Beam](https://reflection.ai/blog/beam-is-available) is now released with open weights, adding another testable deployment option.

**Large agent workloads, with policy boundaries.** A newly reviewed [OpenAI customer case](https://openai.com/index/third-moment-research/) describes research agents operating for days, but its token disclosure cannot establish global output usage. China's new technology-export registration and proposed controls add a deployment-risk watchpoint; its official guidance distinguishes API access from technology export. [Dated evidence and build implications](evidence-and-releases.md).

**SCM remains 0.23×.** No compatible capacity input or sustained frontier self-improvement result justifies a numeric change. Vals' readable methodology remains dated October 15 with 25 models; July 2027 is its last-verified forecast from October 16 because today's text read did not expose the forecast chart. The April 2027 Human–AI 100× forecast remains separate.

### Usage

Track actual reported token throughput per day, with provider, service coverage, reporting period, input versus output split, and workload mix. Keep input tokens, generated output, cached tokens, and undisclosed mixtures distinct. Usage indicates consumption, not maximum sustainable capacity.

**Usage by purpose:** distinguish general applications from AI research and capability improvement where credible data exist. Buddy's mechanism is that less coding and logistics frees researchers for more initiatives and experiments, potentially accelerating progress. Support-work tokens can therefore contribute indirectly to RSI. Track broad reallocation and outcomes without assuming proportional gains. Stronger models and research keep moving bottlenecks; detailed phase shares remain background evidence. AI-assisted R&D and demonstrated closed-loop self-improvement remain distinct.

**Research allocation:** no sourced global share or comparable per-lab RSI-token total is available yet. A rough research-token estimate may be useful, but any allocation range must be labeled a scenario assumption until supported. More research tokens need not yield proportional progress. The [SCM draft](scm-research.md) links token supply, model capability, purpose, and demand.

**Current aggregate:** unavailable. Coverage is partial; no verified global output-token usage total is recorded.

### New capacity

Track additions to sustainable output-token supply under a stated reference model, precision, context length, output length, batching, latency, and reliability target. Distinguish announced, financed, contracted, delivered, energized, commissioned, and serving workloads. Only operating capacity enters available supply; pipeline stages remain separate and are not added together.

Estimate commissioned capacity with declared inference allocation, uptime, power, and serving configuration. Keep demand utilization separate. Count each physical asset once across supplier, cloud, investor, and customer announcements.

**Current modeled operating capacity:** approximately 800 trillion reference-normalized output tokens/day worldwide under v1 assumptions. This is the operating supply baseline; new capacity tracks additions and removals against it. Pipeline stages remain separate. Actual mixed-model output is not measured by this estimate.

### What changes usefulness and demand

[SCM research](scm-research.md) develops the adopted ratio of AI capacity to the fixed humanity token-rate reference, together with the capacity, capability, and demand feedback hypothesis. The measure is adopted for tracking; its causal validity and conversion to useful work remain research questions.

**Chosen strategic assumption · October 8, 2026:** assume leading-human AI expertise in every field. Under this premise, equal token volume could support more expert-level work than the average-human reference suggests. This is Buddy's modeling assumption, not an empirical description of today's entire fleet or the Llama 70B serving reference. No numerical quality multiplier is established; capacity v1 and the 0.23× SCM remain unchanged.

**RSI remains a central growth driver:** track whether autonomous AI research improves useful outcomes per token, token-production efficiency, and the rate of subsequent capability progress. Preserve the separate April 2027 Human AI 100x forecast and Vals' July–August 2027 RSI projection below.

## Quick access

- [Evidence and Releases](evidence-and-releases.md) — sources, comparable measurements, releases, and interview evidence

- [Scenarios and Decisions](scenarios-and-decisions.md) — players, adoption paths, disconfirmation, and experiments

- [SCM draft](scm-research.md) — the developing capacity, capability, and demand model behind the Tokens/day framework

## Forecast register

### RSI and accelerating capability growth

**Buddy's central hypothesis, October 7, 2026:** AI that can improve AI will accelerate the rate of capability progress itself. Vals is a priority because of its RSI forecast, rather than simply its general model rankings.

**Published milestone:** [Vals projects its RSI Index milestone in July 2027](https://www.vals.ai/benchmarks/rsi_index), verified October 16 against its October 15 snapshot (25 models; Opus 5.5 37.31%, Fable 5.1 36.09%). The 0.6 threshold remains a constructed index milestone, not demonstrated closed-loop RSI. Krishnan's interview describes July–August 2027. This is distinct from the **April 2027 Human AI 100x forecast**.

Three connected observations determine whether the mechanism is unfolding:

1. **Research milestone:** record the RSI definition, task coverage, reference anchors, predicted date, actual results, and revisions.

2. **Closed research loop:** observe agents proposing, executing, evaluating, and incorporating improvements into successor systems, with reproducible gains and declining human intervention. Track accepted research results per lab-day, end-to-end cycle time, failed experiments, and human bottlenecks.

3. **Acceleration of capability growth:** compare successive, consistently measured capability-doubling intervals. Track algorithmic efficiency, compute usage, data constraints, and deployment delays so that a changed score or larger compute budget is not mistaken for a changed underlying rate.

The hypothesis gains support when closed-loop research improves and subsequent capability intervals shorten across repeated cycles. It weakens or shifts later if benchmark milestones arrive but research judgment, verification, compute, or data bottlenecks prevent faster successor development. No extra acceleration factor is yet applied to the numerical multiplier curve.

### Production capability

**Buddy's baseline:** approximately 10x production capability by September 2026 and 100x by April 2027. Buddy corrected the target on October 7: September 2027 was executive shorthand, not a separate forecast. Status: conditional model-based forecast, not independently established end-to-end production measurement. The published metric is already defined; its human-effort assumptions need empirical validation.

**Proposed resolution test:** accepted equivalent output per total human hour on a fixed, versioned set of representative production tasks, compared with a specified pre-AI baseline. Include planning, coordination, review, rework, operations, and failed attempts. Report inference cost and elapsed delivery time separately. Freeze the test definition before assessing the April 2027 forecast.

### Public Human AI multiplier model

The [published model](https://growthlab.team/school/artificial-intelligence/model) retains a September 4, 2026 snapshot. Read-only evaluation of its defaults gives approximately 17.0x for that snapshot and a conditional 100x milestone of April 12, 2027. These are model outputs under assumptions, consistent with Buddy's corrected April 2027 target and distinct from measured organizational productivity. See the evidence ledger for the assumptions and uncertainty.

### Insurance economics and adoption

- **Repricing:** production savings may lower competitive prices, but price response depends on delivery costs, switching costs, liability, distribution, and competition. Track comparable bids and total cost to deliver.

- **Build instead of buy:** track signed carrier decisions and deployed systems, not intentions alone. Track maintenance and governance costs alongside build cost.

- **Platform-native distribution:** test whether agent-accessible insurance services win completed customer workflows and distribution. **October 13 update:** [PAP Draft 0.1](https://personalagentprotocol.org/docs/spec) is published, superseding the October 7 forthcoming status; its current partner and transaction-control evidence strengthens the coordination thesis. Production customer outcomes remain the next test. [Evidence](evidence-and-releases.md) · [Conditional tests](scenarios-and-decisions.md).

- **JIT and preprocessed software:** test the boundary between dynamic generation and components worth certifying, caching, or reusing. Optimize verified customer outcomes.

- **Personal agent fleets:** track portable context, connectors, scoped authority, enterprise acceptance, and offboarding. Employee-owned preferences do not establish a right to move employer data.

No calibrated probability is assigned yet. Dates for these adoption claims remain open until each has a specific market, threshold, and observable resolution rule.

## Decision posture

Preserve the ambition: **Insurance should be as thoughtless as ordering dinner.**

The near-term proposed move is a narrow, platform-native insurance workflow with a fixed acceptance suite and measurable human effort. Demonstrate that simpler customer experience and cheaper verified delivery survive integration and review costs. Scale investment when those results reproduce; change the architecture or timing assumptions when they do not.

## Review process

Each forecast update records the prior view, new evidence, revised view, uncertainty, and decision consequence. Failed or inaccessible sources remain gaps, not evidence of no change. Preserve historical forecast entries; do not silently rewrite old forecasts.

## Change history

### October 7 2026

Initialized the forecast, evidence ledger, and scenario framework. Corrected the active 100x target to April 2027 at Buddy's request; September 2027 was executive shorthand. Preserved the original wording as superseded history. Flagged the September 4 source snapshot and its assumptions before treating it as an October frontier view. No public Growth School content was changed.

### October 7 2026 SCM refinement

Adopted AI output capacity divided by a fixed humanity language-throughput reference: 5 tokens/second continuously × 8.3 billion people. Exact capacity v1 implies 0.23×. Preserved raw Tokens/day and separate Usage/New capacity views. Milestones 1×, 10×, and 100× have no assigned dates; existing Human AI and RSI forecasts are unchanged.

### October 8 2026

Rechecked the v1 capacity anchors and Vals RSI forecast; no compatible numeric revision. Added qualified Haiku economics/evaluations, GPT-6 deployment, mathematics verification corrections, and compute/government pipeline evidence. Proposed testing scope expands; the forecast hypotheses and milestone assumptions are preserved. Source coverage and access gaps are recorded in the evidence ledger.

### October 9 2026

Strengthened the platform-convergence interpretation with Google's enterprise agent announcement, alongside enterprise-controlled alternatives. Added access-conditioned evaluations, research-integrity evidence, and funding/pipeline updates. No compatible operating-capacity or research-growth evidence changed the numerical model or milestone scenarios. Preserved the leading-human expertise assumption without adding a quality weight. Historical October 8 findings remain in the evidence ledger.

### October 10 2026

Added evidence on agent-team economics, autonomy boundaries, scientific-data limitations, new tooling and infrastructure pipeline conditions. Freshly verified the existing NIM serving rows through the official export. No compatible capacity or growth-rate evidence justifies changing v1, the adopted SCM, strategic quality assumption or milestone scenarios. Coverage gaps remain explicit in the dated evidence review.

### October 11 2026

Updated PAP from forthcoming specification to published Draft 0.1 and reconciled the scenario milestone. Added qualified deployment and outage evidence. No compatible input changed the numerical model or dates. Source-access and data-refresh limits remain explicit in the evidence ledger.

### October 12 2026

Closed the ECI retrieval gap and preserved a comparison snapshot. Added qualified regional-outage, historical workflow and workload-specific efficiency evidence. No compatible quantitative change to v1 or forecast dates; quality-factor suggestions remain hypothetical. Coverage limits are explicit in the dated ledger.

### October 13 2026

Refreshed inventory evidence and retained the rounded v1 input and SCM. Added Muse evaluations, managed orchestration, PAP transaction-control/partner evidence, and qualified pipeline/policy/financing changes. Corrected the stale forthcoming-PAP summary. No numerical quality factor or forecast-date revision; detailed old-to-new reasoning and coverage gaps are in the dated evidence review.

### October 14 2026

Added Scion research-engineering throughput, bounded MASS cycles, Argon broad access and reviewed optimization outcomes, local-agent and transaction-control evidence, plus qualified financing/power pipeline. Rechecked inventory without changing v1. Older research and recycled news are dated as catch-up; no numerical multiplier, quality factor or milestone-date change. Full uncertainty and decision consequences remain in the evidence ledger.

### October 15, 2026

Added Minos, METR's acceleration-assessment proposal, AA-Robotics, Grok Voice 3, policy/observability changes and dated manufacturing/financing evidence. Old → new: improved measurement and platform-test evidence; no defensible numeric change to SCM, RSI timeline or Human–AI multiplier. Reverified exact inventory; documented X/transcript, DeepSeek, Qwen and ECI access gaps. Prior thesis and estimation method preserved.

### October 16, 2026

Updated model availability, training/serving efficiency, research feedback and approval-boundary evidence; refreshed Vals snapshot and exact Epoch/NIM inputs. SCM v1, quality assumptions and forecast dates are unchanged. Corrected an October 15 ledger denominator transcription error without changing arithmetic or model version. [Review, counterevidence and coverage gaps](evidence-and-releases.md).

### October 17, 2026

Added Anthropic's checkable ECDSA-origin discovery, Reflection Beam deployment and independent evaluation, a scoped Third Moment usage case, and China technology-export registration/proposal evidence. Refreshed input freshness without recalculating or changing v1. Numeric forecasts retained; Vals forecast chart, X and transcript access limitations recorded. [Full dated review](evidence-and-releases.md).
