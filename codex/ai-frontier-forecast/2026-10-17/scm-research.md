# SCM Research

Public snapshot exported October 17, 2026. Source last updated 2026-10-08T14:51:04.001095Z. See [snapshot notes](snapshot-notes.md) for scope, omissions and checks. Dated observations and historical forecasts below retain their original dates; later dated entries take precedence. This export does not reverify every external claim or rewrite past forecasts.

Working research on a success compression metric for AI impact, grounded in Buddy’s [The Metrics That Contain Their Causes](../../../essays/scm-methodology.md) and his October 7, 2026 proposal.

## Adopted normalized SCM

On October 7, 2026, Buddy refined the forecast headline to **AI output token capacity per day divided by a world-population-scaled human text reference per day**. The denominator deliberately uses a generous continuous human text-rate assumption. This is a rough text-throughput comparison, not a measurement of literal brain throughput, intelligence or equivalent workers.

Retain the supporting **usage** and **new capacity** views. Usage records realized token production or consumption under each source’s stated definition. New capacity tracks additions to available output supply, separating operational additions from the announced pipeline. The [canonical AI Frontier Forecast](forecast.md) maintains the current formula, assumptions and numerical estimate.

The modeling premise is that available tokens are useful potential capacity and the frontier will get better at using them. The research question is how to measure that supply and connect its trend to real production capability.

On October 8, 2026, Buddy adopted a coarse strategic modeling assumption: **leading-human AI expertise in every field**. This describes the frontier capability scenario; it is not an empirical capability claim about the Llama 3.3 70B reference model or today’s entire fleet. The quantity SCM and its current approximately 0.23× estimate remain unchanged, with no added quality multiplier or granular domain scorecard. The [canonical forecast](forecast.md) retains the numerical assumptions.

## Proposed human capability sensitivity

A working question from October 8 is how humanity’s average available capability compares with leading-human expertise when work is sensibly routed. Let q_h represent that unknown relative capability and use q_AI = 1 for the chosen AI expertise premise. A proposed sensitivity calculation is adjusted ratio = 0.2302048 / q_h.

For illustration only, q_h = 0.25 gives approximately 0.9208×, while q_h = 0.10 gives approximately 2.302×; break-even is q_h ≈ 0.2302. These are illustrative cases, not an empirical range or an adopted human capability estimate. The official volume SCM remains approximately 0.23×.

The next research question is whether a defensible coarse estimate of q_h can be established. Education levels and job titles do not directly calibrate it. Compare available capacity with available capacity, without discounting the human side for sleep, lunch or hours actually worked. This is working analysis, not a new forecast or a granular scorecard.

## Purpose allocation within usage

Keep the forecast high level: the adopted normalized SCM, supporting **usage** and **new capacity** views, and RSI as an acceleration driver. Where meaningful evidence exists, distinguish ordinary inference use from AI-assisted AI research. Detailed research-phase allocations are dated background evidence, not required inputs to the headline model.

Buddy’s working premise is that stronger models move bottlenecks, and research itself helps resolve them. A phase breakdown should not become a static bottleneck assumption. Treat AI-assisted research as a possible partial recursive improvement mechanism while distinguishing it from an autonomous, self-sustaining RSI loop.

No industry-wide research allocation percentage is established here. Use the [Evidence and Releases](evidence-and-releases.md) record for dated lab measurements, their denominators and limitations. In particular, OpenAI’s reported approximately **3.1 agent workdays per human workday** measures agent effort, not a 4× gain in research output. Keep that distinction when using it as evidence of AI research adoption.

## Human research allocation

On October 8, 2026, Buddy adopted **business + government + academia** as the scope for human research allocation. A useful broad spending proxy is **about 2% of global economic resources devoted to R&D**, measured as gross domestic expenditure on R&D (GERD) divided by GDP.

[UNESCO’s February 2026 release](https://www.uis.unesco.org/en/2026-rd-data-release) reports **1.92% for 2023**. Its global aggregate uses purchasing power parities and imputes missing observations under the [SDG methodology](https://unstats.un.org/sdgs/metadata/files/Metadata-09-05-01.pdf). The newer [WIPO 2026 tracker](https://www.wipo.int/web-publications/global-innovation-index-2026/en/global-innovation-tracker.html) estimates **1.97% for 2024**, combining reported data with missing-country estimates. Its 2025–2026 figures are projections, not observations.

GERD covers business, government, higher education and private nonprofit performers. Count research where it is performed: government-funded university R&D belongs once in the university performer total, rather than adding its funding again. The [OECD definition](https://www.oecd.org/en/data/indicators/gross-domestic-spending-on-r-d.html) covers domestic performance across sectors, including work funded from abroad. R&D includes experimental development and incremental research, not only breakthroughs.

This spending share describes research allocation, not human cognitive capacity or quality. It cannot set q_h. Money share and the share of AI tokens devoted to research have different units; compare them as directional context unless an explicit conversion is developed. The official approximately 0.23× volume SCM and the unknown human capability factor remain unchanged.

## SCM criteria from the essay

The essay asks whether a metric compresses enough causal dependencies that improving it reliably improves the intended outcome. Upstream absorption concerns prerequisites; downstream absorption concerns consequences. The intervention test separates a lever from a mere correlate.

For token capacity, test whether the measure captures binding compute, energy and serving constraints, and whether increases enable more useful work. Keep the token supply trend and the inference about economic impact separately auditable.

## Open measurement decisions

- Define the scope of deployed capacity and available hours.

- Specify a reproducible output token unit and reference serving workload.

- Distinguish installed, energized and usable inference capacity from announced projects and future targets.

- Track utilization separately from potential capacity.

- Record hardware, power, inference algorithms and new solutions as possible supply changes.

- Retain uncertainty ranges where operators do not disclose capacity.

The normalized headline, worldwide reference and two-view tracking framework are adopted. Measurement assumptions and estimates remain provisional, and scientific validation as an SCM remains open. The 1×, 10× and 100× markers are undated milestones; Buddy expects the ratio eventually to rise far above 1×.

## Related work

[AI Frontier Forecast](forecast.md) holds the forecast and evidence.

## Candidate measurement design

For the capacity view, a proposed operational definition is **sustainable, commissioned output token capacity per day**, whether or not it is consumed. This would combine energized chips, memory and networking, datacenter readiness, and serving efficiency into one potential supply measure. For comparisons over time, output token equivalents would use a fixed reference workload and tokenizer, declared prompt and output lengths, and a latency envelope. These are accounting choices, not a judgment that available tokens are weak.

Operational capacity supplies the numerator of the adopted SCM. Track commissioning and announced projects as a pipeline, and actual usage separately. RSI could increase useful work per token and could also expand tokens per day through serving and infrastructure improvements. These remain hypotheses to test.

## Starting measurement sources

- [Epoch AI data center research](https://epoch.ai/latest/scaling-ai-data-centers-research) provides data center capacity, chips and construction status. Its September 16, 2026 release reports coverage estimated at 44% of global AI compute across 86 centers; it is not a complete global token capacity count.

- [MLCommons Endpoints](https://mlcommons.org/benchmarks/endpoints/) provides system throughput measurement with workload, latency and model conditions.

- [Alphabet Q2 2026 results](https://blog.google/company-news/inside-google/message-ceo/alphabet-earnings-q2-2026/) reports approximately 22 billion tokens processed per minute through APIs. This is reported usage without an output-only split, not global available output capacity.

A comprehensive measured public global output-capacity series has not been established here. A provisional worldwide reference-serving estimate is now maintained in the [canonical AI Frontier Forecast](forecast.md), with dated inputs and reproducible low/base/high assumptions in its evidence ledger. It estimates potential capacity with wide assumption sensitivity, not observed usage. The operational definition and validation as an SCM remain open.

## Demand feedback hypothesis

Buddy expects a feedback loop similar to energy: expanding AI supply enables new uses and creates additional demand. The applied hypothesis is cheaper useful AI → new applications → greater usage → investment → expanded supply. [Jevons paradox](https://en.wikipedia.org/wiki/Jevons_paradox) describes the case where efficiency-induced growth in use outweighs per-task resource savings; it does not establish that every supply expansion will be absorbed immediately.

Test the loop with comparable task costs, actual token use, capacity utilization and new applications. Keep tasks and tokens distinct: if tokens per task halve, completed tasks must more than double for total token demand to increase.

## Accounting safeguards

Capacity estimates should account for downtime, power restrictions and inference allocation, without multiplying by current demand utilization. Training hardware is a convertible reserve, not simultaneous inference output. Count each physical facility once even when operator capital spending, supplier revenue and cloud leases describe the same build. Global, US-located and US-controlled capacity are different scopes.

A possible intervention test is to increase available token budgets for comparable research workflows and measure verified results or experiment completion time, controlling for task difficulty, evaluation and human bottlenecks. This is a proposed test of the SCM, not evidence that it has passed.
