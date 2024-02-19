# Forward Mode

When using forward mode, we only have three choices of activity values, `Dual`, `DualOnly` and `Const`.
Dual arguments get a second "shadow" variable.
Usually we will only seed the shadow variable of one Dual input to one and all others to zero, 
and then read the shadow values of our output arguments.
We can also seed more then one input shadow, in which case the shadow of output variables will 
be a linear combination based on the seed values.
If we use a `&mut ` reference as input and output argument and mark it as Dual,
the corresponding shadow seed might get overwritten. Otherwise, the seed value will remain unchanged.

#
| Activity               | Dual            | DualOnly | Const      |
|------------------------|-----------------|----------|------------|
| Non integer input `T`  | Accept `T`,`T`  | Accept `byVal(T)`, `T` | Unchanged  |
| Integer scalar input   | N/A             | N/A      | Unchanged  |
| `f32` or `f64` output `T`  | Return (T,T)    | Return T | Unchanged  |
| Other output types  | N/A             | N/A      | Unchanged  |

`DualOnly` is a potentially faster version of `Dual`.  

When applied to a return type, it will cause the primal return to not be computed.
So in the case of `fn f(x: f32) -> f32 { x * x }`, 
we would now only return `2.0 * x`, instead of
`(x * x, 2.0 * x)`, which we would get with `Dual`.  

In the case of an input variable, `DualOnly` will cause the first value to be 
passed by Value, even when passed by Reference in the original function.
So `fn f(x: &f32, out: &mut f32) {..}` would become
`fn df(x: f32, dx &mut f32, out: f32, dout: &mut f32) {..}`.  
This makes `x` and `out` inaccessible for the user, so we can use it as buffer
and potentially skip certain computations. This is mostly valuable for larger Types, or more complex functions.
