# QuaternionsAndComplexNumbers
A rust module for when you want to use Complex Numbers or Quaternions for whatever reason. :)
Made for fun, maybe not the best choice for actual production (for now).

# Run
Run ``cargo run`` on your console within the "ComplexNumbers" folder

# Features
Create ``ComplexNumber (a + bi)`` and ``Quaternion (a + bi + cj + dk)`` objects and execute different operations with them:S
## Some examples:
### For ComplexNumber:
  ``create a complex number`` using "let n = ComplexNumber::new() (= 0 + 0i)" or the macro "let n = complex!(a, b)" which returns the complex numver a + bi \n
  ``multiply/add/substract complex numbers`` using the usual "*" operator
  ``obtain the conjugate or the norm of a complex number`` using the corescponding method (conjugate(), norm())
  ``print the numbers`` using either the normal macro "println!("{}", n)" or with the method "cprint()"
  ``etc.``
