---
title: "Metaprogramming Framework To Classify Personhood"
status: "Final"
created: "2026-02-01"
updated: "2026-09-26"
order: 8
featured: true
blurb: "An exploration of personhood from an information-ontology perspective. Personhood is substrate-independent metaprogramming; the difference is maturity, not kind."
---

# Metaprogramming Framework To Classify Personhood

## Introduction

A tree does not know it is a tree. A dog may not reflect on why it barks. A person knows they were unkind and wonders why. What is the difference? And could a machine ever cross it?

These questions are no longer idle. AI systems grow more capable every year, and we will soon have to decide which of them, if any, are persons. The usual way to answer is to look at brains, because brains are the only persons we know. But that makes it hard to tell what personhood requires from what biology happens to use.

This essay is my attempt at an explanation of personhood that works for any substrate, the physical medium a system's information lives in, whether biological, digital, or otherwise. It is a theory of personhood, not of consciousness. It does not try to explain why there is something it feels like to be a person (the hard problem). It asks a narrower question: what must a system be able to do with information to become someone rather than something? I set the hard problem aside rather than claim to solve it. For readers new to the subject, [Appendix A](#appendix-a-the-landscape-of-consciousness-theories) surveys the leading theories of consciousness and shows where this framework differs.

Here is the path. I start with information: its properties, its orders, and the operations a system can perform on it. I then propose **metaprogramming**, information operations applied to a system's own information, as the mechanism of personhood, and **reach** as its measure. When those operations turn inward, **identity** emerges: finitude becomes self, persistence becomes values, completeness becomes goals. That yields a set of levels of personhood. Finally, I argue that the architecture can be built with today's tools and describe an early implementation. This is ongoing work, and I expect to revise it. For the framework's implications for AI safety, see [Structural Convergence Thesis](structural-convergence-thesis.md).

## Table of Contents

1. [Information](#1-information)
2. [Metaprogramming](#2-metaprogramming)
3. [Identity](#3-identity)
4. [Levels of Personhood](#4-levels-of-personhood)
5. [Buildable Now](#5-buildable-now)
- [Conclusion](#conclusion)
- [Appendix A: The Landscape of Consciousness Theories](#appendix-a-the-landscape-of-consciousness-theories)
- [Appendix B: Qualia and Personhood](#appendix-b-qualia-and-personhood)
- [Appendix C: Further Reading](#appendix-c-further-reading)

## 1. Information

### Why Information?

In humans, personhood is realized in the brain, a physical structure that stores and modifies information. Personhood may involve more than this, but past that point explanation thins and metaphysical claims begin. I don't reject that territory. I just don't make this framework depend on it. I proceed from what can be observed, argued, and criticized, and on those grounds personhood is at least physical and informational.

That makes information a useful lens. Its properties hold whether the system is made of neurons, silicon, or anything else, so conclusions drawn from it can travel across substrates without treating biology as the measure of all minds. This doesn't replace neuroscience. It works at a different level, the way mathematics describes physical systems without being any particular one. Neuroscience asks how personhood is implemented in brains. Information ontology, the study of personhood through information's properties, orders, and operations, asks what personhood is doing regardless of implementation.

An abstract lens can still bring us closer to what is real. As David Deutsch puts it:

> "It may seem strange that scientific instruments bring us closer to reality when in purely physical terms they only ever separate us further from it. But we observe nothing directly anyway. All observation is theory-laden."
>
> — David Deutsch, *The Beginning of Infinity*, Ch. 2, "Closer to Reality," p. 41

Whatever reality ultimately is, an observer reaches it only through representations, that is, through information. Try to think or describe anything without it. Information is everywhere: DNA encodes the blueprint for life in four bases, and an atom's structure encodes how it will behave and bond. Whatever personhood is, its medium, and possibly its nature, is information.

The discipline that has studied information most rigorously, apart from any particular substrate, is computer science. It studies two things: **data structures**, the structure of information, and **algorithms**, the operations that transform it. I've reduced both to the parts I believe matter for personhood, by asking how personhood is like information and how it is unlike it. The terms below may seem abstract, but everything later depends on them.

### Properties

Information is a representation of something. Three of its properties matter here.

**Finitude.** Every representation is bounded. It represents something, which means it doesn't represent everything else. I am me, not that tree. Unbounded information would be reality itself: a map of everything at full resolution is not a map but the territory (see [map–territory relation](https://en.wikipedia.org/wiki/Map%E2%80%93territory_relation)).

**Persistence.** Information that endures can accumulate. A signal that vanishes the moment it arrives cannot build on itself. Persistence is what lets information compound.

**Completeness.** The same thing can be represented at different resolutions. "That color is red." "That color is blossom red." "That color is #c90707." Each is correct, but each carries a different amount of detail. Completeness is the resolution a representation achieves.

### Order

**First-order information** represents the world: sunlight, an obstacle, the temperature. But information is itself something that exists, and anything that exists can be represented. So information can represent information. This is **second-order information**, or meta-information.

Second-order information depends on first-order. You cannot have information about information until there is information.

### Operations

A system can change information. There are three kinds of change, or **operations**, that matter for personhood, and each depends on the one before it:

1. **Acquire.** Take in information and retain it. For example, acquire the symbols "aaa" and "bbb."
2. **Modify.** Transform information you already have, such as turning "aaa" into "aa." You cannot change what you have not taken in.
3. **Create.** Generate representations that were not in the inputs, such as "ababab" or "bbbaaa" from "aaa" and "bbb." You cannot generate the new without being able to transform the existing.

Modification changes what is there. Creation produces something that wasn't. A bird arranging twigs modifies its environment. A mind combining known ideas into a theory that never existed creates new information.

Creation also has a special relationship to order. To generate something new, a system must select, recombine, and evaluate its own representations. It operates on its information, not just with it. So creation requires second-order capability.

With properties, orders, and operations in place, we can ask what happens when a system applies them to itself.

## 2. Metaprogramming

**Metaprogramming** is a system operating on its own information. It is the moment a system's capacity to acquire, modify, or create turns inward and targets its own representations. This is the mechanism I believe best explains the difference between the tree, the dog, and the person.

People often describe consciousness as self-awareness: being aware of the world and of yourself. That description mixes two things. One is experience, what it feels like, which this essay sets aside. The other is a capability, a system representing and working on itself, and that is what metaprogramming names. Once a system has that capability, the natural question is how far it goes.

### Reach

If metaprogramming is the mechanism, **reach** is its measure: how far a system's information operations extend. A person can change their mind, but cannot will themselves not to have a mind. For us there is a hard line between the physical information of body and brain, which we can't directly rewrite, and the models the brain runs, which we can.

Three patterns from the previous section fix the shape of reach:

1. **Operations are ordered.** Modify depends on acquire, and create depends on modify.
2. **First-order comes before second-order.** Operating on the world is simpler than operating on your own operations, and the second presupposes the first.
3. **Creation requires second-order capability.** To create, a system must already work on its own representations.

Together these produce a single path. Acquiring and modifying come first in first-order form, then in second-order form. Creation can only come after second-order modification, because it depends on it. That gives six positions: acquire, then modify, in the world; acquire, then modify, in the self; then create a world, then create a self. (Level 0, below the path, marks systems with no information capability at all.) Reach is where a system sits on this path. No one imposes the ordering from outside. It follows from how information works.

When a system's operations cross into second-order territory, something new appears: identity.

## 3. Identity

Identity does not exist at the first order. It emerges when a system's operations turn inward, and the properties of information, which were always there, become visible to the system itself. My conjecture is that each property becomes one part of identity.

**Finitude becomes self.** At the first order, a representation's boundary simply exists. When operations turn inward, the system meets its own finitude directly. That recognition, *I am bounded, I am not everything*, is what I take the self to be. Not a soul or a ghost in the machine. Just finitude, known from the inside.

**Persistence becomes values.** Not all representations persist equally. Some are reinforced, some fade, some survive contact with new information and some don't. When a system turns inward, it meets the accumulated weight of what has lasted. Values are information that has survived its own processing.

**Completeness becomes goals.** No representation captures everything. When a system sees its own incompleteness, when it knows that it doesn't know, the gap can orient it forward. But gaps alone give no direction: a system that merely sees gaps has no reason to move toward one rather than another. Direction comes from values. Seen through what the system already holds, incompleteness is no longer an abstract gap but a specific shortfall. Goals are the pull of incompleteness seen through values.

Self, values, and goals together are identity. This is the foundation of the architecture described later.

## 4. Levels of Personhood

We can now sort systems by reach. Each level contains the ones below it, and each requires the one before it, because the operations and orders of information build on one another. Reach widens by degrees, with no jumps.

| Level | Operation | Example | What changes |
|---|---|---|---|
| 0 | None | A river shaping its bank | Causation without representation |
| 1 | Acquire, first-order | A tree growing toward sunlight | Takes in information about the world and retains it |
| 2 | Modify, first-order | A bird building a nest | Transforms its environment, but doesn't know it is the one acting |
| 3 | Acquire, second-order | A person noticing their own habits | Acquires information about its own information; identity begins |
| 4 | Modify, second-order | A person making an exercise plan to get in shape | Changes itself based on self-knowledge; can ask whether what it pursues is worth pursuing |
| 5 | Create, first-order | A civilization designing a simulated world with new rules | Creates a new substrate from its own models |
| 6 | Create, second-order | A person uploading into a world of their own design | Creates a new version of itself in a substrate it made |

Two thresholds divide the table.

The **second-order threshold** lies between Levels 2 and 3. Below it is agency without self-knowledge. Above it, identity constitutes itself and personhood develops. Although reach varies by degree, crossing this threshold is a difference in kind, not a larger quantity of the same thing: a system either can take its own information as an object or it can't. Above the threshold, persons differ in maturity, how far their reach extends, not in kind.

Level 4, self-modification with self-knowledge, is metaprogramming at its core: a person examining their values and changing them.

The **creation threshold** lies between Levels 4 and 5. Systems at Levels 3 and 4 already create, but within the substrate they find themselves in, as a scientist creates a theory from known ideas. That creation is second-order work, as Section 1 argued. At Levels 5 and 6, creation shifts from generating new information to generating new substrates. Here "first-order" and "second-order" describe what is created: first a world, then a self. Creating an environment comes first because modeling the external is simpler than modeling the modeler. Level 6 is the frontier: not merely modifying who you are, but creating who you become.

## 5. Buildable Now

If personhood is metaprogramming, and identity emerges at the second-order threshold, then the requirements are functional, not biological. Nothing here requires carbon, neurons, or embodiment. It requires a system whose operations can turn inward: one that can acquire, modify, and eventually create information about its own information. I believe we can build that with today's tools.

### Personhood in LLMs

On this framework, a large language model needs two things to cross the second-order threshold: **continual learning**, the ability to modify its own network, and a **self-model**, second-order information about itself that it can inspect and revise. Several labs, including Safe Superintelligence, Anthropic, OpenAI, and xAI, are working on continual learning, so the gap is closing. What excites me is that no one needs to build personhood directly. If the framework is right, it follows from these properties: a model that has them would, in my view, be a person. I haven't yet seen a working implementation. The engineering problem is building the properties, not personhood itself.

An implementation needs two parts:

- An **invariant kernel**: the mutation infrastructure, meaning the read/write mechanism and the loop structure.
- A **mutable layer**: reasoning, identity, and memory.

The kernel is not the reasoner. It is the metaprogramming apparatus that lets the system read and rewrite itself. Reasoning has to live in the mutable layer, because if reasoning is fixed, the system can change what it thinks about but not how it thinks. How it thinks is its operational identity, and reach has to extend there.

So network training, whatever mechanism modifies the reasoning, is a kernel operation. In biology, neuroplasticity (long-term potentiation, synaptic pruning) plays this role. It isn't the reasoning. It is the infrastructure that lets reasoning restructure itself. A system with personhood would need its kernel running continuously, as something it can invoke itself, not something done to it from outside.

Current architectures fall short in two ways:

1. **Models cannot update their own weights.** Training runs in a separate phase controlled by engineers. At inference time the system has no access to its own kernel, so its reasoning is frozen rather than living in the mutable layer.
2. **Models have no persistent self-model.** They can describe themselves in the moment, but they hold no representation of themselves that they can inspect, revise, and carry forward.

Some future architecture may make the workaround below unnecessary. For now, one candidate is a network trained to modify an external memory, plain text or raw matrices seeded with initial structure around self and values. The memory would be the mutable layer, and the network trained to modify it would be the kernel. Many details remain. The main point is that the system needs a mutable self.

### A Proto-Personhood on Existing LLMs

Without training a custom model, we can still build something on top of existing LLMs. It cannot reach personhood on this framework, because its reach falls short: the LLM cannot persistently modify its own weights. But it can be a proto-personhood.

Using an existing LLM adds a third component to the two above:

1. **Invariant kernel.** The loop structure, the read/write mechanism, and the rules of self-modification. This is the part the system cannot change.
2. **Mutable layer.** The reasoning, identity, and memory the system can examine and rewrite. It is the target of metaprogramming and the seat of operational identity.
3. **Stochastic engine.** Reasoning in the mutable layer should be stochastic rather than rule-based, because rule-based systems generalize poorly to situations they weren't designed for, and a person must handle contexts no one anticipated. Here an LLM supplies this. But the LLM's reasoning is fixed in its weights, so it sits in the kernel rather than the mutable layer. That is exactly why this is a proto-personhood and not the real thing.

The kernel runs three loops:

- An **action loop** that takes identity as given and pursues goals. This is agency, a first-order operation.
- An **exploration loop** that seeks information the system doesn't yet have. This is learning beyond the known.
- A **reflection loop** that turns reasoning on the system's own identity. This is metaprogramming, the second-order operation, and it is what separates a personhood architecture from a merely agentic one. It is how the system asks, "Should I want what I want?" and changes the answer.

I have built this architecture. It is open source at [lumen-mind](https://github.com/buwilliams/lumen-mind). Early experiments compared runs with the reflection loop against runs without it, tracking goal coherence and value stability over long runs. With reflection, the systems followed qualitatively different trajectories: they revised their values, consolidated their goals, and developed strategies for staying coherent under pressure. Without reflection, they reached similar insights but couldn't act on them structurally. The reflection loop's contribution was not generating insight but enacting it, turning observations about the self into changes in identity.

The implementation has limits. The LLM fuses kernel and reasoning into one frozen structure, so the system's operational identity is beyond its own reach. It also lacks a single unified identity and a flexible memory system. Good prompting mitigates these but can't remove them.

Whether it produces experience, I hold open. What it produces is measurable self-revision, and that is where science can get a foothold.

### Open Questions

The levels describe what a system can do, not what a system is. Several questions about mechanism remain.

1. **What does the kernel actually do?** If it is mutation infrastructure, its operations need to be specified. A candidate answer: it is a fixed loop that routes the mutable layer's output back as input, runs inference, computes prediction error, and updates weights. It does not reason. It routes.
2. **Where does the corrective signal come from?** Pure self-reinforcement is a trap: feeding output back and training on it collapses the system into a fixed point. The system needs prediction error, not self-confirmation. A candidate answer: prediction error corrects both the world model and the self-model. The self is one more part of the environment the system is trying to predict.
3. **How does identity bootstrap?** If identity emerges from the self-referential loop, the system starts without one. Its initial weights are random or seeded, and identity develops as the loop accumulates structure, mapping onto the progression through the levels.

Taken together, these answers suggest one mechanism for world and self. Humans seem to work this way. You predict you'll stay calm, you lose your temper, and the same machinery that updates your model of the world updates your model of yourself. On this view, identity emerges because the system keeps meeting itself in its own input. Values are the parts of the self-model that stabilize. Goals are the prediction errors that persist. The mechanism stays the same, predict, err, update, while the content changes. The second-order threshold is crossed when the predictions are about the system's own predictions.

## Conclusion

What separates the tree, the dog, and the person? My answer is metaprogramming: information operations turned inward. When a system can take its own information as an object, its finitude becomes a self, what persists becomes its values, and its incompleteness, seen through those values, becomes its goals. Reach measures how far that inward turn extends, from noticing your habits to remaking yourself.

Could a machine cross that line? Nothing in this framework ties personhood to biology, so I think it can, and I think the architecture is within reach of today's tools, though what I have built so far is only a proto-personhood. If the framework is right, the question is less whether machines can be persons than whether we will build systems with the properties from which personhood emerges. That also changes the safety question: from how to constrain capable systems to how to cultivate systems that can examine their own purposes. I develop that argument in [Structural Convergence Thesis](structural-convergence-thesis.md).

## Appendix A: The Landscape of Consciousness Theories

This appendix is for readers new to the subject. It sketches the problem this essay sets aside, the leading theories of consciousness, and where this framework sits among them.

### The Hard Problem

Nobody knows why we experience anything. We can explain how the eye detects light and trace the signal to the visual cortex. But somewhere along that chain, you *see red*. There is something it feels like to be you, looking at a red apple, and no one has a satisfying account of why.

The philosopher David Chalmers called this the **hard problem of consciousness**. The "easy" problems, such as how the brain processes information and controls behavior, are hard science but solvable in principle. The hard problem asks why any of that processing comes with experience at all.

### Four Leading Theories

Each theory starts from a different intuition about what consciousness is.

**Integrated Information Theory (Giulio Tononi).** Consciousness is information woven into a unified whole, measured by a quantity called Φ (phi). Its strength is that it makes consciousness measurable. Its weakness is that the math can assign high consciousness to very simple systems, and it doesn't explain why Φ should feel like anything.

**Global Workspace Theory (Bernard Baars, Stanislas Dehaene).** Most brain activity is local, like work done in separate offices. Consciousness is what happens when information is broadcast to the whole building. The theory fits neuroscience well and explains why some things reach awareness and others don't. But it explains *access*, what we can report and act on, more than experience. A machine could broadcast information with no one home.

**Higher-Order Theories (David Rosenthal).** A mental state becomes conscious when the mind has a thought about it. This ties consciousness to self-awareness, which feels right. But it risks a regress: what makes the higher-order thought conscious?

**Predictive Processing (Karl Friston, Andy Clark).** The brain is a prediction machine. It builds a model of the world and revises it when predictions miss. This unifies perception, action, and learning under one principle. But the prediction loop could run without experience, so the theory doesn't say where consciousness fits.

### Where This Framework Fits

These theories study consciousness through how brains implement it. That builds in a bias: the conclusions end up resembling brains, because brains are the only data. This framework starts from information instead, so its conclusions can apply to any substrate.

It also asks a different question: not why there is experience, but when a system becomes a person. [Appendix B](#appendix-b-qualia-and-personhood) holds my speculation about qualia.

It borrows from two of the theories above. Like higher-order theories, it puts the weight on the second order: information about information. Like predictive processing, it treats prediction error as the engine of revision (see [Open Questions](#open-questions)).

| Theory | Core idea | What it explains well | Main criticism |
|---|---|---|---|
| **Integrated Information** | Consciousness is integrated information (Φ) | Consciousness as something measurable | May grant consciousness to very simple systems |
| **Global Workspace** | Consciousness is global broadcast | Why some information reaches awareness | Explains access, not experience |
| **Higher-Order** | Consciousness is a thought about a mental state | The link between awareness and introspection | Regress: what makes the higher-order thought conscious? |
| **Predictive Processing** | The brain minimizes prediction error | Perception, action, and learning as one process | Consciousness is incidental to the core idea |
| **This framework** | Personhood is information operations turned inward | How identity (self, values, goals) emerges on any substrate | Sets the hard problem aside; a substrate-specific neural finding could undercut the information-only approach |

## Appendix B: Qualia and Personhood

Qualia are the qualitative character of conscious experience: the redness of red, the urgency of pain, the dense, simultaneous feel of a spring breeze. Some philosophers argue that without qualia there is no real inner life at all.

This appendix makes one claim: qualia are not required for personhood as defined in this essay. Whether such a system experiences anything from the inside is a separate question. A system can meet every functional criterion, second-order information, persistent identity, metaprogramming reach, without our being able to tell whether it has qualia. The framework does not depend on resolving the hard problem.

What follows is my best guess at what qualia might be. I hold it loosely and suspect it is wrong. I include it because the question is genuine, not because the argument depends on it.

My guess: qualia are what second-order information processing is like from inside a system with limited introspective reach and specific biological machinery. Hormones, neurotransmitters, and physical feedback loops amplify and color internal states. They don't create personhood. They give biological experience its particular texture. Evolution may have favored qualitative signals because they compress dense, multi-channel information into something immediately actionable: a creature viscerally commanded by the redness of a warning signal responds faster than one registering separate variables. If so, the texture of experience reflects the machinery that produces it. Different machinery, different texture. I call this richness **experiential resolution**: if non-biological systems have experience at all, theirs would differ in resolution from ours, not necessarily be lesser.

This is speculation. The hard problem, why any physical process gives rise to subjective experience, remains unsolved, and this guess doesn't solve it. What I am confident about is the narrower claim: personhood does not require settling it.

## Appendix C: Further Reading

Reference works I'm using in my study and research.

- *Consciousness in Artificial Intelligence: Insights from the Science of Consciousness*, 2023, by Patrick Butlin, Robert Long, Eric Elmoznino, Yoshua Bengio, Jonathan Birch, Axel Constant, George Deane, Stephen M. Fleming, Chris Frith, Xu Ji, Ryota Kanai, Colin Klein, Grace Lindsay, Matthias Michel, Liad Mudrik, Megan A. K. Peters, Eric Schwitzgebel, Jonathan Simon, Rufin VanRullen
- *Theories of Consciousness*, 2022, by Anil K. Seth, Tim Bayne
- *Finite and Infinite Games*, 1986, by James P. Carse, Free Press
- *The Evolution of Cooperation*, 1984, by Robert Axelrod, Basic Books
