# Forward Mode

When using forward mode, we only have two choices of activity values.
The first is Dual, the second is Const.
Dual arguments get a second "shadow" variable.
Usually we will only seed the shadow variable of one Dual input to one and all others to zero, 
and then read the shadow values of our output arguments.
We can also seed more then one input shadow, in which case the shadow of output variables will 
be a linear combination based on the seed values.
If we use a `&mut ` reference as input and output argument and mark it as Dual,
the corresponding shadow seed might get overwritten. Otherwise, the seed value will remain unchanged.

#
| Activity               | Dual                  | Const      |
|------------------------|-----------------------|------------|
| Non Integer Input      | Create Shadow input   | Unchanged  |
| Integer Scalar Input   | N/A                   | Unchanged  |
| `f32` or `f64` Output  | Return (T,T)          | Unchanged  |
| Other Output           | N/A                   | Unchanged  |

