# Emma: Build Independence

The implementation of this model is being built in [pathwise](https://github.com/buwilliams/pathwise). This document defines the model. It does not draw a conclusion about what Emma should do.

## 1. Problem Domain: Considerations and Symbol Definitions

Emma is graduating high school, turning 18, and beginning the transition into adult life. She wants more independence and is considering major life decisions such as buying a car, working more, moving out, and choosing a path toward higher income.

These decisions are emotionally heavy because they are not isolated. A car affects money, mobility, job options, insurance, maintenance, and savings. Moving out affects independence, rent burden, work hours, stress, future options, and available time. Pursuing education affects future income, but requires time, money, discipline, and delayed gratification.

The purpose of this model is not to prove the one correct answer. The purpose is to make the important factors explicit so we can use conjecture and criticism to evaluate possible paths.

The guiding idea is:

> A good plan should generate positive momentum across her whole life-state (values, time, assets, education, and health) without creating excessive fragility.

### Core Life-State Model

Let:

$$
L = \{V, T, A, K, W\}
$$

Where:

| Symbol | Meaning |
|---|---|
| $L$ | Life-state |
| $V$ | Values |
| $T$ | Time |
| $A$ | Assets (including income) |
| $K$ | Education |
| $W$ | Health (physical, mental, fitness, emotional, relational) |

A life-state is not just whether Emma can afford something today. A life-state describes whether her overall situation is becoming stronger, weaker, freer, or more fragile.

Each top-level variable is a vector of scalar sub-components defined in the sections that follow. All conditions (viability, desirability, fragility, momentum) are stated over those scalar sub-components.

Two further symbols, fragility ($F$) and recoverability ($R$), are introduced later but sit outside $L$ on purpose. $F$ is a derived measure of how exposed a life-state is to shocks. $R$ is a property of a particular decision, not of the state itself.

All thresholds (e.g., $r_{min}$, $\phi_{min}$, $y_{min}$, $F_{max}$, $R_{min}$) are Emma-specific parameters set by judgment and revisable as her values and circumstances change.

### Values

Let:

$$
V = \{i_1, i_2, i_3, i_4, e, g, s\}
$$

Where:

| Symbol | Meaning |
|---|---|
| $i_1$ | Mobility independence: reliable transportation; ability to get to work, school, friends, appointments, and opportunities |
| $i_2$ | Financial independence: positive cash flow, preserved savings, and reduced dependence on others for money |
| $i_3$ | Residential independence: living outside a parent's home |
| $i_4$ | Decision independence: control over schedule, choices, goals, and personal life |
| $e$ | Enjoyable life experience |
| $g$ | Goal progress |
| $s$ | Stability / emotional safety |

Independence is decomposed into four sub-types because they can conflict with one another. A key conjecture:

> Moving out increases residential independence, but may reduce financial independence if rent consumes too much income.

In notation:

$$
i_3 \uparrow \Rightarrow i_2 \downarrow
$$

This means moving out may feel like independence while reducing total practical freedom.

Net worth is not listed in $V$ because it is captured by the asset components $\sigma$ (savings) and $d$ (debt). The desirability filter requires $\sigma_s \geq d_s$.

### Time

Let:

$$
T = \{p, b, q\}
$$

Where:

| Symbol | Meaning |
|---|---|
| $p$ | Productive hours available per week |
| $b$ | Buffer hours for unexpected events, rest, family, errands, and chaos |
| $q$ | Quality of time / energy level |

Time is not only the number of hours available. Two plans may leave the same number of free hours, but one may leave Emma exhausted while the other leaves her with enough energy to build her future.

A useful distinction:

$$
p_{theoretical} \neq p_{actual}
$$

A plan can work on paper but fail if the remaining hours are low-quality, tired, scattered, or emotionally overloaded.

### Assets

Let:

$$
A = \{c, \sigma, d, r, y, \gamma\}
$$

Where:

| Symbol | Meaning |
|---|---|
| $c$ | Monthly cash flow |
| $\sigma$ | Savings |
| $d$ | Debt / liabilities |
| $r$ | Risk buffer / emergency fund |
| $y$ | Income |
| $\gamma$ | Likely income growth over time |

Assets are everything financial: what comes in, what goes out, what is held, what is owed, and what is likely to change. Income lives here rather than as a separate top-level variable because cash flow, savings, debt, and income are deeply coupled. A plan cannot reason about cash flow without reasoning about income.

Emma currently has meaningful savings. That gives her options. The goal is not merely to spend savings on adult things, but to preserve enough savings that she remains resilient.

A plan becomes financially fragile when:

$$
c < 0 \quad \text{or} \quad r < r_{min}
$$

A plan that meets those floors but produces $y < y_{min}$ is viable without being desirable: it can survive without yet supporting the life Emma wants. $y_{min}$ is a desirability threshold, not a survival threshold.

A useful conjecture:

> Residential independence becomes desirable only when income is high enough that housing, transportation, savings, and enjoyable life can all coexist without excessive pressure.

### Education

Education here covers any path that builds skill or credential, formal or informal: school, certificate, apprenticeship, on-the-job learning, self-study, mentorship. The model treats them as a single category because what matters is the resulting capability, not the venue.

Let:

$$
K = \{k, \tau, \mu, \rho, \delta\}
$$

Where:

| Symbol | Meaning |
|---|---|
| $k$ | A specific education path (formal or informal) |
| $\tau$ | Time required to acquire it |
| $\mu$ | Money required to acquire it |
| $\rho$ | Risk / uncertainty of completion or payoff |
| $\delta$ | Technology trajectory: expected change in the skill's market value as technology progresses |

Education matters because it can transform income, which then feeds the rest of the asset base.

$$
K \rightarrow y \rightarrow A \rightarrow T \rightarrow V
$$

This means education is not merely another goal. It is a lever that changes what life-states are possible.

The $K \rightarrow y$ relationship is not stable across time. People create new knowledge, knowledge produces new technology, and technology reshapes which skills the market actually pays for. A credential that pays well today may pay more, less, or nothing over the years it takes to build a life around it. $\delta$ captures that exposure.

$\delta$ has a sign:

- $\delta < 0$ if the skill is exposed to displacement by emerging technology
- $\delta \approx 0$ if the skill is roughly tech-neutral over the relevant horizon
- $\delta > 0$ if the skill is amplified by emerging technology

$\delta$ cannot be measured with confidence. Knowledge growth is not predictable in detail, so any estimate is a conjecture open to criticism. The point is not to forecast precisely. The point is to refuse the default assumption that today's job market describes tomorrow's.

A useful question for any candidate path:

> If the most plausible technology trajectory in this field plays out over $\tau$, does this skill still produce the income it does today?

When the answer is no, $\rho$ should rise and the path should favor skills or transitions that remain valuable as the field changes.

$K$'s sub-components are not directly scored in momentum and do not appear in the viability or desirability filters because $K$ acts on $y$. A plan with no $K$ progress over a long horizon will eventually fail the income or fragility floors as life and the labor market move on.

### Health

Health is treated holistically: the full state of body, mind, behavior, emotion, and social connection. Each dimension is real, and a plan that wins on every other axis but ruins any of them is not a good plan.

Let:

$$
W = \{\phi, \psi, \zeta, \eta, \nu\}
$$

Where:

| Symbol | Meaning |
|---|---|
| $\phi$ | Physical health: condition, sleep, illness, energy reserves |
| $\psi$ | Mental health: anxiety, depression, cognitive load, capacity for stress |
| $\zeta$ | Fitness: practiced behaviors around diet and exercise |
| $\eta$ | Net emotional impact (signed): how a choice tends to feel day to day |
| $\nu$ | Relational quality: depth and resilience of family, friend, romantic, mentor, and community ties |

Each dimension affects the others, but they are tracked separately because a plan can be strong in one and quietly destroying another. Long hours can sustain mental focus while degrading physical health and fitness. A meaningful job can lift emotional impact while crowding out relational quality.

#### Physical and Mental ($\phi$, $\psi$)

Physical and mental health are the slow underlying state of body and mind. They compound over years. A long commute eats sleep. A high-pressure job raises baseline anxiety. Skipped exercise and poor diet compound silently. A path that produces strong income but ruins $\phi$ or $\psi$ is a path that pays interest forever, often at the worst time.

#### Fitness ($\zeta$)

Fitness is the practice, not the state: what Emma actually does day to day around movement and food. State ($\phi$) follows practice ($\zeta$) over time. A plan that leaves no time or energy for $\zeta$ will eventually erode $\phi$.

#### Emotional Impact ($\eta$)

Many decisions carry emotional weight beyond their financial weight. Emotions also run both ways: meaningful work can produce pride, autonomy can produce relief, good company can produce energy, while a draining job pays in chronic stress, a wrong roommate pays in friction, and a stretched purchase pays in anxiety. These effects are real even when they do not appear on a balance sheet.

$\eta$ is signed: positive when a choice feels net good day to day, negative when it does not.

| Choice | Possible emotional impact |
|---|---|
| Stay at home | Ease and savings, against privacy loss, conflict, and feeling stuck |
| Move out | Sense of freedom and ownership, against financial pressure, isolation, and household labor |
| Long-hour job | Pride in earning and progress, against chronic stress, fatigue, and lost rest |
| Intensive training | Sense of growth and identity, against discipline cost, delayed gratification, and self-doubt |
| Car ownership | Mobility, freedom, pride of ownership, against maintenance anxiety and commute fatigue |

In notation:

$$
\text{a choice becomes more attractive as } \eta(\text{this choice}) - \eta(\text{alternative}) \text{ rises}
$$

#### Relational ($\nu$)

Relationships mean the network of people Emma depends on, contributes to, and grows with: family, close friends, romantic partners, mentors, and community. People are not interchangeable, and a thin network is a real fragility. The same shock that someone with strong relationships can absorb can flatten someone who is alone.

Many plans look efficient on paper but quietly damage $\nu$. Long hours leave no time for friends. A move can sever a community before a new one forms. A high-pressure path can crowd out the patience and presence that close relationships require. Some damage is reversible. Some is not.

#### Floors

A plan should satisfy:

$$
\phi(L_j) \geq \phi_{min}, \quad \psi(L_j) \geq \psi_{min}, \quad \nu(L_j) \geq \nu_{min}
$$

for each stage of the plan. $\zeta$ and $\eta$ contribute to momentum but are not hard floors: a plan can briefly skip exercise or feel hard without becoming nonviable. They become floors implicitly when sustained low values destroy $\phi$ or $\psi$.

Plain English:

> No plan is a good plan if it ruins her body, her mind, or her closeness to the people who matter. Feeling bad for a stretch is survivable; staying broken is not.

### Desirable Life-State

A life-state is desirable when each of its scalar sub-components meets a minimum acceptable threshold and net worth is positive:

$$
L_{desirable} = \{L_s : \text{all conditions below hold}\}
$$

Where the conditions are:

- $i_{1,s} \geq i_{1,min}, \; i_{2,s} \geq i_{2,min}, \; i_{3,s} \geq i_{3,min}, \; i_{4,s} \geq i_{4,min}$
- $e_s \geq e_{min}, \; g_s \geq g_{min}, \; s_s \geq s_{min}$
- $p_s \geq p_{min}, \; q_s \geq q_{min}$
- $c_s \geq 0, \; r_s \geq r_{min}, \; y_s \geq y_{min}, \; \sigma_s \geq d_s, \; \gamma_s \geq 0$
- $\phi_s \geq \phi_{min}, \; \psi_s \geq \psi_{min}, \; \zeta_s \geq \zeta_{min}, \; \eta_s \geq \eta_{min}, \; \nu_s \geq \nu_{min}$

Plain English:

> A desirable life-state is one where every independence type, every named value, productive time and its quality, the financial state (with positive net worth), and every dimension of health all meet acceptable thresholds.

A key distinction:

$$
L_{possible} \neq L_{desirable}
$$

A life-state can be possible without being desirable.

Example:

Emma may be able to move out if she works enough hours, but if the plan leaves her broke, tired, stressed, unable to save, and unable to invest in education, then it may be possible but not desirable.

### Positive Momentum

The target is not merely independence today. The target is positive momentum.

For a life-state $L$, momentum is a weighted sum across the scalar sub-components of $L$:

$$
Momentum(L) = \sum_{x \in components(L)} w_x \cdot x
$$

Where $components(L)$ is the union of the scored scalar sub-components of $V$, $T$, $A$, and $W$, and the weights $w_x$ are Emma-specific importances.

Some sub-components are tracked but not directly scored:

- $K$'s sub-components ($k, \tau, \mu, \rho, \delta$) act through $y$ rather than as direct scores. A path's education effort is reflected in the income it produces.
- $b$ (buffer time) feeds the realism of $p$ and $q$ and does not score directly.
- $d$ (debt) enters momentum negatively through cash flow and the net-worth condition rather than as a direct score.

A good plan should tend toward:

$$
i_1 \uparrow, \; i_2 \uparrow, \; i_3 \uparrow, \; i_4 \uparrow, \; e \uparrow, \; g \uparrow, \; s \uparrow, \; q \uparrow,
$$

$$
c \uparrow, \; \sigma \uparrow, \; r \uparrow, \; y \uparrow, \; \gamma \uparrow,
$$

$$
\phi \uparrow, \; \psi \uparrow, \; \zeta \uparrow, \; \eta \uparrow, \; \nu \uparrow
$$

while preserving the floors of the viability filter.

Plain English:

> A good plan should improve every independence type, every value, the quality of productive time, every dimension of the financial state, and every dimension of health, while keeping the floors of cash flow, risk buffer, productive time, physical health, mental health, and relational quality intact.

### Fragility

Let:

$$
F = \text{fragility}
$$

Fragility increases when she has too little money, too little time, too much debt, too much pressure, too little room for mistakes, declining physical or mental health, weakening relationships, or income that depends on a skill technology is converging on.

A plan should satisfy:

$$
F(L_j) \leq F_{max}
$$

for each stage of the plan.

Plain English:

> No step should make life so fragile that a small mistake, emergency, repair, illness, or job disruption causes the whole plan to fail.

### Recoverability

Let:

$$
R = \text{recoverability}
$$

Recoverability measures how easily Emma can reverse, repair, or step back from a decision if it turns out badly. Trying a part-time job, taking a short course, or getting insurance quotes before buying are high-recoverability decisions. Signing a year lease she can barely afford, taking on high-interest debt, or buying a car that drains her emergency fund are low-recoverability decisions.

Recoverability is distinct from fragility. Fragility describes how exposed a life-state is to shocks. Recoverability describes how easily a specific decision can be undone if it turns out wrong. A decision can be recoverable even when the resulting life-state is fragile (a short lease in a city she can leave) and irrecoverable even when the life-state looks safe (a long contract that locks in obligations).

A plan should satisfy:

$$
R(s_j) \geq R_{min}
$$

for each step $j$ where the decision is large enough that an error would meaningfully change the path.

Plain English:

> Prefer decisions Emma can step back from. Irreversible decisions need more evidence than reversible ones.

---

## 2. Scenario Modeling and Path Evaluation

This section treats life choices as candidate scenarios and sequences of scenarios. The goal is to compare paths, not merely isolated decisions.

### Candidate Scenarios

Let:

$$
S = \{s_1, s_2, s_3, \dots, s_k\}
$$

Where each $s$ is a possible scenario or life strategy.

Examples:

| Scenario | Description |
|---|---|
| $s_1$ | Stay with mom, buy no car, save aggressively |
| $s_2$ | Stay with mom, buy modest car, preserve savings buffer |
| $s_3$ | Move out, buy car |
| $s_4$ | Move out, no car |
| $s_5$ | Stay low-rent, buy reliable modest car, pursue income/credential path, move out later |
| $s_6$ | Work more hours immediately, save more money, delay car purchase |
| $s_7$ | Keep current work, add training or school, delay apartment independence |

Each scenario produces an outcome:

$$
O(s) = L_s
$$

Where $L_s$ is the resulting life-state, with sub-components $V_s, T_s, A_s, K_s, W_s$ each unpacked into their scalar parts.

Plain English:

> Each possible plan creates a different life-state.

### Viability Filter

Before ranking scenarios by preference, first remove scenarios that are too fragile.

Define:

$$
S_{viable} = \{s \in S : c_s \geq 0 \land r_s \geq r_{min} \land p_s \geq p_{min} \land \phi_s \geq \phi_{min} \land \psi_s \geq \psi_{min} \land \nu_s \geq \nu_{min}\}
$$

Plain English:

> The viable scenarios are those where cash flow is nonnegative, risk buffer is high enough, productive time is high enough, and physical health, mental health, and relational quality are at least minimally preserved.

This prevents us from being fooled by a plan that feels exciting but creates a brittle life. A plan that wins on money and time while quietly destroying health or relationships is not viable, no matter how good its momentum score would otherwise look.

### Desirability Filter

A viable scenario is not automatically desirable. It may be survivable but not good.

Define:

$$
S_{desirable} = \{s \in S_{viable} : L_s \in L_{desirable}\}
$$

Plain English:

> Desirable scenarios are viable scenarios that also create a life-state worth wanting.

This distinction matters:

$$
S_{desirable} \subseteq S_{viable} \subseteq S
$$

Meaning:

> Desirable scenarios are a subset of viable scenarios, and viable scenarios are a subset of all possible scenarios.

### Momentum Score

Once nonviable plans are removed, we can compare the remaining plans by momentum. For a scenario $s$ with resulting life-state $L_s$:

$$
Momentum(s) = Momentum(L_s) = \sum_{x \in components(L_s)} w_x \cdot x_s
$$

Possible initial weights:

| Variable | Meaning | Suggested Weight |
|---|---|---:|
| $i_1$ | Mobility independence | 2 |
| $i_2$ | Financial independence | 3 |
| $i_3$ | Residential independence | 2 |
| $i_4$ | Decision independence | 2 |
| $e$ | Enjoyable life | 2 |
| $g$ | Goal progress | 3 |
| $s$ | Stability / emotional safety | 4 |
| $p$ | Productive time | 2 |
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

Cash flow, risk buffer, stability, physical health, mental health, and relational quality should likely receive high weights because when they collapse, the whole life-state becomes fragile.

The best scenario is:

$$
s^* = \arg\max_{s \in S_{viable}} Momentum(s)
$$

Plain English:

> The best scenario is the viable scenario with the highest positive momentum.

### Path-Based Evaluation

Emma's life should not be modeled as one giant decision. It should be modeled as a path.

Let:

$$
P = (s_0, s_1, s_2, \dots, s_n)
$$

Where $P$ is a sequence of scenarios or stages.

Example path:

$$
P_1 = \text{stay low-rent} \rightarrow \text{buy modest car} \rightarrow \text{increase income} \rightarrow \text{move out later}
$$

The goal is:

$$
L_0 \rightarrow L_1 \rightarrow L_2 \rightarrow \dots \rightarrow L^*
$$

Where:

$$
L^* \in L_{desirable}
$$

Plain English:

> The best plan is the sequence of moves that gets Emma from her current life-state to a desirable life-state without creating too much fragility along the way.

### Path Objective

Let:

$$
\mathcal{P} = \{P_1, P_2, P_3, \dots, P_m\}
$$

Where $\mathcal{P}$ is the set of possible paths.

Define path momentum as the momentum of the terminal life-state:

$$
Momentum(P) = Momentum(L_n)
$$

The path is judged by where it ends. The per-step constraints below ensure the journey itself is acceptable. (An alternative scoring such as a weighted sum across stages would reward time spent in good intermediate states; this model uses the simpler endpoint score and lets the constraints carry the journey.)

Then:

$$
P^* = \arg\max_{P \in \mathcal{P}} Momentum(P)
$$

Subject to:

$$
L_n \in L_{desirable}
$$

and:

$$
F(L_j) \leq F_{max} \quad \text{for each step } j
$$

and:

$$
R(s_j) \geq R_{min} \quad \text{for each large decision } j
$$

Plain English:

> The best path is the one that lands in the most desirable reachable life-state, while keeping fragility below the danger threshold at each step and ensuring that large decisions remain recoverable.

### Independence Dimensions

Independence is multi-dimensional, not a ladder.

$$
I = \{i_1, i_2, i_3, i_4\}
$$

Where:

| Stage | Independence Type | Practical Meaning |
|---|---|---|
| $i_1$ | Mobility independence | Reliable transportation |
| $i_2$ | Financial independence | Positive cash flow and savings |
| $i_3$ | Residential independence | Living outside a parent's home |
| $i_4$ | Decision independence | Control over life choices and direction |

Gains in one dimension can come at the cost of another. The model does not prescribe an ordering. Different paths may build the dimensions in different orders, and the momentum, viability, and recoverability filters are what distinguish good orderings from bad ones in any given case.

### Model-Level Falsifiability

The model itself is a conjecture and should be revisable. Conditions that should trigger revision:

- A scenario produces high momentum but feels emotionally wrong to Emma. (A value is missing or mis-weighted.)
- A scenario looks viable on paper but fails repeatedly in practice. (A constraint is uncosted.)
- Emma's stated values shift in a stable, considered way. ($V$ needs updating.)
- A variable in $L$ turns out to be redundant, or a needed variable is missing.
- The weights in $Momentum(s)$ produce rankings that contradict considered judgment across many scenarios.

Plain English:

> The model is a tool for thinking, not a verdict. When it stops fitting reality, change the model.

---

## 3. Questions to Ask Emma and Myself

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
