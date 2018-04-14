.. Function Overloading in Rust
.. Copyright Nicola Musatti 2018

=======================================================================
Function Overloading in Rust - How I Learned to Live Happily Without it
=======================================================================

|

Nicola Musatti

nicola.musatti@gmail.com

@NMusatti

Rust Milano Meetup - 24 January 2018


Who I am
========

.. footer::

   Nicola Musatti - Function Overloading in Rust - Rust Milano Meetup - Mikamai - 24 January 2018

* A long time C++ enthusiast who became disenchanted due to the ever increasing complexity of the
  language
* Doing Java and non programming stuff for a living
* A long time Rust beginner
* I also love Python!


What we are doing here
======================

* What is function overloading and what is good about it
* What can be done when it is not available
* Which of Rust's features can be used to address the same problems
* Some further reaching implications


A simple example
================

* A simple type: Element
* An Element collection type: Collection
* We want to add single elements to a collection
* We want to add all the elements in a collection to another collection


How we would have done it in C
==============================

.. literalinclude:: ../../../cpp/git/rust-overload-cpp/c/first.c
   :language: c
   :lines: 3-22


The same in Rust
================

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/first.rs
   :language: rust
   :lines: 1-16


What's the problem
==================

* Reduced expressivity: I have to repeat type names just to disambiguate
* Names relate less directly to the problem domain
* Functions don't participate in generic code (C++ specific problem)


A C++ version
=============

.. literalinclude:: ../../../cpp/git/rust-overload-cpp/cpp/first.h
   :language: c++
   :lines: 4-21


What is function overloading
============================

* The possibility of *defining* two or more functions with the same name that take parameters that
  differ in type and/or number
* The possibility of *calling* a "function" with different type and/or number of arguments,
  regardless of how this is achieved

|

**Calling is what matters if defining is not too complicated**


Related constructs
==================

* Generics or templates
* Default arguments
* Duck typing
* Inheritance (?)


A C++ template example
======================

.. literalinclude:: ../../../cpp/git/rust-overload-cpp/cpp/first.cpp
   :language: c++
   :lines: 3-15


A duck typing example in Python
===============================

.. literalinclude:: ../../../python/git/rust-overload-python/rust-overload-python/first.py
   :language: python3
   :lines: 3-14,24-32


A default arguments example in Python
=====================================

.. literalinclude:: ../../../python/git/rust-overload-python/rust-overload-python/second.py
   :language: python3
   :lines: 3-14,24-32


Well...
=======

Overloading is better!

* It is cleaner
* It respects the Open/Closed principle
* Type switches are frowned upon in languages that lack Rust-style enums


What about Rust?
================

* A limited form of overloading may be implemented via traits:

  + The number of parameters cannot vary
  + Implementation is somewhat complicated
  + There is a conceptual mismatch: traits provide an additional level of abstraction, overloading
    should not 

* No default arguments
* No duck typing: Rust discourages or prohibits casual completion


Overloading example in Rust
===========================

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/second.rs
   :language: rust
   :lines: 1-23


Overloading example in Rust (cont.)
===================================

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/second.rs
   :language: rust
   :lines: 29-35


Overloading has its problems too!
=================================

Suppose we want to implement a `Point` type that may be built:

* From cartesian coordinates
* From polar coordinates


A (broken) C++ example
======================

.. literalinclude:: ../../../cpp/git/rust-overload-cpp/cpp/point.cpp
   :language: c++


Python's default arguments help
===============================

.. literalinclude:: ../../../python/git/rust-overload-python/rust-overload-python/point.py
   :language: python3
   :lines: 4-14,24-28


Rust to the rescue
==================

...or how I think overloaded functions should be implemented in Rust:

|

**A single function with a single enum parameter that lists all the argument combinations to be
supported**


The Point example in Rust
=========================

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/point1.rs
   :language: rust
   :lines: 1-22


Extending the Point example in Rust
===================================

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/point2.rs
   :language: rust
   :lines: 1-15


Extending the Point example in Rust (cont.)
===========================================

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/point2.rs
   :language: rust
   :lines: 17-33,39-43


The original example in Rust
============================

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/third.rs
   :language: rust
   :lines: 1-20


What is bad about it
====================

* Calling the function is a little more verbose than true overloading
* It breaks the Open/Closed principle: adding supported parameter combinations requires modification
  of existing code
* The argument enum may be somewhat artificial


What is good about it
=====================

* Available alternatives are described in a single place
* Intention is explicitly stated by choosing the appropriate enumeration
* Rust ensures that all declared cases are covered


Wait, there's more to it
========================

* A fundamental part of static typing is setting bounds on parameters
* Check are better made upfront so that the type system may forward constraints automatically
* The argument enum moves argument checking out of the function
* This could be extended to further checks:

  + Valid subranges
  + Combined constraints
  
* Unfortunately the lack of standard constructors makes the syntax less than ideal


A last example
==============

.. literalinclude:: ../../../rust/git/rust-overload-rust/src/sqrt.rs
   :language: rust
   :lines: 1-17,23-25


The end
=======

|

**Q&A or pizza, that is the question!**

