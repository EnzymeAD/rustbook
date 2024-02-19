# Reverse Mode

Reverse Mode reverts the normal control flow, so in this case we will have to seed output variables.
This can be a bit counter-intuitive at first, so we provide an additional helper for float arguments.

When marking a float input as `Active`, we will internally treat it as having a seed of one. So there will
be no extra input argument. We will then return the resulting float, either directly, or if your primal function 
already has a return value, we will update the new function to return a touple, with one additional float per `Active` input.
Those additional float values are in the same order as your `Active` input float values.
If you instead want to seed a float output, you can still mark it as `Active`. We will then append one float argument to 
the list of input arguments, which will work as your seed. 

More advanced types which are passed by pointer or reference can be marked as `Duplicated`.
In this case a shadow argument will be added. Please note that unlike forward mode, the 
shadow here will always be mutable, therefore `*mut` or `&mut`. This is necessary since we have 
to overwrite (zero) your seed values for correctness. A motivation for this is given below.


#
| Activity              | Active    |Duplicated  |      Const |
|-----------------------|-----------|------------|------------|
| `f32` or `f64` Input  | Return additional float | N/A  | Unchanged |
| `&T` or `&mut T` Input | N/A       | Add `&mut T` shadow | Unchanged |
| Other Input Types     | N/A       | N/A          | Unchanged |
| `f32` or `f64` Output | Append float to inputs | N/A | Unchanged |
| Other Output Types    | N/A       | N/A          | Unchanged |

Similar to Forward Mode, we offer optimized versions of our Activity Types.
#
| Activity               | ActiveOnly   | DuplicatedOnly |
|------------------------|--------------|----------------|
| `f32` or `f64` Input   | N/A          | N/A            |
| `&T` or `&mut T` Input | N/A          | Accept `T, &mut T` |
| `f32` or `f64` Output  | Omit primal return value <br> Append float to inputs | N/A |

`ActiveOnly` can not yield any optimization for input values, since it is only 
applicable to floats passed by value. When used on a return type, it has the same 
effect as `DualOnly` in Forward Mode.
So in the case of `fn f(x: f32) -> f32 { x * x }`, 
we would now only return `2.0 * x`, instead of
`(x * x, 2.0 * x)`, which we would get with `Active`.  

`DuplicatedOnly` has the same effect as `DualOnly` on input Arguments.
The original input will now be taken by Value, allowing to use it 
as a scratch space. If the original Type was a mutable reference, we can 
additionally save the computation of any updates to that value.

> <div class="warning">
> Unlike Forward Mode, Reverse Mode will always overwrite (zero) your seed!   
> If you want to call a function generated through Reverse Mode multiple times, you will have to reset your seed values!
> </div>
