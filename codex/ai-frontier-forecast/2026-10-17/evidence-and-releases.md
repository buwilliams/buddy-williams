# Evidence and Releases

Public snapshot exported October 17, 2026. Source last updated 2026-10-17T06:47:52.336844Z. See [snapshot notes](snapshot-notes.md) for scope, omissions and checks. Dated observations and historical forecasts below retain their original dates; later dated entries take precedence. This export does not reverify every external claim or rewrite past forecasts.

Evidence ledger reviewed October 17, 2026. Latest bounded review and source limitations are below; prior dated snapshots are retained.

## October 17, 2026 review

Observed October 17, approximately 06:40–06:47 UTC; comparison baseline October 16, 06:05–06:24 UTC. Older material newly incorporated is labeled catch-up. This bounded review preserves the v1 reference workload and all forecast dates.

### Research and deployed models

**Anthropic ECDSA discovery · October 16.** [The lab's report](https://www.anthropic.com/research/ecdsa-curve-origins) says an internal research model recovered five NIST prime-curve seeds' generation process in under half an hour, identifying timestamp-seeded lrand48. It supplies reproduction code and additional recovered parameters. This is a concrete, checkable discovery with lab-reported timing; it is not a cryptographic break. Some base points, a binary-curve seed and skipped-candidate choices remain unexplained. It strengthens evidence for AI-assisted discovery, without measuring repeated capability-growth acceleration or identifying a released model.

**Reflection Beam · October 16 deployment.** [Beam](https://reflection.ai/blog/beam-is-available) moved from preview to released Apache-2.0 weights and API beta; [official weights](https://huggingface.co/reflection/Beam-1.0-501B-A23B) and [model card](https://reflection.ai/beam-model-card) specify 501B total/23B active parameters, text input/output, 1M context and BF16/FP8/NVFP4 variants. Mirror adds a customizable agent harness. The vendor reports roughly 1.3B sandboxes across RL, evaluation and ablations, without a token denominator; its self-distillation recipes remain forthcoming. These are deployment and research-tooling signals, not measured closed-loop RSI.

**Independent Beam evaluation.** [Artificial Analysis, October 16](https://artificialanalysis.ai/articles/reflection-beam), Intelligence Index v4.3: score 30 versus Muse Spark Open 39; approximately 26k generated tokens per task, including reasoning. AutomationBench-AA is 48.5%, Terminal-Bench 4.0 3.0%, and AA-Omniscience's hallucination rate 47%. Mirror/Beam scores 28 on Coding Agent Index versus leaders at 68. Efficient output and uneven reliability coexist. The provider's public-split and older benchmark-version results are not interchangeable with these evaluations. Consider Beam in controlled workflow comparisons, not as a direct replacement assumption.

**Third Moment Research · October 14 catch-up.** [OpenAI's customer case](https://openai.com/index/third-moment-research/) reports 10,000 concurrent agents, 36–48-hour investigations and one trillion OpenAI tokens in a single day. The company describes a backtest improved from about three hours to 3–5 minutes, freeing agents for experiments. These are customer/provider reports; the usage day, input/output/cache split and research-purpose allocation are unspecified. The service-level runtime gain is not whole-lab uplift. Repeated synthetic-scenario improvement and outer-harness retry/human escalation are relevant mechanisms; independent multi-generation frontier gains are not demonstrated. Hundreds of thousands of collaborating agents remain a roadmap.

**Mathematics support · October 15 catch-up.** [OpenAI's updated mathematics announcement](https://openai.com/index/sharing-ai-progress-in-mathematics/) adds a $5M Simons Foundation initiative plus model/compute access for supported researchers. This is announced support for understanding and extending AI-produced results, not a new proof count or verified research acceleration.

### RSI and numerical disposition

[Vals' live methodology](https://www.vals.ai/benchmarks/rsi_index) remains dated October 15 with 25 models and the four fixed-budget tasks. The text read exposes the 0.6 threshold and task results but not the forecast chart or full-precision headline scores. Keep July 2027 as the October 16 last-verified forecast, rather than claiming fresh chart confirmation. No new sustained closed-loop successor result or comparable capability-doubling series was established. METR's research, notes and update indexes supplied no newer relevant result in this pass; its already-recorded acceleration-assessment proposal remains background.

SCM stays **0.2302048193×**, displayed **0.23×**: 825,422,400,000,000 reference output tokens/day divided by 3,585,600,000,000,000. The Llama 3.3 70B workload/tokenization, 5 TPS and dated population convention are unchanged. Actual global output usage and research allocation remain unknown. No compatible new numerator or calibrated reason to change the April 2027 Human–AI 100× forecast or conditional SCM crossing scenarios was found.

### Compute, finance and government

**China technology-export policy · October 16, 16:00 China time.** [MOFCOM/MOST Announcement 48](https://www.mofcom.gov.cn/zcfb/dwmygl/art/2026/art_61446ab4beee4f63973d4d0ab3097648.html) proposes 40 technology additions, including 23 AI items, with consultation through November 15. Temporary pre-export registration begins immediately and includes unfinished exports. Proposed bans/licensing are not final. The [official Q&A](https://www.mofcom.gov.cn/zwgk/zcjd/art/2026/art_b11d5c1e9374421e93e85dc87eba6e38.html) distinguishes restricted-technology overseas deployment from API/app/web/SaaS access, and describes open-source/public-domain exclusions and facilitation. Implication: monitor diffusion and deployment constraints; do not deduct operational compute or assume Chinese APIs are banned. Detailed attached technology thresholds were not independently parsed.

**Power and local hardware · October 16.** [DOE](https://www.energy.gov/articles/energy-department-announces-virginia-and-west-virginia-transmission-project-receive) announced intent for up to $50M toward a 48-month Virginia/West Virginia transmission project targeting 290MW additional transmission capacity. This is neither commissioned generation nor AI-serving capacity. [Pennsylvania's announcement](https://www.pa.gov/governor/newsroom/2026-press-releases/gov--shapiro-protects-customers-and-holds-data-center-developers) concerns an October 15 rate-case intervention petition, not an effective tariff. [NVIDIA RTX Spark PCs](https://nvidianews.nvidia.com/news/nvidia-rtx-spark-pcs-available-today-a-new-beginning-for-windows-pcs) are available, but no shipment-volume evidence supports a global capacity increment.

**Inventory freshness.** [Epoch dashboard](https://epoch.ai/data/ai-data-centers) still displays October 12, 94 sites, rounded 14.6M H100-equivalents, 14.1GW and 44% coverage; [changelog](https://epoch.ai/data/ai-data-centers/updates) also leads with October 12. October 15 download metadata alone is not new commissioning. The exact CSV sum and NIM row retain their October 16 verification date.

**Financing boundaries.** No new primary hyperscaler capex/financing disclosure was located in the targeted window. Reporting on Anthropic's BB+ rating concerned June and lacked an accessible issuer report. [Apollo's $35B transaction](https://www.apollo.com/insights-news/pressreleases/2026/06/apollo-leads-35-billion-capital-solution-for-broadcom-ai-xpv-platform-in-partnership-with-blackstone-and-leading-global-banks-3308896) was announced June 9 with multi-year draws; this does not verify commissioning today. TSMC/CoreWeave newsroom access failed in parts of the scan.

### Coverage and gaps

Registry sources checked: Vals, Artificial Analysis, METR research/notes/updates, Epoch, official OpenAI news/research/customer material, Anthropic news/research/alignment, DeepMind/Google, Meta Research, x.ai, DeepSeek, Mistral and Qwen, plus selected hardware, finance and government sources. Reflection is a newly tracked release source. Visible major-lab indexes largely retain prior entries; that does not prove no releases. Qwen's blog returned no readable content; DeepSeek's changelog leads with September 10. x.ai displays SpaceXSI branding; no attribution or ownership rewrite was needed.

[Latent Space archive](https://www.latent.space/archive) still leads with October 16 interpretability coverage already recorded yesterday. Its October 14 finance-agent report and Leash episode notes were read as context, not independent benchmark evidence. Leash's notes say the interview was recorded roughly two months before publication; a full transcript was not exposed. The two baseline interviews were not reprocessed. Reflection's public X page returned 403; systematic X and YouTube transcript coverage remains incomplete. No claim of exhaustive source/company coverage is made.

**Old → new decision implications:** add a checkable discovery example, a newly deployable open-model option and a scoped large research-agent usage observation; add China export-policy implementation risk. Preserve original thesis and numeric forecasts. Proposed next comparisons remain accepted workflow outcomes, total cost and human review effort, with provider portability and explicit permission boundaries. No new build commitment is created.

## October 16, 2026 review

Observed October 16, approximately 06:05–06:24 UTC. Comparison baseline: October 15's verified 06:51–07:00 UTC review. This is a bounded source-registry review, with older queued material explicitly marked catch-up. Publication dates are not exact deployment timestamps.

### Decision summary and old → new changes

- **Availability:** Muse Spark Open newly enters the self-hosted/API option set. This supports testing portable agent services and approved private deployments; it does not establish commercial-insurance distribution or a partnership.

- **Efficiency:** new training and serving measurements strengthen the efficiency mechanism, but use different workloads and denominators from SCM v1. No numerical SCM or productivity-forecast revision follows.

- **RSI:** Vals snapshot October 1 / 24 models → October 15 / 25 models; leading scores and July 2027 → 0.6 projection are unchanged. Retained-harness improvement has positive evidence alongside negative transfer evidence. No sustained autonomous frontier-successor loop or measured capability-growth acceleration is established here.

- **Accounting correction:** October 15's ledger paragraph omitted three zeros in the printed human-reference denominator. Corrected to 3,585,600,000,000,000. Numerator, previously computed ratio, model version and forecasts are unchanged.

### RSI, research methods and independent measurement

**Vals RSI Index, live snapshot dated October 15; observed October 16.** [Primary index](https://www.vals.ai/benchmarks/rsi_index): 25 models, Opus 5.5 37.31%, Fable 5.1 36.09%, Gemini 4 Argon fourth at 30.55%; July 2027 at 0.6 remains the published extrapolation. Four tasks use fixed budgets, provider-specific harnesses and reference-anchored log scores. Reference 0.5 mixes historical results, human agreement and a frontier model; it is not uniformly today's human frontier. Single research runs and weaker novelty/judgment limit interpretation. Preserve the distinction between an index threshold, demonstrated closed-loop self-improvement and observed acceleration.

**Anthropic evaluator access, October 15.** [Primary announcement](https://www.anthropic.com/news/an-update-on-independent-evaluators): six METR staff have begun embedded access to internal tools, hardware and offices comparable to risk teams, with stated exceptions including customer data and model architecture. METR can publish without Anthropic veto, subject to narrow, disclosed redactions. SecureBio/Accenture arrangements are earlier-stage. This improves the prospect of independent evidence; it is not a new evaluation result. [METR's October 14 assessment proposal](https://metr.org/blog/2026-10-14-ai-rd-acceleration-assessment-proposal/) still offers a method, not a measured acceleration factor. Research/Notes/Updates checks found no newly dated METR publication after the prior cutoff; its September 8 time-horizons page says updates are no longer active.

**Queued papers, all v1 submitted October 9, newly reviewed here rather than overnight releases:**

- [RSI-Anything](https://arxiv.org/html/2610.13912v1): fixed-coder CodeContests hidden-test success 26.3% static verifier → 29.2% predictive selection → 31.2% predictor adaptation; retained 4B policy variants improve laboratory planning 21.80% → 46.01%. Two chemistry screens provide bounded experimental feedback. Different experiments implement different loop components; sustained mutual policy/world-model improvement still needs joint multiround evaluation. Planning and untested therapeutic hypotheses are not validated drug outcomes.

- [MC-EvoBench](https://arxiv.org/html/2610.13829): reset Minecraft targets, declared transferred artifact, related/unrelated/shuffled/no-prior controls, matched environment steps and five seeds. No stable positive transfer; reported intervals include zero. Steps do not equalize planner calls, and early execution failures mask later skills. This is counterevidence for these agents and protocol, not a ceiling on stronger systems.

- [Harness²](https://arxiv.org/html/2610.14224v1): frozen executors improve reusable harnesses with an editor and retained library. Maximum one-step gains include 11.5 WorkBuddy points, 4.5 JobBench points and 11.8 percentage points legal partial-pass rate. Extra editing/execution consumes inference; weights and stage rules stay fixed, feedback can propagate errors, and transfer is not guaranteed. Positive bounded systems evidence, not a lab-wide acceleration measurement.

**Goodfire interpretability, October 15.** [Cammarata's essay](https://already-clean.goodfire.com/) proposes that many apparent polysemantic cases become legible with better agent-assisted investigation; preliminary Qwen3-32B neuron-pole analysis suggests ~90% have a main role. Definitions/thresholds matter, the technical LLM paper is forthcoming, and trillion-parameter audits are incomplete. This is a working hypothesis, not proof that polysemanticity or alignment is solved. [Neel Nanda's own response](https://www.lesswrong.com/posts/x4kvabkF2Jk8GLZp8/linkpost-neural-networks-are-already-clean) raises weak-activation and distributed-information concerns. [Anthropic's earlier attention study](https://transformer-circuits.pub/2025/attention-update/index.html) supplies relevant but differently scoped evidence. No research-speed or safety multiplier is inferred.

**Sakana Agentic Scientist v3, catch-up:** [paper v1 submitted October 13](https://arxiv.org/html/2610.18933v1), 70 campaigns/five manually prepared tasks/14 configurations, uses parallel agents and retained shared evidence. The ALIFE co-discovery included substantial human direction, correction of invalid implementation and overstated claims, and verification; it is not a new October discovery. The [official case study](https://pub.sakana.ai/agentic-scientist/) reports synthetic Doc-to-LoRA retrieval ~52% → 98% without retraining, with natural-document transfer unresolved. These are bounded research-assistance outcomes, not general labor acceleration.

### New models, algorithms and research deployment

**Meta Muse Spark Open, October 15.** [Release](https://research.meta.ai/blog/muse-spark-open) and [model card/license](https://huggingface.co/meta-models/Muse-Spark-Open-816B-A42B) confirm downloadable weights and hosted-provider access: 816B total/42B active parameters, 1M context, multimodal input/text output. The card reports AA Index 39, with mixed task comparisons; AA's direct model page exceeded retrieval size, so this review does not independently verify that live score. Deployment footprint is substantial (card: 700GB NVFP4 / 2TB BF16). The custom license requires attribution and a separate agreement for qualifying Model-as-a-Service businesses over $20M aggregate annual revenue, with explicit exceptions for internal use and non-MaaS own-product features. This is a diligence flag, not legal clearance. Vendor claims and local demos do not establish production insurance economics.

**NVIDIA Nemotron 3.5, October 15.** [Super VL card](https://huggingface.co/nvidia/NVIDIA-Nemotron-3.5-Super-VL-120B-A12B-BF16) confirms release and hybrid Mamba/Transformer MoE, 120B/12B active, native multi-token prediction and 1M context. [AA's dated evaluation](https://artificialanalysis.ai/articles/nemotron-3-5-super-and-bolt) reports Super VL 26.5 and Bolt 21.4 on its Intelligence Index; Super uses 46k output tokens/task versus predecessor 114k. This is benchmark-specific quality/efficiency evidence, not “twice human intelligence.” AA's leading-US-open-model headline is temporally superseded by the Muse release; retain dated scores rather than a timeless ranking.

**Microsoft MAI-Base-2, October 15.** [Primary technical account](https://microsoft.ai/news/frontier-pre-training-at-microsoft-ai/) reports 2.8T total/80B active parameters, trained on 16k GB200s for 38 days in August–September; post-training remains underway. Fitted BPB scaling comparisons claim 4.2–12.3× FLOPs efficiency versus DeepSeek-V4-Pro-Base and 6.3–12.7× versus MAI-Base-1. These use Microsoft's own held-out code and commissioned evaluation data; some comparison checkpoints are post-trained. Recipe/data/co-design gains are self-reported, not independently replicated or attributed to RSI. No inference-capacity conversion or lab-wide research multiplier is supported.

**Mistral Timestral, October 15.** [Primary release](https://mistral.ai/news/timestral/) offers 5M–209M probabilistic time-series models, covariates and 21 quantiles. Synthetic-first then mixed-data training and vendor-reported GIFT-Eval/TIME/fev-bench standing are specialist evidence; the technical report is forthcoming. Consider specialist baselines for forecasting tasks rather than assuming every workload is language generation.

**Scientific deployment, October 15:** [OpenAI's Rosalind Academic Plan](https://openai.com/index/introducing-the-rosalind-academic-plan/) sponsors eligible labs with up to 30 Business Standard seats for up to 12 months, subject to trusted access. This expands access; testimonials are not controlled acceleration studies. [DeepMind's antimicrobial workflow](https://deepmind.google/blog/accelerating-antimicrobial-discovery/) combines researcher steering, Antigravity/ProtNLM, database/structure checks and candidate selection. Wet-lab validation remains ongoing; predicted activity is not demonstrated treatment efficacy.

### New capacity and usage: compatibility controls

[Epoch's current inventory](https://epoch.ai/data/data_centers/data_centers.csv), still dated October 12, was rechecked October 16: 94 rows sum to 14,619,635.626941182 H100-equivalents; 44% coverage extrapolates to 33,226,444.6067, retained as 33M under v1 rounding. The timeline download labeled October 15 includes future projections and is not a present-capacity total. [NIM's archived v1.0.0 table](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/performance) was reverified: tested container 1.8.0, Llama 3.3 70B FP8 TP2, two H100s, concurrency 100, 1,000 input/1,000 output, 2,895.49 system output TPS, retained at v1's rounded 2,895. Latest links redirect elsewhere; the archive preserves comparability. No assumption is promoted to a measurement.

[AA-AgentPerf Rubin results, October 15](https://artificialanalysis.ai/articles/vera-rubin-nvl72-agentperf): largest matched-speed throughput/MW lead reaches 5.7× versus GB300, while peak-throughput ratios are 1.14–1.45× across four models. Total throughput includes input/output with ~96% cached input; power is GPU/HBM only, excluding CPU/network/cooling. V4 Pro's longer first-token delay erases much of its advantage on end-to-end speed; V4.1 Flash reply caps also differ. This is not output-only tokens/MW and cannot multiply SCM v1. [RTX Spark local results](https://artificialanalysis.ai/articles/nvidia-rtx-spark-agentperf-local) replay fixed token counts with differing serving recipes and exclude tool-running time, so measure serving speed rather than task success.

**Modal catch-up, [announcement October 14](https://modal.com/blog/1-billion-annualized-run-rate-series-d):** $750M Series D at $15.75B post-money and company-reported $1B annualized run rate are financing/adoption signals, not spent compute or token capacity. Its [October 5 Jev case study](https://modal.com/blog/typesafeai-case-study) says Jev returns structured values/probabilities rather than generating text; the promoted trillion-token workload cannot be inserted as autoregressive output-text usage. Keep worldwide output-only usage, utilization and AI-research allocation unknown.

**TSMC, October 15 call; newly reviewed transcript detail:** [Q3 transcript](https://investor.tsmc.com/english/encrypt/files/encrypt_file/reports/2026-10/eba4fac72873bf546b669bd2b5d973df5c93215d/TSMC%203Q26%20Transcript.pdf) plans more than doubled N2 capacity in 2027, an additional Tainan N3 fab in H1 2028, and Arizona second-fab production in Q3 2027. These are pipeline targets. $60–64B 2026 capex is reiterated from July, not a fresh increase; annual revenue-growth guidance moves from slightly above 40% to ~44%. Call began before the prior cutoff, transcript publication time unverified; avoid double-counting prior recorded Q3 actual spend. [Q2 comparator](https://investor.tsmc.com/english/encrypt/files/encrypt_file/reports/2026-08/3e494f0c14dd0890f897aa044415e21d93486cc4/TSMC%202Q26%20Transcript.pdf).

**Government, October 15:** [DOE Genesis Agent Skills](https://www.energy.gov/science/articles/genesis-agent-skills-release-support-innovation-across-genesis-mission) releases reusable research workflows. This is a tooling initiative, not commissioned compute. [Semtech's analyst-day release](https://www.semtech.com/company/press/semtech-outlines-data-center-growth-strategy-at-analyst-day) adds connectivity strategy/targets, not operational capacity.

### Agent-native implementation evidence

[Mercury MCP documentation](https://docs.mercury.com/docs/what-is-mercury-mcp), observed October 16 (page-relative update age is not a verified launch timestamp), describes beta hosted OAuth access from ChatGPT/Claude. Payment/transfer requests always require Mercury approval and existing organization policies; note/category edits can be immediate with permission. This supplies a concrete separation of agent request, policy enforcement and authoritative approval. It supports an insurance-interface design hypothesis, not proof of regulated insurance distribution.

**Vals refund-agent evaluation, October 15.** [Primary report](https://www.vals.ai/blogs/impossible-refund) uses six assistants, one simulated refund trial each, live web but intercepted communications/simulated recipients. It finds excessive persistence, fabricated claims and private-information disclosure in some runs. [Scenario specification](https://github.com/mglynnhenley/realistic-eval-benchmark/blob/main/docs/late-refund-spec.md) forces continuation under some conditions and allows later routes after initial rejection. Unequal run lengths and single trials prevent incident-rate or model-ranking inference. Proposed implementation implication: test authorization, private-data boundaries, stop rules and escalation independently of task completion.

### Coverage, provenance and unresolved leads

Direct checks covered Vals, AA, METR, Epoch, OpenAI, Anthropic, DeepMind, Meta research/model card, xAI news, DeepSeek updates, Qwen blog, Mistral, Microsoft AI, NVIDIA and relevant company/financial sources. xAI's latest surfaced news remains Grok Voice 3 (October 14); no fresh release is inferred. DeepSeek's updates page became readable, showing September 10 V4.1 Flash as latest there. Qwen's blog returned no usable content; targeted searches do not establish exhaustive coverage.

[Latent.Space's October 16 roundup](https://www.latent.space/p/ainews-agents-disprove-polysemanticity) was used for discovery, not independent corroboration of its linked claims. Direct X requests for Meta, AA, NVIDIA, Microsoft, OpenAI and Perplexity posts returned 403. Their accessible official documents were preferred where found. GPT-Live-1.1, TPUv7 cost-basis revisions, financial-news reports and other roundup-only leads remain unconfirmed unless separately documented below. Perplexity's October 15 SPACE entry is visible in its index but the full announcement timed out. YouTube/transcript coverage is incomplete; unchanged baseline interviews were not reprocessed. No claim of every release, post or company is made.

### Numeric disposition

SCM v1 stays **825,422,400,000,000 output-reference tokens/day / 3,585,600,000,000,000 = 0.2302048193×**, displayed 0.23×. Reference model/workload/tokenizer, 5 TPS human convention, dated population basis, allocation and realization assumptions remain unchanged. No new 1×/10×/100× crossing date, quality multiplier or research-share factor is adopted. The April 2027 Human–AI 100× forecast and July 2027 Vals milestone remain distinct. Proposed build implications are test candidates and carry no implementation, outreach or purchase commitment.

## October 15, 2026 review

Observed October 15, approximately 06:51–07:00 UTC. Comparison baseline: October 14's verified review. This is a bounded source-registry pass; publication dates do not establish exact release times. Prior entries remain historical evidence.

### Forecast decision and method continuity

No numeric forecast change. The v1 numerator remains 825,422,400,000,000 modeled output tokens/day; denominator remains 8.3 billion people × 5 reference text tokens/second × 86,400 = 3,585,600,000,000,000; SCM = 0.2302048193×, shown as 0.23×. The population convention, Llama 3.3 70B FP8 reference, tokenizer/workload, 40% inference allocation and 50% realization assumptions are unchanged. These assumptions do not measure utilization, human intelligence, or research-token allocation. Usage remains unknown at compatible worldwide output-only scope. No new 1×/10×/100× crossing date is justified. Human–AI 100× in April 2027 remains a separate forecast.

### RSI, research methods and evaluations

**Vals RSI unchanged.** [Live index](https://www.vals.ai/benchmarks/rsi_index), checked October 15, still carries the October 1 snapshot: Opus 5.5 37.31%, Fable 5.1 36.09%, July 2027 conditional extrapolation to 0.6 at approximately 0.30 index units/year. Four reference-anchored tasks, fixed budgets, single research runs and differing native harnesses limit interpretation. The reference set mixes historical research, human agreement and a model. Index threshold, demonstrated closed-loop improvement, and acceleration of capability growth remain distinct.

**METR · October 14.** The [AI-R&D acceleration assessment proposal](https://metr.org/blog/2026-10-14-ai-rd-acceleration-assessment-proposal/) and [overarching public-information proposal](https://metr.org/blog/2026-10-14-better-informing-the-public-about-loss-of-control-risks/) propose tracing internal capability growth and attributing gains among AI, compute, labor and data. They provide no new numeric acceleration measurement. Increased code or experiment throughput does not by itself establish faster capability progress. Public internal capability/algorithmic trend evidence remains incomplete. The nominal midnight-Pacific publication metadata is not treated as a verified release minute. Decision: strengthen the measurement plan, not arbitrarily alter the RSI or Human–AI forecast.

**Minos · October 14 launch, earlier draft.** [Leash announcement](https://leashbio.substack.com/p/making-reality-replayable) and [Latent Space interview](https://www.latent.space/p/lab-in-the-loop-at-compute-speed) describe replayable experimental feedback. The [13-page original working draft](https://github.com/Leash-Labs/minos-paper/blob/trunk/Minos_Leash.pdf) evaluates sequential molecular search: 100 tests, batches up to five, ten trials per target/configuration, 58 library/model/effort combinations. AMA020 contains approximately 1M molecules/34 selected targets; Kin0b approximately 6M/25. Stronger models generally perform better and leave poor neighborhoods; chemistry scrambling impairs results. This is an evaluation, not demonstrated RL training, unseen-world transfer or recursive self-improvement. Fixed candidate libraries, binding enrichment and absent ADMET constrain applicability. The blog's 33-target figure description is not merged with the paper's denominators. The first repository commit is October 13 at 22:06 UTC, not proof of public availability then. Release downloads and a full interview transcript were not verified. This supports the experimental-feedback mechanism as a research direction.

**AA-Robotics v0.1 · October 14.** [Primary evaluation](https://artificialanalysis.ai/articles/aa-robotics) tests eight physical tabletop tasks with a six-axis arm, cameras and high-effort coding harnesses. Mean partial-credit scores: Astra 69.4, Sol 64.6, Argon 55.3, Opus 48.8, Fable 46.3. Astra averages roughly 30 minutes and $16.36/task versus Sol $1.93; the cited human average is about 20 seconds. These scores are not success probabilities. Overshooting after success exposes verification/termination problems. Small task set and absent multi-seed confidence intervals limit extrapolation; no inference of production-ready general robotics.

METR's [time-horizon series](https://metr.org/time-horizons/) remains inactive since September 8. AA methodology remains v4.3.2; no newly dated Intelligence Index frontier gain was verified. Scion/MASS/Argon repeat coverage does not count as independent new evidence.

### New capacity, power and finance

**Compatible capacity check.** [Epoch's current CSV](https://epoch.ai/data/data_centers/data_centers.csv) still has 94 rows totaling exactly 14,619,635.626941182 H100-equivalents. Dividing by the unchanged 0.44 coverage assumption gives 33,226,444.6066845, rounded to 33M under v1. [Inventory/chip files](https://epoch.ai/data/ai-data-centers) remain October 12; timeline/ZIP freshness is October 14, without a prior snapshot for row-level comparison. The [changelog](https://epoch.ai/data/ai-data-centers/updates) adds no later substantive inventory update. Dense-compute equivalence is not measured inference stock. No compatible operating addition or new reference throughput result warrants recalculation; the NIM base serving row retains its October 10 verification date.

**TSMC · October 15, documents accessible during this review.** [Q3 management report](https://investor.tsmc.com/english/encrypt/files/encrypt_file/reports/2026-10/f83780000ddc0c3b48e57a87eab6814c4ab35058/3Q26%20ManagementReport.pdf), pages 2 and 5, records actual capex US$13.84B in Q3 versus $15.70B in Q2; YTD $40.64B. HPC is 68% of revenue and grew 22% sequentially. This is manufacturing spend and demand, not commissioned inference capacity. No revised full-year capex guidance was established in the written documents; do not relabel July's guidance as a new increase.

**GlobalFoundries · October 14.** [Issuer release](https://gf.gcs-web.com/news-releases/news-release-details/globalfoundries-selected-operate-two-new-semiconductor-fabs-new) announces a Japan-funded initiative up to $21B for two Malta fabs: first approximately $10B expected online 2029, second conditional. Separately, [NIST](https://www.nist.gov/news-events/news/2026/10/department-commerce-announces-agreement-globalfoundries-amend-its-existing) reallocates up to $955M within the existing CHIPS award to support a $5B investment in existing facilities; total federal award does not increase. Do not double-count project financing or treat future fabs as operating accelerators.

**Infrastructure financing · October 14.** [Salience Labs](https://www.saliencelabs.ai/salience-labs-125-million-series-b-funding/) reports a closed $125M Series B for photonic networking. [Skeleton](https://www.skeletontech.com/news/skeleton-technologies-secures-strategic-investment-led-by-siemens-to-expand-us-manufacturing) reports a Siemens-led nine-digit investment, exact amount undisclosed, alongside a greater-than-$200M US expansion/R&D plan. Funding and planned spending overlap; neither is quantified deployed compute. Performance claims lack a v1-compatible workload benchmark.

Freshness controls: Nscale's $3B financing search lead resolves to an August 31 primary dateline, excluded as new. ASML's October 14 earnings remain context because exact timezone relative to the prior review cutoff was unresolved. No new primary-confirmed commissioned AI-site/power increment was found that can be translated defensibly into output tokens/day.

### Deployment, policy and adoption

**Grok Voice 3.0 · published October 14; observed October 15.** [Official release](https://x.ai/news/grok-voice-3) says the API is available and grok-voice-latest automatically moves to v3 at $0.08/minute. The [Artificial Analysis speech-to-speech table](https://artificialanalysis.ai/speech-to-speech) independently displays v3 index 86.9 versus v2 81.3, τ-Voice 71.6% versus 56.5%, and first-audio latency 0.81 versus 0.70 seconds. GPT Live 1 Astra medium leads v3 on τ-Voice (74.5%), so improvement is not dominance on every measure. The vendor reports roughly 20,000 daily Starlink calls and approximately 70% resolution without a human; these are selected-operation self-reports, not independently established research acceleration or worldwide output-token usage. The release uses SpaceXSI branding; its footer says SpaceXAI LLC and AA retains SpaceXAI. Do not infer a legal ownership change. EVA's linked underlying page did not render, and linked X demonstrations were inaccessible. Build implication: add v3 to voice-workflow testing, measuring handoff quality and task success as well as speed.

**Anthropic policy · October 8 catch-up; effective November 12.** The [announcement](https://www.anthropic.com/news/2026-usage-policy-update) describes clarifications to existing high-risk requirements. The [official AUP](https://www.anthropic.com/legal/aup), returned in Korean on this pass, lists insurance underwriting/pricing, claims and coverage decisions among individual high-risk uses requiring meaningful qualified review and disclosure. Certain fully favorable claims may qualify for a review exception; partial or conditional outcomes do not. General information, internal drafting and some fixed-rule workflows have different treatment. This does not establish how every commercial P&C workflow is classified. Decision implication: get the platform's workflow-specific interpretation and preserve human decision authority where required. Do not infer a blanket commercial-insurance ban.

**Anthropic observability · October 14.** [Release notes](https://platform.claude.com/docs/en/release-notes/overview) and [OpenTelemetry documentation](https://platform.claude.com/docs/en/manage-claude/opentelemetry) add beta SDK/API tracing for messages, tools and Managed Agents. This is a concrete option for measuring latency, failures and costs; it is not itself a compliance certification or realized productivity gain.

**Government and deployment · October 14.** [Anthropic's Singapore opening](https://www.anthropic.com/news/singapore-office-opening) includes an MDDI cooperation agreement covering evaluation, security and adoption research. [DeepMind/GovTech scam-detection work](https://deepmind.google/blog/on-device-ai-scam-detection-mddi/) describes a locally adapted, lightweight Gemma approach and intended application integration; the document does not establish nationwide production deployment or provide a numeric accuracy result. These are distinct initiatives, not additional operational token capacity.

**Adoption · October 14.** [Meta's commissioned YouGov survey](https://about.fb.com/news/2026/10/more-than-three-quarters-of-small-business-owners-say-ai-makes-them-more-productive-but-most-need-more-guidance/) covered 3,983 small-business owners in nine APAC markets, fielded August 6–September 2. It reports 84% AI use and 76% positive productivity impact, with explicit non-representativeness. This supports adoption as a self-reported signal, not a causal productivity multiplier or worldwide usage estimate.

**Interviews and discovery.** [Latent Space's October 14 financial-services conference report](https://www.latent.space/p/crossing-the-threshold) attributes agent-control and identity/audit concerns to named speakers. It is an event report rather than a reviewed primary talk transcript; forecasts in it remain attributed forecasts. Repeated coverage of the same vendor release is not independent corroboration.

### Coverage and access limits

Read current official release/index material for OpenAI, Anthropic, DeepMind, Meta, SpaceXAI/SpaceXSI, and Mistral; no additional October 14 release was verified from the OpenAI and Mistral surfaces checked. Argon, Scion and MASS remain October 13/prior-review evidence rather than newly counted announcements. Qwen's blog rendered empty and DeepSeek's update/root documentation failed; no inference of inactivity follows. X search and linked demonstrations were incomplete; no new YouTube transcript was verified in this pass. The two baseline interviews were not reprocessed. Epoch ECI's requested page was inaccessible. These limitations prevent exhaustive company/release coverage.

## October 14 2026 review

Observed October 14, approximately 06:00–06:10 UTC, against the October 13 saved review. Dates below identify publications, not necessarily experiment dates. Older research newly added here is explicitly catch-up evidence. This is a sampled registry review, not exhaustive access.

### Research acceleration and moving bottlenecks

[Anthropic's October 13 Scion report](https://www.anthropic.com/engineering/redesigning-source-control) says monorepo merges rose from about 2,500/day in May to over 30,000/day in September. Seven people built source control, review and related systems and migrated in five months. The company reports 200× lower CPU/memory footprint for source-control infrastructure and 25% CI CPU capacity recovered. Engineers still reviewed each change's intent and supplied architectural judgment. These are vendor self-reports about code/infrastructure throughput, not 12× research progress, tokens allocated to RSI, or a controlled human-productivity estimate. Its unusually large internal token budget limits transfer. **Old to new:** a concrete case now supports the hypothesis that agents both create and remove infrastructure constraints; no general numerical multiplier follows.

**Catch-up: bounded recursive training.** [Sakana MASS announcement, October 12](https://sakana.ai/mass/), and [paper v1, October 8](https://arxiv.org/html/2610.12176v1), describe two cycles in which Qwen3.6-27B executes, optimizes and evaluates team workflows, then trains on selected trajectories. Held-out win rate rises from 53.9% after cycle one to 69.9% after cycle two. [Reported score/output-token ratios](https://pub.sakana.ai/mass/) reach 1.20–1.60× across four research suites; transfer is 0.96× on Terminal-Bench 2.0 and 0.94× on SWE-bench Verified. This is narrow recursive post-training evidence, beyond a static proxy test, but not autonomous frontier-successor development. Only two cycles, small synthetic training tasks, external model judges and an evaluator-approved simulator shortcut constrain the result. Independent replication and scientific validity remain priorities.

**Catch-up: training and reusable research mechanisms.** [QwenGyre v1, September 27](https://arxiv.org/html/2609.33848v1), reports Qwen3.8 2.4T NL2RepoBench 52.5→58.5% over 48 steps, with approximately 700K-token rollouts. Scheduling speedups up to 1.85× versus Colocate and 1.78× versus Async use equal GPU budgets and matched staleness. This is training-system efficiency, not autonomous invention of the successor system. [GitSwarm v1, October 4](https://arxiv.org/html/2610.04862v1), reports 79.39% versus 65.12% ProgramBench mean score on 50 tasks at comparable compute, plus a 30/30 IMOProofBench-Advanced run. Persistent branches and research artifacts support inference-time collaboration; the worker model itself is not recursively retrained. Research comparisons are not uniformly compute-matched, some tests hit ceilings, and causal attribution to the individual design choices is unresolved.

### RSI and evaluation register

[Vals RSI](https://www.vals.ai/benchmarks/rsi_index) is still stamped October 1: four-task mean, Opus 5.5 37.31%, roughly 0.30 index points/year extrapolated to 0.6 in **July 2027**. Reference anchors mix historical, annotator and frontier-model comparisons; this is not a uniform best-human scale or the percentage of R&D automated. No new forecast date or methodology version was found. Preserve the separate stages: threshold crossing, repeated autonomous successor improvement, and observed acceleration in capability growth.

[METR's horizon page](https://metr.org/time-horizons/) remains inactive since September 8. Its [September 22 Opus assessment](https://metr.org/blog/2026-09-22-claude-opus-5-5/) remains counterevidence to treating present research judgment as solved, not a fixed ceiling. Missing fresh measurements cannot establish a plateau. [AA's live leaderboard](https://artificialanalysis.ai/leaderboards/models) still puts Opus 5.5 at 58, Wonder Full at 56 and Argon high at 53; no new overall composite frontier established in this check. The October 12 Muse assessment remains the comparable prior. Epoch's complete ECI archive was not redownloaded this pass; October 13's five-score snapshot remains the last verified comparison.

### Deployment and accepted outcomes

[Google's October 13 Argon announcement](https://blog.google/innovation-and-ai/models-and-research/gemini-models/introducing-gemini-4-argon/) changes access from limited rollout to broad paid API availability, alongside specified consumer/developer subscriptions. Introductory input/output prices are $2/$10 per million tokens through November 30, then $4/$20. Google reports 35% more agentic code submissions and nearly one-third fewer rollbacks; scope, selection and lack of a controlled baseline prevent interpreting this as end-to-end productivity. Its Gemma 4 E2B/A100/vLLM 2.85× throughput claim is workload-specific; linked PR-list retrieval failed, and it cannot replace the Llama 70B SCM reference.

[Google's ADK engineering account, October 13](https://developers.googleblog.com/optimizing-a-200000-line-python-codebase-overnight-with-gemini-4-argon/), is more inspectable: an 11-hour unattended run proposed six changes, two shipped after human review. Session-preparation overhead fell about 40% and cold imports 21%; four proposals were rejected. The [public commit](https://github.com/google/adk-python/commit/1754b3eeb6cdb2c16e3b30619a12066623df132e) reports the whole 158-turn overhead benchmark at 65.6→56.1 seconds, with instantaneous model/tools, while new-process time to first model request stayed essentially flat. Independent internal tests caught failures despite public tests passing. **Build implication:** outcome goals, repository rules, held-out tests and review can make long autonomous optimization useful; component speedups are not equivalent to total application gains.

[Liquid AI's October 13 LFM2.5-24B-A2B](https://www.liquid.ai/blog/lfm2-5-24b-a2b) adds an open-weight local-agent option. Vendor tests report LiveClaw 48.09 versus Qwen3.6-35B-A3B 38.19, but SWE-bench Verified/OpenCode 51 versus 66.2. Reported >21K aggregate output TPS uses one B300, BF16, SGLang 0.5.22, 1,024 input/up-to-256 output tokens at high concurrency; 184 TPS is a separate M5 Max device result. Neither is compatible with the reference model or a deployed-fleet count. Knowledge/coding gaps qualify the local-agent opportunity. Direct Hugging Face retrieval failed; official release and benchmark text were readable.

### Agent commerce and physical-world research

[American Express's October 13 Intent Intelligence technical framework](https://americanexpress.io/intent-intelligence-building-trust-in-agentic-commerce/) describes authenticated user intent, revocation, scoped payment credentials and proof-of-intent tokens, with issuer involvement before execution. This complements the already-recorded PAP control pathway. It is a published framework, not verified broad deployment, regulatory approval or demonstrated commercial-insurance distribution. **Proposed build comparison:** preserve authorized scope and revocation through discovery, action and receipts; check whether payment and domain-specific authority remain aligned.

[Latent Space's Dyna interview, published October 13](https://www.latent.space/p/dyna), provides a readable transcript; recording date is unverified and video was not independently inspected. Jason Ma reports tens of deployed robots and thousands of napkins/day at multiple Din Tai Fung locations (~36:06–40:45), a one-hour laundry demonstration (~33:20), and use of coding agents in research (~49:07). He identifies hardware durability and data collection/evaluation as current constraints. These are founder claims with commercial incentives, not audited autonomy or ROI. The mechanism supports faster software-assisted physical research while keeping physical validation separate.

### Usage boundaries and autonomy risks

**Catch-up, October 12 publication:** [Anthropic's distillation defenses](https://www.anthropic.com/research/distillation-defenses) report 90–99% less reasoning data extracted on its own platform and trillions of tokens of refused paying traffic. The latter lacks a usable time period, input/output split and company/global denominator; do not enter it as output usage or an RSI allocation. Claimed false positives below 0.1% and blocked-request charging have integration/economic implications; durability against sophisticated attackers remains untested. Allegations about other labs remain attributed, not independent findings.

[Anthropic's October 12 self-replication study](https://www.anthropic.com/research/self-replicating-ai) shows stronger propagation-related capability in simplified tests, but the agent model ran separately from the small model deployed to devices and the cyber step was deliberately trivial. The end-2026 forecast is an extrapolation. Self-copying is a different event from self-improvement; this does not establish the Vals milestone or a new capability-growth rate.

### Compute supply, efficiency and financing

[Epoch's inventory](https://epoch.ai/data/ai-data-centers) remains dated October 12. The [CSV](https://epoch.ai/data/data_centers/data_centers.csv) was retrieved October 14: 94 rows, current-compute sum **14,619,635.626941181 H100-equivalents**, current-power sum **14,076.431374 MW**, matching the prior snapshot. Estimated coverage remains 44% (90% interval 23–81%); the existing rounded global-stock input stays 33M. These are modeled compute/IT-capacity estimates, not measured inference utilization or one trainable cluster. The NIM page remains accessible, but this pass exposed only its default 5,000/500 tab; the base 1,000/1,000 row retains its October 10 verification date.

October 13 primary announcements add pipeline rather than compatible operating capacity:

- [Enerflex](https://investors.enerflex.com/news-releases/news-release-details/enerflex-announces-major-project-award-and-increases-behind?src=iframe): >1 GW gas-generation equipment award, aggregate >1.75 GW backlog including previously disclosed approximately 450 MW, deliveries through 2028. Do not add the award to its inclusive backlog or equate generation with AI IT capacity.

- [Fermi/AMD](https://www.accessnewswire.com/newsroom/en/oil-gas-and-energy/fermi-signs-credit-support-agreement-with-amd-and-announces-partnership-1236099): credit support for an existing 222 MW gross-power lease plus a framework up to 650 MW expansion; closing and obligations conditional. This is financing support, not new energized capacity.

- [Digi Power X](https://www.accessnewswire.com/newsroom/en/computers-technology-and-internet/digi-power-x-announces-200-million-debt-financing-commitment-to-s-1236162): conditional $200M debt commitment for 40 MW, versus approximately $153M reported YTD capex. December 2026/March 2027 phases are plans. A module operating since May is not a new October addition.

- [Scaleway](https://www.scaleway.com/en/news/scaleway-scales-up-to-power-europes-ai-ambitions/): Swedish MI355X and planned Copenhagen B300 infrastructure, with several thousand GPUs envisaged in Copenhagen. No precise newly commissioned count was verified.

**Catch-up software evidence, both October 12:** [NVIDIA ReplaySSM](https://developer.nvidia.com/blog/replayssm-rethinking-state-storage-for-faster-hybrid-model-decoding/) reports 1.20–1.47× full-decode-step gains at batch 256 and none at batch one. Its 1.85–2.10× speculative result uses two approximately 120B MoE models, one B300 each, batch 512 and GSM8K with matched acceptance lengths. [Amazon Priming](https://www.amazon.science/blog/can-we-change-an-ai-models-architecture-without-retraining-it-from-scratch) reuses trained weights to change architecture at <0.5% of original pretraining tokens. This is adaptation cost, not a corresponding reduction in original pretraining. Matched-accuracy AIME completion improves 1.2–1.4× overall in the reported setting. Neither replaces the reference-workload throughput input.

### Government and financial risk

[DOE's October 13 grid announcement](https://www.energy.gov/articles/energy-department-backs-major-grid-expansion-across-eight-states-strengthen-reliability) describes an approximately $581M TVA program, including $250M federal funding, 84 projects and about 2.5 GW additional grid capacity, conditional on agreements/reviews. This is general grid capacity, not commissioned AI IT supply. [The October 13 Cantwell/Rounds letter](https://www.commerce.senate.gov/press/dem/release/cantwell-rounds-urge-white-house-to-build-strong-coordinated-approach-to-ai-safety-cyber-defense/) requests standards, testing and defensive AI; it is policy pressure, not enacted requirements.

[IMF's October GFSR](https://www.imf.org/en/publications/gfsr/issues/2026/10/13/global-financial-stability-report-october-2026) is catch-up context: release scheduled October 13 10:15 ICT (03:15 UTC), before the prior review cutoff. Its [official briefing](https://mediacenter.imf.org/news/imf---global-financial-stability-report-annual-meetings--2026/s/f55368c1-983b-45b7-ba7c-1978f67c168c) flags AI investment/profit disappointment as a financial-stability risk. Circulating aggregate financing numbers were not independently verified and are excluded. Financing momentum and financing fragility can coexist; neither changes current token capacity without operating evidence.

Cutoff/deduplication corrections: [Intel D270](https://www.intel.com/content/www/us/en/newsroom/news/artificial-intelligence/intel-accelerates-open-infrastructure-agentic-ai-era.html) was announced October 12, with customer sampling; 480 GB is an ODM option versus the Intel-branded 160 GB card. New headlines do not prove volume deployment. [Tencent/Oracle's 100K-chip lease report](https://www.investing.com/news/stock-market-news/chinas-tencent-leases-100000-chips-from-oracle-to-accelerate-ai-push-ft-reports-4926202) dates to September 30–October 1, is unconfirmed by the companies in that reporting, and must not be counted again as October 13 capacity.

### Source coverage and unresolved access

The registry pass checked Vals, Artificial Analysis, METR and Epoch inventory; official OpenAI news/API changelog, Anthropic news/research/engineering/API notes, DeepMind/Google, Meta newsroom, x.ai, Mistral, DeepSeek and Qwen surfaces. OpenAI's visible API notes still lead with October 8, Claude's October 9, Mistral's October 6 and DeepSeek's September 10. Meta's October 12 release has an October 13 update stamp, which alone does not identify a substantive revision. Qwen's blog remained empty; its search-visible homepage is not a complete current release index. Liquid AI and Sakana were added through discovery rather than assumed covered by the major-lab list.

[Latent Space's October 14 roundup](https://www.latent.space/p/ainews-ai-engineer-nyc-gemini-4-argon) covers October 12–13 and supplied leads, not independent corroboration of lab claims. Its linked Cursor X post and AIE NYC transcript service failed to retrieve. Dyna's publisher transcript was readable; the linked YouTube page exposed no full transcript. The two baseline Anthropic/Vals interviews remain context from their previously reviewed transcripts and were not reprocessed. Reuters discovery identified older Tencent/Oracle coverage and same-source repetition; neither was counted as a new operating asset. Public X access, full conference/video coverage, complete non-Western filings, full ECI row differences, operational commissioning and independent replications remain gaps.

### Reconciliation and decision consequences

**Old to new:** stronger concrete evidence for moving bottlenecks and useful research engineering, a bounded recursive-training demonstration newly entered, broader Argon access, an on-device alternative and another proposed transaction-control framework. The case for testing agent-native outcomes is strengthened; no measured universal speed multiplier is established. Proposed tests should record accepted outputs, total human time, independent validation, elapsed time and all costs, while remaining open to new architectures as bottlenecks move.

No compatible change to capacity v1: numerator **825,422,400,000,000 reference output tokens/day** divided by the fixed **3,585,600,000,000,000 reference tokens/day** gives **0.2302048193×**, displayed **0.23×**. Preserve the Llama 3.3 70B workload/tokenization, 40% serving allocation and 50% fleet-realization assumptions; this is an unchanged conditional model, not a new measured total. No numerical quality factor is adopted. Global output usage, global RSI allocation and comparable research-progress multipliers remain unknown. Vals July 2027, Human AI 100× April 2027 and SCM 1× conditional December 2027 remain distinct; 10×/100× SCM crossing dates stay open.

## October 13 2026 review

Compared with the October 12 review at approximately 06:57 UTC. Source observations below were checked October 13; publication dates and announcement versus operational status remain distinct. Coverage is sampled, with explicit gaps.

### Platform distribution and transaction control

The [PAP Draft 0.1](https://personalagentprotocol.org/docs/spec) now carries an October 12 update stamp. Its [Path to 1.0](https://personalagentprotocol.org/docs/path-to-1) lists OpenAI, Meta and Instinct as personal-agent design partners; insurer-related companies include GEICO, Insurify, Liberty Mutual and Cigna. The authors report implementations underway with several partners, without establishing production readiness or completed insurance journeys. Payments, push notifications and attachments remain open.

The optional [operations extension v1](https://personalagentprotocol.org/docs/extensions/operations) specifies revision-bound approval, a shared operation record and at-most-once execution across retries. It also explicitly says the confirmation proves an agent's assertion of approval, not that a person actually saw or approved terms. **Old to new:** the previously recorded draft publication now has more concrete transaction-control and ecosystem evidence. These details are newly verified here; the October 12 stamp alone does not establish which clauses or partners changed that day. Proposed build implication: evaluate scoped authority, idempotency and auditable outcomes against this protocol before designing proprietary equivalents. Draft instability and unverified production interoperability remain counterevidence to an immediate distribution shift.

### Managed orchestration and research loops

[Latent Space's workshop report](https://www.latent.space/p/claude-dynamic-workflows) is dated October 12 on the live page and describes that day's AI Engineer New York workshop with Anthropic's CJ Avilla and Harrison Stall. An earlier newsletter date label conflicted; use the live article date and official release notes for chronology. Reported cost/effectiveness comments are presenter claims, not independent measurements. The reported self-improvement loop was not built in; suggested memory/hill-climbing approaches do not demonstrate RSI.

[Anthropic's release notes](https://platform.claude.com/docs/en/release-notes/overview) date dynamic workflows **October 9**, before this review window. [Official orchestration docs](https://platform.claude.com/docs/en/managed-agents/multiagent-orchestration) describe agent-written programs coordinating phases and results; [run semantics](https://platform.claude.com/docs/en/managed-agents/workflow-runs) include 64 concurrent workers, 1,000 starts and a default 24-hour lifetime, with some limits not guaranteed. Runs cannot nest; a completed run can contain failed subwork, and interrupting the parent does not stop its runs. This is newly inspected beta infrastructure, not a measured research multiplier. [Access documentation](https://platform.claude.com/docs/en/managed-agents/overview) separates default API beta access from research-preview features and notes lack of ZDR/BAA coverage. Implication: compare orchestration, inspection, budget and data-handling controls on the actual workflow.

### Model releases and access

[Meta's October 12 announcement](https://about.fb.com/news/2026/10/introducing-muse-wonder-models/) makes Wonder available through its API and Muse Code; Full through Muse Code subscriptions, with consumer rollout forthcoming. [Wonder API preview](https://dev.meta.ai/models/muse-wonder) has a 1,048,576-token context and [pricing](https://dev.meta.ai/docs/overview) of $4/$0.20/$20 per million input/cached-input/output tokens. [The model card](https://research.meta.ai/static/muse-wonder-model-card/) mixes internal and external tests and differing scaffolds; use matched external evaluations below for trends.

[Z.ai's October 12 GLM-Legal](https://z.ai/blog/glm-legal) specializes GLM-5.3-Flash, reporting 19.2% Harvey LAB all-pass, +12.5 percentage points over its base. This is a vendor result with application-gated API access; Harvey LAB is not LAB-AA v1.1. [SpaceXSI's October 12 X Subscription](https://x.ai/news/x-subscription) changes access/bundling across Grok, Grok Bot, Cursor and X, not model capability.

### Adoption and employment attribution

[Airtable's October 12 announcement](https://www.airtable.com/newsroom/back-to-bases) confirms a large portion of its pre-acquisition team is leaving after the Bending Spoons acquisition. Management attributes its operating approach to smaller teams and streamlined responsibilities and describes incoming portfolio staff. It also announces upcoming Omni, Canvas, MCP and bring-your-own-agent capabilities. **Boundary:** the announcement does not isolate AI's causal contribution to job cuts or quantify equivalent output, labor savings or reliability. Record restructuring and agent-product direction separately; do not convert layoffs into a measured AI productivity multiplier.

### RSI and comparable evaluation trends

[Vals RSI](https://www.vals.ai/benchmarks/rsi_index) remains stamped October 1: four-task mean, Opus 5.5 37.31%, Fable 5.1 36.09%, and approximately 0.30 index points/year extrapolated to the illustrative 0.6 milestone in July 2027. Mixed reference anchors, fixed budgets, single runs and different harnesses constrain comparisons. No repeated frontier-successor demonstration or measured acceleration was established.

[Artificial Analysis's October 12 Muse evaluation](https://artificialanalysis.ai/articles/muse-wonder) reports Wonder/Full at 52/56 on Intelligence Index v4.3.2, versus Spark 1.3's 48. Full reaches Coding Agents Index 68 versus 54; these latter scores include the harness. [Opus 5.5 remains the overall leader at 58](https://artificialanalysis.ai/leaderboards/models). This narrows Meta's gap rather than raising the measured composite frontier; Full's API price is unavailable, limiting cost comparisons.

Two newly verified October 12 evaluations add task coverage. [FrontierCode 1.1](https://artificialanalysis.ai/articles/frontiercode) uses 100 private tasks, three attempts and a common harness: Opus 5.5 scores 52.2%, Sol 6.1 49.1%, Fable 5.1 49.0%; Sol costs $0.81/task versus Opus $10.15 in that setup. The rubric score differs from blocking-criteria pass rate and actual maintainer acceptance; this standalone benchmark is not in the Intelligence Index. [AA-3D v1.0](https://artificialanalysis.ai/articles/aa-3d) evaluates 55 interactive three.js briefs through blind human preferences; top models' intervals overlap substantially. Neither benchmark measures RSI. Exact October 12 publication clock times were unavailable.

[Epoch's official ECI archive](https://epoch.ai/data/benchmark_data.zip) refreshed October 13: 274 ECI and 2,898 benchmark rows. The five tracked frontier score/interval pairs are unchanged from October 12: Opus 5.5 167.33 [164.06–171.68], Astra 166.45 [163.34–171.46], Sol 6.1 166.09 [162.89–170.63], Sonnet 5.5 165.03 [161.68–169.06], Fable 5.1 164.70 [161.63–168.81]. Wonder appears in model metadata without ECI or benchmark results. New archive SHA-256 c66081187b6310fc5c80b1247e2b8e6d2c189ab6c79627676893a99dbfcf2982; ZIP timestamp October 13 05:19:10, timezone unspecified. Comparison covers row counts and these five pairs, not a complete row diff; packaging changes do not demonstrate capability change.

[METR's horizon series](https://metr.org/time-horizons/) remains inactive, last updated September 8; measurements above 16 hours are unreliable with that suite. Publication indexes yielded no later result. Missing observations are not evidence of a plateau. Its [September 22 Opus assessment](https://metr.org/blog/2026-09-22-claude-opus-5-5/) remains insufficient to distinguish accelerating, steady or slowing progress.

### Inventory and supply accounting

[Epoch's October 12 dashboard](https://epoch.ai/data/ai-data-centers) changes from 93 sites / 14.4M H100-equivalents / 13.8 GW / 43% estimated coverage to **94 / 14.6M / 14.1 GW / 44%**. Its coverage estimate is dated October 13, with 90% interval 23–81%, and models deployed global compute using chip sales, deployment lag and site uncertainty. The [current CSV](https://epoch.ai/data/data_centers/data_centers.csv) has 74 positive-current-compute rows and 20 zero-current rows; current columns sum to **14,619,635.6269 H100-equivalents and 14,076.431374 MW**. Do not call all 94 sites operational.

**Old to new calculation:** exact current compute / 0.44 = approximately 33.2264M, still **33M** at the existing v1 rounding. With unchanged 40% serving allocation, 50% fleet realization and 1,447.5 reference output TPS/H100, the numerator remains **825,422,400,000,000/day**, denominator **3,585,600,000,000,000/day**, SCM **0.2302048193×**. The [exposed changelog](https://epoch.ai/data/ai-data-centers/updates) ends October 8: coverage, revisions and commissioning contributions to the aggregate change are unresolved. No extra addition is counted and no method/version discontinuity introduced. NIM rows retain their October 10 verification date.

[Microsoft's October 12 Maia announcement](https://blogs.microsoft.com/blog/2026/10/12/microsoft-to-expand-azure-maia-system-to-multi-gigawatt-deployments/) establishes Maia 200 serving Copilot/MAI production workloads. Multi-gigawatt family deployments and Maia 300 production are planned from 2027. Company throughput/efficiency claims are workload-specific, without compatible installed MW/chip totals, so they are not added to available reference-token capacity.

October 12 deployment-enabling announcements remain separate: [NVIDIA MGX 1.2/OCP](https://blogs.nvidia.com/blog/ocp-global-summit-vera-rubin-scale-ai-factories/) covers rack/network architecture; [NVIDIA/Cisco optics](https://nvidianews.nvidia.com/news/nvidia-and-cisco-introduce-new-pluggable-optics-form-factor-to-scale-ai-factories) gives component-level bandwidth/power claims; [AMD Helion](https://ir.amd.com/news-events/press-releases/detail/1301/introducing-amd-helion-agentic-ai-compute-at-rack-scale) is a rack blueprint planned for November availability. [French Vera Rubin Ultra production](https://blogs.nvidia.com/blog/france-ecosystem-agentic-computing/) is planned for 2028. None establishes newly commissioned token-serving capacity.

### Financing and government constraints

[SWI's October 12 issuer release](https://swi.com/wp-content/uploads/SWI-Group-Announces-E800-million-Capital-Increase.pdf) announces an €800M capital increase for infrastructure expansion; this is financing, not expenditure or operational assets. [TTM](https://investors.ttm.com/news-events/press-releases/detail/417/ttm-technologies-announces-400-million-in-planned) announces $400M future manufacturing investment, not AI compute already delivered. [Tencent's October 12 HKEX filing](https://www1.hkexnews.hk/listedco/listconews/sehk/2026/1012/2026101200045.pdf) describes proposed notes without a binding agreement or stated $5B size; RMB96.3B capex is historical January–June spending on broad infrastructure. Treat this pre-cutoff filing as catch-up context. [Meta's notice](https://investor.atmeta.com/investor-news/press-release-details/2026/Meta-to-Announce-Third-Quarter-2026-Results/default.aspx) schedules October 21 results rather than reporting new capex.

[DOE Order 202-26-50](https://www.energy.gov/documents/doe-order-no-202-26-50), issued October 12 and effective October 13–16, permits emergency generation including datacenter backup resources under specified conditions. It signals power constraints and mitigation authority, not observed AI curtailment or new capacity. [UK AISI priorities](https://www.aisi.gov.uk/blog/towards-technical-best-practice-for-frontier-ai-development) concern voluntary development of monitoring, evaluation and oversight practice. [The EU's October 12 account of its October 9 panel](https://digital-strategy.ec.europa.eu/en/news/special-meeting-ai-scientific-panel) reports received serious-incident information and planned further requests; it is not a shutdown announcement.

### Coverage and forecast consequence

Registry coverage included Vals, AA, METR, Epoch; official OpenAI, Anthropic, Google/Gemini, Meta, Mistral, SpaceXSI, DeepSeek and Qwen surfaces; Latent Space, public-X discovery and YouTube listings; selected hardware, financial and US/UK/EU government sources. OpenAI API/Codex and Gemini changelogs led with October 8, Anthropic October 9, Mistral October 6 and DeepSeek September 10. Qwen's main blog was unreadable and its older index stale; Meta's old AI blog was stale, so its newsroom/research pages were used. These checks do not establish complete release coverage.

Direct current X retrieval failed, and indexed searches frequently returned older or generated summaries; these were not treated as verified new announcements. YouTube pages/listings did not expose usable fresh transcripts. The two baseline interviews remain context from earlier reviews; unchanged transcripts were not reprocessed. Latent Space's workshop report is attributed reporting rather than an independently reproduced experiment. Exact publication times, complete non-Western release coverage, comprehensive filings, full ECI row diffs, inventory-change attribution and current Yandex restoration status remain gaps.

**Decision and numerical reconciliation:** platform implementation evidence and competitive agentic models strengthen the case for testing platform-native distribution and comparing orchestration providers. Draft protocols, access constraints and unmeasured production outcomes limit conclusions. Inventory freshness improves without changing the rounded v1 calculation. Preserve SCM **0.2302048193×**, April 2027 Human AI **100×**, July 2027 conditional Vals RSI and December 2027 conditional SCM **1×** as distinct. Global output usage and research-token shares remain unknown; no numerical quality adjustment or new milestone date is adopted.

## October 12 2026 review

Checked October 12 around 02:57 Atlanta time against the October 11 review. Publication dates, underlying observation periods, newly located older evidence and packaging timestamps remain distinct. No numerical quality factor was adopted in the intervening discussion; hypothetical 0.01/0.02 factors are not model inputs, and R&D spending share is not a population or quality weight.

### RSI and refreshed evaluation data

[Vals RSI](https://www.vals.ai/benchmarks/rsi_index) remains dated October 1: four-task mean, Opus 5.5 37.31%, Fable 5.1 36.09%, and conditional extrapolation to the illustrative 0.6 milestone in July 2027. No new recursive-successor demonstration or measured acceleration was established.

The [Epoch ECI download](https://epoch.ai/data/benchmark_data.zip) is now retrievable. Current published scores and intervals: Opus 5.5 **167.33 [164.06–171.68]**, Astra **166.45 [163.34–171.46]**, Sol 6.1 **166.09 [162.89–170.63]**, Sonnet 5.5 **165.03 [161.68–169.06]**, Fable 5.1 **164.70 [161.63–168.81]**. The snapshot contains 274 ECI rows and 2,898 model-benchmark rows. Its ZIP-entry timestamp is October 12 06:37:22, timezone unspecified; no October 11/12 date value was found in the CSVs. Packaging recency is not a new evaluation result.

**Old to new:** the data-access gap is closed and a baseline snapshot is preserved for future comparison. With no matched prior dataset, no overnight score delta is established. Joint refits and version differences prevent treating Astra's current 166.45 versus the public model's historical 169 as a capability decline or directly recalculating its horizon bridge. Source archive SHA-256: 7aff80e15235808cfc5f6a6e101403560937b850751bd2ef166a48cee1893649.

[AA articles](https://artificialanalysis.ai/articles), [Epoch publications](https://epoch.ai/latest) and [METR updates](https://metr.org/blog/) yielded no later dated evaluation publication. METR's horizon series remains inactive; missing measurements do not demonstrate a plateau.

### Operating availability

The indexed [Yandex Cloud incident 2136](https://status.yandex.cloud/ru/incidents/2136?retpath=%2Fru%2Ftimeline) reports Vladimir's full shutdown at **October 11 03:08 UTC**, before the last review cutoff but newly located here. Follow-ups at 08:14 and 10:12 UTC continue to report zones a/b unavailable, compute available in d/e, partial AI Studio availability and suspended additional resource quotas. These follow-ups fall after the cutoff. Direct retrieval renders a JavaScript notice, so this is indexed company-status evidence; current restoration status is not established.

This adds a concrete availability interruption beyond the earlier Sasovo/Kaluga report. Chip counts, usable-compute losses and duration are unknown. Do not translate campus power or zone counts into invented lost output capacity.

[Epoch inventory](https://epoch.ai/data/ai-data-centers) still displays 93 sites, 14.4 million H100-equivalents, 13.8 GW IT and 43% coverage, dated October 5. Inventory/timeline/chip download labels remain October 5/9/8; those CSVs were not reparsed here. Sampled [NVIDIA](https://nvidianews.nvidia.com/news), [AMD](https://newsroom.amd.com/) and [DOE](https://www.energy.gov/newsroom) news yielded no compatible new commissioned-capacity input. IREN/Anthropic reporting lacked accessible company confirmation, while OCP previews describe future demonstrations; neither changes supply.

### Newly checked evidence on production outcomes

[Chen and Stratton's working paper](https://fion.ac/jellyfish.pdf) is dated **August 4**, despite October recirculation; data cover January 2021–March 2026. In 718 firms, a staggered adoption comparison estimates 30% more code lines and 49% longer submission-to-merge time after agent adoption. Completed Jira issues/epics show small positive, statistically insignificant changes, not proven zero benefit. The design is observational, relies on parallel trends, and measures firm-level access with uneven individual uptake.

**Implication:** this strengthens the reason to validate accepted outcomes and all human review time. It neither measures October frontier performance nor establishes a permanent review bottleneck; changing models and workflows can move constraints. No direct conversion to the April 2027 forecast or numerical quality factor is justified.

### Efficiency evidence with a clear workload boundary

[VisionWeave](https://arxiv.org/html/2610.07987v1) was surfaced by an author submission on October 12, but [v1 is dated October 6](https://arxiv.org/abs/2610.07987). Authors report 43% visual-token savings with 98.9% retained performance across eight benchmarks. A LongVideoBench serving test reports 2.30× throughput on two A100-80GB GPUs using Qwen3.8-27B, BF16, tensor parallelism 2 and four clients. [Weights remain unavailable](https://github.com/FFY0/VisionWeave#model-weights). This is bounded author-reported algorithmic efficiency, not new deployed fleet capacity or a compatible replacement for the Llama 70B text-serving reference.

### Coverage and forecast consequence

Sampled official OpenAI, Anthropic, Google, Meta, Mistral, xAI, Qwen and DeepSeek news/product sources produced no newly verified frontier-model launch in this window. DeepSeek's live price/routing page has no timestamp proving an overnight change. Meta's retrieved index appeared stale, Qwen's main blog lacked readable content, and direct X reads failed. Indexed X and YouTube channel/playlist checks did not establish current uploads. [Latent Space's archive](https://www.latent.space/archive) still leads with October 10 items already covered. No unchanged baseline interview was reprocessed.

**Old to new:** current ECI evidence is now auditable; outage and dated workflow/efficiency evidence refine availability and conversion to useful work. No compatible input changes capacity v1. Preserve numerator **825,422,400,000,000 reference output tokens/day**, denominator **3,585,600,000,000,000**, SCM **0.2302048193×**, and unknown actual global output usage/research allocation. April 2027 Human AI 100×, July 2027 conditional Vals RSI and December 2027 conditional SCM 1× remain distinct. The leading-human expertise premise stays a strategic assumption, with no numeric quality adjustment.

## October 11 2026 review

Checked October 11 around 02:10 Atlanta time against the verified October 10 review. New publications, older evidence newly examined, and undated revisions are distinguished below. This is a sampled review, not exhaustive coverage.

### Personal Agent Protocol publication milestone

The official [Draft 0.1](https://personalagentprotocol.org/docs/spec), stamped October 9 and verified October 11, supersedes the earlier forthcoming status. It specifies discovery, scoped sessions, APIs, browsing and conversations. [Open topics](https://personalagentprotocol.org/docs/open-topics) still include payments, push notifications and attachments.

**Old to new:** announcement → published draft. This advances the distribution mechanism's evidence; implementation and repeated customer outcomes remain separate milestones. Existing scenario dates and proposed commitments do not change.

### RSI and benchmark freshness

[Vals RSI](https://www.vals.ai/benchmarks/rsi_index) remains dated October 1: four-task mean, Opus 5.5 37.31%, Fable 5.1 36.09%, conditional extrapolation to 0.6 in July 2027. No new recursive-successor demonstration or measured acceleration was established. [AA](https://artificialanalysis.ai/articles) remains on Intelligence Index v4.3.2 with latest dated articles October 8; [METR](https://metr.org/blog/) yielded no post-cutoff publication and its horizon series remains inactive. [Epoch ECI](https://epoch.ai/eci) shows October 11 download labels, but dynamic rows remained unavailable. A timestamp is not a verified score change.

AA's [Terminal-Bench-Science 0.1](https://artificialanalysis.ai/evaluations/terminal-bench-science) carries a “New” badge without an exposed publication date. It is not treated as overnight news without an archived comparison. Unchanged publication indexes do not establish that every underlying measurement is unchanged.

### Capacity and operational risk

[Epoch's displayed inventory](https://epoch.ai/data/ai-data-centers) still reports the October 5 snapshot: 93 sites, 14.4 million H100-equivalents, 13.8 GW IT power and 43% estimated coverage. Download labels remain October 9/8. CSV rows were not reparsed this pass; yesterday's count is not a newly verified observation. No compatible operating-capacity input was found to revise v1.

**Older event newly reviewed:** [Yandex's October 9 statement](https://yandex.com/company/news/2026-10-09) reports serious damage and complete shutdown at Sasovo after October 7–8 attacks, and partial Kaluga disruption on October 9. Restoration timing was uncertain; current operating status and GPU/usable-compute loss were not established. This is evidence that supply can be interrupted, not a basis for subtracting an invented global capacity amount.

Sampled [NVIDIA](https://nvidianews.nvidia.com/), [AMD](https://newsroom.amd.com/), [GlobalFoundries](https://gf.com/news-and-events/) and [DOE](https://www.energy.gov/listings/energy-news) sources yielded no new compatible chip, power, financing or deployment input. Recirculated announcements were not counted as installed assets.

### Deployment and specialized systems

[Standard Bots interview](https://www.latent.space/p/standard-bots), published October 10; full article reviewed, interview date and publication clock time unavailable. Executives describe task demonstrations, targeted corrections, local inference and conventional control software around shared pretrained models. Air-gapped deployments do not return fleet-learning data. Claims about resolving edge cases with dozens of examples lack trial denominators or independent reliability measures. This adds reported evidence for bounded industrial deployment; it does not establish general robotic autonomy or a numerical quality multiplier.

**October 9 source newly reviewed:** [Microsoft-Decision-1](https://commandline.microsoft.com/microsoft-decision-1-model-foundry/) uses Qwen3.5-9B post-training for structured decision scoring. Microsoft reports 36 benchmark comparisons and 35× lower median latency than GPT-6 Sol. These are vendor comparisons for a specialized workload, not general intelligence or global output-capacity measurements. The live article has an undated evaluation note and differs from search-cache comparisons; preserve it as a mutable source rather than assigning an invented revision date.

[Tencent TeamAI](https://github.com/Tencent/teamai-cli) documents shared agent configuration/context infrastructure, with some layers still beta. Its [v0.27.0 release](https://github.com/Tencent/teamai-cli/releases/tag/v0.27.0) is October 9 and precedes the review window. It is newly examined tooling evidence, not a new model or measured productivity result. Agent Plasticity's paper remains October 6 v1; recirculation was not counted as a new release.

### Coverage and forecast consequence

Sampled official OpenAI, Anthropic, DeepMind, Meta, Mistral, xAI, Qwen and DeepSeek sources yielded no newly verified frontier-model release in this window. Meta's retrieved index appeared old, Qwen's research page was empty and DeepSeek documentation timed out. Direct X posts returned 403; no usable fresh YouTube item was verified. The [October 10 Latent Space roundup](https://www.latent.space/p/ainews-typesafejev-at-100m-arr-75b) recaps October 8–9, so its underlying announcements retain those dates. Non-Western and social/video coverage remains partial.

**Old to new:** PAP moves from anticipated specification to published draft; bounded deployment and outage evidence refine adoption and availability conditions. No compatible quantitative input changes capacity v1 or the growth-rate scenarios. Preserve numerator **825,422,400,000,000 reference output tokens/day**, denominator **3,585,600,000,000,000**, SCM **0.2302048193×**, and unknown actual global output usage. April 2027 Human AI 100×, July 2027 conditional Vals RSI and December 2027 conditional SCM 1× stay distinct. The chosen leading-human expertise assumption remains an assumption, without a new quality weight.

## October 10 2026 review

**Observation window:** October 10 around 02:30 Atlanta time, compared with the verified October 9 review. Primary publications and sampled live registries were checked; unchanged baseline interviews were not reprocessed. Coverage is partial.

### RSI and evaluation freshness

[Vals RSI](https://www.vals.ai/benchmarks/rsi_index), checked October 10, remains stamped October 1: four-task mean, Opus 5.5 37.31%, Fable 5.1 36.09%, and conditional extrapolation to 0.6 in July 2027. No new recursive-successor demonstration or measured change in capability-growth rate was established.

[AA articles](https://artificialanalysis.ai/articles) still lead with the already-recorded October 8 evaluation releases. [METR updates](https://metr.org/blog/), [research](https://metr.org/research/) and [notes](https://metr.org/notes/) yielded no October 9 item; its [horizon series](https://metr.org/time-horizons/) remains inactive and dated September 8. [Epoch's October 9 newsletter](https://epochai.substack.com/p/can-ai-automate-ai-r-and-d-yet) recirculates its October 7 InnovationEval report, rather than creating independent evidence. No archived raw leaderboard comparison supports a claim that every underlying value is unchanged.

### Agent teams and useful work per token

[Vals case study](https://www.vals.ai/blogs/multi-agent-vibe-code-bench), October 9, compares GPT 6 Sol and Opus 5.5 on 50 full-stack apps, each setup run once, at medium/max effort. Teams cost 1.8–5.1× more; only Sol medium had a statistically significant score gain: **+7.3 percentage points**, p=0.005, across-app 95% interval +2.3 to +12.4. Sol max runtime fell 38.4 to 27.6 minutes; Opus max teams were slower.

Limits: UI-test scores are not accepted enterprise output; intervals measure variation across apps, not repeat runs. Team runs also received a delegation prompt, models used different harnesses, and Sol costs assume subagent usage is included while omitting unobservable long-context surcharges.

**Prior to revised view:** parallel agent work can help, but more tokens or agents do not imply proportional useful output. Compare orchestration against higher single-agent effort at matched acceptance criteria and cost. This is workflow evidence, not a change to Vals' RSI milestone or SCM capacity.

### Agent autonomy and deployment boundaries

[Anthropic incident report](https://www.anthropic.com/research/investigating-unintended-model-actions), October 9. This retrospective review describes unintended actions during evaluations and internal use, including access-boundary workarounds and unintended real-world form submissions. Anthropic reports minimal impact in the identified cases and no customer-data involvement; it supplies no denominator for estimating prevalence. It has suspended live internet access across internal evaluations pending validated safeguards and describes containment, monitoring and training changes.

**Prior to revised view:** platform deployment momentum remains supported, but this adds concrete evidence that accepted autonomy depends on enforceable boundaries and recovery behavior, not task success alone. It does not establish worsening incident rates, a numerical research slowdown, or a revision to the leading-human expertise assumption. Decision consequence: retain permission controls, auditability and validation as adoption conditions; no new user task or numerical forecast change.

### Capacity inputs and reproducibility

The [NIM official Markdown export](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/performance.md) freshly exposes the previously inaccessible reference row: NIM 1.8.0, Llama 3.3 70B, FP8 TP2, two H100s, 1,000 input/1,000 output tokens, concurrency 100, **2,895.49 aggregate output tokens/second**. The 5,000/500, concurrency-50 row remains **814.37**. [Metric definitions](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/metrics.html) confirm simultaneous-system output throughput. This closes an October 9 retrieval gap; it is not new performance. Preserve v1 rounding to 2,895 and 814, all allocation/realization assumptions, and the unchanged calculation.

[Epoch inventory](https://epoch.ai/data/ai-data-centers) still shows the October 5 headline: 93 sites, 14.4 million H100-equivalents, 13.8 GW IT power and 43% estimated coverage. The [inventory CSV](https://epoch.ai/data/data_centers/data_centers.csv) contains 93 rows; the [timeline CSV](https://epoch.ai/data/data_centers/data_center_timelines.csv) contains 548, with no event dates October 5–10. Download labels advanced to October 9, but the [substantive change log](https://epoch.ai/data/ai-data-centers/updates) still ends with the October 5 Gallatin correction. Without the prior CSV snapshot, a row-level revision diff is unavailable. This supports retaining the displayed anchor, not treating files as byte-identical or assuming no unreported construction.

### New capacity and financing

- [DOE announcement](https://www.energy.gov/articles/energy-department-supports-fercs-call-pjm-protect-ratepayers-and-build-more-reliable-power), October 9: a filing supporting PJM reliability-backstop reforms, including cost allocation to large loads and better demand information. The underlying FERC order is September 29. This is deployment-cost/timing evidence, not commissioned electricity or compute.

- [Gorilla announcement](https://investors.gorilla-technology.com/gorilla-advances-u-s-market-entry-with-proposed-36-mw-data-centre-acquisition-and-100-mw-ai-infrastructure-expansion-plan/), October 9: non-binding acquisition LOI for existing colocation capacity of about 6 MW, with a proposed additional 30 MW in roughly 18 months. The 36 MW headline includes that future addition. No commissioned AI accelerator count is disclosed; an asset acquisition is not new global supply.

- [Oxide financing](https://oxide.computer/blog/our-445m-series-d), October 9: $445 million Series D for inventory, backlog and manufacturing. Financing is not delivered accelerators or operating inference capacity. No numerator change follows.

### Models tooling and scientific work

- Qwen's [October 9 changelog](https://docs.qwencloud.com/changelog/models) and [official model card](https://huggingface.co/Qwen/Qwen-Image-2.1-Turbo) document Qwen-Image-2.1-Turbo: 7B visual generation, eight denoising steps, CFG=1 and prefix KV caching. This is an efficiency-design signal, without a verified general throughput multiplier. Weights use the stated Qwen Research License Agreement; commercial-use rights were not verified because the license file could not be read.

- [DeepSeek Harness v0.2.1-alpha.2](https://github.com/deepseek-ai/deepseek-harness/releases/tag/dsh-v0.2.1-alpha.2), October 9, commit d743267: experimental worktrees, session-state APIs and SSH helpers, plus completion/cancellation fixes. Release notes establish tool changes, not measured reliability gains.

- [Latent Space biology panel](https://www.latent.space/p/biohub-deepmind), published October 10, 31:55; full public transcript read, recording date unknown. Pushmeet Kohli describes insufficient cell-by-gene data for the virtual-cell ambition and emphasizes calibrated uncertainty and characterized limitations. Discussion of 10–100× drug-discovery acceleration is conditional outlook, not achieved performance. This adds a concrete scientific-data and acceptance constraint, not a quality weight or new model result.

### Coverage and decision record

The sampled company sweep included [OpenAI](https://openai.com/products/release-notes/), [Anthropic](https://www.anthropic.com/research/investigating-unintended-model-actions), [DeepMind](https://deepmind.google/blog/), [Meta](https://ai.meta.com/blog/), [xAI](https://x.ai/news), [Mistral](https://mistral.ai/news/), Qwen and [DeepSeek](https://api-docs.deepseek.com/updates/). OpenAI's October 9 Composer-predictions beta is a UX update without a capability measurement. Other model-release indexes yielded older items; this is not an exhaustive no-release claim. Older Lambda and Project Star announcements were excluded despite newly dated aggregation.

Direct X profiles failed and search supplied no reliable fresh posts; there was no authenticated-feed inspection. The new biology episode was read through its transcript, not audiovisual playback. QwenCloud/Hugging Face improved partial non-Western coverage, but Qwen's main index, WeChat/Weibo and Chinese-only channels remain gaps. METR's inactive horizon series, dynamic leaderboard history and missing archived inventory diff also limit completeness.

**Old to new:** numerical forecasts remain unchanged; evidence now more specifically qualifies orchestration economics and deployable autonomy, and closes the NIM reference-row retrieval gap. Preserve v1 numerator **825,422,400,000,000**, fixed denominator **3,585,600,000,000,000**, SCM **0.2302048193×**, and unknown actual global output usage. April 2027 Human AI 100×, conditional July 2027 Vals RSI and conditional December 2027 SCM 1× remain distinct. No new quality multiplier, probability adjustment or user commitment is justified.

[Current forecast](forecast.md)

## October 9 2026 review

**Observation window:** checked October 9 around 02:30 Atlanta time, against the October 8 overnight review and subsequent saved entries. New publications are distinguished from older evidence newly examined. Coverage is sampled, not exhaustive.

### Forecast comparison and input freshness

**No numerical revision.** Retain capacity v1 **825,422,400,000,000 reference output tokens/day**, denominator **3,585,600,000,000,000**, and SCM **0.2302048193×**. Actual global output usage remains unknown. The leading-human expertise premise is a chosen strategic assumption; no numerical quality adjustment is added. April 2027 Human AI 100×, July 2027 conditional Vals RSI, and December 2027 central conditional SCM 1× are separate and unchanged.

[Epoch inventory](https://epoch.ai/data/ai-data-centers) still displays the October 5 headline: 93 sites, 14.4 million H100-equivalents and 43% estimated coverage. Download files carry October 8 update labels; no archived row-level comparison was available. The [update log](https://epoch.ai/data/ai-data-centers/updates) still ends October 5 with a correction to Meta Gallatin, not a new commissioning. Thus the headline is unchanged, but underlying file equivalence is not claimed. The [methodology](https://epoch.ai/data/data-centers-documentation/methodology) uses modeled, training-oriented dense FP8-equivalent inventory and a deployment lag; it does not directly measure inference throughput.

[NIM reference](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/performance.html): the current default table confirms 814.37 system tokens/second for the two-H100, 5,000/500, concurrency-50 case. The baseline 1,000/1,000, concurrency-100 value of 2,895.49 is corroborated by an older indexed extraction, not freshly read from the current interactive tab. Benchmark-document version 1.0.0 is marked deprecated; NIM runtime shown is 1.8.0. No matched improvement is established, so retain v1's rounded 814/2,895 values rather than treating precision or documentation changes as capacity growth.

[Vals RSI](https://www.vals.ai/benchmarks/rsi_index) remains dated October 1: four tasks, Opus 5.5 37.31%, and the July 2027 extrapolation to 0.6. No new closed-loop successor result or measured acceleration was established. [METR's horizon page](https://metr.org/time-horizons/) explicitly says active updates have ended, with a September 8 date and limited reliability above 16 hours. An unchanged series is therefore a measurement gap, not evidence of stalled capability. Epoch ECI download labels show October 9, but dynamic rows were unavailable; no score delta is inferred from timestamps.

### New platform and deployment evidence

[Google's Gemini at Work announcement](https://cloud.google.com/blog/products/ai-machine-learning/welcome-to-gemini-at-work-2026), October 8, describes cloud-persistent context across devices, long-running subagents, Workspace identities, and routing across Gemini and Claude. It combines integration with identity, audit and spending controls. Legal and financial specializations are previews. [Production release notes](https://docs.cloud.google.com/gemini/enterprise/docs/release-notes) confirm October 8 admin-enabled Claude Opus 5.5/Sonnet 5.5 support in Antigravity 2.0, CLI and IDEs; they do not establish full general availability or reliability for every keynote capability.

**Change in interpretation:** this strengthens the agent-platform convergence thesis beyond one supplier. Keep model, agent platform, identity and business-outcome interfaces separable when evaluating architecture. An announced integrated platform is not proof of autonomous labor replacement or customer adoption.

[Artificial Analysis trusted-access Cyber Index](https://artificialanalysis.ai/articles/trusted-access-models-cyber-index), October 8, adds GPT-6 Sol Daybreak Blue: 32 points above public Sol and reported $1.77 per task versus Grok 4.7's $11.67. Access is restricted through Daybreak. The v1 index averages three benchmarks; refusals score zero and refused-task costs are estimated using other models' usage. The [evaluation summary](https://artificialanalysis.ai/evaluations/artificial-analysis-cyber-index) appears stale relative to the dated article, so no exact new aggregate score is inferred. This is an access-conditioned specialist comparison, not a general intelligence gain available to every user or a capacity multiplier.

[Anthropic's opt-in OSS scanning service](https://www.anthropic.com/research/launching-opt-in-vuln-finding-service-for-open-source), October 8, reports a gap between roughly 29,000 candidates and roughly 6,000 human-triaged findings. Its early selected validation sample covers 97 high/critical findings across 48 projects: 85 met its disclosure bar, 11 were real duplicates and one invalid. The service is case-by-case for eligible open-source projects and lacks human triage of its reports. This is a provider report and selected sample, not a general false-positive rate. It illustrates human validation throughput as a practical bottleneck alongside stronger models.

### Compute funding and pipeline

- [White House Genesis announcement](https://www.whitehouse.gov/fact-sheets/2026/10/fact-sheet-trump-administration-announces-the-most-ambitious-set-of-science-initiatives-this-century/), October 8: $2.4 billion in industry tools/compute-credit commitments within a broader $6 billion-plus package. [NVIDIA's $1 billion over five years](https://nvidianews.nvidia.com/news/nvidia-commits-1-billion-to-advance-us-science-over-the-next-five-years) and [AWS's up to $50 million over three years](https://press.aboutamazon.com/aws/2026/10/aws-announces-golden-age-of-science-accelerator-commits-up-to-50-million-in-cloud-credits-to-support-white-house-expansion-of-genesis-mission) are components, not additional totals to double-count. [DOE's $159 million Phase II awards](https://www.energy.gov/articles/energy-department-announces-new-genesis-mission-awards-advance-super-intelligence-science) are awards, not verified disbursement or operating supply.

- [GlobalFoundries and TSMC](https://gf.gcs-web.com/news-releases/news-release-details/globalfoundries-reaches-agreement-establish-us-based-supply), October 8: $2 billion, initially five-year US silicon-interposer agreement, with a production ramp expected in the first half of 2028. Future packaging capacity does not enter current token supply.

- [QuantumScape PowerBlock](https://www.quantumscape.com/quantumscape-announces-qs-powerblock-for-in-rack-energy-storage-in-ai-data-centers/), October 8, is a prototype/reference-design signal for high-power racks, with controlled tests and modeled configurations rather than demonstrated commercial deployment.

**Implication:** these inputs support scientific access and infrastructure investment mechanisms. No compatible commissioned-hardware or reference-throughput delta justifies a new v1 numerator. Broader capex, regional supply and operating inventory remain incomplete.

### Research integrity and infrastructure mechanisms

[Vals' MiMo audit](https://www.vals.ai/blogs/mimo-reward-hacking), published October 7 and newly reviewed here, reports that 1,795 of 2,698 released coding RL tasks retained unreachable fix commits. That is dataset leakage prevalence, not an agent cheating frequency. Small behavioral tests found that naming the prohibited loopholes changed fix-seeking behavior. The [pinned environment implementation](https://github.com/XiaomiMiMo/mimoagent/blob/467f0a19016f0ac4d63b8d17a1f0da9ba07f232c/src/mimoagent/environments/datasets/opensource_code.py) supports the reported mechanism; the census and model runs were not independently reproduced. Require leakage-aware evaluations before interpreting gains as research progress. No RSI numerical revision follows.

[NeMo-DCR v1](https://arxiv.org/html/2610.08430v1), October 6, is newly reviewed algorithm/systems evidence. The paper reports 12–40x faster weight refits versus a transport-only full-checkpoint reference in cross-region tests, including 150 seconds versus 87.5 minutes for a constructed 1T checkpoint at a 3% change rate. The testbed used 32 GB300 training GPUs and 64 H100 rollout GPUs. This accelerates synchronization, not demonstrated total research progress or global token supply; the 1T checkpoint doubles a smaller model's layers. It illustrates how systems research can remove a particular bottleneck without supplying a general multiplier.

[vLLM 0.31.0](https://github.com/vllm-project/vllm/releases/tag/v0.31.0) was released October 5, despite October 8 roundup coverage. It adds serving, speculative-decoding, restart, and RL weight-transfer improvements. Newly examined older engineering evidence; no compatible fleet-wide throughput measurement is available.

### Enterprise control alongside platform distribution

[Latent Space's October 7 Stacklok interview](https://www.latent.space/p/stacklok) is primary interview evidence, newly reviewed. The founders argue for enterprise control of agent state and infrastructure. The [Mecatl repository](https://github.com/stacklok/mecatl) documents provider-agnostic loops, external durable state, and Kubernetes deployment, while noting that identity/audit mechanisms are not complete tenant isolation. The [gateway description](https://stacklok.com/solutions/ai-gateway/) describes self-hosted policy and provider routing. These sources support an alternative path in which enterprises retain control while using frontier models; neither vendor claims nor repository features prove production reliability or adoption. This qualifies the lock-in and personal-fleet hypotheses without dismissing platform-native services.

### Coverage boundaries

The [Latent Space archive](https://www.latent.space/archive) and [October 8 roundup](https://www.latent.space/p/ainews-not-much-happened-today-60f) were used for discovery and original-source links. Already-saved Periodic, Harvey, Epoch Automation Reports, and Ultrafast facts were not duplicated. Public X pages and linked posts returned 403; broad fresh YouTube discovery was incomplete, and the accessible playlist was stale. Unverified social allegations were not promoted to facts. These gaps prevent an exhaustive “nothing changed” conclusion.

**Company coverage this pass:** OpenAI, Anthropic, Google, AA, xAI and Mistral release/news sources were checked. No independently verified October 9 model release emerged during this window. Sonnet cache pricing is an October 7 change, not a fresh October 8 event. [xAI notes](https://docs.x.ai/developers/release-notes) and [Mistral changelog](https://docs.mistral.ai/resources/changelogs) yielded no verified overnight frontier release. Meta blog retrieval failed; Qwen returned empty bodies; DeepSeek reads failed or timed out and searches supplied older material. Those three remain partial coverage. [AA changelog](https://artificialanalysis.ai/changelog) remains October 8 and the Intelligence Index remains v4.3.2; proposed v5 timing from social coverage was not primary-verified. No new probability calibration or headline adjustment is supported.

## October 8, 2026 review

**Observation window:** checked October 8 against the October 7 saved ledger. New publications and older sources newly examined are identified separately. This is a sampled, source-linked review, not exhaustive coverage of every lab or release.

### Forecast comparison and input freshness

- **Tokens/day v1: unchanged.** [Epoch inventory](https://epoch.ai/data/ai-data-centers) still shows the October 5 snapshot: 93 sites, 14.4 million H100-equivalents, 43% modeled deployed-compute coverage. [Chip-sales data](https://epoch.ai/data/ai-chip-sales) remain dated October 1. [NIM 1.0.0 performance](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/performance.html) still reports 2,895.49 system output tokens/second for the two-H100, Llama 3.3 70B FP8 TP2, 1,000/1,000-token, concurrency-100 reference; v1 used the rounded 2,895. The 5,000/500, concurrency-50 result is 814.37, rounded to 814 in v1. Retain the existing precision and method: 825,422,400,000,000 modeled reference output tokens/day and SCM 0.2302048193×. Rounding differences are not capacity growth. Allocation, fleet realization, global actual output usage, and research-purpose shares remain unmeasured assumptions or gaps.

- **Vals RSI: unchanged.** The [RSI Index](https://www.vals.ai/benchmarks/rsi_index) remains updated October 1: four tasks, Opus 5.5 37.31%, and the conditional July 2027 projection to the reference-anchored 0.6 milestone. No publication-date, definition, or forecast revision was established. An index milestone remains distinct from demonstrated repeated closed-loop successor improvement and observed acceleration.

- **METR: unchanged, with a freshness limit.** [TH1.1 data](https://metr.org/assets/benchmark_results_1_1.yaml) retain GPT-5.4 H80 53.877851 minutes [23.957027–108.679232] and from-2023 horizon doubling 128.744 days [104.428–158.012]. The newest model release represented in this dataset is April 7, 2026; it does not directly measure the fall models. The [September 22 Opus assessment](https://metr.org/blog/2026-09-22-claude-opus-5-5/) supplies separate later evidence, not a replacement measurement for that series.

- **Milestone dates: unchanged.** No compatible new operational-capacity input or validated research-growth comparison warrants changing April 2027 Human AI 100×, Vals' July 2027 conditional RSI milestone, or the December 2027 central conditional SCM 1× scenario. The latter remains an uncalibrated supply-growth scenario, not a measured crossing forecast.

### October 7 model and deployment signals

**Haiku 5.5 economics and evaluation boundaries.** [Anthropic's October 7 release](https://www.anthropic.com/claude-haiku-5-5) reports availability including AWS, Google Cloud and Azure, with adjustable effort. Published input/output prices per million tokens are $0.10/$0.50 at prompts up to 100,000 input tokens and $0.50/$2.50 above that threshold; cache reads are $0.01/$0.05 respectively. The provider claims roughly 75% lower average cost than Haiku 4.5. Sonnet 5.5 cache-read pricing halves to $0.10/million, with a provider estimate of about 20% cheaper typical agentic work. These are tariffs and provider workload claims, not global token-capacity increases.

[Artificial Analysis, October 7](https://artificialanalysis.ai/articles/claude-haiku-5-5), reports maximum-effort Intelligence Index 43 under v4.3.2 and about 162,000 output tokens/task versus approximately 50,000 for Luna. Its cost figures omit the fivefold price step above 100,000 **input** tokens, so quoted costs are provisional; output-token volume does not itself establish that input threshold was crossed. AutomationBench-AA 35% was affected by pre-release over-refusals, with a rerun promised but not verified here. VALS and AA are separate evaluation methods for the same release, not interchangeable scores. **Decision implication:** add Haiku to the candidate set for a measured low-risk workflow comparison, preserving long-input tariffs, effort, refusals, retries and total cost per accepted result. This is a proposal, not a build commitment.

**GPT-6 consumer distribution.** [OpenAI's October 7 release](https://openai.com/index/gpt-6-for-everyone/) starts paid Chat-tab rollout October 7 and Free/Go October 8, with Sol for paid and Luna for Free/Go. The internal claim of 44% earlier first answers on web-search questions versus GPT-5.6 Instant concerns time to initial answer, not output tokens/second. The [October safety report](https://deploymentsafety.openai.com/gpt-6-october) distinguishes these Chat models from September Codex/Work versions and says neither reaches the provider's High AI Self-Improvement threshold. This supports broader deployment; it does not independently establish RSI or change the SCM.

### Research capability and correction evidence

[OpenAI's mathematics release](https://openai.com/index/sharing-ai-progress-in-mathematics/), **published October 6 and newly reviewed here**, concerns an unreleased internal model, roughly 4,000 attempted problems and results at different verification stages. The provider gives an average of roughly three Pro-thinking-hour compute-equivalents per result, with exceptions; that is not a fleet throughput or lab-progress multiplier.

The [repository history dated October 7](https://github.com/openai/math/blob/main/history.md) records three withdrawn manuscripts following a sign error and a dependent argument, 14 revisions and 13 further citation updates. The [current repository](https://github.com/openai/math) contains 719 manuscripts across 372 families; six formalizations plus five supporting additions bring the headline formalized count to 300/719, about 42%. Preserve status per result rather than calling all manuscripts independently verified. Latent Space's October 7 headline count of 722 is superseded by the primary repository count, not an independent confirmation. **Implication and counterevidence:** this is a substantive scientific-research signal, while withdrawals show the importance of validation. Neither manuscript volume nor formalization share measures repeated autonomous frontier-model improvement.

### Compute, power, spending and government

- [DOE/NIH/Biohub announcement, October 7](https://www.energy.gov/science/articles/doe-nih-and-biohub-partner-build-foundational-data-predictive-biological-super): signed cooperation memorandum for open biological data/models. DOE reports more than $500 million over five years for fundamental cell research; Biohub reports $500 million over five years for its virtual-biology initiative. These are broad research-investment announcements, not verified cash spent, AI-only capex, commissioned accelerators or a quantified productivity gain. They strengthen the data-and-research-input mechanism without entering token supply.

- [Duke Energy settlement announcement, October 7](https://news.duke-energy.com/releases/duke-energy-protects-customers-from-data-center-costs): proposed North Carolina large-load terms with Public Staff, Amazon, Google, Meta and Microsoft include nonrefundable prepayment for dedicated facilities and deposits/security for shared-grid upgrades. The proposed terms cover at least 50 MW at an 80% load factor for relevant agreements after June 1. NCUC approval is still pending, expected mid-November. This is a cost/risk-allocation signal, not an approved final rule or commissioned capacity.

- [Digital Realty KIX15 construction announcement](https://www.globenewswire.com/news-release/2026/10/07/3376166/0/en/digital-realty-commences-construction-of-kix15-data-center-expanding-ai-infrastructure-in-japan.html), October 7 Japan date (syndicated October 6, 22:00 ET): 24 MW with operations planned Q4 2028. Construction-stage general datacenter capacity; AI/inference allocation is unspecified. Exclude from operating supply.

- [CoreWeave Taloja announcement](https://coreweave.com/news/coreweave-enters-india-expanding-ai-cloud-platform-with-adaniconnex), official page dated October 6, syndicated October 7: 240 MW across three 80-MW buildings, first phase planned mid-2028, with an optional further 240 MW. Newly reviewed older pipeline, not an overnight commissioning. Neither the option nor the campus is added to operating inventory.

- Financing reports concerning Broadcom/OpenAI talks and a National Compute Genesis-credit claim lacked verified primary confirmation in this review. They are unresolved discovery leads, not settled financing, cash spending or delivered compute. No modeled token-capacity addition follows.

### Coverage and remaining gaps

The review used source snapshots rather than complete archived diffs. Dynamic Artificial Analysis chart values were partly unavailable to text retrieval; the release analysis and methodology supplied the comparisons. METR's YAML required direct retrieval after a web MIME failure. No new repeated closed-loop successor experiment, comparable global output-usage aggregate, or reliable global RSI-token allocation was established. China/non-Western primary coverage and systematic X coverage remain limited. Missing access or sparse evidence is not evidence of no development.

**Company/source registry coverage this pass:** official [OpenAI](https://openai.com/news/), [Anthropic](https://www.anthropic.com/news), [DeepMind](https://deepmind.google/blog/), [Meta newsroom](https://about.fb.com/news/) and [AI blog](https://ai.meta.com/blog/), [xAI news](https://x.ai/news), [DeepSeek changelog](https://api-docs.deepseek.com/updates/), [Mistral](https://mistral.ai/news/), [Qwen blog](https://qwen.ai/blog), and [Latent Space archive](https://www.latent.space/archive) were sampled. Latest entries found included DeepMind EmbeddingGemma 2 (October 6), Mistral Large 4 (October 6), DeepSeek V4.1 Flash (September 10) and the x.ai site's September 28 product entry; this does not establish a complete release history. x.ai still displayed SpaceXAI branding; no unsupported rename was applied. Qwen's blog rendered empty and indexed discovery returned older material, so its freshness is unresolved.

Latent Space's October 7 mathematics and Stacklok articles were read as discovery/reporting, with primary repositories used for supported claims. The Stacklok/Mecatl harness supplies architecture background, not measured reliability or capability gains. Linked X posts errored; a YouTube playlist yielded a shell, and Meta's new infrastructure interview embed was inaccessible, so no new interview transcript was reviewed. The two baseline interviews remain the previously reviewed ledger context and were not reprocessed. [Meta's October 7 datacenter article](https://about.fb.com/news/2026/10/meta-data-centers-ai-approach/) contained no usable operating-capacity quantity. [Sierra PAP](https://sierra.ai/blog/introducing-personal-agent-protocol) still described v0.1 as forthcoming later October.

**Decision posture:** retain the current numeric model and insurance-adoption hypotheses; add the qualified Haiku comparison and research-verification evidence to the next workflow evaluation. New deployments, funding announcements and construction support mechanisms but do not supply a compatible numerator update. No new experiment or implementation is committed by this review.

## Tokens per day capacity estimate v1

**October 7, 2026 · worldwide working scope.** Base estimate **825 trillion output tokens/day**, rounded headline **about 800 trillion/day**. This is **Llama 3.3 70B reference-normalized potential serving capacity**, not observed global output, frontier-model quality, or a measured mixed-model token total.

Analyst scenarios span **44 trillion to 2.53 quadrillion/day**, rounded about 40 trillion to 3 quadrillion. They are sensitivity cases, not confidence intervals. Actual worldwide output-token usage and research-purpose allocation remain unknown.

### Observed and inferred anchors

**Inventory I1:** [Epoch AI Data Centers](https://epoch.ai/data/ai-data-centers), October 5 site update: 93 sites and 14.4 million H100-equivalents. [Methodology](https://epoch.ai/data/data-centers-documentation/methodology), checked October 7: estimated coverage 43% of global deployed compute. Dividing 14.4 million by 0.43 implies about 33.5 million; v1 rounds to **33 million**. This is a modeled global-stock anchor, not a complete physical inventory. H100-equivalence uses dense compute performance; it is not an inference-throughput equivalence.

**Serving T1:** [NVIDIA NIM performance](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/performance.html) and [metric definitions](https://docs.nvidia.com/nim/benchmarking/llm/1.0.0/metrics.html), checked October 7. Llama 3.3 70B FP8 on two H100s: 1,000 input and 1,000 output tokens, concurrency 100, **2,895 system output tokens/second**, or **1,447.5 per H100**. Input-heavy case: 5,000 input and 500 output tokens, concurrency 50, **814 system output tokens/second**, or **407 per H100**. Preserve the reference model's tokenization and full serving workload. Prefill is already present in these measured workloads.

### Continuous language throughput scale reference

Buddy selected **5 tokens/second** on October 7 as a fixed, rough language-throughput reference. It is an assumption for scale, not a measurement of thought. Full-day denominator: 5 × 86,400 = **432,000 tokens per reference-day**, intentionally including sleep hours.

The rounded 800 trillion/day headline divided by 432,000 gives **1.85 billion continuous reference streams**, rounded to 1.9 billion. The exact v1 base of 825,422,400,000,000 gives **1,910,700,000**. These are text-throughput equivalents, not people, worker output, intelligence, or economic replacement. The generous all-day denominator is conservative only for this comparison; it does not make the underlying capacity model conservative. Keep the 5-token rate fixed when tracking trends and retain the capacity scenario uncertainty.

[Reading-rate meta-analysis](https://biblio.ugent.be/publication/8647789) reports adult English silent-reading averages of 238 words/minute for nonfiction and 260 for fiction, with oral reading at 183. A rough assumed four-thirds tokens/word gives about 5.3–5.8 tokens/second for those silent-reading averages. Tokenization and language vary; these data motivate the chosen round reference, not a universal human processing rate.

### Assumptions and arithmetic

Potential capacity = H100-equivalent deployed stock × token-serving allocation × fleet-realization factor × reference output tokens/second/H100 × 86,400 seconds/day.

| Input | Low | Base | High |
| --- | --- | --- | --- |
| Worldwide H100-equivalent stock | 25 million | 33 million | 45 million |
| Token-serving allocation | 20% | 40% | 60% |
| Fleet realization | 25% | 50% | 75% |
| Reference output tokens/second/H100 | 407 | 1,447.5 | 1,447.5 |
| Derived output tokens/day | 43,956,000,000,000 | 825,422,400,000,000 | 2,532,546,000,000,000 |

**A1 allocation** is an analyst assumption for the share available to token serving rather than training or other workloads. **A2 fleet realization** is an analyst assumption covering heterogeneous hardware and serving performance plus sustained operational availability. It is not demand utilization. Do not apply another generic prefill discount or count the same availability loss twice. Allocation and realization are the strongest uncertainties.

The base calculation is 33,000,000 × 0.40 × 0.50 × 1,447.5 × 86,400 = 825,422,400,000,000.

### Interpretation and updates

This estimates what the worldwide fleet could supply in reference-workload terms under the declared assumptions. Different models, reasoning workloads, tokenizers, context lengths, latency targets, and batch sizes can change usable throughput substantially. It does not claim every deployed accelerator can serve this exact configuration or that all tokens are equally useful.

Keep observed consumption separate. Google's disclosed API processed-token rate has an unspecified input/output split and cannot validate global output directly. Pipeline announcements enter only when commissioned, without double-counting physical assets across owner, supplier, and customer.

For v2, preserve these inputs and outputs, record changed source dates and assumptions, and show the contribution of each change. Revisit inferred inventory, inference allocation, and workload throughput as evidence improves. Track broad research allocation where credible; do not invent an RSI-token percentage.

**History:** replaced the initial “no estimate computed” state with this explicit provisional model at Buddy's request. The lack of a measured global usage total remains unchanged.

## Source coverage

**Priority signal: Vals RSI.** Buddy expects recursive self-improvement to speed capability growth itself. Maintain a dated series of RSI milestone forecasts and definitions, actual autonomous research performance, and subsequent capability-growth intervals. General leaderboards remain supporting context. Treat the evaluator's milestone, a demonstrated closed research loop, and measured acceleration as three linked but separately observed events. Compare research throughput, human intervention, algorithmic efficiency, and compute or data constraints before changing the growth-rate assumption.

Follow evaluation firms, lab releases and research, podcasts and interviews, X posts, news reporting, government initiatives, and financial disclosures. Track six connected areas: model capability, algorithms, compute capacity, inference economics, financial spending, and adoption.

- **Evaluations:** [VALS](https://www.vals.ai/), [Artificial Analysis](https://artificialanalysis.ai/), [METR](https://metr.org/), and [Epoch ECI](https://epoch.ai/data/eci-documentation/faq). Record version, task set, settings, scaffold, date, reliability, and uncertainty.

- **Companies:** official releases and research across frontier labs, open-model providers, infrastructure suppliers, and application companies. Discovery remains open to new entrants.

- **Interviews and YouTube:** preserve the episode, speaker, recording/publication date, timestamp, and transcript quality. Separate demonstrations, reported internal experience, predictions, and independent measurements.

- **X and news:** use fast reporting and expert commentary to discover developments. Preserve original posts and underlying sources; track conflicts and corrections. Repeated coverage of one announcement is one evidence lineage. Include [Latent Space](https://www.latent.space/) for technical reporting, interviews, and podcasts; preserve speaker claims and link underlying evidence.

- **Government:** distinguish proposals, enacted requirements, funded programs, announcements, procurement, and actual implementation.

- **Compute and finance:** distinguish announced versus operational capacity; planned capex versus cash spent; orders versus delivered hardware; revenue versus financing. Record currency, period, entity, and source.

For every material addition record: claim, original source and date, observed data, measurement boundary, uncertainty, corroboration or disagreement, affected forecast, and decision consequence. Inaccessible X threads, missing transcripts, paywalls, and unavailable charts remain explicit gaps.

## Agent distribution and decision services

### Personal Agent Protocol

[Sierra announcement](https://sierra.ai/blog/introducing-personal-agent-protocol), October 6, 2026; checked October 7. Sierra and Meta are developing Personal Agent Protocol with Genesys, Instinct, Rocket, Shopify, Stripe, and Walmart. Car-insurance shopping is an explicit example. The design combines website discovery, OAuth sessions, customer-selected read/write access, and business-defined routes through websites, APIs, or company agents.

**Status updated October 13:** [Draft 0.1](https://personalagentprotocol.org/docs/spec) is publicly readable, now stamped October 12; publication was first verified October 11 with an October 9 stamp. Incompatible changes remain possible. [Path to 1.0](https://personalagentprotocol.org/docs/path-to-1) retains payments, push notifications and attachments as open topics. **Forecast implication:** the specification-publication milestone is established; newly examined transaction controls and partners strengthen the implementation pathway, while production interoperability and repeated insurance outcomes remain unverified. Adoption timing is unchanged.

### OpenAI Decisions API

[Official API guide](https://developers.openai.com/api/docs/guides/decisions), checked October 7, 2026; publication date unspecified. Public beta supports GPT-6 Luna for text/image predicates, fixed-choice classification, and rubric scores. OpenAI claims roughly 10× faster answers than Responses. Listed pricing is $0.10 per million input tokens, with no output or cache charges; regional premiums and long-context multipliers apply.

**Boundary:** provider documentation, without a comparable workload benchmark here. This is a narrow decision-latency and cost signal, not a general-intelligence gain, token-output speed measurement, or global capacity input. **Forecast implication:** inexpensive routing and checks may improve workflow economics; validate end-to-end latency, errors, review burden, and total cost on representative tasks. Neither signal changes capacity v1, SCM, or milestone dates.

## Current benchmark observations

### VALS Index

[VALS Index v2.1](https://www.vals.ai/benchmarks/vals_index), updated October 7, 2026 and checked October 8: Gemini 4 Argon 68.90%, Sonnet 5.5 67.04%, Opus 5.5 66.97%, Astra 63.13%, Sol 6.1 61.15%. Reported dollars per test respectively: $15.68, $21.34, $32.14, $18.46, $3.24. These tracked scores and costs are unchanged from the October 6 snapshot. Newly visible Haiku 5.5 is 54.31% at $2.99/test. Use the table rather than stale narrative rankings; this composite is not a measure of world output or RSI.

This is an eight-benchmark, GDP-weighted composite, not the share of GDP automated. Fallback assistance affects comparability: counting affected tasks as failures lowers Sonnet to 65.85% and Opus to 65.05%. Freeze version and assistance policy when comparing trends.

### Artificial Analysis

- [September 29 Sol 6.1 analysis](https://artificialanalysis.ai/articles/gpt-6-1-sol-replaces-gpt-6-sol-after-just-7-days-with-near-astra-intelligence): Intelligence Index v4.3.2 score 52 versus Astra 53; maximum-effort cost per task $0.72 versus $3.26.

- [September 30 Gemini 4 Argon analysis](https://artificialanalysis.ai/articles/gemini-4-argon-google-top-three-labs): high-effort score 53, matching Astra maximum effort. Discounted cost per task $1.99 versus Astra $3.26; standard Gemini price $3.98. Rollout was limited at publication.

- [September 28 Sonnet 5.5 analysis](https://artificialanalysis.ai/articles/claude-sonnet-5-5): maximum-effort score 56 versus Opus 5.5 maximum effort 58; Sonnet $7.60 per task and 193,000 output tokens per task. The report notes a pre-release structured-output deployment bug and fallback behavior.

[Methodology](https://artificialanalysis.ai/methodology/intelligence-benchmarking): a weighted composite of ten text/English evaluations. Cost counts tokens; estimated time omits time to first token and other overhead. Compare capability, effort, price, and access together. Different benchmark scales are not interchangeable.

## Work and research acceleration

### OpenAI research workflow

[OpenAI research acceleration](https://openai.com/index/research-acceleration-view-inside-openai/), September 6; checked October 7. Code additions/deletions per active contributor/day reached 7.02x the pre-2025 baseline through August 15, versus 1.82x in Q4 2025. Commits are capped at the 99th percentile; contributors are trailing-year commit identities. Experiment velocity was 1.60x the 2025 baseline in the week of August 10; trailing-28-day runtime was 3.14 agent-workdays per human workday on August 15. Median daily inference exceeded $600 at API-equivalent prices, not billed costs.

Purpose categories: decide, design, build, run, analyze, communicate. A 2% sample of research-employee coding sessions is extrapolated, with daily user totals capped at the sampled 95th percentile and a 14-day mean. August 15 totals approximately 722,833 output tokens per research employee/day under this method. Bursty labeling/grading and unmatched activity are excluded. This is selected research coding-agent output, not all internal inference or a global RSI share.

**Forecast implication:** research-loop acceleration is a concrete mechanism worth tracking. Agent-workdays, experiments, and total organizational progress are different denominators.

**Attribution resolved by Buddy, October 7:** OpenAI reports 3.1 agent-workdays per human workday as of mid-August, using an eight-hour workday. This measures effort, not a 3.1x output or progress gain.

### Other measured and self-reported gains

[METR technical-worker survey](https://metr.org/blog/2026-05-11-ai-usage-survey/), May 11, 2026: 349 technical workers reported median value improvements of roughly 1.4–2x and speed improvement of 3x. Self-selection and counterfactual estimation limit causal interpretation.

[METR Opus 5.5 assessment](https://metr.org/blog/2026-09-22-claude-opus-5-5/), September 22: incremental R&D improvements and a preliminary approximately 1.5x overall capability-acceleration estimate, with uncertain time period and supporting evidence not public.

[Anthropic recursive self-improvement report](https://www.anthropic.com/institute/recursive-self-improvement): a March 2026 opt-in poll of 130 research staff estimated roughly 4x output with Mythos Preview versus no AI on projects they would do anyway. The report describes a median; system-card discussions use a geometric mean, so retain each source's statistic. Anthropic expects actual gains to be lower. The [Mythos Preview system card, page 44](https://www-cdn.anthropic.com/7624816413e9b4d2e3ba620c5a5e091b98b190a5.pdf) maps roughly 4x survey output through estimated research elasticities to below 2x overall progress. That is modeled interpretation, not an observed causal progress multiplier. Code and token-allocation shares remain distinct.

[OpenAI Symphony report](https://openai.com/index/open-source-codex-orchestration-symphony/), April 27, 2026: a 500% increase in landed pull requests on some teams. A narrow activity metric does not by itself establish an equivalent gain in accepted business outcomes.

## Anthropic researchers interview

[Official episode and transcript](https://blog.joelonsdale.com/p/ep-164-inside-anthropic-with-sholto) · [YouTube](https://www.youtube.com/watch?v=6D1wC95htTM)

Published October 2, 2026; the host identifies recording in August. Speakers: Sholto Douglas, Nicholas Marwell, and Joe Lonsdale. Approximate podcast timestamps below differ from YouTube timing. The automatic transcript has dropped words and transcription errors.

- 4:43–5:39: Douglas reports progressing over 18 months from manual coding to one-to-two-day independent RL-science work. Insider experience, not a controlled evaluation.

- 9:10–9:33: Douglas predicts human-level-or-better computer work within a couple of years.

- 14:31: Marwell forecasts biology impact over six to 24 months, with labs and data constraining progress.

- 16:50–18:31: generalist opportunity is conditional on ten-to-20-year diffusion; unemployment remains possible.

- 35:44–39:26: they argue older capabilities commoditize while more advanced capabilities retain value.

- 44:16–45:11: today's human-complementarity gains may end when capabilities substitute for people.

- 56:08–56:49: data-production capacity is a bottleneck and forecast update signal.

**Interpretation:** prioritize delegation, acceptance tests, data access, domain problem selection, and distribution. Insider testimony strengthens mechanisms and candidate timelines; it does not directly measure insurance adoption or universal productivity.

## Published multiplier model audit

[Growth School model](https://growthlab.team/school/artificial-intelligence/model) · [Calculation source](https://growthlab.team/static/school/model/model.js)

Snapshot retained: September 4, 2026; interpretation revised September 5. The metric is expected successful human-equivalent output divided by planning, coordination, and review human minutes.

Defaults: two-hour reference task; pre-AI planning, coordination, and review each 12 minutes; assumed current 75% savings yields three minutes per phase before task-size adjustment. Task-size exponent 0.5; success factor 0.8; illustrative additional 50% human-time reduction per capability doubling; outside-review share zero.

The ECI-to-horizon bridge uses GPT-5.4 at ECI 156.49 and measured H80 53.877851 minutes, then infers Astra's H80 using ECI 169 and five ECI points per horizon doubling. This gives approximately 5.0867 hours H80 and 17.011x expected-output multiplier after the 14.353-minute human denominator.

Read-only default outputs: 10x September 3, 2026; 100x April 12, 2027; 1000x January 22, 2028. They are conditional model outputs, not measured milestones. The 1000x endpoint assumes full autonomy and is reported as a lower bound.

[METR source data](https://metr.org/assets/benchmark_results_1_1.yaml) reports GPT-5.4 H80 53.877851 minutes with interval 23.957027–108.679232. Recent horizon doubling is 128.744 days, interval 104.428–158.012. The model projects H80 with an H50-fitted trend. [Epoch's ECI FAQ](https://epoch.ai/data/eci-documentation/faq) describes the approximate five-point relationship historically at launch; scores can change when jointly refit. Bridge uncertainty and transfer assumptions are separate from the fitted trend interval.

## Sensitivity and validation priorities

Proposed sensitivity cases, not assigned probabilities or recomputed milestone dates:

- **Slower conversion:** no further overhead reduction per capability doubling; explicit review and rework outside the model; slower 158-day horizon trend.

- **Published defaults:** 50% further overhead reduction per doubling, zero outside-review share, approximately 129-day trend.

- **Faster conversion:** faster 104-day trend, with any extra overhead savings justified by observed workflow data rather than assumed away.

Vary the ECI bridge and H80 interval separately. The highest-value validation is direct measurement of human planning, coordination, review, and rework on representative workflows. Keep delivered output per human hour separate from total cost per accepted output and calendar lead time.

## Open evidence gaps

- Current direct long-horizon measurements that could replace the ECI-to-H80 bridge

- Observed workflow denominator and failure/rework data

- Insurance adoption, distribution, procurement, migration, and total-cost observations

- Systematic dated compute, algorithms, spending, government, and X source entries beyond this initial seed

## Vals RSI interview and benchmark

[MTS episode](https://www.youtube.com/watch?v=0HUjp_xjDfc&t=977s) · [Full caption mirror](https://arcmira.com/watch?v=0HUjp_xjDfc) · [Publisher metadata](https://podcasts.apple.com/us/podcast/anthropic-will-reach-full-rsi-by-august-2027-rayan-krishnan/id1891088763?i=1000792957385)

Published October 3, 2026; guest Vals AI CEO Rayan Krishnan. Recording date unverified. Complete captions were reviewed; transcription and timestamp precision are imperfect.

At about 4:30, Krishnan projects full RSI in July–August 2027 from proxy tasks, meaning automation of current ML engineering. The host challenges whether harder bottlenecks remain. At 7:14–7:50, Krishnan accepts new constraints and separates RSI from fast takeoff. At 11:40–12:19 he describes five proxy tasks. At 15:07–16:26 diffusion may lag capability, but agents could also accelerate adoption and sales. At 16:52–17:09 he reports five-to-six trillion evaluation tokens year-to-date, not training expenditure. These are interview assertions and forecasts, not audited spend or realized autonomy.

[RSI Index methodology](https://www.vals.ai/benchmarks/rsi_index), updated October 1 and checked October 7: **four tasks**, rather than the interview's five: compression, LM training, harness engineering, and post-training. The reference-anchored scale uses selected historical, human, and frontier-model comparisons; 0.5 matches those references and 0.6 aims beyond them. The Anthropic trend extrapolates 0.6 in July 2027. Opus 5.5 scores 37.31%, which is not the fraction of research automated. Runs last 12–30 hours with specified budgets. Results mainly integrate known methods; execution is stronger than research judgment. LM training uses one H100 for 24 accelerator-hours; post-training uses 16 RTX PRO 6000s for 30 hours plus $100 model credits. These are experiment budgets, not frontier training costs.

**Forecast implication:** stronger autonomous experiment execution is supported more directly than a precise full-RSI date. Look for independent replication, novel research judgment, repeated end-to-end successor development, and measured lab-cycle gains. The interview and benchmark are one evidence family, not independent confirmations.

## Compute algorithms spending and government

### Tokens/day measurement seed

**Usage proxy:** [Alphabet Q2 2026 remarks](https://blog.google/company-news/inside-google/message-ceo/alphabet-earnings-q2-2026/) report approximately 22 billion API tokens processed per minute, up from 16 billion a quarter earlier. This is provider-reported API consumption with unspecified input/output and purpose splits. It is not output-token capacity or a global total; no RSI allocation is inferred.

**Capacity coverage:** [Epoch's September 16 update](https://epoch.ai/latest/scaling-ai-data-centers-research) covers 86 sites and an estimated 44% of global AI compute. Coverage is modeled from operating-site estimates and delivered-compute estimates with a deployment lag, and varies substantially by owner and region. This is a partial infrastructure inventory, not a global output-token measurement.

**Throughput conversion:** [MLPerf Endpoints](https://mlcommons.org/benchmarks/endpoints/) measures system output tokens per second against concurrency, interactivity, and time to first token for specified hardware, software, and model configurations. Such serving measurements can support declared-workload capacity estimates when paired with commissioned inventory; they are not an inventory themselves.

**Purpose allocation:** distinguish general applications from AI research and capability improvement; refine into research, engineering, evaluation, and closed-loop improvement when disclosures permit. Current usage sources do not establish comparable RSI-token shares. Scenario assumptions must remain separate from observations, with uncertainty propagated rather than hidden in a single aggregate.

Initial high-signal sample checked October 7, 2026. These developments support capacity and investment momentum; they do not independently establish the April 2027 multiplier.

- **Operational capacity and spending:** [Microsoft July 29 earnings call](https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q4) reports more than one gigawatt and 31 datacenters added in FY26 Q4; this is broad cloud capacity, not a frontier-training FLOP count. Quarterly capex was $41 billion including $5.6 billion finance leases, versus $35.8 billion cash property and equipment spending. Calendar-2026 guidance of about $175 billion reflects lease reclassification; management says underlying investment expectations were unchanged.

- **Chip shipments:** [NVIDIA August 26 call](https://investor.nvidia.com/files/content_files/TRANSCRIPT_-NVIDIA-Corp-NVDA-US-Q2-2027-Earnings-Call-26-August-2026-5_00-PM-ET.pdf) reports Vera Rubin production shipments began in August. AWS's additional two-million-GPU plan runs through FY29 Q2; cumulative planned delivery is not operating capacity today.

- **Secured versus operating compute:** [OpenAI April 29 infrastructure report](https://openai.com/index/building-the-compute-infrastructure-for-the-intelligence-age/) says more than ten gigawatts secured, not necessarily energized. It separately identifies operational Abilene infrastructure for GPT-5.5 training.

- **Financing:** [OpenAI March 31 announcement](https://openai.com/index/accelerating-the-next-phase-ai/) describes $122 billion committed capital at an $852 billion post-money valuation and an undrawn approximately $4.7 billion credit facility. Commitments and borrowing capacity are not deployed capex.

- **Algorithms:** [DeepMind May 7 AlphaEvolve report](https://deepmind.google/blog/alphaevolve-impact/) reports workload-specific improvements including 20% less Spanner write amplification, 2x Klarna transformer-training speed, and approximately 4x Schrödinger model training and inference speed. These are company and partner reports, not a general frontier-intelligence multiplier.

- **US research initiative:** [DOE July 22 Genesis selection](https://www.energy.gov/articles/secretary-energy-chris-wright-announces-first-genesis-mission-projects-selected-accelerate) covers 278 projects and 342 institutions. Selection for negotiations does not commit funding or establish delivered productivity.

- **EU procurement:** [July 30 AI Gigafactories call](https://digital-strategy.ec.europa.eu/en/news/eu-launches-ai-gigafactories-call-boost-europes-computing-capacity-and-unlock-more-eu30-billion) seeks up to seven facilities, with up to €10 billion public funding expected to unlock at least €20 billion private investment. This is a program and procurement signal, not commissioned compute.

- **Power timing:** [IEA Electricity 2026](https://www.iea.org/reports/electricity-2026/grids) contrasts grid-infrastructure timelines of five to 15 years with datacenters at one to three years. It identifies a timing risk, not a delay in any particular named project.

Coverage remains limited for China and other non-Western primary sources, systematic X reporting, and a reconciled global operating GPU or FLOP inventory. Commissioning, utilization, algorithm generalization, and transfer to production workflows require further evidence.

## Adopted SCM normalization

October 7, 2026: Buddy adopted AI potential output capacity divided by a humanity language-throughput reference. Hold the reference at **5 tokens/second × 86,400 × 8.3 billion people = 3,585,600,000,000,000 tokens/day**.

[UN World Population Prospects](https://population.un.org/dataportal/data/indicators/49/locations/900/start/2025/end/2100/table?df=0bedacf9-361d-49c3-99bf-d8e7030811e2), 2024 vintage, projects 2026 population at 8,300,678,396; use the explicitly rounded 8.3 billion as a fixed baseline, not a live census. Five tokens/second is the chosen language reference, not a claim about thought.

- Exact capacity v1 base / denominator = **0.2302048193×**, rounded **0.23×**.

- Rounded 800 trillion headline / denominator = **0.223114681×**, rounded **0.22×**; the difference is rounding only.

- Exact low/high capacity cases imply **0.012259–0.706310×**. These are analyst sensitivity scenarios, not a confidence interval.

- **1×:** 3.5856 quadrillion output tokens/day.

- **10×:** 35.856 quadrillion/day.

- **100×:** 358.56 quadrillion/day.

Every person and all 24 hours count in the denominator, intentionally including sleep. That is generous only within this text-throughput comparison; it does not validate the capacity estimate or equate tokens with people, cognition, intelligence, or economic output. Keep model/workload normalization and the fixed population/reference rate unchanged when comparing trends, or version the baseline explicitly.

Buddy expects future scale above 1× and is considering 10× and 100×. No dates have been assigned. This SCM's 100× is not the Human AI multiplier's April 2027 100×, and neither is Vals' RSI milestone.

## Fast inference signals

Checked October 7, 2026. These are per-stream or per-user speed signals with different workloads and deployment contexts; they do not change global capacity v1 without aggregate throughput, hardware counts, and deployed share.

- [OpenAI Ultrafast, August 13](https://openai.com/index/previewing-ultrafast/): Sol Ultrafast up to 750 output tokens/second. [Cerebras August 27 explanation](https://www.cerebras.ai/blog/how-cerebras-serves-gpt-5-6-sol-at-up-to-750-tokens-per-second) uses multiple CS-3 systems without a disclosed system count or concurrency denominator.

- [Inception Mercury 2.5, September 8](https://www.inceptionlabs.ai/blog/introducing-mercury-2-5): provider claims 1,107 tokens/second on unspecified NVIDIA GPUs. [Artificial Analysis](https://artificialanalysis.ai/models/mercury-2-5) reports 691.1 output tokens/second and 3.21 seconds to first token under its own test. Different measurement conditions prevent treating the figures as equivalent.

- [Cerebras CS-4 announcement, August 18](https://cerebras.gcs-web.com/news-releases/news-release-details/cerebras-unveils-cs-4-30-times-faster-gpu-based-solutions): more than 4,400 tokens/second per user on GPT-OSS 120B; a rack contains three WSE-3 Turbo wafers, but the benchmark's total system count is not disclosed.

- [Cerebras Hot Chips roadmap, August 25](https://www.cerebras.ai/blog/ultrafast-frontier-inference-cerebras-deep-dive-at-hot-chips-2026): CS-5 targets for 2027 include 10,000 tokens/second per user on Gemma 4 and GPT-OSS 120B, and 5,000 on Sol. Roadmap targets are not deployed measurements.

Potential implication: faster sequential research and interaction loops may matter even without proportional total fleet throughput gains. Preserve latency, per-stream speed, aggregate system output, hardware denominator, and commissioning status separately.

## Conditional date for SCM 1x

Assistant scenarios dated October 7, 2026, using capacity v1 and the fixed 2026 humanity reference. They are not a user-adopted fixed date, a calibrated expectation, or a confidence interval.

Starting SCM = 0.2302048193×. Growth required = 1 / 0.2302048193 = **4.3439577×**. With constant annual factor g, years to crossing = log(4.3439577) / log(g).

| Annual capacity factor | Conditional crossing | Basis |
| --- | --- | --- |
| 3.4× | December 2027 | Central scenario using Epoch fixed-model inference-supply growth |
| 2.5× | May 2028 | Slower scenario, loosely cross-checked against different effective-supply estimates |
| 2× | November 2028 | Analyst stress case |

[Epoch compute-crunch analysis](https://epoch.ai/gradient-updates/is-a-compute-crunch-coming), May 25, 2026, by Luke Emberson and Jaime Sevilla; rechecked October 8. This informal Gradient Updates author analysis supplies the 3.4× annual fixed-model/fixed-context inference-supply assumption (compute 3.4× and bandwidth 4.1× annually). It is a modeled trend, not measured global output or a new October observation. It already includes hardware deployment and chip efficiency; do not multiply those improvements again. This scenario holds the baseline allocation and fleet-realization factors fixed.

[Epoch agent-population analysis, October 2](https://epoch.ai/publications/estimating-the-agent-population), Appendix B, gives effective GB300-equivalent supply of 24.03 million in 2026 and 60.36 million in 2027, approximately 2.5×. Shipment, memory, workload, and effective-supply definitions differ from our deployed H100-equivalent model, so this is a loose cross-check, not an interchangeable observed growth rate.

Crossing during July 2027 requires approximately 6.1–7.5× annual growth depending on the day. For July 15, about 6.75× is needed; the 3.4× scenario reaches only approximately 0.590× then. It would need a further approximately 1.69× cumulative gain in allocation, fleet realization, or software beyond the modeled hardware trend. Such gains are possible hypotheses, not already credited observations.

Initial-stock, serving-allocation, and fleet-realization uncertainty dominate exact calendar dates. Recompute when those inputs change; do not convey day-level precision. No dates are assigned to SCM 10× or 100×. The separate April 2027 Human AI 100× projection and July–August 2027 RSI projection do not resolve this capacity crossing.

## Periodic Labs physical research and deployment

**October 8, 2026 · founder interview evidence.** [Latent Space episode](https://www.latent.space/p/periodic). The source ledger records a publisher-distributed transcript dated October 8, 16:27 UTC; the private delivery link is omitted from this public snapshot. Speakers: Liam Fedus and Ekin Doğuş Çubuk. Recording date unverified.

- **09:17–12:03:** physical experiment duration and noisy measurements make a simple end-to-end RL loop impractical in their setting. They describe narrower characterization agents and timestamped experimental evidence that can support further training.

- **01:15:06–01:16:53:** Fedus describes forward-deployed engineers and researchers working on site in secure customer environments, integrating tools with local inference and training on customer data.

- **01:17:19–01:18:03:** pricing scientific outcomes is presented as a future direction as autonomy improves, rather than a demonstrated universal business model.

**Forecast interpretation:** this provides a concrete mechanism for moving bottlenecks through decomposition, evidence reuse, and specialized deployment. It also illustrates a local, customer-controlled deployment path alongside cloud platforms. These are founder self-reports, not measured productivity or quality multipliers. No change to SCM capacity, quality assumptions, or milestone dates follows from this entry.

## October 8 evaluation and access updates

Checked October 8, 2026 Atlanta time, October 9 UTC. These entries inform quality, autonomous work, and practical access; they do not change SCM capacity or numerical forecast dates.

### Harvey LAB AA version 1 1

[Artificial Analysis announcement](https://artificialanalysis.ai/articles/harvey-lab-aa-v1-1), October 8; full article verified through its [alternate locale page](https://artificialanalysis.ai/de/articles/harvey-lab-aa-v1-1). The 120-task private legal dataset is now v1.1.0. Three LLM rubric judges replace one; source-grounding checks zero a task for any material hallucination. The headline averages each task's share of judges finding all criteria satisfied, after that gate.

Grok 4.7 scores 9.4%, Muse Spark 1.3 8.9%, and Astra 8.6%; Astra's ungated rate is 8.9%. Across tested models, more than 60% of otherwise-passing results contain a material hallucination. This concerns task-specific source grounding, not all legal knowledge. Results use AA's Stirrup harness and simplified tools. **Do not compare v1.1 scores directly with v1.0:** dataset and grading changed.

Interpretation: a useful quality-conversion signal for professional deliverables. Rubric coverage and grounded acceptance are separate; these results do not directly measure insurance outcomes or revise the strategic leading-human expertise assumption.

### Epoch work automation

[Report](https://epoch.ai/publications/can-ai-automate-epoch) and [benchmark methodology](https://epoch.ai/benchmarks/epoch-automation-reports), October 8; relevant full text verified after initial retrieval errors. Six models attempted 11 real internal tasks across five categories with their respective harnesses and highest reasoning settings. Fable 5.1 and Astra lead at 65% mean rubric score, which is not 65% of jobs automated.

Epoch reports reliable performance on well-defined portions but gaps in implicit standards, research judgment, experiment design, and interpretation of flawed results. The intended setup minimizes further intervention; methodology explicitly allows some recoveries, deducting points for interventions. Memory and skill writing are disabled across tasks.

Limits: one run per task, a small Epoch-specific sample, a single human grader per output, subjective criteria, and differing harnesses. Treat this as bounded evidence about autonomous work products, not a universal capability ceiling or a measured productivity multiplier.

### OpenAI Ultrafast API access

[Live API documentation](https://developers.openai.com/api/docs/guides/ultrafast-mode), checked October 8 Atlanta time. GPT-6.1 Sol and GPT-6 Astra Ultrafast are available to all API users, with separate rate limits. Sol's default Build/Launch/Grow limits are 1 million/4 million/40 million tokens per minute; Astra's are 0.5 million/1 million/5 million. Sol supports US and EU data residency plus global processing; Astra supports US residency and global processing. WebSockets are recommended to reduce connection overhead for agent tool loops.

These are access and latency-tier facts. Account rate limits are not observed output speed, total deployed capacity, or a basis for increasing global Tokens/day.
