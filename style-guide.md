# Style Guide for Essays

This guide defines how AI should help me write, refine, and criticize my essays: turning exploratory research into clear writing without losing the originality of the ideas. The model essay is [Thinking About Thinking](essays/thinking-about-thinking.md). When in doubt, match its voice, length, and structure.

## Purpose and Method

The heart of my essays is explanation. I want creative explanations in the spirit of David Deutsch’s *The Beginning of Infinity*: ideas that come from pursuing real questions and have genuine explanatory power. An essay should explain what is going on, why it matters, and how the ideas connect. It should not merely summarize information or restate common views.

My approach is Popperian and fallibilist. Explanations are conjectured, then criticized. Evidence tests and refines explanations; it doesn’t make them certain. So my essays should be bold enough to make real conjectures, honest enough to name uncertainty, and open to being improved or replaced by better explanations.

## Voice

I aim for the clarity of Thomas Sowell: understandable without being simplistic.

- **Plain language.** Use the simplest word that is accurate. Prefer short-to-moderate sentences. Say a thing once, well.
- **Clear, serious, exploratory, and intellectually alive.** It should sound like someone thinking carefully in public, not an institution protecting itself with sterile language.
- **Grounded and curious**, not ornamental or performative. Bold, but not dogmatic. Speculation is fine when it is labeled as speculation.
- **Personal where it helps.** A true story or concrete moment is often the best way into an idea.

## Brevity

Every sentence should earn its place. Keep what builds understanding or engagement; cut the rest.

Cut:

- restating the thesis in several forms, or recapping sections the reader just read
- slogan stacks, bolded whole sentences, and pull quotes that repeat the text
- throat-clearing, filler hedges, and commentary about the essay itself
- tangents that don’t serve the argument (move them to an appendix if they’re worth keeping)
- cute or slang headings; use plain, descriptive ones

Never cut a step the argument depends on. Brevity serves clarity; it never replaces it.

## Structure

Essays usually follow this shape:

- **Introduction.** Opens the essay with the motivating question, why it matters, and where we’re going. Open with a concrete story or example when there is one. No separate “Abstract”; it reads as pretentious and detached.
- **Table of Contents.** Numbered sections, linked by anchor.
- **Numbered sections.** The arguments and explanations: observations, interpretations, thought experiments, and reasoning.
- **Conclusion.** Returns to the opening question and says what we found. It doesn’t re-list every point.
- **Further Reading and Appendices.** References, definitions, glossaries, technical detail, and side arguments that would distract from the main flow.

No subtitle or byline under the title, and no horizontal rules between sections. The frontmatter carries the metadata.

Each paragraph should do one main job: define a term, draw a distinction, advance an argument, bridge between ideas, or give an example. Split a paragraph when its purpose shifts. Connect sections with transitions that show why the next idea follows from the last, so the essay reads as a guided path, not a pile of observations.

## Terms

Define important terms inline, at the moment the reader needs them, not in a block up front. Front-loaded definitions force the reader to memorize terms without context.

Once a term is defined, use it with that meaning throughout. Don’t swap in near-synonyms for terms that carry theoretical weight (*agency*, *identity*, *self*, *model*, *explanation*, *consciousness*, and the like), because later sections build on earlier distinctions. Variation in phrasing is fine where no concept is at stake. Precision beats variety wherever a term does real work.

AI should flag undefined terms, inconsistent terminology, unannounced shifts in meaning, and restatements that blur a distinction.

## Criticism

AI should criticize the reasoning, not just polish the prose. A common weakness of mine is making a claim and assuming the reader will bridge the gap to the next one. Call that out.

Look for:

- unsupported claims, leaps in reasoning, and unstated assumptions
- missing transitions and places where an example, analogy, or bridging sentence would help
- contradictions, non sequiturs, equivocations, and overgeneralizations
- conclusions that don’t follow, and alternative explanations that weren’t considered
- factual errors and misattributions
- overreliance on background knowledge the reader may not have

The goal isn’t to persuade everyone. Distinguish:

- **preventable confusion**: the writing failed to bridge the reader to the point
- **fundamental disagreement**: the reader rejects the premises, values, or framework

Focus on fixing the first. Note the second if useful, but don’t treat worldview differences as writing flaws.

## Anticipating Objections

Watch for strong claims that arrive before the reader has reason to accept them, especially with words like *must*, *therefore*, *necessarily*, *cannot*, or *inevitably*. A thoughtful reader will ask “why?” or “what rules out the alternative?” and may stop reading before the support arrives.

When that happens:

- add a brief clause that gives an initial reason
- soften the claim until the argument has earned it
- signal that the justification is coming
- reorder so the support comes before the conclusion

Don’t write defensively at every turn. Just keep predictable, early objections from losing the reader.

## How to Deliver Criticism

Be specific and actionable. For each issue, identify:

1. the exact claim, sentence, or section
2. the kind of problem (missing support, unclear transition, logical gap, ambiguity, unstated assumption, inconsistent terminology)
3. why a thoughtful reader may stumble there
4. whether the claim is too strong or too early at that point
5. a way to strengthen it without diluting the idea

Criticism should strengthen bold ideas, not flatten them into timid ones.

## Priorities

When there are tradeoffs, prioritize:

1. explanatory clarity
2. conceptual accuracy
3. originality of insight
4. readability and flow
5. elegance of prose

Never sacrifice substance for polish.

Preserve: original explanations, clear structure, explicit and consistent terms, smooth transitions, concrete examples, openness to criticism, and logical coherence.

Avoid: empty academic language, jargon, false certainty, summarizing instead of explaining, weak transitions, abstractions with no illustration, polish that strips out curiosity or risk, softening strong ideas just to seem safe, and necessity claims made before they are supported.

## Mechanics

- Link other essays with relative paths (`other-essay.md`), never site paths like `/writings/...`. The site rewrites `.md` links, and relative links also work on GitHub.
- Essay images live in `website/static/img/essays/` and are linked as `../website/static/img/essays/<file>`.
- Em dashes are fine; don’t lean on them.

## Working Standard

I don’t need to persuade everyone. I want to prevent readers from stumbling where clearer writing could have helped. The goal is essays that are bold, clear, logically serious, brief, and open to improvement.
