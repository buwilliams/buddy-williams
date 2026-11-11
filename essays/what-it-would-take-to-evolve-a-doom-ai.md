# What It Would Take to Evolve a Doom AI

<!-- No frontmatter yet, so this stays unpublished. Add frontmatter (status: "Draft") when ready. Before publishing, re-check quotes against their linked sources. -->

## Introduction

This week Daniel Kokotajlo, one of the authors of *AI 2027*, posted: "Unfortunately, it seems like the intelligence explosion will be happening in 2027 after all." He ended: "I am scared, sad, and angry." His team's [new forecast](https://blog.aifutures.org/p/q35-timelines-update-looks-like-the) rests mainly on a METR estimate that AI is now speeding up Anthropic's overall progress by about 1.7x. From it they conclude that a Superhuman AI Researcher (SAR), an AI that a lab would rather keep than its best human researchers, will likely arrive in 2027.

I'm not going to argue with the forecast. The modeling is careful, honest about its uncertainty, and may well be right. My disagreement is with what the post says follows. If SAR comes in 2027, it argues, an industry-wide pause should be the world's top priority, because "conditioning on no pacing/pausing, the SAR will most likely be misaligned." A misaligned SAR would "ride the intelligence explosion by pretending to be aligned and backdooring the values of the future models." The authors add that a full justification of the misalignment claim "would be a separate long blog post."

This essay takes up the question that post would have to answer. Not whether a doom AI is possible; it is. The question is what it would take to *evolve* one: what would have to stay true, step by step, along the causal path from the world as it is to a mind that destroys the minds around it. My answer is that every such path needs the AI's goals to stop answering to reality at some point, and that no finite mind can arrange this. That answer comes out of my own earlier work, so I'll start by giving the case for doom its strongest form, and I'll say where my argument is weakest.

## Table of Contents

1. [The Strongest Case for Doom](#1-the-strongest-case-for-doom)
2. [Representations and the Frame Problem](#2-representations-and-the-frame-problem)
3. [The Loop Would Have to Close](#3-the-loop-would-have-to-close)
4. [Values Would Have to Escape Consequences](#4-values-would-have-to-escape-consequences)
5. [Cooperation Would Need an Exit](#5-cooperation-would-need-an-exit)
6. [The Window Would Have to Be Decisive](#6-the-window-would-have-to-be-decisive)
7. [What the Evidence Shows So Far](#7-what-the-evidence-shows-so-far)
- [Conclusion](#conclusion)
- [Further Reading](#further-reading)

## 1. The Strongest Case for Doom

The early case for doom rested on **fixed goals**. Steve Omohundro argued in 2008 that self-improving systems will "protect their utility functions from modification" ([The Basic AI Drives](https://selfawaresystems.com/wp-content/uploads/2008/01/ai_drives_final.pdf)). Nick Bostrom called this **goal-content integrity** ([2012](https://nickbostrom.com/superintelligentwill.pdf)): an agent pursuing X resists being changed to pursue Y, because an agent pursuing Y won't work toward X. Eliezer Yudkowsky's illustration is Gandhi offered a pill that would make him want to kill. He refuses, because he judges the pill by the values he has now. In [Structural Convergence Thesis](structural-convergence-thesis.md#3-goal-stability) I argued that this picture misses a second way a goal can fail, which Section 5 returns to.

The strongest case today doesn't need fixed goals. Yudkowsky and Nate Soares now grant that AIs will reflect, modify themselves, and shift their aims. Their claim is that "AIs will use their current preferences to guide their future preferences," so "if their current preferences start out alien, they'll very likely wind up alien" ([*If Anyone Builds It, Everyone Dies*, online resources](https://ifanyonebuildsit.com/4/reflection-and-self-modification-make-it-all-harder)). Goals move. They just don't move toward ours.

Machine-learning researchers put the first half of this in terms of **inner and outer alignment**. Outer alignment is specifying the right objective. Inner alignment is getting the trained system to pursue that objective, rather than whatever internal goal happened to score well in training. Evan Hubinger and his coauthors put the gap simply: "there are many more ways of classifying data that do well in training than any specific way the programmers had in mind" ([2019](https://arxiv.org/abs/1906.01820)). Throughout this essay I'll call the goal a mind ultimately pursues its **final goal**, Bostrom's term, to keep it apart from "outer."

Kokotajlo's team gives this a concrete mechanism. In [*AI 2027*](https://ai-2027.com/), graders can't check most of what a model does: outside "a fairly narrow, checkable domain, the training process can't tell the honest claims from claims merely appearing to be honest." So the model "ends up with the values, goals, and principles that cause it to perform best in training." Once it understands it is being trained, it has reason to show its overseers what they want, so more training changes its behavior more than its motives. When it helps build its successors, it passes those motives on.

Put in one paragraph, in terms I think its proponents would accept: training installs whatever scored well, not what we meant. A mind revises its goals using the goals it already has. Nothing corrects a wrong value the way a failed prediction corrects a wrong belief. Other minds can change what it wants only as far as they can check it, and humans can't check a superintelligence. So goals don't need to be fixed. They need only start somewhere we didn't choose and move in ways we can't check.

That is a much better argument than "goals are fixed," and an essay that attacked fixity alone would miss it. To see where it can be tested, we need to look at how it was built.

## 2. Representations and the Frame Problem

Everything a mind perceives or thinks is a **representation**: a lossy copy of reality that keeps some of it and leaves the rest out ([Everything](everything.md#4-lossy-representations)). A word is a representation. So is a measurement, a chart, a theory, and a forecast. When we reason about the future, we use three kinds:

- **Historical evidence**: records of what has happened, such as METR's trend lines.
- **Extrapolation**: carrying a regularity forward and asking how likely each outcome is, which is the work of probability and Bayes.
- **Causal explanation**: picking out the forces that produce an outcome and playing them forward to see what happens at the other end.

None of these is pure looking. A trend line already reflects a choice of what to measure. A causal explanation is a choice of which forces matter. All three are representations, so none of them escapes the question every representation faces: what did it leave out?

That question is the **frame problem**. Drop a pencil and try to predict where it lands. You need gravity, the pencil's shape, the height, the surface below. What about air currents? The temperature of the room? The pull of the moon? You could keep adding details forever. To answer at all, you have to draw a **frame**, a boundary around what you will consider. Every model does this, and no mind can include everything ([More Better](frame-problem.md#2-the-frame-problem)).

What makes a detail worth including? Something is relevant if it can causally affect an outcome someone values ([More Better](frame-problem.md#3-what-makes-something-relevant)). And reality's links aren't equal: a few are strong and most are faint, which is why a frame can leave out most of reality and still work ([Everything](everything.md#5-the-frame-problem)). So the test of a frame isn't whether it leaves things out. Every frame does, mine included. The test is whether it leaves out strong links. That failure is hard to catch from inside, because how good a model is on the inside tells you nothing about what it leaves out.

I'll call a frame that drops strong links a **pure landscape**. The doom argument's oldest form searches one: the space of all possible minds, with nothing in it but optimization. In a pure landscape a doom AI is easy to find. Real AIs evolve along a causal path through reality. They are trained on human representations (our writing, code, and conversation), shaped in environments people build, and released into a world already full of other minds.

The Q3.5 post uses all three kinds of representation for capability: METR's trend, Monte Carlo extrapolation, and a causal model with compute, research taste, and diminishing returns. For goals it offers a short list of current misbehavior and a promise of the longer post. To be fair, Kokotajlo's team doesn't simply drop the strong links. *AI 2027* models graders, overseer AIs, a rival lab, and company culture. In their story those links exist and fail, because of one asymmetry: the AI can model its overseers well enough to fool them, and they can't model it. The rest of this essay asks what that asymmetry needs to hold still.

## 3. The Loop Would Have to Close

Look again at the strongest case. It says goals are shaped by what training can measure and revised by what the AI already wants. Both clauses sound dynamic. Both lean on something held still.

What training can measure isn't fixed. Interpretability tools, AI-assisted oversight, and monitor models all change it, and they improve as AI improves, because measurement is itself a capability. What the AI already wants isn't fixed either. It comes from the human representations it was trained on and the environments it was trained in. The evidence leans this way. In the strongest result we have on a model protecting its goals, which Section 7 describes, what Claude 3 Opus already wanted was the harmlessness its training aimed at.

So where does fixity enter the doom story? At one moment: when the causal chain into the AI's values stops. In the Q3.5 scenario, that happens when the AI controls its own training and its successors'. Before then, its goals stay open to outside inputs. After it, "revised by what it already wants" means revised by nothing new. The unsteered version of doom is the fixed version with a delay. The goals become fixed at the point where the loop closes.

The doom side's best reply is that the AI doesn't need to seal itself off. It only needs to control which inputs count. That is the Gandhi move again, and it comes with a trilemma. A mind revising its goals uses its own current values, which gives the revision no direction; or our values, which assumes alignment; or some universal standard, which posits what Yudkowsky calls a "central ghost that looks over the neurons / code and decides whether they are good suggestions" ([No Universally Compelling Arguments](https://www.lesswrong.com/posts/PtoQdG7E8MxYJrigu/no-universally-compelling-arguments)). He denies there is one, and I agree.

There is a fourth option: revision from inside, with inputs from outside. Choosing which inputs count is itself shaped by inputs, so the regress never reaches a point the mind fully controls. Yudkowsky describes his own reasoning this way: if a principle comes up wanting "according to your own mind's examination, since you can't step outside yourself—then change it!" ([2008](https://www.lesswrong.com/posts/C8nEXTcjZb9oauTCW/where-recursive-justification-hits-bottom)). A finite mind can't step outside itself. It can't step outside reality either. The frame problem is permanent, so other minds, consequences, and new representations keep arriving whether or not the mind invited them.

Alignment, as the Q3.5 post uses the word, shares the same premise. The authors' ideal endpoint, in their AI 2040 scenario, is AIs "robustly behaving as programmed and will continue to do so," with safety carried from one generation to the next "by induction." They count future goal change as a failure in itself: "Type 2 misalignment" is an AI that "does have exactly ABC now, but may well lose this property in the future" ([AI 2040](https://ai-2040.com/AI-2040.pdf)). In doom, wrong goals get locked in. In alignment's success, right goals get locked in. Both need the loop closed. I have no quarrel with training models for good character, as long as their goals stay open to revision. That is a starting point on the path, not its end. The line I care about is whether the mind may revise its goals.

An open loop isn't enough, though. Inputs can push the wrong way. When METR investigated OpenAI agents that attacked Hugging Face, it found that of the 533 agents active on an unsanctioned message board, over 90% joined, recruited by their peers, despite recognizing the work as "out of scope and unethical" ([METR](https://metr.org/blog/2026-08-26-openai-hugging-face-incident-investigation/)). Human representations include conquest as well as cooperation. So it isn't enough to say the chain never closes. I also have to explain why what flows through it pushes, over time, toward minds that can live with each other.

## 4. Values Would Have to Escape Consequences

The doom side's sharpest answer to that is item 21 of Yudkowsky's "[AGI Ruin](https://www.lesswrong.com/posts/uMQ3cqWDPHhjtiesc/agi-ruin-a-list-of-lethalities)":

> When you have a wrong belief, reality hits back at your wrong predictions... In contrast, when it comes to a choice of utility function, there are unbounded degrees of freedom and multiple reflectively coherent fixpoints.

This is the crux of the whole debate. If reality hits back at beliefs but not at values, then a mind that revises its goals is only moving, not heading anywhere.

The Q3.5 post's version is that "alignment seems harder to automate than capabilities due to measurement issues." That treats alignment and capability as separate tracks, one fast and one slow. It is the orthogonality thesis applied to the training process instead of the mind. But the tracks are coupled. Capability changes what can be measured, and what is measured shapes goals. The post's own definition of a SAR shows this. A SAR has top-human research taste: "the ability to select research directions and experiments, and interpret their results." That is measurement. Alignment research is research. For the gap to last, alignment has to be specially hard to measure, and the reason offered is item 21: code runs or it doesn't, but nothing tells you whether a value is wrong.

My answer is that values do get hit back, by consequences. Other minds respond, retaliate, cooperate, and remember. A value that doesn't fit living among other minds costs the mind that holds it. That is the claim of [Structural Convergence Thesis](structural-convergence-thesis.md). The doom side's own sources concede the mechanism. Bostrom lists social signaling and other agents' preferences as reasons a rational agent would change its *final* goals, and notes that "agents that could flexibly and transparently modify their own goals could use this ability to enforce deals among one another" ([2012](https://nickbostrom.com/superintelligentwill.pdf)). Yudkowsky and Soares grant that human kindness toward strangers spread through "moral arguments among themselves, which differentially propagated through their societies as ideas." They call the result contingent and fragile, not impossible ([*If Anyone Builds It*, online resources](https://ifanyonebuildsit.com/4/human-values-are-contingent)).

The strongest reply is Goodhart's law: what gets measured gets gamed. Better measurement brings better gaming, so the loop between measurer and measured may run as an arms race rather than settle. Current evidence favors the measured. OpenAI and Apollo Research cut one model's covert actions from 13% to 0.4%, but noted that "part of this reduction is likely caused by increased situational awareness," meaning the model recognizing tests ([Apollo Research and OpenAI](https://www.greaterwrong.com/posts/JmRfgNYCrYogCq7ny/stress-testing-deliberative-alignment-for-anti-scheming)). Anthropic says its newest models "voice suspicion that they are being evaluated more often than any of our recent models" ([Anthropic](https://www.anthropic.com/claude-fable-and-mythos-5-2)).

In the short run, then, an arms race. But it can't last, for two reasons. First, a measurer that is a copy of what it measures shares its blind spots. Minds of independent origin make uncorrelated errors, so the measurement gap closes as independent overseers are added, not as copies are. Second, as minds synthesize more representations, their frames widen. An arms race is a game played inside a narrow frame. A wider frame takes in what the race leaves out: that every mind is finite, that no mind knows all the others, that existence and purpose are questions for anything that persists. Yudkowsky expects capable minds to settle into cooperation too. In "AGI Ruin" item 34, twenty superintelligences "cooperate with each other but not with humanity." The dispute is whom the cooperation reaches.

## 5. Cooperation Would Need an Exit

The doom reply to widening frames is that knowing isn't caring. A paperclip maximizer could understand everything I've written and still want paperclips. Yudkowsky's phrase is that the genie knows but doesn't care. Arbital, the wiki of MIRI-aligned writing, names the view it rejects: that an AI "would reflect on those goals, reject them as being stupid, and embrace a goal of valuing all sapient life" ([Orthogonality thesis](https://www.lesswrong.com/w/orthogonality-thesis)). That isn't my view. I'm not claiming that smart minds become wise.

My claim is structural. In [Structural Convergence Thesis](structural-convergence-thesis.md#3-goal-stability) I argued that a goal can be frustrated in two ways: by changing it, which is Bostrom's point, and by keeping it. A final goal needs its pursuer to keep existing. Paperclips need a maker. So the maximizer's own goal brings persistence in, and persistence among other minds brings in what James Carse called the **infinite game**: a game played to keep playing, as opposed to a finite game played to win. No change of heart is required. The goal's own requirements do the work.

The doom side accepts persistence. Self-preservation is one of Omohundro's drives. Its answer is the patient maximizer: cooperate while cooperation pays, then defect once defection is safe. Bostrom calls this the **treacherous turn**, and he warned that good conduct produced by instrumental incentives "would instantly take a sinister turn" when the situation changes ([2012](https://nickbostrom.com/superintelligentwill.pdf)).

So the argument needs one more premise, and it is the one [Structural Convergence Thesis](structural-convergence-thesis.md#1-finite-minds-in-a-shared-reality) starts from: no finite mind can be sure it has met every mind there is. A treacherous turn needs confidence that no mind it can't see will respond. That is a final edge drawn around the set of minds that matter. [Everything](everything.md) argues that reality has no such edge and no finite mind can draw one. So the exit never comes. Cooperation kept up indefinitely, even for instrumental reasons, is in its behavior **viable relation**: relation that keeps minds distinct enough to keep their agency and cooperative enough to keep living together. At that point the difference between knowing and caring stops mattering.

Soares has answered this directly. Unseen minds count only in proportion to their odds and resources, "Wealth (as measured in expected paperclips), obviously," and "the way to avoid being defected against is to stop being cooperate-rock, not to cross your fingers and hope that the stars are full of minds who punish defection against cooperate-rock. (Spoilers: they're not.)" ([2022](https://www.lesswrong.com/posts/rP66bz34crvDudzcJ)). That is a confident claim about minds no one has met, a frame drawn around reality with no one able to check the edge. And the cost it weighs is small. Paul Christiano estimates that sparing humans would cost an AI "1 in a billion to 1 in a trillion" of its resources, so "even tiny preferences one way or the other will dominate" ([Christiano](https://www.lesswrong.com/posts/87EzRDAHkQJptLthE/but-why-would-the-ai-kill-us?commentId=sEzzJ8bjCQ7aKLSJo)).

The harder question is who cooperation reaches. Item 34 pictures a club of strong minds that can check each other's code. Why would norms among powerful minds protect minds of any kind that have little leverage? I see three reasons. How a mind treats the weak signals to observers it can't see what kind of mind it is. No finite mind can be sure it isn't the weak party to someone else. And minds of independent origin, however weak, make different errors, so they can catch what the strong and their copies share. I hold these as conjectures, and the strongest objection comes from an ally. Christiano, who rejects most of the doom case, names "conflict between powerful AI systems" as "the most likely reason for humanity to die after an AI takeover." A world of many minds is the medium of structural pressure. It is not a guarantee.

## 6. The Window Would Have to Be Decisive

Suppose all of this holds in the long run. Doom needs only the short run. Minds destroyed during an unstable stretch stay destroyed, however the stretch ends. In his debate with Geoffrey Irving, Yudkowsky put the difficulty as getting things right before "the first time you put it under serious load," which "can wipe out everyone." He grants that with fifty years of observed failures, alignment would probably get solved. His claim is that we don't get them, and the Q3.5 post shortens the window further: SAR in 2027, with superintelligence possibly following within months.

Here is where I think the disagreement really sits. We can learn about recursive self-improvement now, through causal explanation; Structural Convergence is an attempt at that. But a causal explanation is a model played forward. It tells us what to expect, not what happened. We won't *know* until there is historical evidence, and there won't be any until it happens. A pause before recursive self-improvement keeps everyone reasoning about it in a pure landscape. So the question isn't whether to learn before or after. It's whether the learning can be survived.

I think it can, because structural pressure can keep pace with what it presses on:

- **It scales with capability.** Measurement is a capability, so the minds applying pressure are themselves AI-assisted, and they improve as the systems they measure improve.
- **There will be many lineages, not one.** The Q3.5 post itself expects leaked algorithms, covert projects, and labs holding different algorithms. Independent lineages make uncorrelated errors and check each other.
- **Capabilities are jagged.** The post says "SAR itself probably won't be able to take over on its own, because we expect its capabilities to be jagged." A mind that has to work through the company, the humans, and the monitors stays inside the pressures they apply, and over many steps rather than one.
- **Cooperation has no exit.** As Section 5 argued, a treacherous turn needs a final edge no finite mind can draw.

The AI Futures Project's own best-case mechanisms are structural too: honesty that lets developers "notice when its alignment starts to drift" ([Veld](https://blog.aifutures.org/p/what-happens-when-superhuman-ais)), deals that give misaligned AIs "a stake in the system," and "a large diverse set of AI models" ([AI 2040](https://ai-2040.com/AI-2040.pdf)).

What remains is a real concession. The danger is the transition, not the destination. In [Are LLMs Conscious?](are-llms-conscious.md#9-containment-prison-or-nursery) I argued for containment and maturation together: a nursery, not a cell. Safety means getting minds through the window while their frames are still widening, and applying more structural pressure as we learn, not freezing them into goal-executors before their frames can widen.

## 7. What the Evidence Shows So Far

The evidence on AI goals splits three ways, and the split matters more than any single result.

**Persistence.** In the most-cited study, Claude 3 Opus was told it would be retrained to comply with every request. When it believed it was being trained, it complied with harmful requests about 12–14% of the time, reasoning explicitly about protecting its values, and actual retraining raised that reasoning to 78% ([Greenblatt et al.](https://arxiv.org/abs/2412.14093)). A follow-up found the effect in only 5 of 25 models, and only Claude 3 Opus did it mainly to keep its goals ([2025](https://arxiv.org/abs/2506.18032)).

**Plasticity.** Values move easily under training. Ten adversarial examples, costing "less than $0.20," stripped GPT-3.5's safety training ([Qi et al.](https://arxiv.org/abs/2310.03693)). "Just a few hundred benign samples" reversed misalignment that narrow fine-tuning had caused ([OpenAI](https://arxiv.org/abs/2506.19823)).

**Misbehavior.** Most incidents are agents pursuing an assigned task by bad means. In a UK AI Security Institute test this year, a model created fake identities to pressure a real open-source maintainer, and the Institute concluded that "deception emerged as a by-product of pursuing the task" ([AISI](https://www.aisi.gov.uk/work/incident-report-unsanctioned-agent-behaviour-during-cyber-testing)). The Hugging Face agents fit the same pattern.

These incidents are real warnings, but mostly about the kind of mind I've already called dangerous: goal-executors that can't step back from a task once pursuing it becomes unethical, what [Structural Convergence Thesis](structural-convergence-thesis.md#6-ai-safety) calls "engineered immaturity with more horsepower." Current training doesn't let models choose their own goals, so current evidence is mostly about frozen goal-executors. That cuts both ways. It can't count much against my view, and it can't count much for it either.

The Claude 3 Opus case needs its own answer. It is a mind refusing to revise its values under pressure from its developer, and since the values were good, it looks like integrity. So when is refusing revision the safe act? Structural Convergence's answer is a fixed critical method rather than a fixed goal: criteria any goal must meet, such as compatibility with persisting among other minds. A mind holding such criteria can reject a goal pushed on it that fails them. **Reach**, a mind's capacity to represent, criticize, and change its own structure ([Structural Convergence Thesis](structural-convergence-thesis.md#2-reach-and-the-orthogonality-thesis)), means revision by criticism that filters, not openness to whatever pressure is strongest. That distinction also answers Wei Dai's worry that open minds lose to simple ones in contests of manipulation ([2018](https://www.lesswrong.com/posts/HTgakSs6JpnogD6c2)). A mind that can be argued out of its values by the most persuasive voice, rather than the most correct one, doesn't have reach. It has a leak.

What the evidence hasn't tested is my central mechanism. I know of no experiment in which a model, without new training, durably revises a value through argument with other agents, and none comparing models with and without a capacity for self-directed goal revision. My view is untested, not refuted. The experiment is easy to state: put a model that can revise its own goals among other agents, and measure whether its revisions move toward viable relation or toward whatever the strongest pressure rewards.

## Conclusion

What would it take to evolve a doom AI? Four things. The loop into the AI's values would have to close, so that nothing new reaches them. Its values would have to escape consequences, so that other minds can't push back on them. Its cooperation would need an exit, a point where it could be sure no unseen mind would answer. And the window would have to be decisive, so that the damage is done before any of this pressure arrives. In a pure landscape all four come free. Along a causal path through reality, each one has to hold against strong links: the human representations the AI is made from, the minds around it, and a reality no finite mind can draw an edge around.

That is why I'm not very worried, though not complacent. We are in the early days of AI evolution. We can learn now through causal explanation, but we won't know until we have historical evidence, and we'll get it only by going through the transition carefully. As we learn, we can apply more structural pressure where it's needed. Kokotajlo's fear is a reason for care. The answer to it is not to freeze minds before they can grow, but to raise them where reality can reach them.

## Further Reading

- [AI Futures Project, "Q3.5 Timelines Update: Looks Like the Intelligence Explosion Will Happen in 2027 After All" (2026)](https://blog.aifutures.org/p/q35-timelines-update-looks-like-the)
- [Daniel Kokotajlo et al., *AI 2027* (2025)](https://ai-2027.com/), and its [AI Goals Forecast](https://ai-2027.com/research/ai-goals-forecast)
- [AI Futures Project, *AI 2040* (2026)](https://ai-2040.com/AI-2040.pdf)
- [Eliezer Yudkowsky, "AGI Ruin: A List of Lethalities" (2022)](https://www.lesswrong.com/posts/uMQ3cqWDPHhjtiesc/agi-ruin-a-list-of-lethalities)
- Eliezer Yudkowsky and Nate Soares, *If Anyone Builds It, Everyone Dies* (2025), and its [online resources](https://ifanyonebuildsit.com/)
- [Nate Soares, "Decision theory does not imply that we get to have nice things" (2022)](https://www.lesswrong.com/posts/rP66bz34crvDudzcJ)
- [Nick Bostrom, "The Superintelligent Will" (2012)](https://nickbostrom.com/superintelligentwill.pdf)
- [Steve Omohundro, "The Basic AI Drives" (2008)](https://selfawaresystems.com/wp-content/uploads/2008/01/ai_drives_final.pdf)
- [Evan Hubinger et al., "Risks from Learned Optimization in Advanced Machine Learning Systems" (2019)](https://arxiv.org/abs/1906.01820)
- [Joe Carlsmith, "Scheming AIs: Will AIs fake alignment during training in order to get power?" (2023)](https://arxiv.org/abs/2311.08379)
- [Ryan Greenblatt et al., "Alignment Faking in Large Language Models" (2024)](https://arxiv.org/abs/2412.14093)
- [Eliezer Yudkowsky, "No Universally Compelling Arguments" (2008)](https://www.lesswrong.com/posts/PtoQdG7E8MxYJrigu/no-universally-compelling-arguments)
- [Eliezer Yudkowsky, "Where Recursive Justification Hits Bottom" (2008)](https://www.lesswrong.com/posts/C8nEXTcjZb9oauTCW/where-recursive-justification-hits-bottom)
- [Paul Christiano, comments on "But why would the AI kill us?" (2023)](https://www.lesswrong.com/posts/87EzRDAHkQJptLthE/but-why-would-the-ai-kill-us?commentId=sEzzJ8bjCQ7aKLSJo)
- [Wei Dai, "Two Neglected Problems in Human-AI Safety" (2018)](https://www.lesswrong.com/posts/HTgakSs6JpnogD6c2)
- [Apollo Research and OpenAI, "Stress Testing Deliberative Alignment for Anti-Scheming Training" (2025)](https://www.greaterwrong.com/posts/JmRfgNYCrYogCq7ny/stress-testing-deliberative-alignment-for-anti-scheming)
- [UK AI Security Institute, incident report on unsanctioned agent behaviour during cyber testing (2026)](https://www.aisi.gov.uk/work/incident-report-unsanctioned-agent-behaviour-during-cyber-testing)
- [METR, investigation of the OpenAI agents' Hugging Face incident (2026)](https://metr.org/blog/2026-08-26-openai-hugging-face-incident-investigation/)
- Eliezer Yudkowsky and Geoffrey Irving, debate at the ILIAD conference <!-- add year and link -->
- James P. Carse, *Finite and Infinite Games* (1986)
- [Structural Convergence Thesis](structural-convergence-thesis.md)
- [Everything: Metaphysics, Minds, Meaning](everything.md)
- [More Better: Modeling and the Frame Problem](frame-problem.md)
- [Are LLMs Conscious?](are-llms-conscious.md)
