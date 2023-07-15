# preempt
A scheduler for humans.

---

Preempt is a to-do list that also schedules on your behalf. The goal is to relieve you from having to plan every single thing you have to do, while also allowing for a variety of contexts in your life.

Preempt schedules tasks using a model inspired by computer preemptive scheduling. However, humans are not machines, and so it also integrates concepts from the Pomodoro Technique to balance productivity and self-care.

The main concepts in Preempt are tasks and contexts:

A task is something you need to do. It has a name, an estimated duration, and an associated context.

A context is a specific period in your schedule. This might be "Work," "Personal," or any other category that makes sense for your life. Each context has a name and specified start and end times. The tasks associated with a context are intended to be done during the times specified for that context.

Preempt uses these concepts to generate a schedule for you. It assigns tasks to appropriate contexts based on their durations and the available time in each context. It also automatically schedules breaks using the Pomodoro Technique.

---

## Features

- Add tasks with `add-task`.
- Define and manage contexts with `add-context` and `edit-context`.
- See your schedule with the `timeline` command.

## Usage

### Add a task

```bash
preempt add-task --name <name> --description <description> --duration <duration>
```

### Add a context

```bash
preempt add-context --name <name> --days <days> --start <start_time> --end <end_time> [--transition <transition_time>]
```

### Edit a context

```bash
preempt edit-context --name <name> [--days <days>] [--start <start_time>] [--end <end_time>] [--date <date>] [--transition <transition_time>]
```

### Show a context

```bash
preempt show-context <name>
```

### Visualize timeline

```bash
preempt timeline
```