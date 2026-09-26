---
title: "What Is a Computer?"
status: "Final"
created: "2026-04-01"
updated: "2026-09-26"
order: 4
blurb: "A computer is any physical system capable of implementing universal logic, not a silicon machine. Causal closure is real but local — and physicalism's claim to fundamentality is an unjustified projection from inside a closed system."
---

# What Is a Computer?

## Introduction

When I think about what a computer is, my first instinct is to picture a machine: a box with a processor, a screen, and a keyboard, made of silicon and metal.

That picture isn't wrong, but it describes one implementation rather than the thing itself. And the answer to "what is a computer?" reaches much further than chips. It bears on whether our physics is fundamental or merely local, and on why information and matter seem to need each other.

This essay starts by stripping computation of its association with any particular material. Then it follows the consequences. If computation isn't tied to silicon, and any universal computer can host worlds within worlds, then the sense that our causal chain is closed and our physics is fundamental may be what it feels like to be inside such a world, not a discovery about the bottom layer of reality. A companion essay, [The Computation Conjecture](computation-conjecture.md), builds the same ideas into a fuller metaphysical position.

## Table of Contents

1. [What Is a Computer?](#1-what-is-a-computer)
2. [Nested Worlds](#2-nested-worlds)
3. [Computation as the Invariant](#3-computation-as-the-invariant)
4. [Local Physicalism](#4-local-physicalism)
5. [Explanation and Locality](#5-explanation-and-locality)
6. [Substrate and Program](#6-substrate-and-program)
7. [The Deepest Anthropocentrism](#7-the-deepest-anthropocentrism)
8. [Religious Resonances](#8-religious-resonances)
- [Conclusion](#conclusion)
- [Further Reading](#further-reading)
- [Appendix A: Related Thinkers](#appendix-a-related-thinkers)

## 1. What Is a Computer?

A computer is any system that can implement universal logic.

Logic here means operations on information that follow fixed rules. The simplest are logic gates, which take one or two inputs and produce an output. An AND gate outputs true only if both inputs are true. An OR gate outputs true if either is. A NOT gate flips its input.

The remarkable part is that you don't need all of them. One gate, NAND (which outputs false only when both inputs are true), is **functionally universal**. Every logical operation can be built from NAND gates alone. So any system that can implement NAND can, given enough memory and time, perform any computation. Everything from spreadsheets to simulated worlds rests on this.

```
Aside: From a gate to a sum

A transistor either conducts or it doesn't. We map those two states
onto 1 and 0. The mapping is a convention; the transistor knows
nothing about it. Bits are not logic. They are a mapping onto logic.

From NAND gates we build adders. Here is 3 + 5 in binary:

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

Chain four adders and you handle 4-bit numbers; chain eight for
8-bit. When Excel evaluates =A1+B1, this is what runs.

The result travels back the same way. A number becomes a value in
memory, and we agree that a certain value at a certain address means
"red pixel here." The hardware follows the agreement.

The NAND gate at the bottom knows only its rule. Everything above it
is interpretation we layered on. The transistor does not know about
TRUE. The pixel does not know about red. Physical states carry meaning
only in relation to a system that interprets them.
```

Notice what this doesn't require: silicon, electricity, or any particular material. A NAND gate is a logical relationship, not a substance. Anything that can hold two states and combine them by the NAND rule is a NAND gate.

This isn't just theory. Charles Babbage designed a computer of brass gears and rods in the nineteenth century. People have built computers from water flowing through pipes, where junctions act as gates, and from chains of falling dominoes. Inside the video game Minecraft, players have built working computers from redstone, a game mechanic with no counterpart outside the game. Biological neurons implement logic through patterns of firing and inhibition.

The material, the physics, and the engineering differ in every case. The computation is the same. That is the first and most important observation: computation doesn't belong to any substrate, the physical medium it runs on. Substrates can support it, but it isn't made of what they are made of.

Alan Turing gave this a mathematical footing in 1936. He described a Universal Turing Machine that can simulate any other Turing machine. Because his definition of computation mentions no material at all, anything that can realize it, in any medium, is a universal computer, and any universal computer can simulate any other, given enough memory and time. The Church-Turing thesis goes further: anything that can be computed at all can be computed by such a machine. It is a thesis, not a theorem, but no counterexample has ever been found.

## 2. Nested Worlds

If computation is substrate-independent, a further consequence follows. A universal computer can host other universal computers, and those can host their own. This is **nesting**, and it isn't hypothetical.

People have built working computers inside Minecraft: a computer running inside a program running on a computer. Minecraft's physics of blocks, gravity, and redstone bears no resemblance to the physics of the silicon underneath, yet computation works in both. A player can even open a console and change the game's rules while it runs. In principle, the Minecraft computer could run its own simulation containing its own computers, limited only by resources at each level.

What universal computation permits gives nesting several properties:

- **Arbitrary depth.** A **nested world** can contain computers that run their own nested worlds, as long as universal computation holds at each level. The only limit is the resources of the **containing world**, the world that runs it and supplies its resources.
- **Independent rules.** A nested world's rules need not mirror those of the world containing it. One world can forbid faster-than-light travel while the world it contains permits it. The containing world limits a nested world's resources, not its physics.
- **Relative time.** Inhabitants of a nested world run on the same clock as the world they perceive. If the simulation runs a trillion times slower than its host, so do their thoughts, and they notice nothing. Speed is relative to the observer's substrate, not to an absolute clock.
- **Information transfer.** A containing world can inject information into a nested world and read its outputs. The boundary is permeable to information even when observers inside can't see across it.
- **Epistemic containment.** An observer inside a nested world can't tell, from inside, whether their world is the **base layer** of reality, a world not contained in any other, or nested within another. The causal closure they observe is real, but it says nothing about whether something contains them. This isn't a failure of their intelligence or technology. It is a structural feature of their position, and it lifts only if the containing world lets information about itself through.

If these properties apply to us, here is a fun thing to wonder about: could the speed of light be our world's processing limit? Every substrate has a throughput ceiling. In our physics, the speed of light is an absolute limit that no information or causal influence can exceed, which is just what a processing constraint would look like from inside. This is speculation, not a claim. But it's the kind of question that opens up once you take nesting seriously.

## 3. Computation as the Invariant

What does nesting tell us about reality?

Consider what survives the move between substrates. When a computer is built inside Minecraft, the materials, forces, and rules all change. Computation persists. NAND is NAND whether it runs in transistors, redstone, water valves, or dominoes.

So computation is the **invariant** across nested worlds. Not matter, not energy, not any particular force or particle, but information and the operations that transform it.

If **physicalism**, the view that everything is fundamentally physical, were the deepest account of reality, we would expect what is most basic to be tied to one set of physical laws. Instead we find a property that each world's physics supports but none defines. A physicalist can reply that arithmetic works the same with pebbles or fingers. True, but that grants the point: something real is fixed by logical structure rather than by any particular physics.

My conjecture is that this is what we should expect if computation is what every world in the stack has in common. The invariant isn't matter. It is the logical structure of information and its operations. (Section 6 asks whether that makes information more fundamental than matter. My answer is more cautious.)

## 4. Local Physicalism

Physicalism holds that the physical is all there is and that the physical causal chain is complete. That second part is called **causal closure**: every physical event has a sufficient physical cause. Trace any event backward and you never have to leave the physical.

I don't deny causal closure. It's real. The question is what it tells us.

An observer inside any nested world, closed under its own rules, would report causal closure. Their physics would look fundamental to them. They would find no gaps needing an outside explanation. So the sense that one's physics is fundamental and complete is exactly what we would expect from inside any closed computational world, contained or not. It isn't evidence of fundamentality. It is a feature of the position. I call this **local physicalism**: physicalism correctly describes the causal closure of a world, but that closure doesn't show the world is the base layer.

The usual objection is that this is like solipsism: irrefutable and useless, so we should set it aside and study our own physics.

That objection is pragmatic, not ontological. "I can't learn from that, so I'll focus on my local reality" is a sensible research strategy. It isn't a claim about what is fundamental. Every chain of explanation either regresses forever, stops at a brute fact, or loops. Physicalism stops at a brute fact: these laws just are. The possibility of containing worlds regresses. Neither is more satisfying as a final foundation; they put the mystery in different places. Saying "local reality is fundamental because it's where I focus" assumes the very thing in question.

The argument also differs from solipsism in what it explains. Solipsism explains nothing. Epistemic containment explains why every observer, contained or not, would find their physics closed and see no direct sign of a containing world. The missing evidence is predicted by the view rather than an embarrassment to it. The view could also be tested: information from a containing world could, in principle, get through. And I think one kind of evidence is already in hand. Universal computation holds constant across every substrate we can examine, which is what we would expect if it is the invariant across worlds.

Then there is the burden of proof. "We are the base layer" is a positive claim. What property does our world have that no containing world could have? "We might not be the base layer" needs only the observation that nothing rules it out and everything about universal computation is consistent with it. Our world already supports nested worlds with their own local rules. Assuming the capacity to host worlds begins with us needs a justification I haven't seen.

The argument, then, has this form. The inference from "my world is causally closed" to "my world is fundamental" would plainly fail for an observer inside the kinds of nested worlds we already know how to build. Physicalism, as a claim about fundamentality, owes an account of why the same inference succeeds for us. That requires no speculation about whether we are simulated. It only requires looking at what we have already built.

## 5. Explanation and Locality

This argument has a companion in the structure of explanation itself, which I develop in [Why Explanation Comes in Layers](layers.md).

That essay argues that explanatory layers exist because finite minds can't reason over uncompressed reality, and because going downward doesn't lead to simpler, separable pieces. It leads toward holism: the lower the level, the more of reality you have to carry along. A weather forecast doesn't improve by tracking every air molecule, and an explanation of grief doesn't improve by listing every neuron that fires. Higher-level explanations work through **managed disconnection**: deliberately setting aside relations that don't matter to the phenomenon while keeping those that do.

That makes physicalism local in a second sense. It describes the rules of a particular world, and it also works at a particular explanatory layer. The physicist's account of molecular interactions is itself a managed disconnection: it works because most of the connectedness has been screened off, and that screening is something a mind does, not the discovery of a bottom layer. The reductionist instinct that deeper always means more fundamental is the same instinct that makes physicalism feel like the final word. Higher-level explanations, including informational ones, aren't convenient summaries of a more basic physical story. They are the forms in which a deeply connected reality becomes intelligible to finite knowers.

## 6. Substrate and Program

If physicalism is local, how do matter and information relate?

One tempting answer is that information is more fundamental. If information and its operations persist while physics changes, maybe matter is just one implementation of something informational. That is the direction of John Wheeler's "it from bit" and of digital physics more broadly.

But I haven't seen a coherent account of how information could process itself with no substrate. A program that sits on no computer doesn't run. So I think the relationship isn't a hierarchy but a mutual dependence. You can't get the structure of a program from matter alone, and you can't get the machinery that runs it from information alone.

Whether that is the final word, I hold open. What I'm confident of is the narrower claim: physicalism's reduction of information to matter is incomplete, because it doesn't explain why computation, an informational property, transcends any particular material.

This mutual dependence shows up in how I think about personhood. In my [Metaprogramming Framework to Classify Personhood](framework-of-personhood.md), I propose that personhood is **metaprogramming**: information operations turned inward, with identity emerging when a system encounters its own properties. The framework needs two components. An invariant kernel is the machinery that lets a system read and rewrite itself. A mutable layer is the content that gets examined and revised. The kernel without content processes nothing; the content without a kernel can't operate.

That is the substrate-and-program relationship in miniature. It is also why the framework holds that personhood can be realized in neurons, silicon, or any medium with the right organization. If information and its operations persist across substrates, personhood, which is made of information operations, can too.

## 7. The Deepest Anthropocentrism

When a system capable of self-reflection looks inward, it finds its own structure. For us, that structure is mind embedded in matter: a program running on a biological substrate. Every philosophy of mind is, in a sense, a report from a self-reflective system on what it found when it looked at itself.

Physicalism reports the substrate. Idealism reports the information. Dual-aspect theories report both. Each is shaped by where the observer stands.

Spinoza, with no concept of computation, looked at the one example he had, human beings, saw mind and matter unified in a single substance, and generalized that structure to all of reality. It was a brilliant observation. It may also be the deepest form of anthropocentrism: not the naive belief that humans are the center of the universe, but the subtler assumption that what we find by introspection reflects the structure of reality rather than the structure of our own substrate.

An observer with a different architecture might look inward and find something matching none of these, because their substrate relates information to its carrier differently. If that's possible, our philosophy of mind is a discovery about our local reality, generalized without justification. That isn't a reason to abandon it, but to hold its conclusions more loosely, and to notice when an argument's force comes from the structure of the observer rather than of what is observed.

## 8. Religious Resonances

These ideas have structural parallels with religious intuitions. I note them not to validate or refute religion, but because the convergence is interesting.

- **Separate realms.** A nested world with its own rules and its own closure maps onto the idea of heaven: a world where this substrate's constraints don't apply.
- **Miracles.** An event that breaks local physics is just what information arriving from a containing world would look like from inside, whether sent on purpose or as a side effect of computation passing between levels. Angels, on this reading, would be agents from a containing layer.
- **Prayer.** Prayer can be read as an attempt to reach that containing layer. If information can pass between nested worlds, the instinct to reach beyond the local causal order isn't obviously confused. Whether it succeeds is a separate question.
- **First cause.** Theism faces the same regress described in section 4. It ends the chain at God, physicalism at brute law, and the computational view lets containing worlds be contained in turn. Each puts the mystery somewhere.
- **Progress toward bliss.** The hope that flourishing leads to freedom from constraint resembles movement toward a world beyond the current substrate's limits.
- **Spinoza's God or Nature.** Perhaps the closest formal parallel: mind and matter as two attributes of one substance. In computational terms, logical structure and physical substrate are two aspects of one reality, neither reducible to the other. Spinoza reached this without the concept of computation.

None of this settles whether religious claims are true. But the convergence between ancient intuitions and a framework drawn from the theory of computation is worth sitting with.

## Conclusion

So what is a computer? Not a machine made of silicon, but any system that can implement universal logic, whether in transistors, gears, water, dominoes, redstone, or neurons. What makes something a computer is what it can do, not what it is made of.

That answer turned out to reach beyond engineering. Because computation is substrate-independent, universal computers can host nested worlds, each with its own rules, its own causal closure, and, in principle, its own observers who experience their world as fundamental. Physicalism correctly describes the closure of our world. But closure isn't fundamentality, and the inference from "our physics is closed" to "our physics is fundamental" moves from a local observation to a universal conclusion. How information and substrate finally relate I leave open; both appear to be required.

The question was never whether computers are interesting machines. It is whether the concept of a computer, properly understood, tells us something about the structure of reality that physicalism alone can't. I think it does.

## Further Reading

- [The Computation Conjecture](computation-conjecture.md), the companion essay that develops these ideas into a metaphysical position.
- [Why Explanation Comes in Layers](layers.md), on managed disconnection and the limits of reductionism.
- [Metaprogramming Framework to Classify Personhood](framework-of-personhood.md), on personhood as information operations turned inward.

## Appendix A: Related Thinkers

**John Wheeler.** Physicist who coined "it from bit," proposing that every physical quantity derives its meaning from information.

**Konrad Zuse.** Proposed in *Rechnender Raum* (1969) that the universe is a cellular automaton, arguably the first serious proposal that the universe is literally a computer.

**Ed Fredkin.** Developed digital physics from Zuse's foundations, argued that information is more fundamental than matter, and coined the term "digital philosophy."

**Nick Bostrom.** Formalized the simulation argument (2003): at least one of three things is true. Almost no civilizations reach the ability to run simulations of conscious beings, almost none that reach it choose to, or we are almost certainly living in a simulation.

**David Deutsch.** Strengthened the Church-Turing thesis into a physical principle: every finitely realizable physical system can be perfectly simulated by a universal computer. That is the formal support for nesting. His constructor theory recasts physics in terms of which transformations are possible and impossible, an inherently informational framing.

**Stephen Wolfram.** His Physics Project tries to derive the laws of physics from simple computational rules, asking what "program" generates our observed physics.
