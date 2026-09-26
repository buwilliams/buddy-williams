---
title: "The Moving Line"
status: "Final"
date: "July 2026"
order: 0
blurb: "Work lies on one timeline: what AI does, what AI can do but humans still do, what only humans can do, and the unbounded frontier beyond. Three boundaries move across it at diffusion, research, and frontier rates. This essay models who gets repriced, who evaporates, and who captures what opens next."
---

# The Moving Line

## Introduction

Three claims about AI's economic impact are circulating right now. Serious people make each of them, and they seem to contradict one another.

The first is a **repricing** claim, made by [me from inside Nevo](https://x.com/BuddyIterate/status/2074457364275532108) and, independently, by [Anthropic's Thariq Shihipar](https://www.youtube.com/watch?v=9fubhllmsBU&t=918s). Work that took a software consultancy three weeks, about 120 billable hours, now takes six hours with frontier coding agents. That is a 20× speedup. If the ratio held across an implementation-heavy business and clients kept paying by the hour, a ten-person delivery team billing about $2.9M a year would compress toward $144K. Firms would need twenty times the volume or watch their revenue collapse.

The second is an **elasticity** claim, [stated most visibly by Jeff Bezos](https://x.com/jaynitx/status/2057415266863390748): AI will produce a labor shortage, not mass unemployment. The radiologists and software engineers everyone expects to disappear will be elevated instead. People who have been digging basements with shovels are about to be handed bulldozers.

The third is a **transformation-failure** claim, [made by Kai-Fu Lee](https://x.com/kaifulee/status/2074197757489991804): the overwhelming majority of enterprise "AI transformation" initiatives fail, because organizations bolt AI pilots onto functional tasks without touching the business core, and much of existing leadership is unequipped for the change.

I don't think any of these claims is wrong. My conjecture is that each one correctly observes a different variable, and the variables move at different speeds. The contradiction dissolves once we stop asking "which claim is true?" and start asking "which boundary is each observer watching, and how fast is it moving?"

This essay turns that conjecture into a model. The model places all work on one timeline with four segments, divided by three moving boundaries, and then follows the consequences: why prices fall before volume arrives, why some work reprices while other work leaves the market entirely, which advantages last, and why the firms that see the change most clearly often cannot act on it. It ends with three predictions, each with a condition that would refute it.

I am not a disinterested observer. I run delivery teams at an enterprise software consultancy, and this model describes the ground shifting under my own feet. That is a reason to scrutinize the argument harder, not to discount it. The last section asks what the model allows someone in my position to actually do, and the answer is not that anyone, including me, gets to stand on solid ground.

## Table of Contents

1. [Rates, Not States](#1-rates-not-states)
2. [The Research Boundary Moves](#2-the-research-boundary-moves)
3. [The Repricing Mechanism](#3-the-repricing-mechanism)
4. [The Amdahl Inversion](#4-the-amdahl-inversion)
5. [The Coasean Floor](#5-the-coasean-floor)
6. [Advantage as Distance from the Line](#6-advantage-as-distance-from-the-line)
7. [The Double Bind: Value Networks and Identity](#7-the-double-bind-value-networks-and-identity)
8. [The Whole Model](#8-the-whole-model)
9. [Three Predictions](#9-three-predictions)
10. [What Remains Executable](#10-what-remains-executable)
- [Conclusion](#conclusion)
- [Appendix A: The Repricing Arithmetic](#appendix-a-the-repricing-arithmetic)
- [Appendix B: Key Terms](#appendix-b-key-terms)
- [Appendix C: Open Problems](#appendix-c-open-problems)

## 1. Rates, Not States

Most arguments about AI and jobs are arguments about states. "AI can write code, so programmers disappear." "Demand for software is unlimited, so programmers thrive." Both compare a snapshot of capability to a snapshot of the labor market. But the economy is not a snapshot. It is a set of processes running at different speeds, and what happens to a particular firm or worker depends on which process reaches them first.

So the method here is to translate every static claim into a claim about rates. When someone says "AI does the work twenty times faster," the useful questions are: can AI do it, does AI actually do it, and what new work is opening beyond it? This is the difference between predicting an equilibrium and predicting a path. For anyone living through the transition, the path is the whole story. Equilibria do not send invoices.

Place all work on one timeline:

> **Work AI does** | *diffusion boundary* | **Work AI can do, but humans do** | *research boundary* | **Work only humans do** | *frontier boundary* | **Unbounded frontier**

The segments are states; the boundaries carry the rates.

- The **diffusion boundary** separates what AI actually does from what it could do. Its rate is how fast capable work becomes work AI performs in production.
- The **research boundary** separates what AI can do from what only humans can do. Its rate is how fast work moves from human-only into AI-capable.
- The **frontier boundary** separates defined human work from the unbounded frontier beyond it. Its rate is how fast unformulated possibility becomes concrete human work: new problems, products, and roles that did not exist to be automated before.

As the boundaries move, the segments change width. When research outruns diffusion, the middle segment widens: AI can do more than organizations let it do. When diffusion catches up, latent capability becomes actual production, and repricing follows. When the frontier opens faster than research advances, the domain of human work grows even while old work is compressed.

## 2. The Research Boundary Moves

The boundary most AI discussions notice is the research boundary. Today its left side has a recognizable profile: work that is specifiable, bounded, and verifiable, the kind you could hand to a competent stranger with a written brief and check without watching them do it. Its right side holds work tangled up with organizational context, ambiguity, politics, trust, and accountability. When the research boundary passes a kind of work, that work becomes technically compressible: the 120 hours become 6.

Nearly every confident claim about AI and work assumes this boundary is a fact about the *work*. It is not. It is a fact about the *models*, and models improve. Two years ago, writing production code sat in the human-only segment. Today it is AI-capable, and in leading organizations it has crossed the diffusion boundary into work AI actually does. Verification (reviewing code, writing tests, checking invariants) was until recently the standard example of what would stay human. Agents now write the tests and run the harnesses. The research boundary is not a property of tasks any more than a coastline is a property of the ocean floor. It is where the tide currently reaches, and the tide is coming in.

So "is this work compressible?" is a state question whose answer expires. The better questions are how fast research is making it compressible and how far behind diffusion is. Every category in this essay that looks static should be read as a position on the timeline.

Does the research boundary ever stop? Is anything permanently human-only? I think so, but not where most discussions put the line. We need to distinguish **contingent** work from **constitutive** obligations.

Contingent work is human-only because the models have not reached it yet. Verification was contingent all along, however permanent it looked. Constitutive obligations are different in kind: they are not computations at all, but social or legal relations. Liability is the clearest case. "Someone must be accountable when the system fails" is not a task a better model performs. It is a fact about courts, insurance, and the human need to trust before depending. Even where law adapts to machine-made decisions, the accountability does not vanish; it relocates to whoever deploys, insures, or certifies the system. Trust between institutions, the legitimacy of a consequential decision, the assignment of responsibility: a model can *inform* these relations, but it cannot *be* a party to them. Call this set of relations the **constitutive core**.

There are two opposite ways to get this wrong. Mistaking contingent work for constitutive fills the graveyard of "AI will never X" predictions. Mistaking constitutive obligations for contingent work turns law, trust, and accountability into engineering problems, which they are not. How much of today's human-only work is truly constitutive is, I think, the most important open question in this subject (Appendix C). Every "non-compressible" claim ahead either names a relation or names a place the tide has not reached yet.

## 3. The Repricing Mechanism

Start with the 120-hour task that becomes a 6-hour task (the arithmetic is in Appendix A). The repricing reading says consulting revenue falls 95% unless volume rises 20×. The elasticity reading says volume *will* rise, because enterprises have roadmaps far longer than their budgets, and custom software may be one of the most demand-elastic goods in the economy. Projects that were uneconomical at the old price become thinkable at a twentieth of it. Which reading is right?

Both, at different speeds. Here is the mechanism: **potential supply responds to research, realized pricing responds to diffusion, and new volume responds to the frontier.**

Price moves fast because price discovery needs only one competitor to diffuse the capability and quote the AI-assisted price. Volume moves slowly because most clients still sit in the middle segment, where AI can do the work but humans do it. Moving that work across the diffusion boundary takes change management, data readiness, compliance review, redesigned workflows, and organizational will. All of these run at firm speed, not model speed. An insurance carrier does not integrate twenty new systems this year merely because systems got cheap. And the eventual volume is not limited to today's backlog; the frontier boundary opens problems and products that were not previously economical, or even imaginable as projects.

The gap between fast price and slow volume is the **interregnum**: the period when the old revenue model is dead and the new demand has not yet arrived. The repricing claim correctly describes the interregnum. The elasticity claim correctly describes the equilibrium beyond it.

The first casualty of the interregnum is not the consultancy but the *pricing model*. Hourly billing works only while hours track value. Once research and early diffusion separate them, hourly billing becomes unstable no matter what any single firm decides. A firm that keeps selling hours into a market that has repriced hours is volunteering to be exposed. So the real question is not "how do we do twenty times the volume?" but "how do we stop selling time?" Answering it requires locating the firm's work on the timeline, which brings us to Amdahl.

## 4. The Amdahl Inversion

Amdahl's Law comes from parallel computing. It says the total speedup of a system is limited by the fraction of work that cannot be parallelized. If half the work is inherently serial, infinite processors buy at most a 2× speedup.

Applied to consulting, the standard move is reassuring. Coding compresses 20×, but discovery, stakeholder alignment, verification, and deployment do not, so the engagement as a whole speeds up far less than the headline number. The revenue collapse is overstated.

That reassurance holds for a single engagement. But a firm is not an engagement. A firm holds a portfolio of work, and the reassuring argument quietly assumes every firm's portfolio contains a healthy share of serial, non-compressible work. It need not. A firm's mix of work is not handed down by nature; it was chosen by decades of contracts and client relationships. In my corner of the industry, the historical division of labor is stark. Clients kept the serial work (planning, requirements, alignment, politics) and outsourced the specifiable core (architecture and implementation). In many engagements, the work the client retains is perhaps a tenth of the total effort or less.

Why did the line between client and consultancy land there? Because contracts govern best what can be specified, bounded, and verified. Over decades, the market assigned consultancies the specifiable work.

Now recall the research boundary's profile from Section 2: it advances first through work that is specifiable, bounded, and verifiable. The properties that made implementation outsourceable are the properties that make it automatable. This is not bad luck. The client–consultancy line was drawn along roughly the same contour the research boundary is now crossing. An implementation-pure consultancy is not partially exposed; it is close to fully exposed by construction, because the logic of contracting selected its portfolio to contain little besides the compressible fraction. Amdahl still limits each engagement, but the limiting work sits on the client's side of the contract, so it offers the firm little protection. The firm's serial fraction is, in the economist's word, **endogenous**: a variable its history chose, not a constant imposed on it.

Inside the client organization, Amdahl becomes useful again. Within a particular transformation, someone must establish trust, satisfy regulators, change workflows, and hold accountability. No one can choose those bottlenecks away in the moment. This, I conjecture, is a deep reason diffusion is slow: its bottlenecks are the organizational work described in Section 3, and they run at organizational speed however fast research moves.

But the economy is not one fixed transformation. Across organizations and over time, workflows are redesigned, tasks disappear, and new work appears. Amdahl explains why a given transformation stays slow; it does not fix the economy-wide quantity of work. And the serial work binding a transformation today is not the serial work that will bind it tomorrow. Research annexes the contingent portion generation by generation, and much of the work of diffusion may prove contingent too. What research never annexes is a constitutive obligation itself.

Put these levels together with Section 2's distinction and a pattern appears. As research and diffusion pass contingent work, the remaining economic value tends to gather around constitutive obligations: trust, liability, and accountability. Call this the **residual-concentration conjecture**. The obligations persist, but the labor and revenue attached to them need not, so the conjecture names the residual's likely destination without saying how large it will be.

The obvious response is for the consultancy to move its portfolio to the right: stop selling implementation and start selling the residual. That is correct as far as it goes, and Section 7 shows why it goes less far than it seems. But first, a second force is reshaping the market, one that does not just reprice work but removes it from the market altogether.

## 5. The Coasean Floor

Ronald Coase asked why firms exist at all, why any work happens inside organizations rather than being bought task by task on the open market. His answer was transaction costs. Finding a counterparty, negotiating terms, checking quality, and enforcing the agreement all cost something. When those costs exceed the cost of doing the work in-house, the work stays in-house.

Every consulting engagement carries these costs in concrete form: procurement, master service agreements, security reviews, onboarding, access provisioning. Call their sum the engagement's **transaction-cost floor**, or the **Coasean floor**. Its key property is that it is roughly fixed per engagement. A six-week procurement cycle costs the same whether it precedes six months of work or six hours of it.

Now run the compression from Section 3 against this floor. When the production work inside an engagement shrinks 20×, the fixed transaction costs come to dominate the cost of hiring outside help. Past some threshold, the rational client does not negotiate a lower price. It stops contracting for that class of work and does it in-house, because no price makes a six-week procurement cycle sensible for a six-hour task.

This is easy to mistake for repricing, but it is structurally different. The low end of the consulting market does not reprice; it **evaporates**. Work falls below the floor and leaves the market. I am already watching the early form of this: clients running the same coding agents internally and asking, reasonably, why they need us. For the moment the answer is that we use the agents better than they do. Section 6 asks how long that answer lasts.

An existing relationship offers a partial escape. An active master agreement and established trust lower the floor, so an incumbent vendor can delay evaporation by bundling compressed implementation into work the client already trusts it to do. But a client who trusts a firm to build against a specification will not necessarily trust it to redraw the specification, the workflow, or the organization around it. That is the identity problem of Section 7, showing up inside the Coasean one. The existing relationship is borrowed time: it preserves work inside the old boundary without granting access to the new one.

Evaporation is why the right historical comparison for consulting is travel agencies, not radiologists. Radiologists faced a capability threat and adopted the capability. Travel agencies faced a transaction-cost collapse: booking a flight yourself became easier than engaging an intermediary. Routine mass-market booking was disintermediated, leaving a smaller market in corporate travel, complex itineraries, and high-touch advice. The consulting products that survive evaporation will share that profile: engagements too large, too risky, or too organizationally entangled to fall below the floor, such as transformation programs, accountability for regulated systems, and embedded capacity. What these have in common is that they cluster around the constitutive core, not around artifacts. Approached from transaction costs, the floor points to the same residual that the research boundary did.

One refinement matters for reconciling the three original claims. Repricing and evaporation hit the *same* firms, but the volume, when elasticity delivers it, goes to *different* parties, and much of it goes to no outside firm at all: work clients now do internally never re-enters the external market. The recovery is real for the economy and partly illusory for the industry. That is how Bezos can be right about labor in aggregate while the consulting industry has a very bad decade. The two are the same event seen from different distances.

How high the floor sits, and how it moves, is uncertain; research may compress transaction costs too (Appendix C). Whatever its height, a firm's ability to stay above it depends on what kind of advantages it holds.

## 6. Advantage as Distance from the Line

My firm's current answer to "why do clients still need us?" is that we use coding agents better than they do. That is true today, and the timeline shows what kind of truth it is. The work has crossed our diffusion boundary but not the client's. The advantage lives in the gap between two actors' diffusion boundaries.

That gap closes from both directions. Each generation of models automates some of the technique an expert user used to supply. Early models rewarded elaborate prompts; later ones inferred more of the user's intent on their own. Prompt engineering was a real differentiator, and the models absorbed much of it within perhaps eighteen months. Orchestration skill, such as coordinating multiple agents and building verification harnesses, stands a little further up the beach, but on the same slope. The better a capability gets at operating itself, the less durable an advantage in operating it can be.

This suggests a classification every professional-services firm should run on its own balance sheet. The timeline supplies an ordering heuristic, not a calculation: **an advantage's half-life grows with its distance from the boundary that can erase it and shrinks with that boundary's speed toward it.** Any advantage of the form "we are better at operating the capability" is exposed from both sides. Diffusion teaches clients the technique, and research teaches the capability to run itself.

What sits deeper in the human-only segment? Only things whose distance is not measured by task difficulty alone:

- **Proprietary context**: knowledge of a client's systems, data, politics, and history that a general model does not reliably have, that is expensive to transfer, and that changes as the organization changes.
- **Liability and trust**: being the accountable party. This is constitutive, in Section 2's terms, and so outside a boundary that only crosses computations.
- **Organizational position**: being inside a client's change process rather than a vendor to it.
- **Distribution**: owning the relationship through which capability reaches the client at all.

The durable advantages are not skills but *relations*: positions in a web of trust, knowledge, and access. That is what the constitutive core predicted.

Now the uncomfortable part. Look at where each kind of asset builds up by default. Tool skill builds up in whoever uses the tools, for now the consultancy. Context, trust, and position build up in whoever does the serial work, and Section 4 showed that under the historical division of labor, the serial work lives inside the client. The implementation-shaped consultancy has spent decades accumulating the asset closest to the moving boundaries, while its clients accumulated the ones deeper in the human-only segment. This is the Amdahl inversion restated as an asset ledger. It turns a strategy question into a solvency question: the firm's remaining advantage has a half-life, the boundary rates are not under its control, and time is running.

The prescription seems to write itself: move up the ledger. Sell context, trust, and position instead of implementation. Here is why that goes less far than it appears.

## 7. The Double Bind: Value Networks and Identity

Clayton Christensen's *The Innovator's Dilemma* is usually summarized as "incumbents miss the new thing." That misses his actual finding. Christensen showed that incumbents often see the disruption clearly and respond *rationally*, and that the rational response can still lead to failure. A firm's **value network** (its existing customers, pricing structures, sales motions, and margin expectations) judges every strategic option. Because the network rewards the business the firm already has, it reliably favors moves that serve existing customers and protect existing economics.

For a consultancy, the value network's advice is: use AI to deliver implementations faster at better margin. That is rational, and it is a melting-ice-cube strategy, improving margins on territory research has already made compressible. The alternative is to move into the client's serial territory by selling discovery, diffusion, and organizational redesign. But that means a different buyer, a different sales motion, and a different margin structure, and the existing network votes against all three.

Whether AI-assisted consulting counts as disruption in Christensen's strict sense does not matter here. I am borrowing only his explanation of how a value network constrains an incumbent even when the people inside understand what is happening. That is the first layer of the bind, and determined leadership could override it. The second layer is the one I have come to think is decisive.

Call it **identity**. It has an external face and an internal one.

Externally, identity is brand: the compressed prior in the client's head about what the firm is *for*. A brand that has meant "hand them a spec, get back working software in a regulated environment" for twenty years cannot be re-indexed to "let them into our planning, our politics, our org chart" with a new website. Procurement does not update its priors from marketing.

Internally, identity is culture, and culture is not the mission statement but the *selection function*: the accumulated pattern of who gets hired, promoted, and retained. A delivery culture selects, year after year, for people who excel at shipping against specifications. People who thrive in ambiguous, political, artifact-light diffusion work become rarer and carry less status. A firm that decides to sell diffusion may still employ some of them, but it is not organized to find them, empower them, or reproduce what they do.

This is how identity constrains diffusion. A capability crosses a firm's diffusion boundary only when the firm can operationalize it, sell around it, and reorganize the people accountable for it. Identity changes at a generational pace, because changing a selection function means changing the population it has already selected. A firm survives only if it can move its own diffusion boundary and its market position before research makes the old position worthless. The double bind is that the value network votes against moving, and even a firm that overrules the vote finds its brand and people move more slowly than research.

Christensen's own cases point the same way. Incumbents rarely reposition their core organization into the disruptive segment. The exceptions usually create an autonomous organization (a protected business unit, joint venture, or spin-out) with separate economics and, tellingly, separate *hiring*. The usual counterexample, IBM's turn into a services company under Lou Gerstner, took about a decade, a near-death experience, and an outsider CEO. That reads to me less like a refutation than a statement of the price.

This leads to a conjecture I will call, half-seriously, the **insight-irrelevance theorem**: *in identity-bound transitions, insight is not the missing ingredient, and supplying more of it does not unblock the transition.* The constraint is on the firm's diffusion rate, not on what anyone knows, so better analysis and more conviction do not fix it through the existing identity's normal ways of allocating people and money, at any level of seniority.

This does not make leadership irrelevant. Leadership can create and protect a genuinely autonomous identity. What it cannot do, on this view, is reason the existing organization into diffusing at research speed. This is one of the boldest conjectures in the essay, and it is a claim about a strong tendency, not a proof. Perhaps some mechanism can speed identity change without autonomy or near-death (Appendix C). But I have watched enough transformation initiatives to believe the tendency.

If the overwhelming majority of transformations keep failing even as technical understanding spreads, identity explains what a capability account cannot: research is advancing while the organization's diffusion boundary stays put. Lee's point that much of existing leadership is unequipped may then understate the problem. These organizations need new identities, and leadership swaps are attempted because identity swaps are rarely on the menu.

## 8. The Whole Model

Read the timeline from Section 1 from right to left. Each boundary converts the segment on its right into the segment on its left. The frontier turns unbounded possibility into defined human work. Research turns human-only work into AI-capable work. Diffusion turns latent capability into actual production. All three move, and the changing widths between them describe the transition.

Research currently moves fastest. Diffusion follows unevenly: a new entrant can start with its diffusion boundary close to research because it has no installed workflows or identity to protect, while an incumbent client may lag by years. The space between them is not a contradiction but a stock of latent capability: work AI can do that humans still do.

The frontier moves the same direction. A problem beyond it cannot be assigned to humans or machines, because it has not yet been formulated as work. Once people turn it into a product, role, or tractable problem, it enters the human-only segment, where research and diffusion may eventually reach it. But the frontier can keep opening new territory while both advance behind it. So the labor question depends on relative rates: whether research consumes defined human work faster than the frontier creates it, and whether diffusion turns that possibility into reality fast enough to matter now.

Each public claim from the introduction watches a different part of this motion:

| Claim | Boundary it watches | What it sees |
|---|---|---|
| Repricing | Research, plus the earliest diffusion | Capability advancing and a few competitors using it, enough to reset market prices |
| Transformation failure | The gap between research and diffusion | Capability that exists but stalls in organizations |
| Elasticity | The frontier | Falling costs opening problems and demand that were out of reach |

Research explains what becomes possible, diffusion explains when it becomes real, and the frontier explains what becomes worth doing next.

The interregnum follows from these rates in sequence:

1. Research moves the specifiable core of professional work into the AI-capable segment.
2. An early competitor diffuses the capability and collapses the price of hour-denominated work.
3. Incumbent organizations lag in the middle segment, delaying volume and producing transformation failures.
4. Where transaction costs stay high, work falls below the Coasean floor and evaporates, as clients move capable work across their own diffusion boundaries.
5. The frontier opens new demand, but to different parties and identities than the ones that were repriced.

Throughout, to the extent that constitutive obligations keep commanding labor and value, they stay in the human-only segment while the work around them moves left.

A model this tidy should make a reader suspicious. The right response to suspicion is to ask what the model forbids.

## 9. Three Predictions

**First: some agencies fail, and the model says which ones and how.** The first to fail are implementation-pure agencies in lightly regulated industries, priced in hours, holding no assets deep in the human-only segment: no proprietary context, no constitutive trust, no organizational position. The failure mode is specific. It is not the gradual margin squeeze a repricing story predicts, but discontinuous evaporation, with whole classes of engagement vanishing from the pipeline as clients move capable work across their own diffusion boundaries and below the Coasean floor. Agencies in regulated industries last longer, held above the floor by trust and liability, but that is duration, not immunity, unless they convert the borrowed time into assets farther right on the timeline.

*Refuted if* implementation-pure agencies broadly maintain revenue through the transition by volume growth alone, without changing what they sell.

**Second: new entrants win, for a time, and research sets the expiration date.** AI-native vendors, especially their forward-deployed engineering teams, capture the relocated residual because they arrive with no prior to overwrite. Their brand already says "we are the capability," and their diffusion boundary starts close to research. They enter the diffusion market fresh rather than as reformed consultancies fighting their own identity.

"For a time" is not a hedge; it follows from the boundary rates. Much of diffusion work is contingent. The research boundary that passed coding and then verification will press into deployment discipline, workflow redesign, and the translation of organizational intent into system behavior. So today's AI-native diffusion business is tomorrow's implementation shop. The entrant's founding advantage is a small gap between its diffusion boundary and research, and only continuous movement keeps that gap small as its identity hardens into incumbency.

*Refuted if* incumbent consultancies capture most of the diffusion market through their existing identities, without autonomous units, joint ventures, spin-outs, or new companies.

**Third: client-side workers have the strongest chance to capture the upside, especially by moving.** The assets deep in the human-only segment (proprietary context, organizational position, and trust) pool *inside* client organizations, in the people doing the serial work.

That does not mean workers automatically capture that value. An employer can absorb the gain through higher output expectations, smaller headcount, or by writing employee context into its systems. But the labor market gives an individual a mechanism a firm lacks: they can redraw their identity by changing employers. The worker arrives under a new prior, not as the incumbent expected to do the old job faster, but as the person hired to move the new organization's diffusion boundary. The new role can reprice compensation, authority, and attention around that expectation, while placing the worker where context, trust, and position accumulate. Call this **identity-speed-of-one**: an individual can reposition in months, while a firm takes years. The elevation thesis is most true, soonest, for client-side workers willing and able to use it.

*Refuted if* workers hired explicitly to drive AI-enabled change fail to command more compensation, scope, or organizational position, while client-side technical roles stagnate or shrink.

## 10. What Remains Executable

The model ends on a personal question, and I won't pretend it is hypothetical. If the insight-irrelevance theorem is right, the person inside an incumbent consultancy who sees all this clearly, even the person whose job is to lead exactly this transition, cannot fix it through the existing unit's normal mechanisms. Not for lack of authority or conviction, but because the constraint was never insight. So what moves does the model allow? I count two.

**The first is to relocate on the timeline at identity-speed-of-one.** A person can move to a vendor-side role close to the research boundary, or to a client-side role chartered to move the organization's diffusion boundary. Either way, the new employer supplies a new prior that reprices the person around the new work rather than the old function. This answers a question that deserves a principled answer: why would someone who sees the whole model leave rather than fix the firm? Because the fix is rarely executable through the existing identity, while the labor market can reposition the individual now. Leaving is the third prediction applied to oneself.

**The second is a new identity under a protected structure.** This is Christensen's escape for a group rather than an individual: an autonomous unit, joint venture, spin-out, or new company, with separate economics and a brand carrying no implementation prior. It claims the entrant advantage of the second prediction on purpose, selling the residual from day one, in the regulated industries where the Coasean floor sits highest and holds longest.

The model's own logic forces a caveat on each path.

On the second path: "same people, new brand" satisfies the brand half of identity and only partly satisfies the culture half, because culture travels with people. That is why Christensen's surviving spin-outs used separate hiring, not just separate letterhead. A new brand staffed entirely by people selected for delivery carries the old selection function inward even as it escapes the old prior outward. So the model predicts that the old firm with a new logo fails by the same mechanism as the incumbents. It succeeds only as a genuine re-founding, with the team re-selected for the new product: diffusion work, tolerance for ambiguity, and political fluency, not delivery excellence alone.

On the first path: if much of diffusion work is contingent, as the second prediction argued, research will eventually press into forward-deployed engineering and client-side transformation roles too. The new role is not a safe harbor. It is a position deeper in the human-only segment, with a longer half-life, but a half-life all the same. Nobody in this model stands on solid ground; there are only positions on the timeline and the rates at which its boundaries move. The individual's advantage is not safety. It is the ability to reposition faster than a firm, and the willingness to do it more than once.

Which path is better? The Coasean floor names the variables, but its height alone does not decide. What matters is how the cost of using an outside firm changes relative to the client's cost of moving capability across its own diffusion boundary:

- If production costs collapse while external transaction costs stay rigid, and the client's staff can use the capability themselves, low-end work moves in-house. The first path then points toward a client-side role leading diffusion.
- If transaction costs fall along with production costs, small-scale outside work survives but becomes fragmented and commoditized. The first path then points toward roles near the research or frontier boundary, not toward a firm selling hours.
- If constitutive trust and liability keep some engagements large and valuable enough to stay above the floor, a protected new identity can sell the residual. That is the ground the second path is built on.

I don't yet know which force dominates, and the answer will likely differ by industry. The model doesn't choose the path for me. It tells me what to watch: internal diffusion costs, external transaction costs, and how much value stays attached to constitutive trust.

## Conclusion

I opened with three claims that seemed to contradict one another: consulting revenue collapses twentyfold; AI brings a labor shortage and elevation rather than unemployment; and most AI transformations fail. The model puts all three on one timeline. The repricing claim sees research and the earliest diffusion. The transformation-failure claim sees the gap where research has arrived and diffusion has not. The elasticity claim sees the frontier opening new demand beyond the work being compressed. None of them is wrong; each is watching a different boundary.

If the model is right, its deepest irony is that the transition is hardest on the organizations built to sell technical change, and offers the most upside to the client-side workers everyone expected it to displace. The bulldozer goes to the person standing in the basement. Its deepest discomfort is that it exempts no one, including the entrants, the engineer whose role merely sits farther right, and the author.

I don't find this bleak, and the reason is not consolation. It is the same epistemology the essay runs on. The frontier boundary is generative. It turns unbounded possibility into problems, products, and roles that could not have been specified in advance, any more than "forward-deployed engineer" could have been specified five years ago, or "software consultant" fifty years before that. Research and diffusion may move through each new segment in time, but neither bounds the frontier ahead. We navigate by conjecture, pursuing the opportunities we can currently see, knowing the timeline will extend and that the extension is where new opportunities come from. The world has never honored anyone's demand that their position be permanent. What it does offer, and what a fallibilist should ask of it, is an endless supply of problems worth solving. Solid ground was never the offer. Motion was.

I hold all of this fallibly. The insight-irrelevance theorem is stated more strongly than the cases strictly prove. The size of the constitutive core and the path of the Coasean floor are open empirical questions. The graveyard of "the models will never do X" predictions counsels humility about every residual I have placed in the human-only segment. The predictions in Section 9 carry their refutation conditions on purpose. If implementation-pure agencies sail through unchanged, if incumbents capture diffusion through their existing identities, or if client-side hires chartered with AI-enabled change fail to gain compensation, scope, or position, the model is wrong, and I want to know it before the interregnum teaches the lesson the expensive way.

## Appendix A: The Repricing Arithmetic

The compression figures used in the essay, for a representative engagement and delivery team.

**Task compression.** Original effort *o* = 120 hours (three weeks); new effort *n* = 6 hours. Remaining fraction *r* = *n* ÷ *o* = 0.05, a 95% reduction. Compression ratio *k* = *o* ÷ *n* = 20.

**Revenue exposure.** A team of 10, at about 60% billable utilization over a 48-week, 40-hour-week year, at $250/hour: 10 × 0.60 × 48 × 40 × $250 ≈ $2.9M a year. Under full 20× compression of the billable work at unchanged hourly pricing, the same delivered scope yields about $144K.

Two corrections from the essay apply. First, compression applies to the share of work in the AI-capable segment, not to whole engagements. Section 4 argues, though, that for implementation-pure firms that share approaches the entire book, so the naive figure is closer to right than the standard Amdahl objection suggests. Second, the arithmetic assumes hourly pricing survives the transition. Section 3 argues it does not, which makes this a statement about the instability of the pricing model, not a forecast of any firm's revenue.

## Appendix B: Key Terms

**Work timeline.** The ordered model: work AI does; work AI can do, but humans do; work only humans do; and the unbounded frontier. Three moving boundaries (diffusion, research, frontier) divide the four segments.

**Diffusion boundary.** Separates work AI actually does from work AI can do but humans still do. Its rate is how fast capability becomes production.

**Research boundary.** Separates work AI can do from work only humans can do. Its rate is how fast human-only work becomes technically compressible; it currently favors specifiable, bounded, verifiable work.

**Frontier boundary.** Separates defined human work from the unbounded frontier. Its rate is how fast unformulated possibility becomes concrete problems, products, and roles.

**Unbounded frontier.** The open-ended domain of problems and possibilities not yet formulated as work. It is a segment beyond the frontier boundary, not a stock of existing tasks.

**Contingent work.** Work that is human-only because the research boundary has not reached it yet (for example, verification until recently, and likely much of diffusion work).

**Constitutive obligation.** An obligation that persists because it is a social or legal relation rather than a computation: liability, accountability, trust between institutions, the legitimacy of consequential decisions. The **constitutive core** is the set of such relations. Their persistence does not guarantee how much labor or value stays attached to them.

**Interregnum.** The interval between fast price compression and slow volume arrival, when the old revenue model is dead and the new demand has not yet arrived.

**Serial fraction.** The share of work that does not compress. At the firm level it is endogenous, chosen by the firm's contracting history. Within a given transformation at a given moment it binds, as Amdahl's Law says. Across the economy and over time it keeps changing.

**Residual-concentration conjecture.** As contingent work is compressed, the remaining economic value tends to gather around constitutive obligations. The obligations persist; the labor and revenue attached to them are not conserved.

**Coasean floor (transaction-cost floor).** The roughly fixed per-engagement cost of using an outside firm (procurement, contracting, security review, onboarding). Work whose production cost falls far enough below it leaves the market entirely, which the essay calls **evaporation**, rather than repricing. An established relationship lowers the floor within its trusted mandate.

**Advantage half-life.** An ordering heuristic: an advantage lasts longer the farther it stands from the boundary that can erase it and the slower that boundary moves. The durable advantages (proprietary context, liability and trust, organizational position, distribution) are relations rather than skills.

**Value network.** A firm's existing customers, pricing structures, sales motions, and margin expectations, which together judge its strategic options (from Christensen).

**Identity.** A firm's brand (the compressed prior in the client's mind about what the firm is for) and culture (the selection function that determines who is hired, promoted, and retained). Identity constrains the firm's diffusion rate and its ability to reposition as research advances.

**Identity-speed-of-one.** An individual's ability to reposition through an employer change in months, while a firm's selection function takes years to change. The new employer applies a new prior, repricing the person around the role they were hired to perform rather than the function they used to perform.

**Insight-irrelevance theorem.** A conjecture, not a proof: in identity-bound transitions, insight is not the missing ingredient, and the transition is not executable through the existing identity's normal mechanisms, however well anyone understands it. Leadership can still create and protect a genuinely autonomous identity.

## Appendix C: Open Problems

**The size of the constitutive core.** How much of the human-only segment (diffusion, trust work, accountability, judgment) is constitutively human rather than merely unreached by research? Every durability claim in the model depends on this. Mistaking contingent for constitutive repeats the "AI will never X" error; mistaking constitutive for contingent treats law, trust, and institutional legitimacy as engineering problems. The strongest candidate for a genuinely constitutive *skill*, as opposed to a relation, is taste under uncertainty: judgment about which conjectures are worth the compute. I hold it loosely, because it may only be contingent work farther right on the timeline.

**The path of the Coasean floor.** The market-structure predictions, and the choice between the two executable paths, depend on whether research compresses transaction costs (procurement, verification, trust between firms) as fast as production costs. If transaction costs and identity priors fall together, small-scale exchange survives and implementation work becomes gig work. If trust and liability keep transaction costs rigid, in-housing dominates. The answer likely differs by industry. This is the constitutive-core question in disguise: transaction costs are rigid exactly to the extent that they are constitutive.

**The speed and shape of the research boundary.** The model treats research as advancing along a single gradient, from specifiable to ambiguous. Is that right, or does it advance unevenly, leaping past work assumed safe while stalling on work assumed doomed? An uneven research boundary would complicate even the rough ordering of advantage half-lives and would call for a richer geometry.

**The relative boundary rates.** Does research move through defined human work faster than the frontier creates new human work? How quickly does competition help diffusion close the gap behind research? The answers determine whether the human-only segment grows or shrinks, how long the interregnum lasts, and how much of the eventual demand reaches the parties first repriced. Electrification suggests a long diffusion lag: roughly forty years passed between clearly superior capability and realized productivity, because factories were built around the steam-driven shaft. Modern competitive pressure is the main reason to expect this gap to close faster.

**Speeding up identity-bound diffusion.** Autonomous units create a new identity beside the old one. Can the core organization move its own diffusion boundary faster, deliberately, without autonomy, near-death, or outside leadership as in the IBM case? If so, the insight-irrelevance theorem weakens, and some incumbents can escape without re-founding. I know of no clean example, which is evidence for the stronger version, but not proof.
