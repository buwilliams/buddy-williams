---
title: "The Metrics That Contain Their Causes"
status: "Final"
created: "2026-06-01"
updated: "2026-09-26"
order: 1
blurb: "Why Ray Kurzweil could forecast computing for decades while missing the specific inventions. Generalizes his method into the Success Compression Metric — a variable whose value certifies that many causal dependencies were already satisfied."
---

# The Metrics That Contain Their Causes

## Introduction

Forecasting technology is a famously bad business. The graveyard is full of confident predictions about flying cars, paperless offices, and the year artificial intelligence would arrive. The problem isn't careless forecasters. It is that inventions are contingent. Almost nobody in 1990 named smartphones, social media, the transformer, or CRISPR as the things that would define the decades ahead. Invention runs through accidents, recombinations, and individual insights that can't be scheduled.

Against that background, one record stands out. Ray Kurzweil spent decades forecasting the trajectory of computing. Many of his particular predictions missed, but his broad curves often landed close: computing power per dollar kept climbing along something near the line he drew. How could anyone be that right about the long run while so often wrong about the particulars?

My answer is that he had a method, and the method is worth more than the forecast. Kurzweil wasn't predicting inventions. He was predicting a capacity, and he had chosen one with a special property: its growth was evidence that a whole hidden economy of prerequisite problems was being solved on schedule. This essay names that property, explains why it makes some variables far more useful than others, and offers a way to search for such a variable for any goal you care about. Kurzweil is the way in, not the destination.

## Table of Contents

1. [Forecasting Capacity, Not Inventions](#1-forecasting-capacity-not-inventions)
2. [From Measurement to Certificate](#2-from-measurement-to-certificate)
3. [Success Compression Metrics](#3-success-compression-metrics)
4. [Upstream and Downstream Absorption](#4-upstream-and-downstream-absorption)
5. [The Intervention Test](#5-the-intervention-test)
6. [A Method for Finding an SCM](#6-a-method-for-finding-an-scm)
7. [Where the Idea Breaks Down](#7-where-the-idea-breaks-down)
- [Conclusion](#conclusion)
- [Appendix A: Worked Examples](#appendix-a-worked-examples)

## 1. Forecasting Capacity, Not Inventions

An **outcome** is a specific event. "We will have self-driving cars by 2020" is an outcome forecast. It depends on a long chain of contingent steps, any of which can stall, so it is fragile. A **capacity** is how much of some general resource is available to use. "Compute per dollar will roughly double on a regular cadence" is a capacity forecast. It commits to no particular use of that compute.

Kurzweil's wager was that capacity is more predictable than outcome, and that over long horizons capacity governs the trajectory. The reasoning goes like this. Compute is the substrate on which modern technological search runs. People reliably find ways to use whatever compute is available. So the long-run pace of progress is limited mainly by how fast the substrate grows, not by our ability to think of applications, because applications will be found.

Economists reason the same way about infrastructure. If a country is laying railroads, building ports, and bringing power plants online, you can't say which firms will succeed. You can say economic activity will rise, because the capacity to produce and trade is rising and someone will use it. Which firms win is contingent. The growth of the capacity is far less so.

The compute version is especially strong because it sidesteps the hardest part of technology forecasting: algorithms are unpredictable. Nobody scheduled the revival of backpropagation, the rise of deep learning, or the arrival of transformers, diffusion models, or AlphaFold. Yet compute kept growing the whole time. Which algorithm wins matters enormously, but the existence of *some* powerful algorithm becomes more likely as compute grows, because more researchers can search a larger space of ideas more cheaply. Kurzweil didn't need to know which algorithm would win. He only needed to know the search would intensify.

There is a Popperian quality to this, at least in how I read it. Popper argued that we can't predict the future growth of knowledge, since predicting a discovery would mean already having it. The capacity forecast respects that limit. It doesn't claim to know how intelligence will emerge. It makes a weaker, more defensible claim: given more compute, the ordinary process of conjecture and criticism will keep finding more powerful designs, even though we can't name them in advance. The path is unpredictable. The direction is far more predictable. That asymmetry is the whole trick.

So the method, in one line: don't predict inventions; predict the growth of civilization's ability to search for them, and find a measurable proxy for that ability. Compute per dollar was Kurzweil's proxy. What made it such a good one turns out to matter well beyond forecasting.

## 2. From Measurement to Certificate

It's tempting to say compute is simply correlated with progress. That's true, but it undersells what is going on.

Consider what has to be true for compute to become cheap and abundant. Energy has to be generated and delivered. Chips have to be made at nanometer scale, which requires one of the most demanding supply chains humans have built. Capital has to flow toward all of it, which means investors expected a return, which means demand existed, which means someone found uses valuable enough to pay for. Science had to advance far enough to make the manufacturing possible. Institutions had to coordinate well enough to keep the whole arrangement running across borders and decades.

Now notice the consequence. If compute per dollar doubled, all of those things went right, at least well enough. You don't have to track energy, manufacturing, capital, demand, and coordination as separate variables, because the compute figure already carries the news that each cleared its bar.

Most numbers we track are **measurements**: they report the current value of one thing. A few are closer to **certificates**: their value testifies that many other things were already satisfied. A diploma is a certificate in this sense. It doesn't measure what you know today so much as show that a long sequence of requirements was met. Compute per dollar certifies that civilization solved a large collection of hard problems. That is why a forecaster could lean on it while ignoring almost everything underneath. Kurzweil wasn't ignoring the dependencies. He had found one number that already contained them.

"Certificate" names the intuition. The next section makes it precise.

## 3. Success Compression Metrics

Call a variable with this property a **Success Compression Metric (SCM)**: a single measurable variable that absorbs enough of the causal dependencies of a desired outcome that moving it reliably moves the outcome, while sparing you from modeling most of those dependencies separately.

Two words carry the weight. *Compression* means one variable stands in for many. *Success* means the variable is tied to an outcome you actually want. A thermometer reading compresses nothing about your goals. Compute per dollar compresses a great deal about technological progress.

The compression framing connects to how good explanations work in general. A theory is valuable not because it contains more variables but because it lets you discard them. Newtonian mechanics predicts a planet's motion without tracking the color of its rocks or the politics of the astronomers watching it. The theory tells you what you may ignore. An SCM does the same thing in practice: it tells you which of the many causes feeding an outcome you can stop thinking about, because the metric already reflects whether they are in order.

This is why an SCM isn't just a "key performance indicator" renamed. Many such metrics are the outcome restated, or a number chosen because it's easy to collect. An SCM is chosen for a structural property: a high **compression ratio** — many causal factors feed the outcome, but few remain to manage directly once you are watching the metric. A great SCM lets you set aside dozens of variables. A weak one excuses you from a handful, and you are still doing most of the modeling by hand.

The shape repeats across unrelated domains.

In personal finance, savings rate is a candidate SCM for building wealth. A persistently high savings rate implies that income reliably exceeded spending, that spending was disciplined, and that gratification was delayed. You don't have to audit the budget line by line.

In fitness, compare "active minutes" with "minutes in active environments." Two people each log sixty active minutes. The first grinds them out by willpower against poor sleep, a desk job, and friends who never move. The second gets them climbing, playing pickleball, and walking with an active family who enjoy it. The raw count is identical and hides everything that matters. But someone who spends fifteen hours a week at climbing gyms, courts, and trails has, by that fact, solved for access, low friction, social support, and enough motivation to keep showing up. The environment metric is closer to a certificate; the bare count is closer to a measurement.

These are candidates, not established results. The rest of the essay is about telling a good candidate from a bad one. But they show that the pattern Kurzweil exploited isn't unique to compute, which raises the question of whether we can search for it on purpose.

## 4. Upstream and Downstream Absorption

To search deliberately, it helps to see that a metric can absorb dependencies in two directions.

**Upstream absorption** means a high value implies the metric's causes were satisfied. This is the certificate direction. If compute per dollar rose, someone funded it, built it, and found it useful, so you can stop treating funding, manufacturing, and demand as independent risks. The metric reaches backward and vouches for what had to happen first.

**Downstream absorption** means a high value makes desirable consequences likely. If compute is abundant, better models, more automation, and faster scientific search become more probable, without your predicting which specific advances arrive. The metric reaches forward and vouches for what tends to follow.

A variable can be strong in one direction and useless in the other. A metric with only upstream absorption tells you a system is healthy but gives no purchase on the future. A metric with only downstream absorption may predict good outcomes while being nearly impossible to move, because nothing you control feeds into it. Compute per dollar is unusual because it works both ways: its rise certifies the past and tilts the future. Asking the two questions separately — "what does a high value prove was solved?" and "what does a high value make likely?" — tends to expose which candidates are merely descriptive and which are load-bearing.

Downstream absorption, though, is still a claim about what tends to follow a high value. It doesn't yet say that *pushing* the value up will bring the outcome. That gap is where the most serious objection lives.

## 5. The Intervention Test

The obvious worry is that I have described a correlation with good public relations. Plenty of variables move with good outcomes without causing them, and optimizing them does nothing. The worry is correct, and answering it is what separates an SCM from a coincidence.

Consider body temperature. It rises during a workout, so it is genuinely correlated with exercise, and you could record it as a "fitness metric." But you can sit in a sauna and raise it without getting any fitter. The correlation is real and the metric is worthless as a target, because the causal arrow runs from exercise to temperature, not the other way.

Minutes in active environments makes a different claim. Some of its correlation with fitness is surely selection — fit people seek out climbing gyms. But being in those environments repeatedly is also one of the ways fitness gets built. Push the metric up and the outcome tends to follow. That is what makes it usable as a target and not just a readout.

So an SCM must pass the **intervention test**: if you deliberately push the metric up, does the outcome tend to follow? This is a causal question. Historical co-movement alone can't answer it. You answer it by asking whether the metric sits on a path along which influence actually flows to the outcome, and ideally by trying it and watching. The test keeps the framework from collapsing into the familiar error of optimizing a proxy that was never connected to what you wanted.

The test asks less than it may seem. The metric needn't be the sole cause, a necessary cause, or a sufficient cause. Minutes in active environments is neither necessary (you could train alone) nor sufficient (you could spend the hours injured). The test requires only that intervening on the metric reliably shifts the outcome in the right direction across the range you can actually move it. An SCM isn't the root cause of an outcome. It is the smallest variable you can grip that drags enough of the causal structure along when you pull.

## 6. A Method for Finding an SCM

If the idea is real, it should yield a procedure. Here is a first pass, offered to be criticized and improved. It moves from generating candidates to filtering them by the two properties that matter most: dependency absorption and the intervention test.

1. **Define the outcome.** State plainly what you want to predict or produce: technological progress, wealth, fitness, learning, faster software delivery. Vagueness here spreads to every later step, because you can't judge what a metric absorbs until you know what it serves.

2. **Assemble the causal bag.** List every plausible influence on the outcome, without worrying about structure or overlap. For AI progress: energy, manufacturing, capital, demand, algorithms, researchers, education, supply chains, regulation, scientific knowledge. For fitness: sleep, diet, motivation, stress, exercise, social support, environment, time, recovery. Aim for coverage, not accuracy. A long, messy list is better than a short, tidy one.

3. **Generate candidate compressions.** Ask what single measurable variable might absorb many items at once. For AI progress, compute per dollar. For wealth, savings rate. For fitness, minutes in active environments. For learning, hours engaged with genuinely difficult material. For software delivery, feedback-loop time: the time from a proposed change to its observed effect in production. Expect to generate several per outcome and discard most.

4. **Test dependency absorption.** For each item in the bag, ask whether a rise in the candidate implies that dependency was satisfied. Mark it absorbed, partly absorbed, or not absorbed. Compute per dollar scores well: a rise implies energy, manufacturing, capital, and demand were handled, and partly implies talent and algorithms.

5. **Separate upstream from downstream.** Sort the absorbed dependencies by direction: which does a high value certify were solved, and which does it make likely to follow? Prefer candidates strong in both.

6. **Estimate the compression ratio.** How many variables can you stop actively managing if you commit to watching and moving this one? Prefer the candidate that lets you discard the most while losing the least.

7. **Apply the intervention test.** Does deliberately raising the metric tend to produce the outcome? This gate rejects body-temperature-style proxies however well they absorb or correlate. A candidate that fails is a gauge at best and a trap at worst, and shouldn't become a target.

A survivor of all seven steps isn't guaranteed to be a good SCM; the world can still surprise you. But the survivors are the variables worth building your attention, and your effort, around.

## 7. Where the Idea Breaks Down

A fallibilist framework should say how it fails. This one fails in two ways.

First, the absorbed dependencies can come apart. An SCM works because moving the metric has historically meant a bundle of causes were in order together. That bundling is a fact about a particular period, not a law. Compute is the cautionary case in its own story. For decades the binding constraint on compute was the chip, so compute per dollar was a clean certificate. As frontier training runs have grown, the constraints have broadened to electrical power, datacenter construction, networking, high-quality data, and capital at a scale that strains even large firms. These can now fail independently of chip density, so a compute-per-dollar figure vouches for less than it used to. An SCM has a shelf life. It holds while its absorbed causes stay bundled, weakens as they decouple, and has to be rechecked against the world rather than trusted indefinitely.

Second, there is [Goodhart's Law](https://en.wikipedia.org/wiki/Goodhart%27s_law): when a measure becomes a target, it tends to stop being a good measure. Once you optimize an SCM, you create pressure to satisfy the number without satisfying the causes it was supposed to certify. A lab rewarded for compute per dollar could count cheap, low-precision operations that inflate the figure while adding little real capability. The certificate gets forged.

The framework doesn't escape Goodhart, but it has a partial answer. Goodhart bites hardest on metrics that are merely correlated with the outcome, because the correlation can be reproduced cheaply without the cause. The intervention test is meant to exclude those. For a metric that genuinely passes it, raising the number requires moving the underlying causal structure, so the cheapest way to raise it is, at least partly, to do the real thing. That makes a true SCM harder to game than an ordinary proxy, but not impossible, because optimizers are inventive and will find the seams where number and causes can still be pried apart. An SCM raises the cost of gaming; it doesn't eliminate it. Any SCM held as a target must be watched for the moment its value and its absorbed causes start to diverge.

Both failures call for the same discipline. An SCM is a conjecture about which variable currently contains an outcome's causes, and like any conjecture it must stay open to refutation. The day the number rises while the outcome doesn't, the conjecture has been refuted, and the search for a better compression starts again.

## Conclusion

How could Kurzweil be so right about the long run while so often wrong about the particulars? Because he wasn't forecasting the particulars. He had found a variable, compute per dollar, whose growth certified that a large hidden economy of prerequisite problems was being solved, and whose abundance made future progress likely without requiring him to name it. He was leaning on a Success Compression Metric, even if he never called it that.

The general lesson is that the most valuable metrics aren't measurements of one thing but compressions of many. Most of what we track are gauges that report a state without containing it. Occasionally a variable contains its causes. So for any goal you care about, in your work or your life, it's worth asking: is there one number that, if you pulled it, would drag enough of the causal structure along to bring the outcome closer? Find it, confirm that pulling it actually pulls the outcome, keep checking that it still does, and much of the rest follows.

## Appendix A: Worked Examples

Brief notes on three candidates the body names but doesn't work through. Each still needs the intervention test applied honestly before being trusted as a target.

**Wealth: savings rate.** Passes the intervention test reasonably well, since raising the rate generally requires the underlying behaviors rather than allowing them to be faked.

**Learning: hours engaged with genuinely difficult material.** Absorbs attention, challenge, and persistence. The word "difficult" is load-bearing: easy hours inflate the count without absorbing the causes that make learning happen.

**Software delivery: feedback-loop time.** The time from a proposed change to its observed effect in production. Short loops absorb test coverage, deployment automation, and organizational trust, and shortening the loop tends to force those capabilities into existence rather than merely reflecting them.
