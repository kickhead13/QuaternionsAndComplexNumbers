# QuaternionsAndComplexNumbers
A rust module for when you want to use Complex Numbers or Quaternions for whatever reason. :)
Made for fun, maybe not the best choice for actual production (for now).

# Run
Run ``cargo run`` on your console within the "ComplexNumbers" folder

# Features
Create ``ComplexNumber (a + bi)`` and ``Quaternion (a + bi + cj + dk)`` objects and execute different operations with them
## Some examples:
### For ComplexNumber:
  ``create a complex number`` using "let n = ComplexNumber::new() (= 0 + 0i)" or the macro "let n = complex!(a, b)"<br>
  ``multiply/add/substract complex numbers`` using the usual "*" operator<br>
  ``obtain the conjugate or the norm of a complex number`` using the corescponding method (conjugate(), norm())<br>
  ``print the numbers`` using either the normal macro "println!("{}", n)" or with the method "cprint()"<br>
  ``etc.``<br>
### For Quaternion:
  ``mostly the same stuff but implemented for the case of quaternions :3``
