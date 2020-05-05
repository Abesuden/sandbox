# Cython Notes

[![Language](https://img.shields.io/badge/Language-Cython-informational.svg)](https://github.com/abesuden/sandbox/Cython)
[![Requires](https://img.shields.io/badge/Requires-Python3-1abc9c.svg)](https://github.com/abesuden/sandbox/Cython)
[![Requires](https://img.shields.io/badge/Requires-GCC-1abc9c.svg)](https://github.com/abesuden/sandbox/Cython)

[![Cython Logo](https://upload.wikimedia.org/wikipedia/en/thumb/c/ce/Cython-logo.svg/1200px-Cython-logo.svg.png)](https://github.com/abesuden/sandbox/Cython)

## Requirments

Install these if they are not already installed:

 * gcc
 * python3
 * pip3

## Install

Cython

```
pip3 install cython
```

## Build Cython Files

In order to run Cython programs, a build file has to be created. The naming convention for the setup file is `setup.py` and the following is an example of the build file:

```
from distutils.core import setup
from Cython.Build import cythonize

setup(ext_modules = cythonize('fileName.pyx'))
```
> Note: fileName.pyx can be replaced with whatever fileName you would like.

In order to import an use Cython created modules, the build will need to produce a `.so` file.

---

An alternative maybe to use the following short hand:

```
cython --embed fileName.py -o fileName.c
```
> Has not been tried yet

## Create Cython Program

A great resource for learning how to code with Cython can be found [here](pythonprogramming.net/introduction-and-basics-cython-tutorial). 

```
cpdef int add(int x, int y):
    cdef int num = 0
    num = x + y
    return num
```
> cpdef is C-Python where cdef is C

## Create The C Code File

Use the following to genereate the C file and an html document.

```
cython -a fileName.pyx
```

## Working Example

A `typescript` can be found in this repo which captured a successful build as well as a running version of the `testOne.pyx` Cython program.

> Note: there has to be a `.so` file generated during the build so that the `import` will work.

## File Types Generated

Cython tends to generate a lot of different file types, so it is important to understand what each one means. Below is a list of commonly created files and files generated with the commands found in this repo:

<p align="center">

| Extension | Description   | Created By |
|:----------|:-------------:|:----------:|
| .py  | python3 code file  | Developer |
| .pyx | cython code file   | Developer |
| .c   | C code file        | Cython    |
| .o   | object file        | Cython    |
| .so  | shared object file | Cython    |
| .html| shared object file | Cython -a |

</p>

> The html file is a useful UI for understanding the conversion from python to C.