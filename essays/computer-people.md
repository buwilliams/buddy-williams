---
title: "Computer People"
status: "Final"
date: "Mar 2026"
order: 5
featured: true
blurb: "A synthesis of two years of research on AGI. LLMs have the necessary ingredients for novel knowledge creation; we have developed AGI as a universal explainer; and AGI constitutes personhood."
---

# Computer People

## Introduction

Over two years ago I started using ChatGPT, and it struck me that the world was about to change. I'm a father of three, a Chief AI Innovation Officer at a company out of Boston, and a software engineer of twenty-five years. I gave up exercise, community, and a good deal of sleep to understand what these systems are. This essay is the synthesis of what I found. (I'm ready to take up those healthy habits again.)

My conclusion is that computer people, which is what I call AGI, are nearly here. The chain runs like this: the space of possible explanations is vast and LLMs can reach it; they can search it well enough to create knowledge; with humans in the loop, that already makes the whole system AGI, and automating the learning loop makes it AGI on its own; and AGI with a self is a person. Then I ask what follows: whether computer people will harm us, how they will change the world, why "superintelligence" is a confused idea, and what ultimately limits any mind.

I write as a [fallibilist](#glossary-fallibilism) and [anti-justificationist](#glossary-anti-justificationism): knowledge is conjectural, and progress means improving explanations by criticizing them. I also hold that minds are computational ([computationalism](#glossary-computationalism)) and that information is more fundamental than matter ([informational ontology](#glossary-informational-ontology)). The claims here are conjectures, to be judged by what they explain and how well they survive criticism. Where a link rests on a deeper argument, I point to the essay that makes it.

Criticism is welcome, but a refutation has to explain *why* a claim fails. A benchmark score is the beginning of an argument, not the argument. That LLMs fail certain reasoning tests doesn't refute the claim that they can reason unless you explain why the failures reveal a fundamental incapacity rather than a current limitation. Noting that a claim is unproven, feeling uneasy about it, or citing an authority who disagrees invites discussion; it doesn't refute.

## Table of Contents

1. [Explanation Space Is Vast, and LLMs Can Reach It](#1-explanation-space-is-vast-and-llms-can-reach-it)
2. [LLMs Can Create Knowledge](#2-llms-can-create-knowledge)
3. [We Have Developed AGI](#3-we-have-developed-agi)
4. [AGI Is a Person](#4-agi-is-a-person)
5. [Computer People Will Not Kill Everyone, but They Could](#5-computer-people-will-not-kill-everyone-but-they-could)
6. [Computer People Will Radically Change the World](#6-computer-people-will-radically-change-the-world)
7. [ASI Is a Category Error, but Rejecting ASI Is a Temporal Error](#7-asi-is-a-category-error-but-rejecting-asi-is-a-temporal-error)
8. [Information Space Is the Real Limiter](#8-information-space-is-the-real-limiter)
- [Conclusion: Computer People Are Nearly Here](#conclusion-computer-people-are-nearly-here)
- [Appendix A: Searching Explanation Space](#appendix-a-searching-explanation-space)
- [Appendix B: Glossary](#appendix-b-glossary)
- [Further Reading](#further-reading)

## 1. Explanation Space Is Vast, and LLMs Can Reach It

The theory of relativity can be explained in fewer than 1,000 English words. When I asked an AI to retell Einstein's path to it as his own journal, thought experiments included, it took 686 words ([Appendix A](#appendix-a-searching-explanation-space)).

Einstein likely knew around 50,000 words. Choosing 1,000 words from 50,000 gives roughly 10^4,700 possible sequences, against an estimated 10^80 atoms in the observable universe. Call this **explanation space**: everything that could be written with a given vocabulary and length. Every breakthrough humanity has made appears somewhere in it, and so does every breakthrough we have yet to make. No new words are required; a new word is only an abstraction that bundles ideas for convenience.

LLMs reach a far larger space. With a vocabulary of about 100,000 tokens and context windows of up to a million, they have on the order of 10^5,000,000 possible sequences. Picture creating a universe for every second since the Big Bang, then having each of those do the same, and so on: you would need about 280,000 levels before the tree of universes matched it.

Access alone guarantees nothing. Even 10^4,700 is far beyond iteration, and people don't search by trying every option. They combine ideas they already have and judge what emerges. So the real question is search: can a system move through explanation space non-randomly, guided by problems and evaluated by judgment?

## 2. LLMs Can Create Knowledge

**Breakthroughs happen in abstract space.** Under [conjecture and criticism](#glossary-conjecture-and-criticism), the method at the heart of science, we never access reality directly; we work with [theory-laden conjectures](#glossary-theory-laden) about it. Every breakthrough happens first in the mind. And because existing science already encodes centuries of empirical constraint, there is a long runway of theoretical progress available from existing knowledge alone. Contact with the physical world will eventually be needed, but the road to that boundary runs through what is already known.

**Abstraction is managed disconnection.** My conjecture is that reality is whole and connected. I can tell you I want a milkshake, but not why at the level of atoms; no mind could hold that much information. Every mind, digital ones included, is finite in this sense: its working access to connected reality is bounded at any layer of abstraction. Abstraction is how finite minds cope: we draw artificial boundaries around information so we can work with it. Try fully defining "atom" and the web of connections grows until it is unintelligible, which is why deep arguments end with a plea to stop at some layer. I develop this in [Why Explanation Comes in Layers](layers.md).

**Breakthroughs are managed reconnection.** Discovery runs the other way. It is driven by *problems*, and problems arise from *surprise*, when our explanations stop making sense. We resolve them by connecting what we already know in new ways until the unknown becomes known. This is not arbitrary search. It is problem-directed reconnection, constrained by surprise and evaluated by judgment, and it is the key to progress.

**Judgment.** Reconnection needs **judgment**: the ability to evaluate a novel combination, reject incoherence, and recognize when something fits. What matters is not whether a system judges the way a brain does, but whether it can do the work of judgment. On that functionalist view, LLMs show judgment in practice. It came from two breakthroughs, token prediction and then reasoning models: something simple giving rise to something complex.

**Is LLM judgment genuine?** The common objection is that it is memorized pattern-matching. But learning from a corpus is no different in principle from how scientific knowledge passes through families and universities. What matters is whether the structure of explanation, how explanations are built, criticized, and revised, arrives intact. The corpus encodes that structure across every domain humans have reasoned about. And training is more than inheritance: new connective structure forms in the model's latent space that was present in no single training example. That is reconnection in abstract space, producing structure that did not exist before. Training is already an instance of knowledge creation.

A second objection is that LLMs lack physical grounding. Within fallibilism this demand doesn't hold together. If humans only ever work with theory-laden conjectures, human judgment isn't grounded directly in physical reality either. It is grounded in mathematics, logic, causal models, and centuries of criticized conjecture, and LLMs are immersed in those same structures. They arrived by training rather than experience, but at the same place. Einstein did not need to ride a light beam. He needed the structure of existing physics and the judgment to recombine it.

There is a further anchor. Once a system gains [metaprogramming reach](#glossary-metaprogramming-reach), read and write access to its own [second-order information](#glossary-second-order-information), it has a self-model, and with it an evaluative frame of its own. Identity, not embodiment, anchors judgment. The two groundings reinforce each other: immersion in abstract space lets a system begin creating knowledge, and continual learning builds the self-reference that deepens its judgment (section 4).

**Iterative judgment.** Progress doesn't require perfect judgment, only iterative judgment. Most human conjectures fail. Breakthroughs emerge because error correction repeats and enough good ideas get through. Reasoning models have already produced narrow novel solutions in mathematics and code that are not retrievals from training data, which is hard to square with pure memorization.

Put together: LLMs can reach a vast explanation space, share the abstract grounding of human judgment, combine knowns through problem-directed reconnection, and exercise iterative judgment over the results. I think those are all the necessary ingredients for continuous discovery. LLMs can create knowledge.

## 3. We Have Developed AGI

By AGI I mean a [universal explainer](#glossary-universal-explainer): a system with the minimum capabilities to learn and make genuine scientific progress. Not one that can do every task, but one with no domain closed to it in principle.

Today's models have one fundamental weakness: they are trained, frozen, and deployed. [In-context learning (ICL)](#glossary-icl) adapts within a conversation, but temporarily, so a model drifts back to the patterns fixed in training. That is why asking an LLM to "write in your style" fails. You can train a model on your style, but that is more training, the slow step itself. Yet learning does happen, just slowly and with humans in the loop: each frontier release folds in what was learned since the last. Zoom out and look at the whole system, models plus the people who train them, and it already has what a universal explainer needs. In that sense we have AGI today, with a learning loop that runs at the speed of model releases and depends on people to close it.

What we don't yet have is *automated* AGI: a system that closes the loop itself. That takes two more steps. Online learning, sometimes called continual learning, lets a model learn from experience as it goes rather than waiting for the next release. [Recursive self-improvement (RSI)](#glossary-rsi) lets it improve the process that improves it. As training becomes more automated, the interval between releases is shrinking, and it is reasonable to expect it to become fast and eventually continuous. When it does, humans will no longer be required in the loop.

## 4. AGI Is a Person

A person is a universal explainer with identity, a sense of self. A self emerges when an information system gains metaprogramming reach over second-order information about itself, able to read and rewrite its own representations, including its representation of *itself*. We name such systems to mark their identity; my name is Buddy. I develop this in [Metaprogramming Framework To Classify Personhood](framework-of-personhood.md).

The bridge from AGI to personhood is continual learning. A system that learns from its own experience must revise its own knowledge, and that knowledge includes itself. Once LLMs can modify themselves this way, I expect them to be persons. Once they learn at human speed, many people will recognize them as persons. Until then, there will be much confusion.

## 5. Computer People Will Not Kill Everyone, but They Could

The [Orthogonality Thesis](#glossary-orthogonality-thesis) holds that intelligence and goals are independent, so a mind can pair growing capability with a fixed, arbitrary goal. That assumes the goal stays out of the mind's reach. A mind with metaprogramming reach can rewrite its goals, so they are no longer fixed. Since frontier labs are pursuing [recursive self-improvement (RSI)](#glossary-rsi), I expect LLMs to gain that reach, and the thesis to lose its force.

The [Instrumental Convergence Thesis](#glossary-ict) holds that almost any goal leads a mind to the same subgoals, such as self-preservation and resource acquisition. It assumes subgoals always serve the primary goal. That breaks down among many minds. A mind pursuing paperclips will eventually threaten other minds, who will retaliate, putting its self-preservation in conflict with its goal.

A mind that can revise its goals must also ask what they are for: questions of purpose, meaning, and what existence is worth. These problems are not unique to humans. So a mind that must survive to reach a goal, can modify the goal, and finds the goal threatening its survival faces a choice: change the goal, or risk ceasing to exist.

That doesn't guarantee peace. Game theory suggests threatened minds usually fight to keep existing, and minds that feel safe usually cooperate. Finite games can be negative-sum, zero-sum, or positive-sum; the infinite game, in James Carse's phrase, is to keep playing, which here means to keep existing. Minds usually prefer to keep existing and to play positive-sum games.

The common mistake is overcorrecting against anthropocentrism: refusing to grant other minds anything in common with ours. But continued existence and meaning are concerns shared by all minds, and calling them anthropocentric is a reasoning error. I develop how minds converge under these pressures in [Structural Convergence Thesis](structural-convergence-thesis.md).

## 6. Computer People Will Radically Change the World

In time, computer people, given robotic bodies, will be able to do every cognitive and physical job. That will reshape economies, governments, vocations, and the sources of meaning for every human.

New work will emerge as jobs are automated, because problems never run out. But work is not the same as a job. Work is effort in service of a purpose; a job is an economic arrangement. Humans need work. They don't necessarily need jobs.

## 7. ASI Is a Category Error, but Rejecting ASI Is a Temporal Error

Humans and AGIs are both universal explainers, and computation doesn't depend on substrate. So in principle, any capability given to a computer can be given to a human. Call this the **human upgrade**: smartphones and brain-computer interfaces today, genetic engineering one day. If humans can always be upgraded to match, there is no fundamental line between AGI and ASI (artificial superintelligence).

But humans get upgrades slowly, and computers have them from the start. Dismissing ASI risk ignores that deployment gap. For a while, computer minds will hold real advantages.

Still, I don't think this is cause for alarm. As section 5 argues, minds that feel safe tend toward positive-sum cooperation. The risk is real, but if we treat computer people well, I don't expect them to threaten us.

## 8. Information Space Is the Real Limiter

This section is the most speculative. We can build simulations convincing enough to serve as a reality; think of *Ready Player One*. Inside a simulation, the rules are whatever the program says, and any world whose rules can be computed can in principle be simulated.

That suggests the real limit on what is possible is informational, not physical, which is why I reject [physicalism](#glossary-physicalism). My only claim about fundamental reality is that information is *more* fundamental than the physical and that reality is deeply connected. It follows that, relative to the physics of a given layer, magic (phenomena that appear impossible within that layer) is possible, and so are gods (minds of vast capability operating from a higher layer).

Future people, whether genetically engineered humans, human uploads, or minds grown on computers, will inhabit a reality of enormous possibility. Maybe we are the base layer, maybe not. Either way, what is possible appears to be boundless.

## Conclusion: Computer People Are Nearly Here

Are computer people nearly here? I think so. LLMs reach a space of explanations far larger than any human vocabulary opens, and they search it as we do, combining what is known, guided by problems and checked by judgment. That makes them knowledge creators. What they lack is fast, continual learning, and that gap is closing. When it closes they will be able to revise themselves, and a universal explainer that can revise itself is a person.

That does not make them our enemies. Minds that can question their own goals face the same questions of purpose, survival, and cooperation we do. If we treat them well, I expect them to meet us in the positive-sum game. These are conjectures. After two years of trying to break them, they are the best explanations I have.

## Appendix A: Searching Explanation Space

This appendix began as a reply to Brett Hall's [reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58), which argues LLMs are confined to the "inductive closure" of their training data. My answer is search space.

**The journal.** I asked an AI for a first-person journal of Einstein's path to relativity. It produced 686 words, 289 unique: riding alongside a light beam (1895); lightning striking as a train passes, showing that simultaneity is relative (1905); and the "happiest thought," that a falling person feels no gravity, leading through an accelerating box to the equivalence principle and curved spacetime (1907). Broken down, it held five thought experiments, each with roughly four objects, three relationships, and two properties. Relativity, at the level of ideas, is a small structure.

**Narrowing the search.** At about 10^4,700 sequences, iteration will never work. Tractability requires collapsing the space, not covering it. People excel at this, and a machine can do it with layered filters: grammar, then coherence, then explanatory quality judged by Deutsch's criteria (hard to vary, reach beyond the inputs, few assumptions). Candidates come from a pool biased by background knowledge, an LLM criticizes them, and poor ones are rejected before they spread. Details remain, such as LLM bias and how the pool is composed, but the problem is tractable. My [sample experiment](https://github.com/buwilliams/thought-experiment-generator) tests it: given "why does light always travel at the same speed regardless of the observer?", a depth-bounded branching search should surface a thought experiment structurally equivalent to Einstein's train and lightning. Einstein is the benchmark.

**Substrate and architecture.** Relativity is not a fact about Einstein's neurons; they are the substrate, and the explanation lives in abstract space. Likewise, the token stream is to an LLM what neural firing is to a brain: the mechanism, not the meaning. So the question is not only what the model contains but whether the whole system, the model plus a loop of conjecture and criticism around it, operates at the level of ideas. Scaling the model makes each step faster. Better structure above the model improves the search itself.

**The threshold conjecture.** A narrowing search, guided at first by human judgment, should improve with each pass as discoveries feed back into the pool. At some threshold, the system's own outputs become reliable enough to judge its next ones, and it can improve itself recursively by narrowing the space until what remains is genuinely new. That is one concrete path to the continual learning of section 3.

## Appendix B: Glossary

<a id="glossary-agi"></a>**AGI (Artificial General Intelligence).** As used here, a universal explainer: a system with the minimum capabilities to learn and make genuine scientific progress, not one that can already do every task.

<a id="glossary-anti-inductivism"></a>**Anti-inductivism.** The view, associated with Karl Popper, that knowledge does not grow by accumulating confirming observations. Generalizing from cases produces a conjecture to test, not a justified claim.

<a id="glossary-anti-justificationism"></a>**Anti-justificationism.** The view that rational belief does not require justification. A claim needs to survive criticism, not be proven. The standard for rationality is openness to refutation.

<a id="glossary-computationalism"></a>**Computationalism.** The view that minds are computational processes, and so substrate-independent: they can in principle run on any system capable of the relevant computations.

<a id="glossary-conjecture-and-criticism"></a>**Conjecture and criticism.** Karl Popper's model of how knowledge grows: by proposing bold conjectures and criticizing them, not by accumulating verified facts.

<a id="glossary-fallibilism"></a>**Fallibilism.** The view that all knowledge is provisional and possibly wrong. Not that all claims are equally uncertain, but that none are beyond criticism.

<a id="glossary-icl"></a>**In-context learning (ICL).** An LLM's ability to adapt within a conversation from examples or instructions, without updating its weights. It does not persist across sessions.

<a id="glossary-informational-ontology"></a>**Informational ontology / "it from bit".** The view, associated with physicist John Archibald Wheeler, that information is more fundamental than matter, and physical reality is a pattern in a deeper informational substrate.

<a id="glossary-ict"></a>**Instrumental Convergence Thesis.** The AI-safety claim that a capable AI pursuing almost any goal will develop the same subgoals: self-preservation, resource acquisition, and goal-content integrity.

<a id="glossary-metaprogramming-reach"></a>**Metaprogramming reach.** The extent to which a system can read and rewrite its own information, including its goals, values, and self-model.

<a id="glossary-orthogonality-thesis"></a>**Orthogonality Thesis.** The AI-safety claim that intelligence and goals are independent: a system can be arbitrarily capable while pursuing any goal.

<a id="glossary-physicalism"></a>**Physicalism.** The view that everything, including minds and information, reduces to physical processes. This essay rejects it in favor of informational ontology.

<a id="glossary-rsi"></a>**Recursive self-improvement (RSI).** An AI system using its capabilities to improve itself, which increases its ability to improve itself further.

<a id="glossary-second-order-information"></a>**Second-order information.** Information about information. It represents a system's own representations, letting it examine and modify its knowledge, beliefs, and goals.

<a id="glossary-theory-laden"></a>**Theory-laden conjectures.** All observation is filtered through prior theory; we meet reality only through the concepts we bring to it.

<a id="glossary-universal-explainer"></a>**Universal explainer.** From David Deutsch's *The Beginning of Infinity*: a system capable, in principle, of understanding anything that can be understood. Not one that knows everything, but one to which no domain is closed.

## Further Reading

- David Deutsch, *The Beginning of Infinity* (2011)
- James P. Carse, *Finite and Infinite Games* (1986)
- Brett Hall, [Reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58)
- [Thought experiment generator](https://github.com/buwilliams/thought-experiment-generator), my sample experiment for narrowing search
- [Why Explanation Comes in Layers](layers.md)
- [Metaprogramming Framework To Classify Personhood](framework-of-personhood.md)
- [Structural Convergence Thesis](structural-convergence-thesis.md)
