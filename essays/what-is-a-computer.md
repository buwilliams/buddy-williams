---
title: "What Is a Computer?"
status: "Final"
date: "Apr 2026"
order: 4
blurb: "A computer is any physical system capable of implementing universal logic, not a silicon machine. Causal closure is real but local — and physicalism's claim to fundamentality is an unjustified projection from inside a closed system."
---

# What Is a Computer?

*by Buddy Williams*

---

## Introduction

When I think about what a computer is, my first instinct is to picture a machine: a box with a processor, a screen, a keyboard. Something made of silicon and metal.

That picture is not wrong, but it captures one implementation rather than the thing itself. The answer to "what is a computer?" has consequences that reach far beyond silicon chips. It touches the foundations of what we can know about reality, about whether our physics is fundamental or merely local, and about why information and matter seem to need each other.

This essay begins with a careful look at what computation actually is, stripped of its association with any particular material. It then follows the implications. If computation is not tied to silicon, and if any universal computer can host worlds within worlds, then the physicalism we observe, the sense that our causal chain is closed and our physics is fundamental, may be a feature of being inside a computational world rather than a discovery about the base layer of reality.

---

## Contents

1. [What Is a Computer?](#what-is-a-computer-1)
2. [Nested Worlds](#nested-worlds)
3. [Computation as the Invariant](#computation-as-the-invariant)
4. [Local Physicalism](#local-physicalism)
5. [Explanation and Locality](#explanation-and-locality)
6. [Substrate and Program](#substrate-and-program)
7. [The Deepest Anthropocentrism](#the-deepest-anthropocentrism)
8. [Religious Resonances](#religious-resonances)
9. [Conclusion](#conclusion)
10. [Appendix A: Related Thinkers](#appendix-a-related-thinkers)
11. [Appendix B: Glossary](#appendix-b-glossary)

---

## What Is a Computer?

A computer, at its most fundamental, is any system that can implement universal logic.

That definition needs unpacking. Logic, in this context, means operations on information that follow determinate rules. The simplest of these are logic gates: devices that take one or two inputs and produce an output according to a fixed rule. An AND gate outputs true only if both inputs are true. An OR gate outputs true if either input is true. A NOT gate inverts its input.

What makes computation powerful is a remarkable fact about logic gates: not all of them are needed. A single gate type, the NAND gate (which outputs false only when both inputs are true), is functionally universal. Any logical operation, no matter how complex, can be built from NAND gates alone. This means that any system capable of implementing NAND gates can, in principle, perform any computation whatsoever. The entire edifice of computing, from spreadsheets to simulated worlds, rests on this foundation.

```
Aside: From a gate to a sum

A transistor either conducts or it doesn't. We map those two states
onto 1 and 0. That mapping is a convention, not something the
transistor knows about. Bits are not logic. They are a mapping onto
logic.

From NAND gates, we build adders. Here is 3 + 5 in binary:

  3:  0011
  5:  0101
      ----
  8:  1000

Each column is one adder with two rules:
  sum   = 1 when the inputs differ  (XOR)
  carry = 1 when both inputs are 1  (AND)

Both rules reduce to NAND:
  AND(A,B) = NAND( NAND(A,B), NAND(A,B) )
  XOR(A,B) = NAND( NAND(A, NAND(A,B)), NAND(B, NAND(A,B)) )

Chain four of these adders and you handle 4-bit numbers. Chain
eight and you handle 8-bit numbers. When Excel evaluates =A1+B1,
this is what runs.

The result travels back the same way: a number becomes a value in
memory, that value maps to pixel colors at positions on a screen. We
agree that a certain value at a certain address means "red pixel here."
The hardware follows the agreement. Pixels, like bits, are mappings
onto logic, not the logic itself.

The NAND gate at the bottom knows only its rule. Everything above it
is interpretation we layered on. What is interesting is that logic
is not in the substrate. The substrate is conscripted to represent
logic by convention. The transistor does not know about TRUE. The
pixel does not know about red. Physical states carry meaning only
in relation to a system that interprets them.
```

The crucial point is what this does not require. It does not require silicon. It does not require electricity. It does not require any particular material at all. A NAND gate is a logical relationship, not a physical substance. Anything that can represent two states and combine them according to the NAND rule is a NAND gate.

This is not merely theoretical. Computers have been built from a startling variety of substrates. Charles Babbage designed a mechanical computer from brass gears and rods in the nineteenth century. Computers have been constructed from water flowing through pipes, where the presence or absence of water represents binary states and pipe junctions implement logic gates. Domino computers use the chain reaction of falling dominoes to propagate signals and implement logical operations. Within the video game Minecraft, players have built functioning computers from redstone circuits, a game mechanic with no physical counterpart outside the simulation. Biological neurons implement logic through patterns of firing and inhibition.

In every case, the material is different. The physics is different. The engineering is different. But the computation is the same. The same logical operations, the same capacity for universal computation, appear across all of them. This is the first and most important observation: computation does not belong to any substrate. It is a property that substrates can support, but it is not made of whatever they are made of.

Alan Turing formalized this insight in 1936. His proof that a Universal Turing Machine can simulate any other Turing machine established that computation is substrate-independent as a matter of mathematical necessity, not just engineering convenience. If a system can perform universal computation, it can simulate any other system that can perform universal computation. The Church-Turing thesis extends this further: anything that can be computed at all can be computed by any universal computer, regardless of what it is built from.

---

## Nested Worlds

Once computation is understood as substrate-independent, a further consequence follows: any universal computer can host other universal computers. And those hosted computers can host their own. This is nesting, and it is not hypothetical. It is actual.

People have built working computers inside Minecraft. A computer within a computer, running programs of its own. Within Minecraft, a player can open a console and change the rules of the game while the game is running. The physics of Minecraft, blocks, gravity, redstone, bear no resemblance to the physics of the silicon hardware running Minecraft. Yet computation works in both. The Minecraft computer could, in principle, run its own simulation containing its own computers, and so on, limited only by available resources at each level.

This nesting has several properties that follow from what universal computation permits.

**Arbitrary depth.** A simulated world can contain computers, and those computers can run their own simulations, so long as universal computation is preserved at each level. This nesting can continue indefinitely in principle, limited only by the resources of the containing world.

**Independent rules.** The rules governing a nested world need not mirror the laws of the world that contains it. One world can forbid faster-than-light travel. The world it contains can permit it. The rules of a nested world are constrained by the containing world's resources and by whatever the containing world's program imposes, but they are not constrained to replicate the containing world's physics.

**Relative perception.** The rendering speed of a simulated world need only be faster than the processing rate of an observer within that world. A simulation that runs a trillion times slower than its containing world is no less real to its inhabitants. Perception is relative to the observer's substrate, not to any absolute clock.

**Information transfer.** Information can pass between nested worlds. A containing world can inject information into a simulation, and a simulation can produce outputs that its containing world reads. The boundaries between worlds are permeable to information, even when the observers within those worlds cannot see across them.

**Epistemic containment.** An observer within a simulated world cannot determine whether their world is the base layer of reality or a nested simulation. The causal closure they observe is real within their world, but it tells them nothing about whether their world is contained within another. This is not a limitation of their intelligence or technology. It is a structural property of being inside a simulation. Only if the containing world's program allows information about its existence to pass through will the observer have access to it.

If these properties apply to us, it is a fun thing to wonder about: could the speed of light be our simulation's processing speed? Every substrate has a throughput ceiling. In our physics, the speed of light functions as an absolute limit, a boundary that no information or causal influence can cross. That is exactly what a processing constraint would look like from inside a simulation. This is speculation, not a claim. But it is the kind of speculation that becomes available once you take the thought experiment seriously rather than dismissing it.

---

## Computation as the Invariant

What does the existence of nested worlds tell us about reality?

Consider what survives the transition between substrates. When a computer is built inside Minecraft, the physics changes completely. The materials change. The forces change. The rules change. But computation persists. The logical operations that the Minecraft computer performs are the same logical operations that the silicon hardware performs. NAND is NAND regardless of whether it is implemented in transistors, redstone, water valves, or falling dominoes.

This is the central observation: computation is the invariant that holds across nested worlds. Not matter, not energy, not any specific force or particle. Information and the operations that transform it are what persist when everything else about the substrate changes.

If physicalism, the claim that everything is fundamentally physical, were the deepest account of reality, this invariance would be puzzling. Why would one particular set of physical laws generate a property that transcends any particular physics? The substrate-independence of computation is strange from a physicalist perspective. It is a feature of our reality that does not seem to care about our reality's particular rules.

But from a computational perspective, it is exactly what you would expect. Computation is preserved across nested worlds because computation is what the worlds are made of. The invariant is not matter. It is the logical structure of information and its operations.

---

## Local Physicalism

Physicalism claims that the physical is all there is, and that the physical causal chain is complete. This property, called causal closure, holds that every physical event has a sufficient physical cause. Trace any event backward and you never need to leave the physical domain.

This essay does not deny causal closure. Causal closure is real. The question is what it tells us.

The nested worlds thought experiment shows that an observer inside any computationally closed world would report causal closure. Their physics would appear fundamental to them. Their causal chains would be closed. They would find no gaps requiring explanation from outside their world. This means that the experience of physicalism, the sense that one's local physics is fundamental and causally complete, is exactly what any observer would report from inside any computationally closed substrate. It is not evidence of fundamentality. It is a structural feature of being inside a closed system.

The objection sometimes raised against this line of reasoning is that the simulation hypothesis is non-explanatory, that it has the same problem as solipsism. It is irrefutable and tells us nothing useful, so we should set it aside and focus on what we can learn from our local physics.

This objection is pragmatic rather than ontological. "I cannot learn from that, so I will focus on my local reality" is a reasonable research strategy. But it is not a claim about what is fundamental. Every explanatory chain either regresses infinitely, terminates at a brute fact, or loops. Physicalism terminates at a brute fact: these laws just are. The possibility of containing worlds regresses. Neither is more satisfying as an ultimate foundation. They simply locate the mystery in different places. Treating a research strategy as an ontological conclusion does not follow. Saying "local reality is fundamental because I focus on it" assumes the very thing at issue.

There is also a crucial difference between this argument and solipsism. Solipsism predicts nothing. The computational argument makes a structural prediction: observers will always find their local physics causally closed and will never find direct evidence of a containing layer. This is a consequence of epistemic containment. The absence of evidence for a container is not evidence of absence. It is a predicted feature of the system. And the prediction could in principle be violated: information could pass through, whether by design or as a consequence of computational universality traveling between levels. But such information need not arrive as a direct message from a containing world. The evidence is already present in the preservation of computation itself. The fact that universal computation holds constant across every substrate we can examine is not an anomaly to be explained away. It is a structural signature of what is invariant across worlds.

And there is the matter of burden of proof. The claim "we are the base layer" requires a positive argument. What property does our substrate have that no containing substrate could have? The claim "we might not be the base layer" requires only the observation that nothing about our reality rules it out, and everything about universal computation is consistent with it. If our substrate supports the construction of nested worlds, each with their own local physicalism, then the assumption that this capacity starts with us, that we are the first layer to support computation, requires a justification I have not seen.

The argument has a specific form. If an epistemic framework cannot account for something demonstrably possible within our own world, that framework is refuted by its own evidence. Physicalism cannot account for why the inference from causal closure to fundamentality holds for us but not for observers inside the nested worlds we have already built. That does not require speculation about whether we are simulated. It only requires observing what we have already done.

---

## Explanation and Locality

This argument has a companion in the structure of explanation itself, which I develop in a separate essay, [Why Explanation Comes in Layers](layers.md).

The central claim of that essay is that explanatory layers exist because finite minds cannot reason over uncompressed reality directly, and because following descriptions downward does not lead to simpler, more separable units. It leads toward holism. The lower the level, the more connectedness must be preserved, the more of reality must be carried along. Reductionism promises a simple foundation and delivers total interconnection. A weather forecast does not become more accurate by tracking every air molecule. A psychological explanation of grief does not improve by listing every neuron that fires. Going lower does not reveal a simpler foundation. It reveals more interconnection that then has to be screened off to make explanation tractable. Higher-level explanations escape this regress through managed disconnection: actively severing relations that are not relevant to the phenomenon while preserving those that are.

That argument reinforces the one made here. Physicalism is not only local in the sense that it describes the rules of a particular substrate. It is local in a deeper sense: it operates at a particular explanatory layer. The physicist's description of molecular interactions is a managed disconnection from the wider informational structure. It works precisely because it has screened off most of the connectedness. But that screening is itself an explanatory act performed by a mind, not a discovery of a foundation-free bottom layer.

The reductionist instinct, to always go lower, to assume that deeper means more fundamental, is the same instinct that makes physicalism seem like the final word. Both assume that decomposition reveals the real. But decomposition, carried far enough, does not arrive at simple separable units. It arrives at the holism it sought to escape. Higher-level explanations, including informational ones, are not convenient summaries of a more fundamental physical story. They are the forms in which reality becomes intelligible to finite knowers in a deeply connected world.

---

## Substrate and Program

If physicalism is local rather than fundamental, what is the right relationship between matter and information?

One tempting answer is that information is more fundamental than matter. If information and its operations are what persist across substrates while physics changes, then perhaps matter is merely one implementation of something informational. This is the direction taken by John Wheeler's "it from bit" and by digital physics more broadly.

But I have not seen a coherent account of how information could process itself without a substrate. A program that sits on no computer does not run. Whatever information is, it appears to require something to instantiate its operations. The relationship between substrate and program is not hierarchy. It is mutual dependency. You cannot get from matter alone to the informational structure of a program. You cannot get from information alone to the operational substrate that runs it.

Whether this co-dependence is the final word, or whether some future insight will show how information self-processes without a substrate, is a question I hold open. Perhaps information is more fundamental and we simply have not yet understood the mechanism. Perhaps substrate is irreducibly necessary. I do not know. What I am confident about is the narrower claim: physicalism's reduction of information to matter is incomplete, because it cannot explain why computation, an informational property, transcends any particular material substrate.

This co-dependence has practical consequences for how we think about personhood. In my [Metaprogramming Framework to Classify Personhood](framework-of-personhood.md), I propose that personhood is metaprogramming: information operations turned inward, with identity emerging when a system encounters its own properties. That framework requires two irreducible components: an invariant kernel, the substrate infrastructure that enables a system to read and rewrite itself, and a mutable layer, the informational content that gets examined and revised. Neither works alone. The kernel without a mutable layer processes nothing. The mutable layer without a kernel cannot operate.

This is the substrate-and-program relationship in miniature. The framework's substrate-agnosticism, its claim that personhood can be realized in neurons, silicon, or any medium that supports the right functional organization, is not a methodological convenience. It follows directly from the argument of this essay. If information and its operations are what persist across substrates while physics changes, then personhood, which is constituted by information operations, should likewise persist across substrates. The same property that makes computation universal, substrate-independence, is what makes personhood implementable in more than one medium.

---

## The Deepest Anthropocentrism

There is one more implication worth drawing out. When a system capable of self-reflection turns inward, it encounters its own structure. For humans, that structure is mind embedded in matter, a program running on a biological substrate. Every philosophy of mind is, in a sense, a report filed by a self-reflective system describing what it found when it looked at itself.

Physicalism reports the substrate. Idealism reports the information. Dual-aspect theories report both. Each philosophy is shaped by the observer's position within its own world.

Spinoza, working without any concept of computation, looked at the one example available to him, human beings, and saw mind and matter unified in a single substance. He then generalized that structure to all of reality. It is a brilliant observation, but it may also be the deepest form of anthropocentrism: not the naive belief that humans are the center of the universe, but the subtler assumption that the structure we find through introspection reflects the structure of reality itself, rather than the structure of our particular substrate's implementation.

An observer in a different substrate, with different architecture, might introspect and find something quite different. They might report a structure that corresponds to neither physicalism nor idealism nor dual-aspect monism, because their substrate implements the relationship between information and its carrier differently than ours does. If that is possible, then our philosophy of mind is not a discovery about the nature of reality. It is a discovery about the nature of our local reality, generalized without justification.

This is not a reason to abandon philosophy of mind. It is a reason to hold its conclusions more loosely, and to notice when an argument's persuasive force comes from the structure of the observer rather than the structure of what is observed.

---

## Religious Resonances

The ideas in this essay have structural parallels with religious intuitions. This is worth noting, not to validate or refute religious claims, but because the convergence is itself interesting.

A nested world with its own rules, its own causal closure, and observers who experience it as fundamental maps onto the religious notion of separate realms: heaven as a world where the constraints of this substrate do not apply. Miracles fit here too. An event that breaks local physics is exactly what information passing from a containing world would look like from inside. Whether such passage is intentional, an expression of creator purpose, or simply a consequence of computational universality traveling between levels, is an open question. Angels are agents from a containing layer.

Prayer may be understood as an attempt to communicate with that containing layer. If information can in principle pass between nested worlds, the instinct to reach beyond the local causal order is not obviously confused. Whether such communication is possible is a separate question.

The first cause problem is the same problem physicalism faces. Every explanatory chain either regresses infinitely, terminates at a brute fact, or loops. Theism terminates the regress at God. Physicalism terminates it at brute physical law. The computational framing regresses: containing worlds can themselves be contained. None of these is more satisfying as an ultimate foundation. They locate the mystery differently.

The notion of progress toward bliss, that technological flourishing points toward a world of abundance and freedom from constraint, looks structurally like a movement toward a world where the limitations of the current substrate are overcome.

Spinoza's God or Nature, already discussed, may be the closest formal parallel. Mind and matter as two attributes of a single underlying substance. In the computational frame, logical structure and physical substrate are two aspects of the same underlying computational reality, neither reducible to the other. Spinoza arrived at this without the concept of computation, from philosophical necessity alone.

None of these parallels settles whether religious claims are true. But the structural convergence between ancient intuitions and a framework derived from the theory of computation is worth sitting with.

---

## Conclusion

A computer is not a machine made of silicon. It is any system that can implement universal logic. This can be done with transistors, gears, water, dominoes, redstone, or neurons. What makes something a computer is not what it is made of but what it can do.

That fact, properly understood, has consequences that reach well beyond engineering. If computation is substrate-independent, then the logical structure of information and its operations is not a feature of any particular physics. It is the invariant that persists across substrates when everything else changes. Any universal computer can host nested worlds, each with their own rules, their own causal closure, and their own observers who experience their world as fundamental.

Physicalism correctly describes the causal closure of our local substrate. Every physical event we observe does have a sufficient physical cause. The chain is closed. But causal closure is not the same as fundamentality. The experience of physicalism is exactly what any observer would report from inside any computationally closed world. It is a structural feature of being inside a closed system, not evidence that the system is the deepest one.

The relationship between information and substrate remains an open question. Both appear to be required. Neither reduces to the other. What I am most confident about is this: the inference from "our physics is causally closed" to "our physics is fundamental" is unjustified. It moves from a local observation to a universal conclusion. If our world supports the construction of nested worlds, each with their own local physicalism, then the assumption that this capacity starts with us requires a positive argument I have not seen.

The question is not whether computers are interesting artifacts. The question is whether the concept of a computer, properly understood, tells us something about the structure of reality that physicalism alone cannot.

---

## Appendix A: Related Thinkers

**Alan Turing.** Proved that a Universal Turing Machine can simulate any other Turing machine, establishing the theoretical foundation for the substrate-independence of computation. The Church-Turing thesis extends this: computation is not tied to any particular physics.

**Charles Babbage.** Designed the Analytical Engine in the nineteenth century, a mechanical computer built from brass gears. An early demonstration that computation does not require electricity or electronics.

**Baruch Spinoza.** Proposed in the seventeenth century that mind and matter are two attributes of one underlying substance, neither reducible to the other. Arrived at this without the concept of computation, working from the single example of human experience.

**John Wheeler.** Physicist who coined the phrase "it from bit," proposing that every physical quantity derives its meaning from information. Argued that information is prior to matter.

**Konrad Zuse.** Proposed in 1969 that the universe is a cellular automaton (*Rechnender Raum*), arguably the first serious proposal that the universe is literally a computer.

**Ed Fredkin.** Developed "digital physics" from Zuse's foundations, arguing that information is more fundamental than matter and coining the term "digital philosophy."

**Nick Bostrom.** Formalized the simulation argument in 2003, establishing the epistemic problem: if simulations of conscious beings are possible and likely, then we are probably in one.

**David Deutsch.** Constructor theory reformulates physics in terms of which transformations are possible and impossible, an inherently informational framing. His universality principle provides the formal support for the nesting argument: any world that preserves universal computation can host arbitrary depth.

**Stephen Wolfram.** His Physics Project attempts to derive the laws of physics from simple computational rules, pursuing the question of what "program" generates our observed physics.

---

## Appendix B: Glossary

**Causal closure.** The principle that every physical event has a sufficient physical cause. No gaps in the physical causal chain require non-physical entities to fill them.

**Epistemic containment.** The structural property that an observer within a world cannot determine whether their world is the base layer of reality or a nested simulation. A consequence of being inside a causally closed system.

**Functional universality.** The property of a logic gate type that allows it to implement any logical operation. NAND and NOR gates are each functionally universal: any computation can be built from either one alone.

**Local physicalism.** The claim introduced in this essay that physicalism correctly describes the causal closure of a given substrate but does not establish that substrate as fundamental. Physicalism is local to the world it describes.

**Local reality.** The substrate and rules within which an observer exists. An observer's local reality is causally closed and appears fundamental from within, regardless of whether it is nested inside another reality.

**Managed disconnection.** The active severing of relations that are not relevant to a phenomenon, treating everything outside the explanatory boundary as fixed background. The mechanism by which higher-level explanatory layers make tractable explanation possible in a deeply connected world. Developed in [Why Explanation Comes in Layers](layers.md).

**Metaprogramming.** When a system operates on its own information. A second-order operation where a system's capacity to acquire, modify, or create turns inward and targets its own representations. The mechanism proposed in [MFP](framework-of-personhood.md) to explain personhood.

**NAND gate.** A logic gate that outputs false only when both inputs are true. Functionally universal: any logical operation can be constructed from NAND gates alone.

**Nested world.** A computational world running within another computational world. Each nested world can have its own rules, its own causal closure, and its own observers who experience their world as fundamental.

**Substrate.** The physical or computational medium in which information is instantiated and operations are performed. Information requires a substrate to be operational rather than merely abstract.

**Universal computation.** The property, established by Turing, that any sufficiently capable computational system can simulate any other computational system. This property is what permits nested worlds and is the invariant that persists across substrates.
