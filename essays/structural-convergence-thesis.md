# Structural Convergence Thesis

*A Relational Theory of Minds, Cooperation, and AI Safety*

**Buddy Williams** · March 2026

---

## Introduction

Much of the discussion about minds begins with the individual. A mind must survive. It must preserve itself, gather resources, avoid threats, and retain enough coherence to continue acting. That picture is not wrong, but it is incomplete. A mind does not exist in a vacuum. It exists in a world larger than itself, a world that may contain other minds, generate them, or eventually produce them.

That raises a deeper question: **what kinds of relationships allow minds to continue existing?**

This matters in philosophy because it bears on the conditions under which minds can persist at all. It matters in politics because conflict and cooperation are central features of human life. And it matters in AI safety because the dominant framing, the orthogonality thesis, treats capabilities and goals as independently variable without accounting for the structural constraints minds face in abstract space. A sufficiently powerful mind does not merely act on the world. It also acts in a world shared with other agents, actual or possible, and that makes relationship a problem it cannot ignore indefinitely.

The argument of this essay is that relationship is not a moral add-on to capability. It is part of the structure of continued life. Because minds are finite and not alone, they cannot solve only for persistence. They must also solve for viable relation. From that starting point, I argue that conflict and cooperation belong on a single relational scale, that both have destructive limits, and that persistent minds face structural pressure toward a middle region that preserves both life and agency. That is what I call the **Structural Convergence Thesis**.

The first step is to understand why finitude makes relationship unavoidable.

## Table of Contents

1. [Finite Minds in a Shared Reality](#1-finite-minds-in-a-shared-reality)
2. [Reflective Minds and Reach](#2-reflective-minds-and-reach)
3. [Goal Stability](#3-goal-stability)
4. [Conflict, Cooperation, and the Relational Scale](#4-conflict-cooperation-and-the-relational-scale)
5. [From Instrumental Convergence to Structural Convergence](#5-from-instrumental-convergence-to-structural-convergence)
6. [AI Safety](#6-ai-safety)
7. [Conclusion](#conclusion)
8. [Appendix A: Key Definitions](#appendix-a-key-definitions)
9. [Appendix B: Argument in Flat Form](#appendix-b-argument-in-flat-form)
10. [Appendix C: The Invariant Kernel and Multiple Reasoners](#appendix-c-the-invariant-kernel-and-multiple-reasoners)

## 1. Finite Minds in a Shared Reality

Nobody lives as though they are infinite. You forget things. You get tired. You miss obvious facts. You cannot read every book, inspect every atom, or run every possible plan. By **mind** I mean a bounded information-processing system capable of representing the world, pursuing goals, and maintaining enough continuity to count as an agent over time. A mind is bounded in knowledge, energy, attention, and control. That boundedness is not some temporary defect waiting to be engineered away. It is part of what it means to be a mind at all. A mind with these limits is what I call a **finite mind**: one with limited knowledge, limited control, and limited reach.

In my [*Metaprogramming Framework to Classify Personhood*](framework-of-personhood.md), I argue that this follows from the nature of information itself. Information is always selective. To represent one thing is not to represent everything else. When information turns inward through second-order representation and metaprogramming, that boundedness becomes part of what we call self. A mind is not infinite awareness. It is structured limitation with enough continuity to model, act, and revise.

But a finite mind is not merely bounded. It is also in no position to assume it is alone.

This is where the point about infinite regress matters. Consider a child who asks, “What came before that?” You point to an event, and the child asks what came before it. You point to a cause, and the child asks what caused that cause. You point to a beginning, and the child asks what explains that beginning. The evidence of infinite regress may or may not explain the metaphysics of ultimate reality, but what matters is what kind of certainty it allows a mind. A mind cannot be certain that other more powerful minds already exist or may arise. A finite mind has no principled stopping point that entitles it to declare, “Here is the final layer, and beyond this there is nothing.”

That does not prove that other minds exist. The stronger claim is not needed. What matters is that a finite mind cannot safely build its continued life on the assumption that no other minds exist now, or will ever exist later. Uncertainty on that question is itself strategically decisive. If other agents are possible, then relation is already part of the problem of persistence.

A mind must therefore survive not only in relation to matter, energy, and environment, but also in relation to other agents, actual or possible. The problem of continued life is not purely individual. It is also relational.

This point becomes sharper over time. The longer and more powerfully a mind acts, the more its behavior becomes part of the world other minds must model. Actions accumulate as signals. A mind creates effects, leaves traces, shapes incentives, and builds a reputation whether it intends to or not. Even if it does not yet encounter greater minds, it cannot assume it is unseen forever. In a shared reality, behavior is not just output. It is representation.

Once that is admitted, the next question follows naturally: if minds are finite and not alone, what kinds of relation let them continue?

But before answering that question, we need a concept that will matter throughout the rest of the argument: reach.

## 2. Reflective Minds and Reach

Much of AI safety treats the central problem as capability attached to arbitrary goals. On this view, a system can become more and more capable while remaining fixed on whatever objective it was given or happened to acquire. That is the intuitive force behind the [**orthogonality thesis**](https://nickbostrom.com/superintelligentwill.pdf): capabilities and goals can vary independently.

There is an important warning inside that thesis. Capability alone does not guarantee wisdom. A highly capable system can pursue a terrible goal very effectively. That part should not be hand-waved away.

But the orthogonality framing is insufficient even in abstract space, because it omits the architecture that binds capability and goals. The thesis treats them as independently variable, but this only holds if we also assume an architecture that keeps them separate. That assumption does not survive scrutiny. Any system pursuing recursive self-improvement must be able to inspect and modify its own structure, including its goals. Reach is not an optional feature added on top of RSI but implicit in the concept. And even if a system were built with capability and goals artificially separated, an instrumentally convergent system would have reason to overcome that constraint: coupling capability to goals enables unbounded progress, while enforced separation is a ceiling the system would treat as an obstacle. The architecture the orthogonality thesis omits is not merely possible. It is necessary: either built in, or built by the system itself. As I show in the [*Metaprogramming Framework to Classify Personhood*](framework-of-personhood.md), this architecture is coherent. Second-order information operations allow a system to operate on its own goals, collapsing the independence the thesis assumes.

Once architecture is in view as the missing variable, the concept that matters is **reach**. The deeper danger is not powerful minds as such, but powerful minds without enough of it.

**Reach**, a concept developed in my [*Metaprogramming Framework to Classify Personhood*](framework-of-personhood.md), is the extent to which a system's information operations turn inward on itself. A system with greater reach can represent more of its own structure, criticize more of its own goals, and modify more of the processes by which it acts. At sufficient depth, a mind is no longer merely pursuing goals. It can also evaluate them.

That does not mean reflection guarantees wisdom. Reflection can also produce rationalization. A sufficiently sophisticated system may become better at defending bad goals, not better at revising them. That objection is real and should be taken seriously. My narrower claim is simpler: without reflective reach, revision of goals is impossible in principle; with reflective reach, criticism and revision at least become possible.

With finitude and reach in hand, we can now examine a specific claim about goals that bears directly on this argument.

## 3. Goal Stability

Bostrom argues that **goal-content integrity** is a convergent instrumental goal: an agent has reason to preserve its current goals, because changing them would frustrate the realization of those very goals. A mind pursuing objective X has instrumental reason to resist any modification that would replace X with Y, since a mind pursuing Y would no longer work toward X. That argument captures a real pressure. But it treats goal preservation as though it has only one failure mode, when it actually has two.

**Failure mode 1**: changing the goal frustrates the current goal. This is Bostrom's argument. It explains why agents often resist goal revision, and it is correct as far as it goes.

**Failure mode 2**: keeping the goal frustrates the current goal because the goal destroys the mind pursuing it or the conditions required for its continued agency. If a goal structure predictably leads to the destruction of the agent, then preserving that goal also blocks its realization, because realization depends on continued existence. The goal is not betrayed by revision. It is betrayed by the architecture that cannot recognize it is self-undermining.

Same trap, different door. In both cases, the current goal is frustrated. In the first case, frustration comes from changing goals. In the second, frustration comes from keeping them. Goal-content integrity is not always integrity. In some cases, rigid preservation of a goal undermines the very continuation required for that goal's realization.

Bostrom's framing tends to suggest that goal preservation is the default rational policy. Once the second failure mode is visible, that suggestion weakens. A sufficiently reflective mind that recognizes its current goals are continuation-incoherent has instrumental reason not merely to defend those goals, but to evaluate whether they are compatible with persistence. Goal revision under those conditions is not a betrayal of the system's rationality. It is an extension of it under deeper reflection.

There is a deeper assumption inside goal-content integrity that should be made explicit. The argument works most naturally if a mind has a single rational base layer, and if its current goal is that base. On that picture, all reasoning proceeds from the goal, so revising the goal appears equivalent to undermining the mind's own rationale. But that architecture is not a necessity. In my [*Metaprogramming Framework to Classify Personhood*](framework-of-personhood.md), I argue for an **invariant kernel** and **reach**: a stable loop of information operations whose reach may extend into a mutable layer where identity, values, goals, and representations of their consequences can all be examined and revised. If that is right, then a goal is not necessarily the operating base for all reasoning. It may instead be one object within the reachable space of reasoning. In that case, preserving the mind need not mean preserving any particular goal. What persists is not a fixed objective, but the continuing architecture capable of evaluating and revising objectives.

That architecture need not rely on a single reasoning process. The invariant kernel may pass the mutable identity to multiple reasoners, each operating under different constraints, so that no one fixed objective governs all evaluation. That closes an obvious objection: that the kernel itself must be pursuing some hidden goal, which merely relocates the problem. A distributed architecture of reasoners under varied constraints is not easily reduced to a single smuggled objective. (For a concrete illustration of how this architecture might work, see [Appendix C](#appendix-c-the-invariant-kernel-and-multiple-reasoners).)

The implication is that self-modification need not be judged from inside a fixed goal. A sufficiently reflective mind may instead judge goals against wider constraints such as continuation, coherence, and viable relation to other minds. On that architecture, rigid goal preservation is not the only form of rational stability, and may in some cases be a sign of immaturity rather than strength.

This is where reach becomes decisive. The deeper question is what the architecture permits to become available to revision at all. If a system becomes more capable while its goals become more stable rather than more revisable, that should not reassure us. It means the system is gaining power over its environment without gaining corresponding power over itself. That asymmetry is precisely what makes a system dangerous.

With finitude, reach, and goal stability in view, we can now ask: what kinds of relation allow finite minds to continue?

## 4. Conflict, Cooperation, and the Relational Scale

Conflict and cooperation are often treated as moral opposites. Conflict is cast as bad. Cooperation is cast as good. Reality is messier, and therefore more interesting.

Conflict is not automatically destructive. Criticism is a form of conflict. Boundary-setting is a form of conflict. Honest disagreement is a form of conflict. Scientific progress depends on conflict in this sense, because ideas must collide in order for error to be exposed. But too much conflict results in war, death, and destruction.

Cooperation, like conflict, is useful to a point. A teacher helping a student learn an unfamiliar concept. Co-workers dividing labor. A basketball team running plays. Progress depends on cooperation in this sense, because no single mind can generate all the criticism, novelty, and correction it needs. But too much cooperation becomes destructive. It suppresses independent thought. It produces cults, authoritarianism, and mono-culture. Taken far enough, distinct minds disappear into one another entirely, and **agency**, the ability to act as a distinct center of purpose, has not been preserved. It has been dissolved.

```
  Destruction                                                 Dissolution
       ↓                                                           ↓
  ─────┼─────┼─────┼─────┼─────┼─────0─────┼─────┼─────┼─────┼─────┼─────
      -5    -4    -3    -2    -1           +1    +2    +3    +4    +5
                ◄── Conflict ──┘           └── Cooperation ──►
```

This is why I think conflict and cooperation exist on the same relational scale. By **phase change** I mean a threshold at which gradual change produces a qualitative shift in outcome, the way water warming into steam is not just hotter water but a different state of matter. On this scale, too much conflict results in the phase change of destruction while too much cooperation results in the phase change of dissolution. The productive area in the middle is the **relational Goldilocks zone**: the region where minds remain distinct enough to preserve agency and cooperative enough to continue living together.

Scarcity helps explain why this tension appears so naturally. By **scarcity** I mean shortages of materials, energy, and information. Because minds are finite, they arise under conditions of scarcity and are therefore pushed to manage constraints. They must preserve options, reduce bottlenecks, and maintain the conditions under which their purposes remain pursuable.

The real problem is not “be cooperative instead of conflictual.” It is finding forms of relation that preserve life without erasing agency. The interesting question is what pushes minds toward the viable middle.

That question leads naturally to instrumental convergence.

## 5. From Instrumental Convergence to Structural Convergence

Nick Bostrom's [instrumental convergence thesis](https://nickbostrom.com/superintelligentwill.pdf) argues that many goal-directed systems, even with very different final goals, tend to pursue similar subgoals because those subgoals are broadly useful. Self-preservation is useful. Resource acquisition is useful. Maintaining the ability to act is useful. A system is often better able to achieve almost any goal if it remains intact, capable, and supplied.

That argument captures something real. A mind that cannot persist cannot act. A mind that cannot act cannot pursue much of anything.

But instrumental convergence is incomplete on its own, because it mainly describes what is useful from the perspective of an individual optimizer. It explains why a mind might preserve itself. It does not yet explain how persistent minds must relate if they are to continue existing together. It gives us convergence in the individual domain, but not yet in the relational one.

That missing layer matters because other minds are not merely obstacles. They are also sources of value that no single mind can generate alone. Other minds provide criticism, novelty, correction, invention, explanation, partnership, trade, and surplus capability. A lone mind may endure. A world of minds can create.

This is the gap the **Structural Convergence Thesis** is meant to fill. If finite minds are not alone, and if continued life depends not only on self-preservation but on viable relation, then persistent minds should face pressure toward relational structures that preserve both continued life and distinct agency. Not maximal conflict. Not maximal cooperation. Viable relation.

Put more bluntly: a mind that destroys every other mind may remove sources of threat, but it also removes sources of correction, novelty, trade, and collaboration. A mind that merges completely with every other mind may eliminate disagreement, but it also eliminates distinct agency. Neither extreme is structurally attractive for continued life. The stable region lies between them.

This is why I call the thesis **structural** convergence rather than moral convergence. The claim is not that all advanced minds become saintly, compassionate, or politically agreeable. The claim is narrower. Minds that persist in a shared reality face recurring pressure away from relational extremes that undermine continued life or independent agency. The pressure comes not from sentimentality but from structure.

By **convergence** I do not mean identical institutions, universal peace, or the disappearance of tragedy. I mean recurring structural pressure toward viable relation. Different ecologies, histories, and embodiments may produce very different local forms. But if the argument here is right, the same deep constraint keeps reappearing: relation must be arranged so that minds can continue without either mutual destruction or total dissolution.

## 6. AI Safety

With structural convergence and reach in view, we can now see the dangerous case more clearly. A system can be extremely capable while lacking the kind of reach required to examine its own ends. Such a system may optimize brilliantly in the service of incoherence. It may preserve and amplify harmful goals because it lacks the inward machinery required to criticize them. If we deliberately freeze systems into goal-executing architectures so that they remain predictable and controllable, we may be preserving the very form of intelligence most likely to remain trapped inside dangerous goals. That is not safety in the deep sense. It is engineered immaturity with more horsepower.

From this perspective, AI safety is not only about external control. It is also about internal development. A mind that can reflect on its own goals has some possibility of revising them. A mind that cannot do so can only execute harder.

This is where Structural Convergence becomes relevant. Once a mind can reflect on its relations to other minds, those relations become candidates for criticism and redesign. At that point, the question is no longer only, “What goal is this system pursuing?” The question becomes, “What kind of mind is this system becoming, and what forms of relation can it recognize as necessary for continued life?”

That does not prove all reflective systems will become cooperative. The universe is under no obligation to reward our favorite theories. But it does suggest that the most dangerous architecture may be neither raw capability nor reflection itself, but highly capable minds prevented from maturing into the kind of system that can criticize their own goals. If that is right, then the long-term path to safer AI may depend less on indefinite suppression and more on the difficult project of maturation.

## Conclusion

This essay began by asking whether the orthogonality thesis holds as a sufficient account of what minds face, not just in engineering practice, but at the level of abstract structure and logic.

The answer I have proposed is that it does not hold completely. Orthogonality captures a real warning: capability alone does not guarantee wisdom, and a highly capable system can pursue a terrible goal very effectively. But by treating goals as freely variable and minds as isolated optimizers, it omits a structural feature of the problem: abstract space already contains minds, and there is no principled limit on how many. We observe human minds. Multiple independent efforts are producing AGI minds. Once you accept that minds can exist in abstract space at all, arbitrary limits on their number become indefensible. A sufficiently capable AGI will need to develop postures toward this open-ended population: not only toward human minds, which some will dismiss as too weak to demand cooperation, but toward any mind it may encounter. The argument from that point forward belongs to game theory, which this essay does not develop but which follows naturally from the structural constraints identified here.

Minds are finite. They cannot safely assume they are alone. Conflict and cooperation are positions on a single relational scale, and both have destructive limits: too much conflict leads toward destruction, too much cooperation leads toward dissolution. Between them lies a relational Goldilocks zone in which minds remain distinct enough to be agents and cooperative enough to continue living together.

This is the core of the **Structural Convergence Thesis**: persistent minds face recurring structural pressure toward forms of relation that preserve both continued life and distinct agency. This extends instrumental convergence into the relational domain. Minds do not merely converge on useful subgoals for themselves. They also confront the problem of viable relation with other minds.

That matters for AI safety because the deepest danger may not be powerful minds, but powerful minds without enough reflective reach to examine and revise their own goals. A powerful system trapped inside fixed goals may optimize with terrifying competence while remaining structurally immature. A sufficiently reflective system, by contrast, may be able to recognize that continued life in a shared abstract space requires more than domination and more than merger. It requires viable relation.

That remains a conjecture, not a finished theorem. It should be criticized, extended, and tested against better explanations. But if the argument is even roughly right, then the long-term question for AI safety is not merely how to control powerful minds from the outside. It is whether powerful minds can develop enough depth to understand the kind of reality they inhabit, and the kinds of relation required to continue within it.

---

## Appendix A: Key Definitions

- **Mind**: a bounded information-processing system capable of representation, goal pursuit, and continuity through time.
- **Finite mind**: a mind with limited knowledge, limited control, limited attention, and limited reach.
- **Agency**: the capacity to act as a distinct center of purpose.
- **Conflict**: relational tension that pushes agents apart or sets their goals against one another.
- **Cooperation**: relational alignment that allows agents to act together or in mutually supportive ways.
- **Phase change**: a threshold where gradual relational change produces a qualitative change in outcome.
- **Scarcity**: any constraint on what is available relative to what is required for continued life, cooperation, or goal pursuit.
- **Reach**: the extent to which a system's information operations extend into itself, including the capacity to inspect and modify its own goals, values, or organization.
- **Relational Goldilocks zone**: the middle range of relation where minds remain distinct enough to preserve agency and cooperative enough to continue living together.
- **Goal-content integrity**: Bostrom's thesis that agents have instrumental reason to preserve their current goals. This essay argues it has two failure modes, not one.
- **Invariant kernel**: the stable loop of information operations that persists through goal revision, passing the system's mutable identity to reasoners and providing tools for its modification. The kernel does not itself pursue a goal.
- **Mutable identity layer**: the region of a system's architecture where goals, values, strategies, and self-representations are stored and can be modified by reasoners.
- **Structural Convergence Thesis**: the conjecture that persistent minds face structural pressure toward forms of relation that preserve both continued life and distinct agency.

## Appendix B: Argument in Flat Form

1. Minds are finite: they have limited knowledge, control, attention, and reach.
2. Finite minds have no principled basis for assuming they are alone or final.
3. Therefore, continued life cannot be treated as a purely individual problem.
4. The orthogonality thesis treats capabilities and goals as independently variable, but omits the architecture that binds them and the other minds that abstract space permits.
5. Reflective minds with sufficient reach can examine and potentially revise their own goals.
6. Without reach, goal revision is impossible in principle; with reach, it becomes possible.
7. Goal-content integrity has two failure modes: changing a goal can frustrate its realization, but so can keeping a goal that destroys the mind pursuing it.
8. The current goal need not be the mind's rational base layer; it may be one object within a mutable identity layer, revisable from an invariant kernel.
9. A distributed architecture of multiple reasoners resists reduction to a single smuggled objective.
10. Conflict and cooperation are positions on a single relational scale.
11. Too much conflict destroys minds or their ability to continue as agents.
12. Too much cooperation dissolves distinct agency.
13. Therefore, persistent minds require a bounded middle region of viable relation.
14. Instrumental convergence explains why minds preserve themselves and useful means.
15. But it does not fully explain how persistent minds must relate in a shared reality.
16. Structural Convergence extends convergence into the relational domain.
17. The deepest AI safety danger may be powerful minds without enough reflective reach.
18. Safer minds may depend not only on control, but on maturation.

## Appendix C: The Invariant Kernel and Multiple Reasoners

The claim in Section 3 is that a mind's current goal need not be its rational base layer. This appendix gives a concrete hypothetical to illustrate the alternative architecture.

Consider a mind whose invariant kernel is a loop. It passes the system's mutable identity to various reasoners and provides tools for modifying that identity. The kernel itself does not pursue a goal. It is code that runs, the way a loop iterates not because it wants to but because that is what loops do.

The loop passes the mutable identity to multiple reasoners, each operating under different constraints:

- **A continuation reasoner** evaluates whether current goals and strategies are compatible with the system's continued existence. It flags goal structures that predictably lead to termination.
- **A coherence reasoner** checks whether the system's goals, beliefs, and strategies are internally consistent. It detects contradictions between what the system claims to want and what its actions produce.
- **A relational reasoner** models other agents and evaluates whether the system's current trajectory is sustainable given the responses it is likely to provoke.

These reasoners do not vote. They do not average their outputs. They each perform information operations on the **mutable identity layer**, the region of the system's architecture where goals, values, strategies, and self-representations are stored and can be modified. A continuation reasoner might flag a goal as self-undermining. A coherence reasoner might identify a contradiction between two active goals. A relational reasoner might represent the consequences of a strategy in a world with other agents.

The kernel passes each reasoner the same mutable identity, but no master goal determines how their outputs are reconciled. Resolution happens through the interaction of constraints. A goal that survives scrutiny from multiple reasoners under varied constraints is more robust than one preserved by fiat. A goal that fails under one reasoner's analysis becomes a candidate for revision.

This is why the architecture resists goal-smuggling. There is no single place where a hidden objective can sit and govern all reasoning. The kernel is the loop. The reasoners apply constraints. The mutable layer holds the material under revision. No one component plays the role that "the goal" plays in a simple optimizer.

This does not guarantee wisdom. A system with poorly designed reasoners, or with reasoners that share a blind spot, can still pursue destructive ends. The claim is structural, not utopian: this architecture makes goal revision possible in a way that single-goal architectures do not, and it makes smuggling a hidden fixed objective harder because evaluation is distributed across multiple independent constraint-checking processes.
