# Forward Mode

In Forward mode we are only allowed to mark input arguments 
The return value of forward mode with a Duplicated return is a tuple containing as the first value the primal return value and as the second value the derivative.

In forward mode Duplicated(x, 0.0) is equivalent to Const(x), except that we can perform more optimizations for Const.

