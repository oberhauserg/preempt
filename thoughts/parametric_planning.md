# Parametric planning

Outside of rigidly structured appointments and meetings - almost nothing in life is 100% planned. Instead, we make todo lists - perhaps prioritized. A simple collection of things to be done whenever we can do it.

However, this isn't as simple as simply executing each task in a round-robin fashion for n minutes until the task is complete.

Consider the following:
1. There's usually a spacial dimension to planning - going shopping could be coupled with getting your oil changed if you're already going to be out.
2. Deadlines inform priority - A minor task might not be super high priority in general, but will become higher priority once a deadline approaches.
3. Some tasks come with prerequisites - For example:
    * Before going to the doctor, an appointment must be made.
    * The planning system should understand that the output of making an appointment will be a new task: going to the appointment.
    * A doctor appointment will probably be planned over school or work time, and that context should not be active during this time.
4. The start or stop times of entire contexts might not be rigid. For example, walking into work might have >30 minute variation per day depending on a number of factors. When predicting into the future, the planning should take these points into consideration and offer a fuzzy prediction.
