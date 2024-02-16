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
| `&` or `&mut` Input   | N/A       | Add &mut shadow | Unchanged |
| Other Input Types     | N/A       | N/A          | Unchanged |
| `f32` or `f64` Output | Append float to inputs | N/A | Unchanged |
| Other Output Types    | N/A       | N/A          | Unchanged |



> <div class="warning">
> Unlike Forward Mode, Reverse Mode will always overwrite (zero) your seed!   
> If you want to call a function generated through Reverse Mode, you will have to reset your seed values!
> </div>
