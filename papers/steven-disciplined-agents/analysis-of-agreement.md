# Analysis of Agreement

## The paper's central idea

Build AI agents according to what customers will actually delegate and what the complete production system can reliably deliver, rather than according to model capability alone.

The paper distinguishes two forms of reliability:

- **Required reliability:** How dependable an agent must be before customers will delegate a task.
- **Attainable reliability:** How dependable the full workflow is in production, including interpretation, execution, verification, and recovery.

Both judgments change through experience. The proposed method is to begin with the smallest useful promise, constrain the agent's authority, design recovery before expanding autonomy, and use bounded deployments to learn. The Reliability Matrix assigns tasks to one of four postures: deploy now, constrained deployment, assist and improve, or human in the loop.

## Areas of agreement

The paper is broadly aligned with my essays in several important ways:

- Model capability is not the same as dependable production. Workflow design, verification, recovery, and organizational learning remain important bottlenecks.
- AI can compress execution faster than organizations can learn. This is consistent with *The Moving Line* and *The Outcome Economy*.
- The surrounding architecture matters more than model accuracy alone, which fits the substrate-versus-architecture distinction in *Can LLMs Create Knowledge?*
- The framework is provisional and evidence-driven. Its cycle of proposing, testing, observing, and revising fits Cyclic Rationality and the Explorer posture described in *Living Above the Models*.
- Narrow deployments, human escalation, reversibility, and explicit mitigation are valuable tools for learning without exposing people to unlimited harm.
