# Emma: Build Independence

The implementation of this model is being built in [pathwise](https://github.com/buwilliams/pathwise). This document defines the model. It does not draw a conclusion about what Emma should do.

Emma is graduating high school, turning 18, and beginning the transition into adult life. She wants more independence and is considering major life decisions: a car, more work, moving out, a path toward higher income. These decisions are not isolated. A car affects money, mobility, job options, insurance, and savings. Moving out affects independence, rent burden, work hours, stress, and time. Pursuing education affects future income but costs time, money, discipline, and delayed gratification.

The purpose of this model is not to prove the one correct answer. The purpose is to make the important factors explicit so we can use conjecture and criticism to evaluate possible paths.

The document has three main sections plus an appendix. Section 1 states the core conjecture. Section 2 formalizes it. Section 3 explains it in detail. The appendix lists questions for discovering values and constraints.

---

## 1. Conjecture

Time is Emma's most valuable resource. It is finite and spent whether or not she chooses where it goes. A life worth living is an enjoyable life.

Enjoyment is not the absence of difficulty. Hardship pursued toward an outcome she wants is part of a good life. Hardship endured for an outcome she doesn't want is wasted life. The target modifies the difficulty.

The lived experience along the way is most of life. A path that ends well but spends years in misery is not the same as a path that ends equally well and is good to live through. The journey is what Emma lives. The destination is what she lives in.

The best path for Emma is therefore one that creates momentum toward a life she wants, where the momentum is recoverable, adjustable, and good to live through.

- **Momentum**: each step moves her life-state toward something more desirable, not merely something survivable.
- **Recoverability**: any step can be reversed, repaired, or stepped back from if it turns out badly. Mistakes should not end the game.
- **Adjustability**: the path is not chosen once. It is revised at each stage as new information arrives.
- **Journey**: the path is judged by the lived experience across stages, not only by where it ends.

The model that follows expands and formalizes this conjecture.

---

## 2. Formalized Model

### 2.1 State

$$
L = \{V, T, A, K, W\}
$$

$$
V = \{i_1, i_2, i_3, i_4, e, g, \beta\}
$$

$$
T = \{p, b, q\}
$$

$$
A = \{c, \sigma, d, r, y, \gamma\}
$$

$$
K = \{k, \tau, \mu, \rho, \delta\}
$$

$$
W = \{\phi, \psi, \zeta, \eta, \nu\}
$$

| Symbol | Meaning |
|---|---|
| $L$ | Life-state |
| $V$ | Values |
| $T$ | Time |
| $A$ | Assets (including income) |
| $K$ | Education |
| $W$ | Health |
| $i_1, i_2, i_3, i_4$ | Mobility, financial, residential, decision independence |
| $e$ | Enjoyable life experience |
| $g$ | Goal progress |
| $\beta$ | Stability / emotional safety |
| $p$ | Productive hours per week |
| $b$ | Buffer hours |
| $q$ | Quality of time / energy |
| $c$ | Monthly cash flow |
| $\sigma$ | Savings |
| $d$ | Debt / liabilities |
| $r$ | Risk buffer / emergency fund |
| $y$ | Income |
| $\gamma$ | Income growth rate |
| $k$ | Education path |
| $\tau$ | Time required for $k$ |
| $\mu$ | Money required for $k$ |
| $\rho$ | Completion / payoff risk for $k$ |
| $\delta$ | Technology trajectory of $k$ |
| $\phi$ | Physical health |
| $\psi$ | Mental health |
| $\zeta$ | Fitness practice (diet, exercise) |
| $\eta$ | Net emotional impact (signed) |
| $\nu$ | Relational quality |

Subscripts: $x_s$ denotes $x$ in scenario $s$; $x_j$ denotes $x$ at stage $j$ of a path. Thresholds ($r_{min}$, $\phi_{min}$, $y_{min}$, $H$, $R_{min}$, etc.) are Emma-specific parameters.

### 2.2 Cross-Variable Relationships

Education's effects on time, assets, and projected income:

$$
p_{actual} = p_{baseline} - \tau_{weekly} \quad \text{during training}
$$

$$
\sigma_{after} = \sigma_{before} - \mu
$$

$$
E[\Delta y] = (1 - \rho) \cdot \Delta y_{\text{if complete}}
$$

$$
\hat{y}(L_s, H) = y_s \cdot (1 + \gamma_s)^H \cdot (1 + \delta_s)^H
$$

Emotional comparison between alternatives:

$$
\text{a choice becomes more attractive as } \eta(\text{this}) - \eta(\text{alternative}) \text{ rises}
$$

### 2.3 Viability and Desirability

$$
L_{viable} = \{L : c \geq 0 \land r \geq r_{min} \land p \geq p_{min} \land b \geq b_{min} \land q \geq q_{min} \land \phi \geq \phi_{min} \land \psi \geq \psi_{min} \land \nu \geq \nu_{min}\}
$$

$$
\begin{aligned}
L_{desirable} = L_{viable} \cap \{L : \;& (\forall k \in \{1,2,3,4\}: i_k \geq i_{k,min}) \land e \geq e_{min} \land g \geq g_{min} \land \beta \geq \beta_{min} \\
& \land\; y \geq y_{min} \land \sigma \geq d \land \hat{y}(L, H) \geq y_{min} \land \zeta \geq \zeta_{min} \land \eta \geq \eta_{min}\}
\end{aligned}
$$

$$
S_{viable} = \{s \in S : L_s \in L_{viable}\}, \quad S_{desirable} = \{s \in S : L_s \in L_{desirable}\}
$$

$$
S_{desirable} \subseteq S_{viable} \subseteq S
$$

### 2.4 Recoverability

$$
R(s_j) = 1 - \frac{w_\lambda \lambda_j + w_\xi \xi_j + w_\Delta \Delta_j}{w_\lambda + w_\xi + w_\Delta}
$$

Where $\lambda_j, \xi_j, \Delta_j \in [0, 1]$ measure lock-in duration (as a fraction of $H$), exit cost (as a fraction of $\sigma$), and state disruption (fraction of $L$ materially altered). $w_\lambda, w_\xi, w_\Delta$ are Emma-specific weights. $R(s_j) \in [0, 1]$.

### 2.5 Momentum and Path Objective

$$
Momentum(L) = \sum_{x \in components(L)} w_x \cdot x
$$

Where $components(L)$ ranges over the scored scalar sub-components of $V$, $T$, $A$, and $W$: $i_1, i_2, i_3, i_4, e, g, \beta, p, b, q, c, \sigma, r, y, \gamma, \phi, \psi, \zeta, \eta, \nu$. ($K$'s sub-components and $d$ are tracked but not directly scored.)

Single-scenario optimum:

$$
s^* = \arg\max_{s \in S_{desirable}} Momentum(s)
$$

Path: $P = (s_0, s_1, \dots, s_n)$. Path momentum sums life-state quality across stages, weighted by stage duration $d_j$:

$$
Momentum(P) = \sum_{j=0}^{n} d_j \cdot Momentum(L_j)
$$

Path objective:

$$
P^* = \arg\max_{P \in \mathcal{P}} Momentum(P)
$$

subject to:

$$
L_n \in L_{desirable}, \quad L_j \in L_{viable} \;\forall j, \quad R(s_j) \geq R_{min} \;\forall \text{ large decisions } j
$$

### 2.6 Adjustability

The path objective is applied iteratively. At each stage $j$, re-optimize over the paths still reachable from $L_j$:

$$
P^*_j = \arg\max_{P \in \mathcal{P}_j} Momentum(P)
$$

subject to the same constraints. Take $s_{j+1}$ as the first step of $P^*_j$. Observe $L_{j+1}$. Re-optimize.

The model is a policy, not a fixed plan.

---

## 3. Explanation

### 3.1 Values

$V$ has seven sub-components: four independence types plus enjoyable life experience ($e$), goal progress ($g$), and emotional safety ($\beta$).

Independence is decomposed into four because the sub-types can conflict. The canonical example is moving out: it raises $i_3$ (residential independence) while often reducing $i_2$ (financial independence). Mobility ($i_1$) is reliable transportation. Decision independence ($i_4$) is control over schedule, choices, and direction. The model does not prescribe an ordering for climbing them; different paths build different orders, and the momentum, viability, and recoverability filters distinguish good orderings from bad ones in any given case.

Net worth is not in $V$. It is captured by $\sigma$ and $d$ in $A$, with the desirability filter requiring $\sigma_s \geq d_s$.

### 3.2 Time

Time is more than the count of available hours. Two plans may leave the same number of free hours but one leaves Emma exhausted while the other leaves her with energy to build a life. Each of $p$, $b$, and $q$ is a viability floor. A plan that wins on $p$ but collapses $q$ is not viable.

A useful distinction: $p_{theoretical} \neq p_{actual}$. A plan can work on paper but fail if the remaining hours are tired, scattered, or emotionally overloaded.

### 3.3 Assets

$A$ contains everything financial: what comes in ($y$), what is held ($\sigma$), what is owed ($d$), what is set aside ($r$), what flows month to month ($c$), and what is projected to change ($\gamma$). Income lives here rather than as a separate top-level variable because cash flow, savings, debt, and income are deeply coupled. A plan cannot reason about cash flow without reasoning about income.

A plan becomes financially fragile when $c < 0$ or $r < r_{min}$. A plan that meets those floors but produces $y < y_{min}$ is viable without being desirable: it can survive without yet supporting the life Emma wants. $y_{min}$ is a desirability threshold, not a survival threshold.

A useful conjecture: residential independence becomes desirable only when income is high enough that housing, transportation, savings, and enjoyable life can all coexist without excessive pressure.

### 3.4 Education

Education here covers any path that builds skill or credential, formal or informal: school, certificate, apprenticeship, on-the-job learning, self-study, mentorship. The model treats them as one category because what matters is the resulting capability, not the venue.

Education is a lever on income. The chain $K \rightarrow A \rightarrow T \rightarrow V$ says it formally: $K$ lifts $y$ inside $A$, and that lift propagates to $c$, $\sigma$, and $r$. With more $A$, Emma can spend money to free time (a reliable car, less commute, fewer side jobs), which raises both the count and quality of $T$. With better $T$, every dimension of $V$ can move.

$K$'s sub-components are not inert. They connect to the rest of the model: $\tau$ (training time) reduces $p$ during the training period; $\mu$ (training money) subtracts from $\sigma$; $\rho$ (completion risk) discounts the expected post-training income gain.

$\delta$ (technology trajectory) reflects that the $K \rightarrow y$ relationship is not stable across time. People create new knowledge, knowledge produces new technology, and technology reshapes which education paths the market pays for. $\delta$ is signed: negative if exposed to displacement by emerging technology, near zero if tech-neutral over the relevant horizon, positive if amplified.

$\delta$ cannot be measured with confidence. The point is not to forecast precisely but to refuse the assumption that today's job market describes tomorrow's. A useful question for any candidate path: if the most plausible technology trajectory in this field plays out over $\tau$, does this education path still produce the income it does today? When the answer is no, $\rho$ should rise and the path should favor education that remains valuable as the field changes.

$\delta$ and $\gamma$ are independent. A field can decline while a particular practitioner sees positive $\gamma$ from accumulated experience, geographic advantage, or moving up the value chain. Conversely, $\gamma$ can be negative for personal reasons (a career change, a location move, a step back into training) while $\delta$ stays neutral. The model tracks them separately and combines them only in $\hat{y}$.

The horizon-projected income condition $\hat{y}(L_s, H) \geq y_{min}$ is what makes a stagnant $K$ in a declining field actually fail desirability. Even if $y_s \geq y_{min}$ today, projected $\hat{y}$ falls below the threshold within $H$ years if $\delta$ is sufficiently negative.

### 3.5 Health

Health is treated holistically: the full state of body, mind, behavior, emotion, and social connection. Each dimension is real, and a plan that wins on every other axis but ruins any of them is not a good plan.

$W$ has five sub-components. Each affects the others, but they are tracked separately because a plan can be strong in one and quietly destroying another. Long hours can sustain mental focus while degrading physical health and fitness. A meaningful job can lift emotional impact while crowding out relational quality.

$\phi$, $\psi$, and $\nu$ are floors in the viability filter. $\zeta$ and $\eta$ contribute to momentum but are not hard floors: a plan can briefly skip exercise or feel hard without becoming nonviable. They become floors implicitly when sustained low values destroy $\phi$ or $\psi$.

#### Physical and mental health

$\phi$ and $\psi$ are the slow underlying state of body and mind. They compound over years. A long commute eats sleep. A high-pressure job raises baseline anxiety. Skipped exercise and poor diet compound silently. A path that produces strong income but ruins $\phi$ or $\psi$ is a path that pays interest forever, often at the worst time.

#### Fitness

$\zeta$ is the practice, not the state: what Emma actually does day to day around movement and food. State ($\phi$) follows practice ($\zeta$) over time. A plan that leaves no time or energy for $\zeta$ will eventually erode $\phi$.

#### Emotional impact

Many decisions carry emotional weight beyond their financial weight. Emotions also run both ways. Meaningful work can produce pride, autonomy can produce relief, good company can produce energy, while a draining job pays in chronic stress, a wrong roommate pays in friction, and a stretched purchase pays in anxiety. These effects are real even when they do not appear on a balance sheet.

$\eta$ is signed: positive when a choice feels net good day to day, negative when it does not.

| Choice | Possible emotional impact |
|---|---|
| Stay at home | Ease and savings, against privacy loss, conflict, and feeling stuck |
| Move out | Sense of freedom and ownership, against financial pressure, isolation, and household labor |
| Long-hour job | Pride in earning and progress, against chronic stress, fatigue, and lost rest |
| Intensive training | Sense of growth and identity, against discipline cost, delayed gratification, and self-doubt |
| Car ownership | Mobility, freedom, pride of ownership, against maintenance anxiety and commute fatigue |

A choice becomes more attractive as $\eta(\text{this}) - \eta(\text{alternative})$ rises.

#### Relationships

$\nu$ measures the network of people Emma depends on, contributes to, and grows with: family, close friends, romantic partners, mentors, and community. People are not interchangeable, and a thin network is a real fragility. The same shock that someone with strong relationships can absorb can flatten someone who is alone.

Many plans look efficient on paper but quietly damage $\nu$. Long hours leave no time for friends. A move can sever a community before a new one forms. A high-pressure path can crowd out the patience and presence that close relationships require. Some damage is reversible. Some is not.

> No plan is a good plan if it ruins her body, her mind, or her closeness to the people who matter. Feeling bad for a stretch is survivable. Staying broken is not.

### 3.6 Viability and Desirability

Two filters separate plans by quality.

$L_{viable}$ contains life-states that survive: nonnegative cash flow, sufficient risk buffer, all three time dimensions above their floors, and $\phi$, $\psi$, $\nu$ above their floors. Failing any of these means the plan is nonviable, regardless of how attractive it looks elsewhere.

$L_{desirable}$ is a strict subset of $L_{viable}$. A desirable life-state additionally has each independence type meeting its minimum, named values ($e$, $g$, $\beta$) above their floors, current and projected income meeting $y_{min}$, positive net worth ($\sigma \geq d$), and fitness and emotional impact above their floors.

The distinction matters because a plan that feels exciting but creates a brittle life is filtered out as nonviable before any momentum comparison happens. And among viable plans, only desirable ones produce a life worth wanting.

### 3.7 Momentum

Momentum is a weighted sum across the scored scalar sub-components of the life-state. The weights are Emma-specific importances. Cash flow, risk buffer, stability, and the three health floors should likely receive higher weights because their collapse makes the whole life-state fragile.

$K$'s sub-components do not appear in the score directly because their effect already shows up in $y$, $\sigma$, $p$, and the horizon condition. $d$ enters negatively through cash flow and the net-worth condition.

Possible initial weights:

| Variable | Meaning | Suggested Weight |
|---|---|---:|
| $i_1$ | Mobility independence | 2 |
| $i_2$ | Financial independence | 3 |
| $i_3$ | Residential independence | 2 |
| $i_4$ | Decision independence | 2 |
| $e$ | Enjoyable life | 2 |
| $g$ | Goal progress | 3 |
| $\beta$ | Stability / emotional safety | 4 |
| $p$ | Productive time | 2 |
| $b$ | Buffer time | 2 |
| $q$ | Quality of time | 3 |
| $c$ | Cash flow | 4 |
| $\sigma$ | Savings | 3 |
| $r$ | Risk buffer | 4 |
| $y$ | Income | 3 |
| $\gamma$ | Income growth | 2 |
| $\phi$ | Physical health | 4 |
| $\psi$ | Mental health | 4 |
| $\zeta$ | Fitness | 2 |
| $\eta$ | Emotional impact | 3 |
| $\nu$ | Relational quality | 4 |

The best single scenario is the desirable scenario with the highest momentum. Selecting from $S_{desirable}$ rather than $S_{viable}$ aligns the single-scenario optimum with the path-level objective: both demand a life-state worth wanting, not merely one that survives.

### 3.8 Recoverability

Recoverability measures how easily Emma can reverse, repair, or step back from a decision if it turns out badly. State-level resilience is handled by the viability floors (a state above its floors can absorb shocks; a state at its floors cannot). Recoverability applies to decisions, not states.

$R(s_j) \in [0, 1]$ is the recoverability of decision $s_j$, computed from three dimensions. $\lambda_j$ is lock-in duration as a fraction of the planning horizon (a 12-month lease over a 5-year horizon has $\lambda \approx 0.2$). $\xi_j$ is exit cost as a fraction of savings (breaking a lease for two months' rent on $10,000 in savings has $\xi \approx 0.1$). $\Delta_j$ is state disruption: roughly the fraction of $L$'s sub-components materially changed by the decision.

Examples:

| Decision | $\lambda$ | $\xi$ | $\Delta$ | $R$ |
|---|---:|---:|---:|---:|
| Try a part-time job | 0.05 | 0.00 | 0.10 | 0.95 |
| Take a short course | 0.10 | 0.05 | 0.15 | 0.90 |
| Sign a 12-month lease (5-yr horizon) | 0.20 | 0.10 | 0.40 | 0.77 |
| Take on high-interest debt | 0.50 | 0.30 | 0.50 | 0.57 |
| Buy a car that drains $r$ | 0.30 | 0.50 | 0.40 | 0.60 |

A plan should satisfy $R(s_j) \geq R_{min}$ for each step $j$ where the decision is large enough that an error would meaningfully change the path.

> Prefer decisions Emma can step back from. Irreversible decisions need more evidence than reversible ones.

### 3.9 Paths

Emma's life is a sequence of life-states, not a single decision. A path $P = (s_0, s_1, \dots, s_n)$ is a sequence of scenarios over time, each producing its own life-state.

Example scenarios (illustrative, not prescriptive):

| Scenario | Description |
|---|---|
| $s_1$ | Stay with mom, buy no car, save aggressively |
| $s_2$ | Stay with mom, buy modest car, preserve savings buffer |
| $s_3$ | Move out, buy car |
| $s_4$ | Move out, no car |
| $s_5$ | Stay low-rent, buy reliable modest car, pursue income/credential path, move out later |
| $s_6$ | Work more hours immediately, save more money, delay car purchase |
| $s_7$ | Keep current work, add training or school, delay apartment independence |

A path's quality is judged by the lived experience across its stages, weighted by how long each stage lasts. The terminal life-state must be desirable. Every intermediate life-state must be viable. Every large decision must be recoverable enough.

Scoring across stages rather than only at the endpoint reflects that the journey is what Emma lives. A path that reaches the same destination through better intermediate states is a better path. Stage durations $d_j$ weight the sum so that long stretches of a given life-state count more than brief ones; a year of stress is worse than a month of stress at the same intensity.

### 3.10 Adjustability

The path is not chosen once. Emma applies the model at each stage, using current information to choose the next step, observes the result, and applies the model again from her new life-state. The model is a policy, not a fixed plan.

Two implications follow.

First, the recoverability constraint is what makes adjustability possible at all. A decision with low $R(s_j)$ removes future flexibility, regardless of whether the decision looked right at the time. Adjustability is the practical reason recoverability matters: it preserves the ability to redirect when new evidence arrives.

Second, adjustability and falsifiability are different responses to new information. Adjustability adjusts the path within the existing model. Falsifiability adjusts the model itself when reality stops fitting it. A path that needs frequent adjustment is not necessarily a sign that the model is wrong. A model that produces consistently poor adjustments is.

### 3.11 Falsifiability

The model is itself a conjecture and should be revisable. Conditions that should trigger revision:

- A scenario produces high momentum but feels emotionally wrong to Emma. (A value is missing or mis-weighted.)
- A scenario looks viable on paper but fails repeatedly in practice. (A constraint is uncosted.)
- Emma's stated values shift in a stable, considered way. ($V$ needs updating.)
- A variable in $L$ turns out to be redundant, or a needed variable is missing.
- The weights in $Momentum$ produce rankings that contradict considered judgment across many scenarios.
- The horizon-projected income $\hat{y}$ produces predictions that are repeatedly wrong by large margins. (The functional form, or $\delta$ or $\gamma$ estimates, may need revision.)

> The model is a tool for thinking, not a verdict. When it stops fitting reality, change the model.

---

## Appendix: Questions to Ask Emma and Myself

This section is for discovering values, desires, constraints, and hidden tradeoffs. The goal is not to interrogate Emma. The goal is to help her sort through what she wants and what each possible path would actually cost or create.

### Questions for Emma: Independence

1. When you say you want independence, what does that mean to you right now?
2. Which feels most important: having your own car, having your own place, making your own decisions, or not needing help with money?
3. What parts of your current situation make you feel least independent?
4. What would make you feel more like an adult?
5. What kind of independence are you willing to wait for if waiting makes it more stable?
6. What kind of independence feels urgent?
7. Do you want independence from something, independence toward something, or both?

### Questions for Emma: Car

1. What would having a car unlock for you?
2. Is the car mainly about work, friends, freedom, privacy, school, dating, or not relying on others?
3. How often would you realistically use the car each week?
4. What would life look like without a car for the next 6–12 months?
5. What is the maximum amount of your savings you would feel comfortable spending on a car?
6. How much emergency money do you want left after buying a car?
7. Would you rather have a nicer car and less savings, or a modest car and more safety?
8. How would you feel if the car needed a $1,000 repair three months after buying it?

### Questions for Emma: Money

1. How much money do you want to keep untouched as an emergency fund?
2. How much monthly pressure feels okay, and how much feels scary?
3. What bills do you expect to pay yourself?
4. What bills do you hope someone else helps with for now?
5. How important is it to keep saving money every month?
6. How would you feel if your savings dropped from $10,000 to $3,000?
7. How would you feel if your savings dropped near zero but you had a car?
8. What does “financially safe” feel like to you?

### Questions for Emma: Work and Income

1. How do you feel about your current job?
2. Do you want to work more hours, fewer hours, or about the same?
3. What parts of work give you energy?
4. What parts of work drain you?
5. How much income would make you feel more secure?
6. Would you rather work more now, train for better income, or do some of both?
7. What kind of work do you not want to be doing two years from now?
8. What kind of work could you imagine feeling proud of?

### Questions for Emma: Education

1. Are there any jobs, careers, or lifestyles that seem interesting to you?
2. Would you rather learn through school, hands-on work, online training, apprenticeship, or a short certificate?
3. How long are you willing to train for a better income path?
4. Which of these paths still feels valuable if the work itself changes a lot in the next ten years?
5. What subjects or activities come naturally to you?
6. What do people often tell you you are good at?
7. What kind of work environment would you hate?
8. What kind of work environment would fit your personality?
9. Would you rather optimize for money, flexibility, meaningful work, stability, or low stress?

### Questions for Emma: Moving Out

1. What feels hard about staying where you are?
2. What do you imagine would feel better if you moved out?
3. What would you be afraid of if you moved out?
4. What would make moving out feel safe rather than stressful?
5. Would you rather move out sooner with more financial pressure or later with more stability?
6. What would be a good reason to delay moving out?
7. What would be a good reason to move out sooner?
8. If moving out meant working so much that you had less time for friends, goals, or rest, would it still feel worth it?

### Questions for Emma: Enjoyable Life

1. What does a good week look like to you?
2. How much time do you want for friends?
3. How much time do you want for rest and doing nothing?
4. What activities make life feel worth living?
5. What would make adult life feel exciting rather than heavy?
6. What are you afraid of losing as you take on more responsibility?
7. What do you want to protect in your life even while growing up?

### Questions for Emma: Health

1. How do you feel in your body most days?
2. How well do you usually sleep, and what tends to mess it up?
3. What physical activities make you feel good, and which ones do you avoid?
4. What does a sustainable movement routine look like for you (not punishing, not optional)?
5. How do you want to think about food: pleasure, fuel, both, something else?
6. How do you usually handle stress? What works and what does not?
7. What habits would you want to keep no matter what job or living situation you end up in?
8. What would be a warning sign that a job, schedule, or living situation was wearing on you?
9. Is there anything about your mental health you want to be deliberate about as life changes?
10. When you imagine yourself at 25, what do you hope your health looks like?

### Questions for Emma: Relationships

1. Who are the people you most want to stay close to over the next few years?
2. Which relationships refill you, and which ones drain you?
3. What kind of friendships do you want to build now that high school is ending?
4. What role do you want family to play in your adult life?
5. Are there relationships you want to grow into (mentors, romantic, community) that you do not yet have?
6. How much time per week do you want to protect for the people who matter to you?
7. What would be a warning sign that a path was starting to isolate you?
8. If a job or move offered more money but cost you closeness with people you love, what would feel like a fair trade?

### Questions for Emma: Future Self

1. What would make you proud one year from now?
2. What would make you feel stuck one year from now?
3. What do you hope your life looks like at 19?
4. What do you hope your money situation looks like at 19?
5. What do you hope your work situation looks like at 19?
6. What do you hope your relationships and social life look like at 19?
7. What is one thing you would like future-you to thank current-you for?

### Questions for Myself: Dad’s Role

1. Am I helping her clarify her desires, or am I subtly replacing them with mine?
2. Am I protecting her from a bad decision, or protecting myself from the discomfort of watching her struggle?
3. Have I separated my values from her values?
4. Am I giving her enough agency in the process?
5. Am I making the complexity simpler for her, or accidentally making her feel more overwhelmed?
6. Am I presenting conclusions too early before she feels heard?
7. Am I treating her desire for independence as legitimate, even when I think her current strategy may be flawed?
8. Am I helping her become capable, or merely helping her avoid mistakes?

### Questions for Myself: Model Quality

1. What important variables are missing from the model?
2. Which assumptions are guesses that need better information?
3. Which costs are recurring, and which are one-time?
4. Which risks are likely, and which are catastrophic but rare?
5. Which scenario creates the most positive momentum?
6. Which scenario feels good emotionally but fails structurally?
7. Which scenario is financially safe but emotionally unacceptable?
8. Which path gives her the most real independence over 12–24 months?
9. What would make moving out sooner a good plan?
10. What would make buying a car now a good plan?
11. What would make delaying both the car and apartment a good plan?

### Questions for Myself: Communication

1. What is the next small conversation, not the whole mountain?
2. What does Emma need to feel before she can hear the math?
3. What part of the plan should be shared now, and what part should remain in my working model?
4. How can I communicate that I am on her side?
5. How can I make the model feel empowering rather than controlling?
6. How can I show respect for the fact that she saved $10,000?
7. How can I help her feel proud before discussing tradeoffs?
8. How can I present criticism of bad plans without making her feel criticized?
