# Forward Mode

Forward mode (often also called Dual mode) is generally recommended if the output dimension is greater than the active input dimension, or if the dimension is similar. Dimension here refers to the total number of scalar values in all input (output, respectively) arguments.

We annotate input and output arguments either with `Const`, `Dual`, or `DualOnly`.

In Forward mode we are only allowed to mark input arguments with `Dual` or `Const`.
The return value of forward mode with a `Dual` return is a tuple containing as the first value the primal return value and as the second value the derivative.

In forward mode `Dual` with two arguments `x, 0.0` is equivalent to `Const` passing only `x`, except that we can perform more optimizations for `Const`.


