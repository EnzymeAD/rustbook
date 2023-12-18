# Why Autodiff?

We propose to add automatic differentiation to Rust.  This would allow Rust users to compute the derivatives of arbitrary functions.  This feature would open new opportunities for Rust in scientific computing, machine learning, and other fields.

A few voices of autodiff users.

Jan Hückelheim (Argonne National Lab, US):  
_Automatic differentiation (AD,also known as autodiff or bakc-propagation) has been used at Argonne and other national laboratories, at least, since the 1980s. For example, we have used AD to obtain gradients of computational fluid dynamics applications for shape-optimization, which allows the automated design of aircraft wings or turbine blades to minimze drag or fuel consumption. AD is used extensively in many other applications including seismic imaging, climate modeling, quantum computing, or software verification._

_Besides the aformentionend "conventional" uses of AD, it is also a cornerstone for the development of ML methods that incorporate physical models. The 2022 department of energy report on Advanced Research Directions on AI for Science, Energy, and Security states that "End-to-end differentiability for composing simulation and inference in a virtuous loop is required to integrate first-principles calculations and avanced AI training and inference". It is therefore conceivable that ADusage and development will become even more important in the near future._
[1](https://www.anl.gov/sites/www/files/2023-05/AI4SESReport-2023.pdf)

Prof. Jed Brown (UC Boulder, US):  


# Background


## What is autodiff used for?

TODO with examples.  What kind of problems does this solve that nothing else solves?

Autodiff is widely used to evaluate gradients for numerical optimization, which is otherwise intractable for a large number of parameters. Indeed, suppose we have a scalar-valued loss function $f(\theta)$ where the parameter vector $\theta$ has length $n$. If the cost to evaluate $f(x)$ once is $c$ (which will often be $O(n)$), then evaluating the gradient $\partial f/\partial x$ costs less than $3n$ with autodiff or tedious and brittle by-hand implementation, but $cn$ otherwise. Optimization of systems of size $n$ in the hundreds to billions are common in applications such as calibration, data assimilation, and design optimization of physical models, in perception and control systems for robotics, and machine learning.

Derivatives are also instrumental to thermodynamically admissible physical models, in which models are developed using non-observable free energy functionals and dissipation potentials, with observable dynamics represented by their derivatives. Commercial engineering software requires users to implement these derivatives by hand (e.g., Abaqus [`UHYPER`](https://abaqus-docs.mit.edu/2017/English/SIMACAESUBRefMap/simasub-c-uhyper.htm#simasub-c-uhyper-t-vartodefine1) and [`UMAT`](https://abaqus-docs.mit.edu/2017/English/SIMACAESUBRefMap/simasub-c-umat.htm#simasub-c-umat-t-vartodefine1)) and constitutive modeling papers routinely spend many pages detailing how to efficiently compute the necessary derivatives since these are among the most computationally intensive parts of simulation-based workflows and numerical stability is necessary.

