# rsille skills

Coding agent skills for [rsille](https://github.com/nidhoggfgg/rsille), a Rust
framework for terminal applications and graphics.

## Install

Run from your application directory:

```bash
npx skills add nidhoggfgg/rsille.skills --skill rsille
```

Choose your coding agent when prompted, or add `-g` to install across projects.
You can also copy the complete `rsille/` directory into your agent's skills directory.

## Included skill

[rsille](rsille/SKILL.md) targets rsille 3.0.3 and covers widgets, layouts,
background tasks, input handling, Canvas graphics, animation, and headless tests.
It includes a [component catalog](rsille/references/components.md) and a
[standalone application starter](rsille/assets/starter).

Example prompt:

> Use the rsille skill to build a terminal task manager with a filter input,
> keyboard navigation, and headless tests.

## Develop from the framework repository

The framework repository includes this repository as its `skills/` Git submodule:

```bash
git submodule update --init skills
```

Commit and push skill changes in this repository first, then commit the updated
`skills` submodule reference in the framework repository.
