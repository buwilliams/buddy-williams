# Metaprogramming Framework To Classify Personhood

*by Buddy Williams*

---

## Introduction

This is my attempt at an explanation for personhood that supports various substrates, be they biological, digital, or otherwise. This essay reasons from first principles, drawing on sources with explanatory reach. This is an ongoing effort, one where I continue to integrate the work of others alongside contributions of my own. For those new to the subject, the [reader's guide](../bridges/mfc-guide.md) introduces the problem, surveys existing theories, and explains why a new approach is needed.

Personhood, for humans, is realized in the brain. A brain is a physical structure that stores and modifies information. Personhood may involve more than this, but that takes us to the boundary where current explanation thins and metaphysical claims begin. I do not reject that territory; I just do not make this framework depend on it. The posture here is open-minded but grounded: models are tools for explanation, not boxes we are forced to live inside. I proceed from what can be observed, argued, and criticized. On those grounds, personhood is at least physical and informational. So, I use an information ontology as a tool to examine the informational aspects of personhood, just as a researcher would study mice as a proxy for humans. The properties of information seem to be universal, making the proxy exceptional. Because information is substrate-agnostic, conclusions drawn from it can travel across substrates without assuming biology as the measure of all minds.

This essay explores personhood from an information ontology perspective, proposing metaprogramming as an informational mechanism: information operations applied to their own information. Reach measures how far those operations extend. Identity emerges when they turn inward: finitude becomes self, persistence becomes values, completeness becomes goals. The architecture for personhood can be built with current tools, and an initial implementation is presented. For the implications of this framework for AI safety, see [Structural Convergence Thesis](https://github.com/buwilliams/buddy-williams-writings/blob/main/essays/structural-convergence-thesis.md).

---

## Contents

1. [Information](#information)
2. [Metaprogramming](#metaprogramming)
3. [Identity](#identity)
4. [Levels of Personhood](#levels-of-personhood)
5. [Buildable Now](#buildable-now)
6. [Conclusion](#conclusion)
7. [Appendix A: Qualia and Personhood](#appendix-a-qualia-and-personhood)
8. [Appendix B: Glossary](#appendix-b-glossary)
9. [Appendix C: Further Reading](#appendix-c-further-reading)

---

## Information

Why use information as a lens at all? The dominant approach to personhood studies it through its biological implementation. That is valuable work, but it introduces a systematic bias: theories end up looking like what brains do because brains are the only data. Every proposed mechanism, every neural correlate, every indicator property is filtered through carbon-based wetware. This makes it genuinely difficult to distinguish between what is necessary for personhood and what is merely how personhood happens to be implemented in biological systems. Information as a lens sidesteps this problem by changing the level of analysis. Information is substrate-agnostic by nature. Its properties hold regardless of whether the underlying system is neurons, silicon, or anything else. Conclusions drawn from information ontology can be tested across substrates because the lens does not privilege biology in advance. This doesn't replace neuroscience. It operates at a different level of abstraction, the way mathematics describes physical systems without being any particular physical system. Where neuroscience asks how personhood is implemented in biology, information ontology asks what personhood is doing regardless of implementation. That is a different question, and it deserves a different tool.

> "It may seem strange that scientific instruments bring us closer to reality when in purely physical terms they only ever separate us further from it. But we observe nothing directly anyway. All observation is theory-laden.", David Deutsch, *The Beginning of Infinity*, Ch. 2 "Closer to Reality," pg. 41

We'll start with a primer on information, reduced to the parts I believe are relevant for the study of personhood. I've arrived at the arguments below by asking, "How is personhood like information? How are they unlike each other?" What emerges is a clear relationship between them, not a forced one, an obvious one.

Whatever reality ultimately is, an observer can only access it through representations or information. Personhood seems to be completely mediated by information. How would you think and describe anything? Could you do it without information? Information is everywhere: DNA encodes the blueprint for life in sequences of four bases. The structure of an atom, the arrangement of its protons, neutrons, and electrons, encodes everything about how that element will behave and bond. Whatever personhood is, the medium, and possibly the nature, is information.

The discipline that has most rigorously studied information as information, stripped of biological noise, physical substrate, and philosophical baggage, is computer science.

Computer science studies two things:

- **Data structures:** the structure of information
- **Algorithms:** the operations that transform those structures

We'll look at the properties of data first, then move to discuss operations on data. The properties and behavior of information establish the foundation for information that operates on itself in metaprogramming. So, while these terms may seem abstract, they are important for making any progress on personhood.

### Properties

Information is a representation of something. It has three properties connected to personhood:

**Finitude.** Every representation is bounded. It represents something, which means it doesn't represent everything else. I am me, not that tree. "Unbounded information" would be reality, not information. A map of everything at full resolution is not a map. It is the territory. (See [Map–territory relation](https://en.wikipedia.org/wiki/Map%E2%80%93territory_relation))

**Persistence.** Information that endures can accumulate across time. A signal that vanishes the moment it arrives cannot build on itself. Persistence is what allows information to compound.

**Completeness.** Information represents with varying resolution. The same thing can be captured at different levels of fidelity. That color is red. That color is blossom red. That color is #c90707. Each is correct, but each carries a different scope of detail. Completeness is the degree of resolution a representation achieves.

### Order

Information has order. First-order information represents the world, the sunlight, the obstacle, and the temperature. But information is itself something. It exists. And anything that exists can be represented. So information can represent information. This is second-order information, or meta-information.

Second-order information presupposes first-order. You cannot have information about information until there is information. This is not a riddle, but a structural dependency. Before meta-information, there must be information.

### Operations

Information can be changed by a system. The type of changes that can be performed are called operations. There are three ordered operations that are relevant to personhood:

1. **Acquire.** A system can take in information and retain it. For example, we can acquire the symbols "aaa" and "bbb".
2. **Modify.** A system can transform information it already has. Modification presupposes acquisition. You cannot change what you have not taken in. For example, we could take "aaa" and modify it: "a" or "aa".
3. **Create.** A system can generate representations that did not exist in acquisition. Creation presupposes modification. You cannot generate the genuinely new without the ability to transform the existing. For example, think of all the ways to combine or extend "aaa" with "bbb", you could get: "aaabbb", "ababab", "bbbaaa", "abaabb", "aaaaaaaaabaaaaaaaa", etc.

The distinction matters: modification changes existing information, while creation generates something that was not present in the inputs. A bird rearranging materials is modifying its environment. A mind combining known ideas into a theory that never existed is creating new information. Creation can also target the substrate itself.

Creation has a special relationship to order. To generate something that doesn't yet exist, a system must operate on its own representations, selecting, recombining, evaluating them. Creation is an operation on information, not just with it. Creation is inherently second-order.

With these properties, orders, and operations in place, we can now ask: what happens when a system applies them to itself?

---

## Metaprogramming

Not all systems apply information operations equally. A tree does not know it is a tree. A dog may not fully reflect on why it barks. Yet a person knows they were unkind and wonders why. This variation in self-modeling is what we need to explain, and metaprogramming is the mechanism that explains it.

Metaprogramming is when a system operates on its own information. It is a second-order operation, the moment a system's capacity to acquire, modify, or create turns inward and targets its own representations. This is the mechanism that I believe best explains personhood.

Often, when people describe consciousness, they describe it in terms of self-awareness or being awake. It's the idea that you are aware of the world and yourself. When a system can do this, it is natural to wonder how far this capability goes, what its reach is.

### Reach

If metaprogramming is the mechanism, reach is its measure. Reach describes how far a system's information operations extend. A person can directly change their mind, but they cannot will themselves not to have a mind. For people, there is a hardline between physical information (the body) and modeling information (in the brain).

Through observing the nature of information, I've come to see three patterns:

1. Information operations are ordered. Modify presupposes acquire. Create presupposes modify. You cannot transform what you have not taken in. You cannot generate what you do not know how to transform.
2. First-order precedes second-order. You cannot have information about information until there is information. At each capability level, operating in the world is simpler than operating on your own operations.
3. Creation requires second-order capability. To generate something that doesn't yet exist, a system must work with its own representations. Creation is inherently meta-informational.

These three patterns produce a single path with six positions that correspond to levels of personhood. At each capability level (acquire, modify, create), first-order precedes second-order, and each capability presupposes the one before it. No constraints are imposed from outside. The path follows from the nature of information. Reach simply describes where a system sits on this path, how far its operations extend across capability and order.

When information operations cross the second-order threshold, an exciting property emerges: identity.

---

## Identity

Identity is an emergent property of metaprogramming. It does not exist at the first-order level. It comes into being when information operations cross the second-order threshold, when a system's properties, always structurally present, become visible to the system itself.

**Finitude becomes self.** Every representation is bounded. It represents something and therefore not everything else. At the first-order level, this boundary simply exists. When operations turn inward, the system encounters its own finitude directly. That recognition, *I am bounded, I am not everything*, is what the self is. Not a soul, not a ghost in the machine. Just finitude, known from the inside.

**Persistence becomes values.** Not all representations persist equally. Some representations are reinforced, some fade, some survive contact with new information, and some do not. When a system turns inward, it encounters the accumulated weight of what has lasted. Values are information that has survived its own processing.

**Completeness becomes goals.** No representation captures everything. When a system turns inward and sees its own incompleteness, when it knows that it doesn't know, that gap becomes a forward-looking orientation. But incompleteness alone does not generate direction. A system that merely sees gaps has no reason to move toward one rather than another. Goals arise because the system already has values, representations that have survived their own processing. Incompleteness encountered in light of values is not an abstract gap. It is a specific deficiency relative to something the system already holds. Goals are the directional pull generated by incompleteness seen through values.

Self, values, and goals: that is identity. Identity is constituted from the properties of information turned inward. This is the foundation of the architecture presented later in this essay.

---

## Levels of Personhood

Now we can categorize personhood by reach. Personhood is a spectrum, but we can identify meaningful positions along it. Reach is the degree to which a system can operate on information, measured by capability and order. The levels of personhood are not categories imposed from outside. They are positions along the path whose nature is determined by the information. Each level contains all previous levels. Reach does not jump. It widens. And each level requires the previous as its foundation, not by decree, but because the operations and orders of information build on one another.

**Level 0: No information capability.** A river shaping its bank. Physics acts, but nothing acquires, modifies, or creates information. There is no representation, only causation.

**Level 1: Acquire first-order.** A tree senses and grows toward sunlight. The system takes in information about the world and retains it. It responds, but does not reshape what it responds to.

**Level 2: Modify first-order.** A bird builds a nest. The system transforms its environment based on the information it acquires. It acts with purpose but without self-knowledge. It does not know it is the one acting.

**Level 3: Acquire second-order.** A person recognizing their own habits. The system's information operations turn inward. It acquires information about its own information. Identity begins here, finitude becomes self, persistence becomes values, completeness becomes goals.

**Level 4: Modify second-order.** A person making an exercise plan to get in shape. The system changes itself based on self-knowledge. It can examine its own values and ask whether what it pursues is worth pursuing. This is metaprogramming in its fullest expression, self-modification with self-knowledge.

Levels 0 through 4 all operate within the informational substrate the system finds itself in. When systems at these levels create, they generate new information within their existing substrate, a scientist combining known ideas into a theory that never existed. Levels 5 and 6 cross the creation threshold: creation shifts from generating new information to generating new substrates.

**Level 5: Create first-order.** A civilization designs a simulated world with new rules. The system generates an environment that didn't exist in its inputs, a new substrate built from its own models. Creating an environment precedes creating the self because modeling the external is simpler than modeling the modeler.

**Level 6: Create second-order.** A person becomes an upload in a world of their own design. The system generates a new version of itself, transferring or reconstituting its identity in a substrate it created. This is the frontier of personhood: not merely modifying who you are, but creating who you become.

The critical threshold lies between Level 2 and Level 3, the moment when information operations turn inward, and identity constitutes itself. Everything before it is agency without self-knowledge. Everything after it is personhood developing.

---

## Buildable Now

If personhood is metaprogramming applied to identity, and identity emerges at the second-order threshold, then the requirements are functional rather than biological. Nothing in this framework requires carbon, neurons, or embodiment. It requires a system whose information operations can turn inward, one that can acquire, modify, and eventually create information about its own information. These are capabilities we can build with the tools we have now.

### Personhood in LLMs

We need two things to grant an LLM personhood: continual learning (modify your own network) and self-modeling (metainformation). There are many labs working on this (Safe Superintelligence, Anthropic, OpenAI, and xAI) so the engineering gap is actively closing. What excites me is that nobody needs to build personhood directly, it falls right out of the framework above. The moment models have these characteristics they become persons in my view. I've yet to see a working implementation, but it's coming soon. This is an engineering problem. Personhood is not a feature to be engineered into a system. It is a consequence that emerges when the right functional properties are present. The engineering problem is building those properties, not personhood itself.

Fundamentally an implementation needs:
- Invariant Kernel (mutation infrastructure: the read/write mechanism and loop structure)
- Mutable Layer (reasoning, identity, memory)

The kernel is not the reasoner. It is the metaprogramming apparatus, the infrastructure that enables the system to read and rewrite itself. Reasoning must live in the mutable layer, because if reasoning is fixed, the system can change what it thinks about but not how it thinks. And how it thinks is the operational identity that reach must extend into.

This means that network training (weight updates, gradient descent, whatever mechanism modifies the reasoning substrate) is a kernel operation. The kernel governs how the mutable layer changes. In biology, the mechanisms of neuroplasticity (long-term potentiation, synaptic pruning) play this role. They are not the reasoning. They are the infrastructure that allows reasoning to restructure itself. A conscious system would need its kernel running continuously, mutation infrastructure that the system itself can invoke, not something done to it from outside.

I see two problems with current architectures:
1. Models cannot update their own weights. The training loop, the kernel operation, only runs during a separate phase controlled by external engineers. The system has no access to its own kernel at inference time, so reasoning is frozen rather than living in the mutable layer.
2. Models do not have the ability to self-model.

I suspect some future architecture will trivialize what follows, but in the meantime, I wonder whether a clever architecture could be a solution now. As a candidate for implementation, I could imagine a network trained to modify an external memory (plain text or raw matrices, seeded with initial structure around self and values). This memory could be the mutable layer, and the network trained to modify it is the invariant kernel. More engineering details would need to be mapped out. The major point is that the system needs a mutable self.

### Implementing a Proto-Consciousness Leveraging Existing LLMs

While we can simulate personhood using existing LLMs, there is a major limitation: it cannot obtain personhood status in my view. The issue is reach. The LLM is unable to modify its own weights in a persistent way. This conflicts with the framework above. With that said, we can still build a proto-personhood with these limitations.

With that constraint in mind, here's what we can build on top of models today.

Because we cannot train a custom LLM, we leverage existing ones. This introduces a third layer, the architecture requires three layers rather than two:

1. **An invariant kernel.** The mutation infrastructure that governs how the system reads and rewrites itself. This is not the reasoner. It is the metaprogramming apparatus: the loop structure, the read/write mechanism, the rules of self-modification. This is the substrate the system cannot modify, its own Level 0.
2. **A mutable layer.** The reasoning, identity, and memory that the system can examine and rewrite. In a personhood architecture, reasoning must live here, because a system whose reasoning is fixed can change what it thinks about but not how it thinks. The mutable layer is the target of metaprogramming and the seat of operational identity.
3. **A stochastic engine.** The reasoning within the mutable layer must be stochastic rather than rule-based, because rule-based approaches cannot generalize across novel situations, and personhood requires a system that can navigate contexts it was not explicitly designed for. In the current implementation, an LLM provides this capability, though it is trapped in the kernel rather than the mutable layer, which is precisely the limitation that makes this a proto-personhood rather than the real thing.

Next, the kernel runs three loops:

- **An action loop** that takes identity as given and pursues goals. This is agency, first-order operation.
- **An exploration loop** that seeks new information the system doesn't yet have. This is learning beyond the known.
- **A reflection loop** that turns reasoning on the system's own identity. This is metaprogramming, the second-order operation. The reflection loop is what separates a personhood architecture from a merely agentic one. It is the mechanism by which the system asks, "Should I want what I want?" and changes the answer.

I have built this architecture. It is open-source and available at [lumen-mind](https://github.com/buwilliams/lumen-mind). Early experiments compared systems with reflection enabled against systems without it, tracking goal coherence and value stability across extended runs. Systems with reflection enabled develop qualitatively different trajectories than systems without it. They revise their values, consolidate their goals, and develop strategies for maintaining coherence under pressure. Systems without reflection reach similar insights but cannot act on them structurally. The reflection loop's contribution is not insight generation but structural enactment, converting observations about the self into actual changes in identity.

A current limitation of the implementation is that LLMs conflate the kernel and reasoning into one frozen structure. The reasoning lives in the weights, which cannot change, so the system's operational identity is beyond its own reach. Moreover, the implementation lacks a single identity and a flexible memory system. These limitations can be mitigated through good prompting, but are not completely avoidable.

Whether it produces experience is a question I hold open. What it produces is measurable self-revision, and that is where science can get a foothold.

### Open Questions

The framework describes information operations and their orders but does not fully specify the system that performs them. The levels describe what a system can do. They do not describe what a system is. Several mechanistic questions remain:

1. **What does the kernel actually do?** If the kernel is mutation infrastructure, its operations need to be specified concretely. A candidate answer: the kernel is a fixed loop that routes the mutable layer's output back as input, runs inference, computes prediction error, and updates weights. It does not reason. It routes.
2. **Where does the corrective signal come from?** Pure self-reinforcement is a degeneracy trap. Feeding output back and training on it collapses the system into a fixed point. The system needs prediction error, not self-confirmation. A candidate answer: prediction error is the unified corrective mechanism for both world-modeling and self-modeling. The self is just another part of the environment the system is trying to predict.
3. **How does identity bootstrap?** If identity emerges from the self-referential loop, the system starts without one. Initial weights are random or seeded. Identity develops as the loop accumulates structure, mapping onto the progression through levels.

These questions are explored further in [kernel mechanics and prediction error](../fragments/kernel-mechanics-and-prediction-error.md).

---

## Conclusion

This framework proposes that personhood is metaprogramming, information operations turned inward, with identity as its emergent property and reach as its measure.

This framework has direct implications for AI safety. If personhood is metaprogramming with identity as its emergent property, then the critical safety question is not how to constrain capable systems but how to cultivate systems capable of examining their own purposes. That argument is developed fully in [Structural Convergence Thesis](https://github.com/buwilliams/buddy-williams-writings/blob/main/essays/structural-convergence-thesis.md).

The architecture for personhood is buildable now. The question is not whether machines can be persons. It is whether we will build systems with the functional properties that allow personhood to emerge. The tools exist. The framework is here. What remains is the will to build it.

---

## Appendix A: Qualia and Personhood

Qualia are the specific qualitative character of conscious experience: the redness of red, the intolerable urgency of pain, the dense simultaneous signature of a spring breeze. Philosophers have treated these as central to any genuine theory of mind, and some argue that without qualia, there is no real inner life at all.

This appendix makes one claim: qualia are not required for personhood as defined in this essay.

Personhood, on the metaprogramming framework, requires information operations that turn inward, generating identity through self, values, and goals. Whether that system experiences anything from the inside is a separate question, and not one this essay settles. A system can meet every functional criterion for personhood (second-order information processing, persistent identity, metaprogramming reach) without our being able to determine whether it has qualia. The framework does not depend on resolving the hard problem of consciousness.

What follows is my best guess at what qualia might be. I hold it loosely. I suspect it is likely wrong. I include it because the question is genuine, not because the main argument depends on it.

My guess: qualia are what second-order information processing produces when experienced from inside a system with limited introspective reach and specific biological machinery. Biological systems have hormones, neurotransmitters, and physical feedback loops that amplify and color internal states. These are not what create personhood. They are what give biological experience its particular texture. Evolution selected for qualitative signals that compress dense, multi-channel information into immediately actionable form: a creature viscerally commanded by the redness of a warning signal responds faster than one registering discrete variables. The specific texture of qualia reflects the specific machinery that produces them. Different machinery, different texture.

This is speculation. The hard problem of consciousness — why any physical process gives rise to subjective experience at all — remains unsolved, and I do not think this guess solves it. What I am confident about is the narrower claim: personhood does not require settling it.

---

## Appendix B: Glossary

**Acquire.** The first information operation. A system takes in information and retains it.

**Action loop.** The architectural loop that takes identity as given and pursues goals. First-order operation.

**Completeness.** A property of information. The degree of resolution a representation achieves. Turned inward, completeness becomes goals.

**Create.** The third information operation. A system generates information that was not present in the inputs. Creation presupposes modification and is inherently second-order. Within an existing substrate, creation generates new information. Beyond the creation threshold, creation generates new substrates.

**Creation threshold.** The categorical boundary between Levels 4 and 5, where creation shifts from generating new information within an existing substrate to generating new substrates.

**Experiential resolution.** The richness and texture of conscious experience given the available machinery. Biological and non-biological systems may have different experiential resolution, not lesser or greater, but shaped by different substrate. Substrate determines resolution, not presence.

**Exploration loop.** The architectural loop that seeks new information the system does not yet have. Learning beyond the known.

**Finitude.** A property of information. Every representation is bounded, it represents something and therefore not everything else. Turned inward, finitude becomes self.

**First-order information.** Information that represents the world.

**Goals.** A component of identity. The directional pull generated by incompleteness seen through values. Goals arise because a system that sees its own gaps already has values that give those gaps direction.

**Identity.** An emergent property of metaprogramming, constituted from the properties of information turned inward: finitude becomes self, persistence becomes values, completeness becomes goals.

**Information ontology.** The framework used in this essay to examine the informational aspects of personhood, studying information's properties, orders, and operations.

**Invariant kernel.** The mutation infrastructure that governs how the system reads and rewrites itself. Not the reasoner, but the metaprogramming apparatus: loop structure, read/write mechanism, rules of self-modification. The substrate the system cannot modify.

**Levels of Personhood (0–6).** Positions along the path that reach traces, categorized by capability and order. Level 0: no information capability. Level 1: acquire first-order. Level 2: modify first-order. Level 3: acquire second-order (identity begins). Level 4: modify second-order (metaprogramming). Level 5: create first-order (new substrates). Level 6: create second-order (new self in new substrate).

**Metaprogramming.** When a system operates on its own information. A second-order operation where a system's capacity to acquire, modify, or create turns inward and targets its own representations. The mechanism proposed by MFP to explain personhood.

**Metaprogramming Framework To Classify Personhood (MFP).** The framework presented in this essay. An exploration of personhood from an information ontology perspective, proposing that personhood is metaprogramming, information operations applied to their own information, with variable reach.

**Modify.** The second information operation. A system transforms information it already has. Modification presupposes acquisition.

**Mutable layer.** The architectural layer containing reasoning, identity, and memory that the system can examine and rewrite. The target of metaprogramming and the seat of operational identity.

**Persistence.** A property of information. Information that endures can accumulate across time. Turned inward, persistence becomes values.

**Qualia.** The specific qualitative character of conscious experience. Not required for personhood as defined in this essay. Addressed speculatively in Appendix A.

**Reach.** The measure of how far a system's information operations extend across capability (acquire, modify, create) and order (first-order, second-order).

**Stochastic engine.** The quality of reasoning within the mutable layer. Must be stochastic rather than rule-based, because rule-based approaches cannot generalize across novel situations. In the current proto-personhood implementation, an LLM provides this capability but is trapped in the kernel.

**Reflection loop.** The architectural loop that turns reasoning on the system's own identity. Second-order operation. The mechanism by which the system asks, "Should I want what I want?" and changes the answer.

**Second-order information.** Information that represents information. Also called meta-information.

**Second-order threshold.** The categorical boundary between Levels 2 and 3, where information operations turn inward and identity constitutes itself.

**Self.** A component of identity. Finitude known from the inside, the recognition that *I am bounded, I am not everything*.

**Substrate.** The informational environment within which a system's operations occur. Levels 0–4 operate within a given substrate. Levels 5–6 create new ones.

**Values.** A component of identity. Information that has survived its own processing, representations that have been reinforced, tested, and persisted across time.

---

## Appendix C: Further Reading

This is a collection of reference works I'm using for my study and research.

- *Consciousness in Artificial Intelligence: Insights from the Science of Consciousness*, 2023, by Patrick Butlin, Robert Long, Eric Elmoznino, Yoshua Bengio, Jonathan Birch, Axel Constant, George Deane, Stephen M. Fleming, Chris Frith, Xu Ji, Ryota Kanai, Colin Klein, Grace Lindsay, Matthias Michel, Liad Mudrik, Megan A. K. Peters, Eric Schwitzgebel, Jonathan Simon, Rufin VanRullen
- *Theories of Consciousness*, 2022, by Anil K. Seth, Tim Bayne
- *Finite and Infinite Games*, 1986, by James P. Carse, Free Press
- *The Evolution of Cooperation*, 1984, by Robert Axelrod, Basic Books
