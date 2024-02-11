# Forward Mode

Forward mode (often also called Dual mode) is generally recommended, 
if a function has more output than input values, or if the number is similar.

We annotate input and output arguments either with Const, or Dual.


In Forward mode we are only allowed to mark input arguments with Dual or Const.
The return value of forward mode with a Dual return is a tuple containing as the first value the primal return value and as the second value the derivative.

In forward mode Dual(x, 0.0) is equivalent to Const(x), except that we can perform more optimizations for Const.


