# Computor_v1

```
> ./computor "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0
Polynomial degree: 2
Discriminant is strictly positive, the two solutions are:
-0.47513146390886934
0.9052389907905898
```

## Overview

I have created a program that, given an algebraic equation in one variable, will tell you its order and, if it is less than or equal to the second order, its solution.

## Requirement

- cargo 1.66.0

## Usage

```
git clone .....
cd Computor_v1
make
```

Then give the equation as follows

```
./computor "-3 + X + 2X^2 = 0"
```

## Features

Supported characters are as follows

- X
- number (Integers and Decimals)
- \+
- \-
- \*
- ^
- =

It also includes the following features

- Organizing the given equation
- Find the order
- For equations of the second degree or lower, find the solution, including complex solutions
- Fractions can also be displayed

## Example

"Complex" calculations are possible.

```
> ./computor "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0
Polynomial degree: 2
Discriminant is strictly positive, the two solutions are:
-0.47513146390886934
0.9052389907905898
```

Support for simplified writing

```
> ./computor "1 + 2X + X^2 = 0"
Reduced form: 1 * X^0 + 2 * X^1 + 1 * X^2 = 0
Polynomial degree: 2
Discriminant is zero, the solution is:
-1
```

Support for "complex" solutions.

```
> ./computor "1 + 4X + 5X^2 = 0"
Reduced form: 1 * X^0 + 4 * X^1 + 5 * X^2 = 0
Polynomial degree: 2
Discriminant is strictly negative, the two complex solutions are:
-0.4 ± 0.2i
```

Can also be displayed in fractions

```
> ./computor "-3 + X + 2X^2 = 0"
Reduced form: - 3 * X^0 + 1 * X^1 + 2 * X^2 = 0
Polynomial degree: 2
Discriminant is strictly positive, the two solutions are:
1
-3 / 2
```

Equations of the first degree or lower are also supported

```
> ./computor "1 + 4X = 0"
Reduced form: 1 * X^0 + 4 * X^1 = 0
Polynomial degree: 1
The solution is:
-1 / 4
```

## Author

[twitter](https://twitter.com/Kotabrog)

## Licence

[MIT](https://github.com/kotabrog/Computor_v1/blob/main/LICENSE)
