---
title: "A Return to Meaning"
status: "Working"
date: "Aug 2026"
order: 0
featured: true
blurb: "Modern science leans hard on formalism, and it works, but there is a reason why language evolved before mathematics. An argument for a return to meaning, emergence, and abduction."
---

# A Return to Meaning

## Introduction

The essay you are reading rests on a single idea: there is more to language than meets the eye. Language is how meaning is expressed and considered. It is where people and minds voice their most profound desires, beliefs, and curiosities. Meaning, in turn, comes from caring—nothing means anything to a mind that wants nothing.

You've probably noticed how much of modern science and technology feels like pure math. Our culture reduces problems to equations, optimizes them, and treats the formal structure as the only real thing. It works—spectacularly. But along the way meaning and philosophy were discarded as lesser-than approaches, and the people who employ them were relegated to the "humanities" in favor of the almighty STEM fields (science, technology, engineering, and math).

That exclusion had a side effect worth naming. Questions about meaning did not stop being asked. They stopped being askable inside serious institutions, so the people who cared most about them went looking elsewhere. Some retreated into metaphysics, the kind that never has to answer to anything. Some came to reject formal method outright, treating rigor as the enemy rather than the tool. Some simply opted out. They get called crazy, and often enough the label fits. The sadness is that they are right in all the wrong ways: they correctly noticed that something real was missing, then concluded that the answer was to abandon the thing that works.

The demotion of meaning is a crime against the foundations of science and curiosity. Both run on guesses about what matters and where to look, and those are exactly what got demoted. But it was not a conspiracy. It was a pragmatic decision made by a society with limited resources, and given the constraint it was close to right. Formalism paid—faster, more measurably, and in ways that compounded. Philosophy did not.

Demoting meaning did not make it less necessary. The questions of meaning still get answered every day—only now they are settled by whoever has the power and status to assert an answer, rather than by anyone obliged to defend one. That is the sadness in it: rigor kept its hold on everything except the questions that decide how people live, what matters, and what is worthy.

The choices about what to formalize in the first place, the goals that make one model better than another, the values that decide what even counts as a solution—these never disappeared. They were just pushed outside the formalism and into informal judgment. As the systems we build grow larger and start interacting with each other, that outer layer is becoming hard to ignore again.

And those systems are becoming minds. It is tempting to assume that machines good enough at calculation will bypass all of this, but anything that acts has to act on something it values. Every mind we build inherits the problem of meaning rather than dissolving it, and there are about to be a great many of them.

The constraint that justified the trade is lifting. What follows is a short argument that we now get to return to meaning, emergence, and abduction—the very things the age of formalism set aside—and that the systems we are building will not leave us the choice.

## Table of Contents

1. [What Reason Is](#1-what-reason-is)
2. [Reasoning in Computers](#2-reasoning-in-computers)
3. [What Compression Is, and Why Minds Do It](#3-what-compression-is-and-why-minds-do-it)
4. [Why Language Came First](#4-why-language-came-first)
5. [Where Meaning Comes From](#5-where-meaning-comes-from)
6. [Why Formalism Is Useful](#6-why-formalism-is-useful)
7. [The Cost of Formalism](#7-the-cost-of-formalism)
8. [Why Meaning Comes Back](#8-why-meaning-comes-back)
9. [Conclusion](#conclusion)
- [Appendix A: The Argument in Thirteen Steps](#appendix-a-the-argument-in-thirteen-steps)
- [Appendix B: Key Terms](#appendix-b-key-terms)

## 1. What Reason Is

Start with something you do constantly and almost never examine.

Suppose you are deciding what to eat for dinner. There is leftover pasta in the fridge. There is also a restaurant you have been meaning to try. You have had a long day, and you are trying to spend less this month. You eat the pasta.

Look at what just happened, because there were two different things going on.

There were *moves*: if this, then that. These two cannot both be true. That conclusion follows from those reasons. And there was the *material* the moves ran on: that money saved this month matters to you, that being tired counts for something, that you would rather be someone who sticks to a plan than someone who does not.

The material comes in two kinds. **Values** are what you care about—what you are trying to get, avoid, protect, or become. **Norms** are what you will accept as a reason: what counts as evidence, what counts as a good argument, what would be enough to change your mind.

**Rationality** is logic operating over norms and values. Not logic by itself. Logic by itself decides nothing, because it has nothing to decide about. Hand it no values and it will sit there, perfectly valid and perfectly idle.

This holds for any **agent**—anything with goals that acts to pursue them—whether that is a person, an animal, a firm, or a machine.

The process of reasoning looks similar regardless of the topic, and much the same from one person to the next. The content it operates on changes with both.

Now the part that changed history. The process of reasoning—the part that is the same for everyone—turned out to be buildable in a machine. We know it as a computer.

## 2. Reasoning in Computers

A **logic gate** is a switch that takes inputs and produces an output by a fixed rule. The most famous is the NAND gate, which outputs "off" only when both of its inputs are "on," and "on" in every other case. That sounds too simple to matter. It isn't. Wire enough NAND gates together and you can build any computation that can be built at all. Every processor you have ever used is, underneath the packaging, an enormous pile of them.

So the moves have a physical implementation, which brings us to the premise this essay stands on: a mind is at least a computational system, signals running over elementary gates. Not *only* that—the claim is not that a mind is nothing more than machinery. The claim is that a mind is not *less* than that. Whatever else is going on, signals are moving and being transformed along the way.

That human minds are computers is called computationalism, or computational theory of mind (CTM). To learn more about CTM:
- Alan Turing's 1936/1950 work (computability and ["Computing Machinery and Intelligence"](https://courses.cs.umbc.edu/471/papers/turing.pdf)) supplies the formal backbone.
- Allen Newell & Herbert Simon's Physical Symbol System Hypothesis (especially ["Computer Science as Empirical Inquiry: Symbols and Search"](https://iiif.library.cmu.edu/file/Newell_box00038_fld02800_doc0003/Newell_box00038_fld02800_doc0003.pdf), 1976, and Newell's 1980 "Physical Symbol Systems"). A physical symbol system (manipulating symbols that designate and can be interpreted) is necessary and sufficient for general intelligent action. This directly equates human intelligence with the capacities of a suitably organized computer.
- McCulloch and Pitts's 1943 paper [*A Logical Calculus of the Ideas Immanent in Nervous Activity*](https://doi.org/10.1007/BF02478259), which showed that networks of firing neurons can be described as logical operations.
- The Stanford Encyclopedia of Philosophy's survey of [the computational theory of mind](https://plato.stanford.edu/entries/computational-mind/), which lays out the arguments and the objections to them.

While computers can reason, notice that they did not encode norms and values, because there is nothing in a single switch that cares about anything. Values do not come from the logic machinery, but machinery whose operations turn inward and model themselves can generate values rather than receive them, which is the argument of [Metaprogramming Framework To Classify Personhood](framework-of-personhood.md).

## 3. What Compression Is, and Why Minds Do It

Grant the premise and a second problem follows immediately. Any system like this has finite capacity, and the information reaching it does not. Something has to give.

What gives is detail. A mind handles more than it can hold by replacing large amounts of detail with smaller labels that carry the part it needs. That substitution is **compression**.

Consider the phrase "traffic jam." A complete account of the thing would list several thousand vehicles, their positions, their velocities, the reaction time of every driver, and the state of every brake light. Nobody has ever wanted that account. Two words do the job, because two words carry the part you need: you are going to be late, and there is nothing to be done about it.

Stack these labels—words built on words, models built on models—and you get what we call **layers of abstraction**. Your doctor says you have the flu. A virologist describes a virus hijacking your cells to make copies of itself. A biochemist describes proteins changing shape. Each description is true. Each throws away what the others keep. None of them is the "real" one. Which layer is right depends on who is asking and what they intend to do about it.

## 4. Why Language Came First

Language is the compression of experience, values, and intent—the compression of minds.

The layers in the last section compress the world: what is out there, and how it behaves. Language compresses what is doing the looking. Look again at "traffic jam." Those two words did not merely stand in for several thousand vehicles. They told you that you would be late and that nothing could be done about it. That is not a description of the road. It is the road as it stands to someone trying to get somewhere.

This runs in both directions. Language carries meaning between minds, which is why one mind can skip work another already did instead of rediscovering it from scratch. It also runs inside a single mind: people think in words, argue with themselves, and talk their way to conclusions no one else will ever hear. A reasoning model does the same thing in its trace. Compression of meaning is the job. Transmission is one use of it.

Language evolved long before mathematics. The ordering is not an accident. Understanding what matters, expressing it, and coordinating with others on it are older needs than calculation, and language compresses them better than calculation ever could. It is the higher layer, not the cruder one.

Watch how much a physical description leaves out. A physicist can give you a complete account of a pot of water on a stove: convection currents, vapor pressure, the phase change at 100°C. It is a superb compression of the physical process. It is also completely silent on the only fact that explains why the pot is on the stove at all. Somebody wanted soup.

That fact is not in the water. It is not recoverable from the water, no matter how precisely the water is measured. It exists in an agent: the experience of being hungry at eleven at night, the value of not wanting to spend an hour on dinner, the intent to eat something now. And a single sentence transmits all three.

This is what four words like "I'm saving for a house" accomplish. They carry years of intention, a rough sense of the sacrifices involved, and a whole set of decisions the speaker will predictably make. No account of the speaker's bank balance conveys any of it.

## 5. Where Meaning Comes From

If compression throws away most of the original detail, what makes the surviving part worth keeping?

The answer is that what survives is useful only if it helps an agent decide what to do. In decision theory, the field that studies choice under uncertainty, that usefulness has a name: **utility**. Utility is not a property sitting inside the information. It is a relationship between the information and what the agent is trying to accomplish.

Take a weather report: a 30% chance of rain tomorrow. The number is identical for everyone who reads it. A farmer three years into a drought reads it as bad news—only a 30% chance. A couple planning an outdoor wedding reads the same number as a reason to spend money on a tent. One number, opposite meanings, because the two parties want opposite things.

What the agent is trying to accomplish rests, in turn, on its values—the material from the opening section, the part the logic could not supply. And this is where meaning comes from. Meaning is not some mystical substance added to information from outside. It is what information becomes when it meets an agent that has values.

## 6. Why Formalism Is Useful

**Reduction** is explaining something by breaking it into smaller parts and the rules those parts follow. Mathematics is reduction at its purest, and physics is where it was first turned loose on the world.

Its power comes from a bargain. Hold a frame fixed, and the regularities inside that frame can be compressed with extraordinary force. Newton's law of gravitation is the standard example, and it shows what the bargain costs. To get the law, you have to agree to ignore nearly everything about a planet: its color, its composition, its name, the mythology attached to it, the fact that anyone is looking at it. Keep only mass, position, and motion. That set of choices *is* the **frame**—what is being measured, why it matters, and what counts as the thing being counted. Fix it, and one short equation predicts the motion of every body in the solar system.

That is an astonishing return, and at the time the bargain looked free. Physics made the trade first and made it best, and its success became the template every other field reached for. Economics turned into equations. Psychology turned into statistics. Biology turned into molecular models. In each case the formal structure came to be treated as the main object of attention, while the informal work of deciding what to formalize slipped quietly into the background.

## 7. The Cost of Formalism

The work slipped into the background. It did not go away.

That work is creative, messy guesswork. "Deciding what to formalize" makes it sound orderly, like a procedure that could be written down and handed to someone. It is nothing of the kind. It is proposing something that might carve the problem correctly, discovering that it does not, and proposing again.

Somebody had to guess that mass and distance were the properties worth keeping and that color was not. Somebody had to guess what would count as a solution, and what would count as an object of the theory in the first place. Those guesses cannot be derived from inside the system they produce. They come before it. The formal system inherits them and then, having been polished, shows no sign that it ever needed them.

Imre Lakatos made this visible in *[Proofs and Refutations](https://en.wikipedia.org/wiki/Proofs_and_Refutations)*. He took a single theorem—Euler's formula, which says that for a polyhedron the number of vertices minus the number of edges plus the number of faces equals two—and reconstructed its history as a classroom argument. The formula looks eternal and inevitable. Its history was neither. A student produces a shape that breaks it. Rather than abandon a theorem everyone believes, the class rewrites the definition of "polyhedron" to exclude the offending shape. Then somebody finds another one. The definition shifts again. The theorem survives, not because it was proven once and for all, but because the meaning of its terms kept being renegotiated to keep it alive.

What appears in the textbook as a clean deduction was, in the making, a long argument about what the words should mean. The polished final form hides the meaning-making that produced it.

That hiding is the price of the bargain. The formalism is not wrong. What went missing is our awareness that guesswork is holding it up.

And notice what the classroom was actually doing. It was not fumbling toward a formalism it had not yet reached. It was reasoning in language, because language is the only mechanism that can represent meaning—the compression of minds. And meaning was the entire subject of the argument: what "polyhedron" ought to pick out, and why that mattered. A formal system has no way to represent that. It can only run on a shape already chosen. That activity has a name: philosophy. It is not immature mathematics. It is the layer mathematics stands on.

## 8. Why Meaning Comes Back

Meaning is coming back, and the first reason is a happy one. Technology is freeing us from the scarcity of reasoning. As cheap reasoning is turned on the problems that used to consume everything—food, energy, disease, the cost of making anything at all—we are freed to take up the questions that were always waiting behind them: what a good life is, what well-being consists of, what all the effort was for.

The second reason does not wait for us to be ready. The systems we build are becoming more numerous, more independent, and more tightly linked to one another. When that happens, the question of what matters starts to count for more than the quality of the calculation performed inside any one of them.

Consider a patient moving between a hospital measured on bed turnover, an insurer measured on claims cost, and a specialty clinic measured on appointments per day. Each organization is doing competent work inside its own frame. Each metric is honest. Each is being optimized by people who are not cutting corners. And the patient falls through every gap between them. Audit any one of the three and you will find no error, because there is no error to find. The failure does not live inside any of the systems. It lives in the space between their frames.

A pattern that exists only in the interaction between parts, and in no part by itself, is called **emergent**. The traffic jam from earlier is the plain version of this. No car contains the jam. You can disassemble a single car down to the bolts and never find it, because the jam is not a fact about any car—it is a fact about the relationship between them.

This is why the frame question cannot be answered by computing harder. More precision inside a frame will never tell you the frame is wrong. That information is not in there.

So how does anyone get a new frame? Not by deduction, which works out what necessarily follows from what you already hold. Not by induction, which expects an observed pattern to continue. The move is a third one: proposing the best available explanation before the evidence is in. That is **abduction**.

It is not exotic. A doctor facing a cluster of symptoms that match no familiar disease does it. So does a mechanic listening to an unfamiliar noise, and a detective standing in a room with too few facts. None of them is deriving a conclusion. Each is proposing a frame and then testing it against what happens next. Nearly everyone does this constantly, and almost nobody has a name for it—which is itself a symptom of how thoroughly this layer was pushed out of view.

It is tempting to think the problem is about to be automated away. Machines are getting very good at the formal half, and a system good enough at calculation might seem likely to pick its own frames as a side effect.

It does not work that way. Logic decides nothing on its own; anything that acts must act on values, and values are what say which frame is worth having. A machine capable of choosing frames is a machine with something to care about. That does not retire the problem of meaning. It gives the problem another mind to live in—and then another, and then many, each with its own reading of what matters. Meaning is not a human complication that better engineering will remove. It is what any mind runs into the moment it has to decide something.

## Conclusion

The turn to formalism was not a mistake. It was the right move for its constraints: resources were scarce, the problems that pressed hardest were material ones, and holding a frame still and compressing inside it paid better than anything else on offer. Nothing here asks anyone to give it up.

The claim is narrower. The move was never self-sufficient. It always depended on prior work—deciding what to formalize, deciding which goals count, deciding what would even qualify as a solution—and that work was never solved. It was relocated, into informal judgment, where it sat unexamined because it was reliably cheap.

It is no longer cheap. When systems are numerous, autonomous, and coupled to each other, the frame question stops being settled background and becomes the live problem. It is not a problem any single system can answer by computing harder inside itself, because the answer is not located inside it.

And we will not be facing it alone. The minds we are building differ from us in substrate, and so will many of their particular problems—but substrate is the shallow difference. Every mind is finite: it meets more than it can hold and has to decide what is worth keeping, and that deciding is meaning-making. Greater capability moves the threshold; it does not remove the condition. Meaning does not retire. It acquires more minds that need it.

Answering it means working where meaning, emergence, and abduction live, and that work has an instrument. Language is how a mind holds meaning, intent, and the terms it cooperates on—the only form in which any of that can be represented at all. Philosophy is that instrument handled with care. It was never the immature phase of mathematics, and the need for it does not belong to us in particular: every mind with values needs some way to reason about them and convey them. It is the layer the rest of it stands on, returned to after formalism rather than before it.

## Appendix A: The Argument in Thirteen Steps

The essay above is the expanded version. This is the spine, stated compactly so that any individual step can be attacked on its own.

1. Minds are at least computational systems: signals operating over elementary gates (NAND and equivalents).

2. As information grows, new labels are needed to convey more with less; this is called compression. We typically describe these compressions as layers of abstraction. This works for varying observer perspectives (scientists and laymen).

3. An extra layer of compression appeared between minds: natural language.

4. Language does not only compress the world. It compresses the ongoing work of other minds—their goals, hoped-for futures, limits, and ways of seeing—so one mind does not have to rediscover all of that from scratch. A physics description of boiling water is an excellent compression of the physical process, yet it says almost nothing about the fact that a mind wanted soup.

5. What remains after most of the original detail has been set aside is useful only if it helps an agent decide and act. In decision theory that usefulness is called utility, and utility for an agent is grounded in its values. Values are the source of meaning.

6. Numbers and formal symbols travel well once everyone already shares the same frame of reference. They travel poorly when the frame itself still needs to be shared or updated between minds that do not fully match (applied use of the soup example in #4).

7. Mathematics is reduction at its strongest: once a frame is held fixed, the regularities inside it can be compressed with great power.

8. Science has leaned hard into this inward, formalizing move, treating the resulting mathematical structures as the main object of attention.

9. The price of that success is that the work of inventing and choosing the frames was pushed outside the formal systems and left to informal judgment. As Imre Lakatos showed in *Proofs and Refutations*, the living messiness of meaning-making was lost in the polished final form.

10. As the systems we build grow larger, more independent, and more tightly linked, the open question of which frames to use begins to matter more than the calculations performed inside any single frame.

11. That higher layer is emergent: it cannot be found by looking harder inside any one system; it appears only in the interaction between them.

12. The process of proposing candidate frames when the evidence is incomplete is abduction.

13. Therefore the next useful compressions will require an explicit return to meaning, emergence, and abduction after this long period of intense reductionism.

## Appendix B: Key Terms

**Rationality.** Logic operating over norms and values. Logic alone decides nothing, because it has nothing to decide about.

**Norms.** What an agent will accept as a reason: what counts as evidence, what counts as a good argument, what would be enough to change its mind.

**Logic gate.** A switch that takes inputs and produces an output by a fixed rule. The NAND gate outputs "off" only when both inputs are "on." Enough of them, wired together, can perform any computation that can be performed at all.

**Compression.** Replacing a large amount of detail with a smaller label that carries the part you need. "Traffic jam" in place of the positions and velocities of several thousand vehicles.

**Layers of abstraction.** A stack of compressions describing the same thing, each discarding what the others keep. Which layer is correct depends on who is asking and what they intend to do.

**Language.** The compression of experience, values, and intent—the compression of minds, as distinct from the layers of abstraction that compress the world.

**Agent.** Anything that has goals and acts to pursue them. A person, an animal, a firm, or a machine.

**Utility.** In decision theory, the worth of an outcome to a particular agent given what that agent is trying to accomplish. A relationship between information and goals, not a property of the information.

**Values.** What an agent cares about—what it is trying to get, avoid, protect, or become. Grounds its utility, and the source of meaning.

**Meaning.** What information becomes when it meets an agent with values. Identical data carries different meaning to agents that want different things.

**Frame.** The set of choices about what to keep and what to discard: what is being measured, why it matters, and what counts as the thing being counted. Formalism compresses inside a frame. It cannot choose one.

**Reduction.** Explaining something by breaking it into smaller parts and the rules those parts follow. Mathematics is its purest form; physics is where it was first turned on the world; the other sciences followed.

**The work.** The creative, messy guesswork of inventing and choosing frames: proposing something that might carve a problem correctly, finding that it does not, and proposing again. Cannot be derived from inside the formal system it produces, because it comes before it.

**Emergent.** A pattern that exists only in the interaction between parts and in no part by itself. A traffic jam is emergent; no single car contains it.

**Abduction.** Proposing the best available explanation before the evidence is in. Distinct from deduction (what necessarily follows from what is already held) and induction (expecting an observed pattern to continue).
