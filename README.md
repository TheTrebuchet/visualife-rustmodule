# visualife-rustmodule
a python module that could be later integrated into visualife, that would would be able to speed up svg generation significantly


## the main strategy I can see is:
Each object can hold an infinite number of references to other objects, hence we have a tree
After or during creation of the new object, we give the parent its reference.
When we finally create the drawing we call only the master canvas.
Each (and only) canvas will have a method for creating the so-called tree and this will be a problematic operation, this will nicely utilize the rust performance.
Any kind of style can be easly cloned from the parent.
This solves most of the objectives we have.