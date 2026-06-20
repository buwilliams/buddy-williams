# The Metrics That Contain Their Causes

## Introduction

Forecasting the future of technology is a famously bad business. The graveyard is full of confident predictions about flying cars, paperless offices, and the year artificial intelligence would arrive. The reason is not that forecasters are careless. It is that inventions are contingent. Almost nobody in 1990 named smartphones, social media, the transformer architecture, or CRISPR as the things that would define the decades to come. The specific path of invention runs through accidents, recombinations, and individual acts of insight that cannot be scheduled in advance.

Against that background, one record stands out as strange. Ray Kurzweil spent decades forecasting the trajectory of computing, and while many of his particular predictions missed, his broad curves often landed close. Computing power per dollar kept climbing along something near the line he drew. The puzzle is not whether he was lucky. The puzzle is how anyone could be that right about the long run while being so often wrong about the particulars.

I think the answer is a method, and that the method is more valuable than the forecast. Kurzweil was not predicting inventions at all. He was predicting a capacity, and he had chosen a capacity with a special property: its growth was evidence that a whole hidden economy of prerequisite problems was being solved on schedule. This essay is about that property, why it makes some variables far more powerful than others, and how you might deliberately search for such a variable for any goal you care about. The Kurzweil case is the way in, not the destination.

## Table of Contents

1. [Forecasting Capacity, Not Inventions](#forecasting-capacity-not-inventions)
2. [From Correlation to Certificate](#from-correlation-to-certificate)
3. [Success Compression Metrics](#success-compression-metrics)
4. [Upstream and Downstream Absorption](#upstream-and-downstream-absorption)
5. [Why Correlation Is Not Enough](#why-correlation-is-not-enough)
6. [A Method for Finding an SCM](#a-method-for-finding-an-scm)
7. [Where the Idea Breaks Down](#where-the-idea-breaks-down)
8. [Conclusion](#conclusion)
9. [Appendix A: Working Definitions](#appendix-a-working-definitions)
10. [Appendix B: Worked Examples](#appendix-b-worked-examples)

## Forecasting Capacity, Not Inventions

The first move is to separate two things that are easy to conflate: forecasting an outcome and forecasting a capacity.

An outcome is a specific event. "We will have self-driving cars by 2020" is an outcome forecast. It depends on a long chain of contingent steps, any one of which can stall, and so it is fragile. A capacity is a measure of how much of some general resource is available to be used. "Computation per dollar will roughly double on a regular cadence" is a capacity forecast. It does not commit to any particular use of that computation.

Kurzweil's wager was that capacity is more predictable than outcome, and that over long horizons the capacity is what governs the trajectory. The argument runs roughly as follows. Compute is the substrate on which modern technological search runs. Human ingenuity reliably finds ways to exploit available compute. Therefore the long-run rate of progress is constrained mainly by how fast the substrate grows, not by our ability to think of applications, because applications will be found.

This is the same shape of reasoning an economist uses about infrastructure. If you know a country is laying railroads, building ports, and bringing power plants online, you cannot say which firms will succeed. You can say economic activity will rise, because the underlying capacity to transact and produce is rising and someone will use it. The identity of the winners is contingent. The growth of the capacity is not.

What makes the compute version of this argument especially strong is that it sidesteps the single hardest part of technology forecasting, which is that algorithms are unpredictable. Nobody scheduled the resurgence of backpropagation, the rise of deep learning, the arrival of transformers, diffusion models, or AlphaFold. Yet compute kept increasing the entire time. The specific algorithm that wins turns out to matter enormously, and yet the existence of *some* powerful algorithm becomes more and more likely as the compute budget grows, because more researchers can search a larger space of possibilities more cheaply. Kurzweil did not need to know which algorithm would win. He only needed to know that the search would intensify.

There is a Popperian quality to this. Kurzweil was not claiming to know how intelligence would emerge, which would be a claim to prophecy he could not support. He identified a relatively stable variable, computation per dollar, and then made a far weaker and more defensible claim: given enough computation, the ordinary process of conjecture and criticism will discover increasingly powerful cognitive architectures, even though the specific discoveries cannot be named in advance. The particular path is unpredictable. The direction is not. That asymmetry is the whole trick.

So the strongest one-line version of his method is this: do not try to predict inventions, predict the growth of civilization's ability to search the space of inventions, and find a measurable proxy for that search capacity. Compute was his proxy. The next two sections ask what made it such a good one, because the answer turns out to generalize well beyond forecasting.

## From Correlation to Certificate

It is tempting to say compute is simply correlated with progress, or that it is the environment in which progress happens. Both descriptions are true and both undersell what is going on.

Consider what has to be true of the world for compute to be cheap and abundant. Energy has to be generated and delivered. Chips have to be manufactured at nanometer scale, which requires one of the most demanding supply chains humans have ever built. Capital has to be allocated toward all of it, which means investors had to expect a return, which means demand had to exist, which means someone had to find uses valuable enough to pay for. Scientific knowledge had to advance far enough to make the manufacturing possible at all. Institutions had to coordinate well enough to keep the whole arrangement running across borders and decades.

Now notice the consequence. If compute per dollar doubled, then all of those things must have gone right, at least well enough. You do not have to model energy, manufacturing, capital, demand, and coordination as separate variables and track each one, because the compute figure already carries the news that each of them cleared its bar. The number is not just a measurement of how much computation exists. It is evidence that a long list of hard prerequisite problems were solved.

This is the reframe at the heart of the essay. Most numbers we track are measurements: they report the current value of one thing. A few rare numbers are closer to certificates: their value is a guarantee that many other things have already been satisfied. A diploma is a certificate in exactly this sense. It does not measure what you know today so much as testify that a long sequence of requirements (courses passed, exams cleared, years served) was met. Compute is a certificate that civilization solved a huge collection of hard problems, and that is why a forecaster could lean on it so heavily while ignoring almost everything underneath. Kurzweil was not ignoring the dependencies. He had found a single number that already contained them.

The word "certificate" is doing intuitive work here, not technical work. It names the feeling of the idea. The next section gives the idea a precise definition and a name we can use carefully for the rest of the essay.

## Success Compression Metrics

Call a variable with this property a **Success Compression Metric**, or SCM: a single measurable quantity that absorbs enough of the causal dependencies of a desired outcome that moving it reliably moves the outcome, while sparing you from having to model most of the dependencies separately.

Two words in that definition carry the weight. "Compression" is the claim that one variable stands in for many. "Success" is the claim that the variable is tied to an outcome you actually want, not just to some arbitrary quantity. A thermometer reading compresses nothing about your goals. Compute compresses a great deal about the goal of technological progress.

The compression framing is worth taking literally, because it connects this idea to how good explanations work in general. A scientific theory is valuable not because it contains more variables but because it lets you discard them. Newtonian mechanics is powerful precisely because you can predict a planet's motion without tracking the color of its rocks, the history of its formation, or the politics of the astronomers studying it. The theory tells you which variables you are allowed to ignore. An SCM does the same thing operationally. It tells you which of the dozens of causes feeding an outcome you can stop thinking about, because the metric already reflects whether they are in order.

This is why an SCM is not simply a "key performance indicator" or a "north star metric" rebranded. Many such metrics are merely the outcome restated, or a number chosen because it is easy to collect. An SCM is selected for a specific structural property: a high compression ratio between the count of causal factors that feed an outcome and the count you still have to manage directly once you are watching the metric. The fewer you have to manage, the better the SCM. A great one lets you set aside dozens of variables. A weak one only excuses you from a handful, which means you are still doing most of the modeling by hand.

It helps to see the shape repeat across unrelated domains.

In personal finance, savings rate is a candidate SCM for building wealth. A persistently high savings rate implies that income was generated, that spending was disciplined, and that gratification was delayed. You do not have to audit the budget line by line. The rate testifies that the underlying behaviors are in order.

In fitness, the transcript that seeded this essay proposed "minutes spent in active environments" over the more obvious "active minutes." The point of that swap is instructive enough to keep. Two people can each log sixty active minutes. The first grinds them out by willpower against poor sleep, a sedentary job, and friends who never move. The second accumulates them by climbing, playing pickleball, and walking with an active family who enjoy it. The raw count of active minutes is identical and hides everything that matters. But "minutes in active environments" begins to absorb the hidden causes. Someone who spends fifteen hours a week at climbing gyms, courts, and trails has, by that very fact, solved for access, for reduced friction, for social reinforcement, and for at least enough motivation to keep showing up. The environment metric is closer to a certificate; the bare-count metric is closer to a measurement.

These examples are not offered as established results. They are candidates, and the rest of the essay is partly about how to tell a good candidate from a bad one. But they show that the pattern Kurzweil exploited is not unique to compute. It is a general shape that good metrics can have, and the interesting question is whether we can search for it on purpose.

## Upstream and Downstream Absorption

To search for an SCM deliberately, it helps to notice that a metric can absorb dependencies in two directions, and that the strongest metrics do both.

Upstream absorption means the metric's value implies that its causes were satisfied. This is the certificate direction discussed above. If compute grew, then someone funded it, someone built it, and someone found it useful, so you can stop modeling the funding, the building, and the demand as independent risks. The metric reaches backward and vouches for what had to happen before it could rise.

Downstream absorption means the metric's value implies that desirable consequences become likely. If compute is abundant, then better models, more automation, and faster scientific search become more probable, without your having to predict which specific advances arrive. The metric reaches forward and vouches for what tends to follow once it is high.

The distinction matters because a variable can be strong in one direction and useless in the other, and you want both. A metric with only upstream absorption tells you a system is healthy but gives you no purchase on the future. A metric with only downstream absorption may predict good outcomes while being almost impossible to move, because nothing you control feeds into it. Compute is unusually good because it works in both directions at once: its rise certifies the past and tilts the future. When you evaluate a candidate SCM, asking these two questions separately ("what does a high value prove was solved?" and "what does a high value make likely?") tends to expose which candidates are merely descriptive and which are genuinely load-bearing.

This double absorption is also what gives an SCM its leverage as a target rather than just a gauge, and that raises the most important objection to the whole idea, which the next section confronts directly.

## Why Correlation Is Not Enough

The obvious worry is that I have described nothing more than a correlation with good public relations. Plenty of variables move together with good outcomes without causing them, and optimizing such a variable does nothing. This worry is correct, and meeting it is what separates an SCM from a coincidence.

Consider body temperature and exercise. Core body temperature rises during a workout, so it is genuinely correlated with exercise, and you could collect it as a "fitness metric." But raising your body temperature does not produce fitness. You can sit in a sauna and move the number without moving the outcome at all. The correlation is real and the metric is worthless as a target, because the arrow of causation does not run from the metric to the outcome. It runs the other way, or from a common cause to both.

Now contrast minutes in active environments. The claim there is stronger and different in kind. Spending those minutes does not merely accompany fitness. It tends to *produce* it, because being in the environment repeatedly is one of the mechanisms by which fitness is built. Increasing the metric increases the outcome. That is what makes it usable as a target and not just as a readout.

So an SCM must pass what I will call the intervention test: if you deliberately push the metric up, does the outcome tend to follow? This is a causal question, not a correlational one, and it cannot be answered by looking at historical co-movement alone. It is answered by asking whether the metric sits on a path along which influence actually flows to the outcome. The test is what stops the framework from collapsing into the familiar error of optimizing a proxy that was never causally connected to the thing you wanted.

It is worth being precise about what the intervention test does and does not require. It does not require the metric to be the sole cause, or a necessary cause, or a sufficient cause. Fitness has many causes, and minutes in active environments is neither necessary (you could train alone) nor sufficient (you could spend the hours injured). What the test requires is weaker and more practical: that intervening on the metric reliably shifts the outcome in the desired direction across the range you can actually move it. An SCM is not the root cause of an outcome. It is the smallest variable you can grip that drags enough of the causal structure along with it when you pull.

## A Method for Finding an SCM

If the idea is real, it should yield a procedure. The following is a first pass, offered as a method to be criticized and refined rather than a finished recipe. Its steps move from generating candidates to filtering them by the two properties the essay has argued matter most: dependency absorption and causal traction.

**Define the outcome.** State plainly what you are trying to predict or produce. Technological progress, wealth, fitness, learning, the speed of software delivery. Vagueness here propagates into every later step, because you cannot judge what a metric absorbs until you know what it is supposed to serve.

**Assemble the causal bag.** List every plausible influence on the outcome without worrying about structure, overlap, or importance. For AI progress that includes energy, manufacturing, capital, demand, algorithms, researchers, education, supply chains, regulation, and scientific knowledge. For fitness it includes sleep, diet, motivation, stress, exercise, social support, environment, time, and recovery. The goal at this stage is coverage, not accuracy. You are trying to surface the dependencies a good metric will need to absorb, so a long messy list is better than a short tidy one.

**Generate candidate compressions.** Ask what single measurable variable might absorb many items in the bag at once. For AI progress, compute. For wealth, savings rate. For fitness, minutes in active environments. For learning, hours engaged with genuinely difficult material. For software delivery, the length of the feedback loop from idea to deployed change. These are hypotheses, and you should expect to generate several per outcome and discard most of them.

**Run the dependency-absorption test.** For each item in the causal bag, ask whether a rise in the candidate metric implies that the dependency was satisfied. Compute scores well: a rise implies energy, manufacturing, capital, and demand were handled, and at least partly implies talent and algorithms. Mark each dependency as absorbed, partially absorbed, or not absorbed. A candidate that absorbs many items is compressing a lot; a candidate that absorbs few is barely earning its keep.

**Separate upstream from downstream.** For the absorbed dependencies, sort them by direction. Which ones does a high value certify were solved (upstream), and which ones does a high value make likely to follow (downstream)? A candidate strong in both directions is far more useful than one strong in only one, for the reasons given earlier.

**Estimate the compression ratio.** Ask the blunt practical question: how many variables can I stop actively managing if I commit to watching and moving this one? The answer is the metric's compression ratio, and it is the closest thing the framework has to a score. Prefer the candidate that lets you discard the most while losing the least.

**Apply the intervention test.** Finally, ask whether deliberately increasing the metric tends to produce the outcome. This is the gate that rejects body-temperature-style proxies no matter how well they absorb or correlate. A candidate that fails here is a gauge at best and a trap at worst, and should not be promoted to a target.

A candidate that survives all seven steps is not guaranteed to be a good SCM, because the world can still surprise you. But the survivors are the variables worth building your attention, and your interventions, around.

## Where the Idea Breaks Down

A fallibilist framework should name the conditions under which it fails, and this one fails in two distinct ways that are worth keeping in view.

The first failure is that the absorbed dependencies can come apart. An SCM works because, historically, moving the metric meant a bundle of causes were all in order together. But that bundling is a contingent fact about a particular regime, not a law. Compute is the cautionary case in its own story. For decades the binding constraint on computation was the chip, so compute per dollar served as a clean certificate. As frontier training runs have grown, the constraint has broadened to include electrical power, datacenter construction, network infrastructure, high-quality data, and capital at a scale that strains even large firms. As those constraints separate from chip density, a single compute figure absorbs less than it used to, because the dependencies it once vouched for can now fail independently of it. The lesson is that an SCM has a shelf life. It holds while its absorbed causes stay bundled and weakens as they decouple, which means an SCM must be rechecked against the world rather than trusted indefinitely.

The second failure is the one any reader who knows Goodhart's Law will raise immediately: when a measure becomes a target, it tends to stop being a good measure. The moment you optimize an SCM, you create pressure to satisfy the number without satisfying the causes it was supposed to certify. A firm told to maximize compute can buy low-quality flops that inflate the figure while contributing little to actual capability. The certificate gets forged.

The framework does not escape Goodhart, but it does have a partial answer, and locating that answer precisely is more useful than pretending the problem away. Goodhart bites hardest on metrics that are merely correlated with the outcome, because the correlation can be reproduced cheaply without reproducing the cause. The intervention test is designed to exclude exactly those metrics. An SCM that genuinely passes the intervention test is one where moving the number requires moving the underlying causal structure, so the cheapest way to raise it is, at least in part, to do the real thing. That makes a true SCM harder to game than an ordinary proxy, but not impossible, because optimizers are inventive and will find the seams where the metric and the causes can still be pried apart. The honest position is that an SCM raises the cost of gaming rather than eliminating it, and that any SCM held as a target must be watched for the moment its absorbed causes and its measured value begin to diverge.

Both failures point at the same discipline. An SCM is a conjecture about which variable currently contains an outcome's causes, and like any conjecture it must stay exposed to refutation. The day the number rises while the outcome does not is the day the conjecture has been refuted, and the search for a better compression should begin again.

## Conclusion

The question this essay started from was how Kurzweil could be so right about the long run while being so often wrong about the particulars. The answer is that he was not forecasting the particulars at all. He had found a variable, computation per dollar, whose growth was evidence that a large hidden economy of prerequisite problems was being solved on schedule, and whose abundance made future progress likely without committing him to naming it. He was leaning on a Success Compression Metric, even if he never called it that.

Generalized, the idea is that the most valuable metrics are not measurements of one thing but compressions of many. Their value is twofold. Looking backward, a high value certifies that a long list of causes were satisfied, which is why you can stop modeling them one by one. Looking forward, a high value makes the outcome you want more likely, which is why moving the metric is worth doing. The discipline that keeps the idea honest is the intervention test, which insists that the metric cause the outcome and not merely accompany it, and the steady awareness that any such metric can decay as its absorbed causes come apart or be gamed once it becomes a target.

The practical upshot is a question worth asking of any goal you care about, in your work or your life. Among all the things you could measure, is there one variable whose movement would carry the rest along, one number that, if you pulled it, would drag enough of the causal structure with it to bring the outcome closer? Most of the things we track are not that. They are gauges that report a state without containing it. But occasionally there is a variable that contains its causes, and finding it lets you stop managing the world by hand and start moving the one lever that moves the others. Find that variable, confirm that pulling it actually pulls the outcome, and most of the rest is downstream.

## Appendix A: Working Definitions

**Success Compression Metric (SCM).** A single measurable variable that absorbs enough of the causal dependencies of a desired outcome that intervening on the variable reliably moves the outcome, while freeing you from having to model most of the dependencies separately.

**Dependency absorption.** The property by which a metric's value implies that certain causal prerequisites were satisfied, so that those prerequisites need not be tracked independently.

**Upstream absorption.** Absorption in the backward direction: a high value of the metric implies that its causes were satisfied.

**Downstream absorption.** Absorption in the forward direction: a high value of the metric makes desirable consequences more likely.

**Compression ratio.** An informal measure of an SCM's strength: how many causal variables you can stop actively managing if you commit to watching and moving the metric.

**Intervention test.** The requirement that deliberately increasing the metric tends to produce the outcome, distinguishing a causal SCM from a merely correlational proxy.

## Appendix B: Worked Examples

The following are candidate SCMs, not confirmed ones. Each is offered to show the shape of the reasoning, and each would need the intervention test applied honestly before being trusted as a target.

**Technological progress: compute per dollar.** Strong upstream absorption (energy, manufacturing, capital, demand) and strong downstream absorption (more automation, faster scientific search). Caveat: weakening as the binding constraint broadens beyond chips.

**Wealth: savings rate.** Upstream absorption of income generation, spending discipline, and delayed gratification. Passes the intervention test reasonably well, since raising the rate generally requires the underlying behaviors rather than allowing them to be faked.

**Fitness: minutes in active environments.** Absorbs access, reduced friction, social reinforcement, and sustained motivation. Plausibly passes the intervention test where a bare count of active minutes does not, because the environment is part of the mechanism rather than a side effect of it.

**Learning: hours engaged with genuinely difficult material.** Absorbs attention, challenge, and persistence. The qualifier "difficult" is load-bearing, since easy hours inflate the count without absorbing the causes that make learning happen.

**Software delivery: feedback-loop time.** The interval from a proposed change to its observed effect in production. Short loops absorb test coverage, deployment automation, and organizational trust, and shortening the loop tends to force those capabilities into existence rather than merely reflecting them.
