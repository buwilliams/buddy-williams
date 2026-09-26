---
title: "In Search of the Modeling Process"
status: "Final"
date: "Aug 2026"
order: 1
blurb: "A search for the computational process behind model creation: how observations, purposes, values, and abstractions become reusable reasoning programs—and perhaps the primitive underlying creativity itself."
---

# In Search of the Modeling Process

## Introduction

For several years I've been trying to teach AI to do something I can't fully explain, even to myself. I call it **modeling**. The difficulty of explaining it is the problem itself: if I could spell the process out precisely, I could probably build it.

This essay is a research journal, not a finished theory: the ground I've covered, the pieces I think I've found, and the one that still eludes me. I don't think that missing piece is a larger language model. I think it's a process.

## Table of Contents

1. [What Is Modeling?](#1-what-is-modeling)
2. [Observation and Purpose](#2-observation-and-purpose)
3. [The Bridge](#3-the-bridge)
4. [Why Current AI Falls Short](#4-why-current-ai-falls-short)
5. [Philosophy Before Verification](#5-philosophy-before-verification)
6. [Reasoning Programs](#6-reasoning-programs)
7. [Creativity and the Missing Primitive](#7-creativity-and-the-missing-primitive)
- [Conclusion: Why This Matters](#conclusion-why-this-matters)

## 1. What Is Modeling?

Every discipline has structure. Science has the scientific method, mathematics has proofs, economics has supply and demand, and software engineering has design patterns and algorithms. Someone had to invent these: to find where one concept ends and another begins, and to decide that certain observations belong together under a new abstraction.

A **model** is a representational structure like these. **Modeling** is the process that creates one before it exists. (I use both terms the same way in [More Better: Modeling and the Frame Problem](frame-problem.md).)

Map making is a useful analogy. Walk into unexplored territory and the land could hold anything: mountains, rivers, cities, or nothing. As you explore, the possibilities shrink. You notice patterns, measure, compare, and find relationships, until you step back and see that this isn't a pile of observations. It's a forest. Or a market. Or natural selection. A new category is born, and the map becomes simpler than the territory while keeping what matters. That act is modeling.

## 2. Observation and Purpose

I increasingly think observation is itself a computational operation. Before we observe, the possibilities are effectively unbounded, and each observation rules some out. But the work isn't just collecting facts. It's discovering which constraints matter, and that takes questions. When I study a flower, I don't just record its color. I ask why it grows here, what surrounds it, what predators shape it, and how rainfall matters. The questions decide what I look for, and the model can only be as good as the observations they produce.

Observation alone isn't enough, because every model exists for a **purpose**. A road map, a topographical map, and a political map of the same region are all correct; each is built for a different purpose. Or take a pot of boiling water. Why is it boiling? Because a heating element is transferring energy into it. Or: because someone wanted ramen. Same event, same observations. What differs is the purpose behind the question, and so which answer is useful. Purpose also decides which details count as relevant at all, which is the heart of the [frame problem](frame-problem.md#2-the-frame-problem).

This mattered to me: there are endlessly many possible observations, and endlessly many possible purposes. Both sides of modeling are open-ended.

## 3. The Bridge

This led me to what I currently believe is the central structure. Every model is a **bridge**. On one side are observations; on the other, a purpose. The bridge is the abstraction that compresses the observations into a representation useful for that purpose. It is what makes information meaningful.

Some bridges are better than others. They explain more with fewer concepts, compress better, transfer more easily into other minds, or survive criticism longer. Others collapse almost at once. Making sense of anything, on this view, means building bridges and criticizing them.

There is one more layer. If purposes are endless, how do we choose among them? With **values**. A business can make money through fraud or by creating value; our ethics rule out whole regions of possibility before we start modeling the problem in front of us. Values shape purpose, purpose shapes representation, and representation shapes action.

## 4. Why Current AI Falls Short

For years I've tried to teach language models this process. One experiment used a deliberately underspecified request: *Make me money.*

A thoughtful person sees at once how incomplete that is. Who is asking? How much, and by when? Under what ethical constraints, with what skills and resources, in what country and decade? The request opens into a large modeling exercise. Current language models rarely do this. They answer immediately, jumping to execution before building an adequate representation of the problem. What they lack isn't knowledge or skill at carrying out a plan; it's modeling. Execution is downstream of representation.

The closest architecture I've found is neuro-symbolic AI. The probabilistic side does the observing; the symbolic side supplies structure. Together they resemble the two halves of modeling. I've built several such systems, including Harmoniq Work and Refine, and they work surprisingly well. But they share one limitation: I built the symbolic structure, and the AI only operated inside it. Today's agent frameworks are similar. The harness is fixed, and the model fills in the gaps.

What interests me is the level above the harness. Can a system build the harness itself, revise its own representations, and invent better abstractions?

## 5. Philosophy Before Verification

This is where philosophy becomes indispensable. AI research rightly invests in verification: benchmarks, evaluation, ground truth. But verification comes after a model exists. Before it comes concept formation, deciding what things exist and how to carve them up.

Karl Popper's idea of conjectures and refutations has shaped my thinking here. No amount of evidence proves a general explanation true, so progress comes from proposing better explanations and criticizing them hard. Criticism tells us how to weed out bad explanations. It leaves open how we come up with good candidates in the first place. That question is the modeling problem.

## 6. Reasoning Programs

One idea I keep returning to is thinking in programs rather than workflows. Not programs as code, but as executable reasoning structures. Logic, mathematics, physics, and economics are reusable reasoning programs built over centuries. Facing a new problem, people don't reason from scratch. They retrieve and adapt existing programs, sometimes compose several, and occasionally invent new ones.

A reasoning program is a bridge that has become reusable. Later problems can retrieve it, adapt it, criticize it, and improve it. Over time a library grows: not of answers, but of ways of thinking.

## 7. Creativity and the Missing Primitive

Here is my boldest conjecture, and the one I'm least sure of: **creativity is the construction of new bridges between observations and purposes.** A system becomes creative when it can change its purposes freely and generate new abstractions that connect them to observations, bridges that then hold up under criticism. Creativity isn't randomness or novelty for its own sake. If the conjecture is right, modeling is the computational process underlying creativity.

That leaves the question I can't yet answer: what is the primitive operation beneath all this? I can tell an AI to make observations, build a model, or compare explanations. Telling it to do those things isn't the same as implementing observation, model construction, or comparison.

I suspect there is a smaller computational process underneath: something recursive and reusable, a primitive that generates reasoning programs the way reasoning programs generate models. I suspect today's language models already hold nearly all the knowledge needed to run it. What they lack is the process.

## Conclusion: Why This Matters

If this primitive exists, it would change how we think about intelligence. Today's AI systems execute remarkably well inside representations someone else built. I think the frontier is shifting from execution to representation: building better models, better abstractions, better bridges.

Perhaps this process is the missing ingredient for recursive scientific discovery, or for recursive self-improvement. Perhaps it underlies creativity itself. I don't know; it remains a conjecture. But until I either find the primitive or show that it can't exist, I won't be satisfied. That has become the research.
