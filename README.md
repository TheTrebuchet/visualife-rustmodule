# visualife-rustmodule
a python module that could be later integrated into visualife, that would would be able to speed up svg generation significantly


## the main strategy I can see is:
The strategy will be different from now on

we want a working rust module just for use in rust and we want python classes that can be like a bridge between that rust module and python

that way in the future we could have a separate working rust and python modules

currently I can see the python module being also written in rust, pyo3 will be used solely for managing python objects, rust structs and converting the inputs to rust types

precisesly:
there must be a crate that only works in rust (visualirs)
for this there must be a separate crate that communicates python with rust (visualipy)

so the rust parent holds references to rust structs
and it all works beautifully in rust
it could be that one mutable reference of an object exists
but...

but in python... we create a python object
and when we add a child, the Python object must receive references of this struct
then changes to the python object change the struct with appropriate methods
a struct is already used by the rust API

so looking from the user's side
user calls:
  canvas = Canvas()
this creates a visualipy module object
and at the same time struct visualirs, held by this Python object

when the user calls:
  circle = Circle()
the same thing happens

when adding a circle to canvas
  canvas.add(circle)
then the circle reference goes first to canvas python object
and then to the canvas struct from visualirs

when we call build svg
  canvas.generate()
we call struct canvas, which holds references to only rust structs, so everything works fast in rust
it returns a rust string to visualip, which converts it very quickly into a python string and returns it to python

modifying objects also works via visualipy
it must have defined methods for changing variables of rust structs
each method changes a variable in the python object and in the assigned struct

from rust's perspective, each struct has to be in an arc mutex
because it will have two mutable references, one in parent python object and one in parent rust struct

## Objectives to acomplish

class SvgViewport

  def __init__
  def id
  def svg_width
  def svg_height
  def get_viewbox
  def set_viewbox
  def viewport_name
  def set_background
  def is_inner_viewport
  def parent_viewport
  def insert_viewport
  def style
  def text_style
  def innerHTML
  def clear
  def error_msg
  def scale_x
  def scale_y
  def svg_width
  def svg_height
  def scale_x
  def scale_y
  def text_length
  def __points_as_string
  def __points_as_string
  def __prepare_attributes
  def __create_svg_header

def get_viewbox
def set_viewbox
def viewport_name
def set_background
def is_inner_viewport
def parent_viewport
def insert_viewport
def style
def text_style
def innerHTML
def close
def clear
def error_msg
def scale_x
def scale_y
def svg_width
def svg_height
def scale_x
def scale_y
def text_length
def __points_as_string
def radial_gradient
def start_clip_path
def close_clip_path
def start_marker
def close_marker
def add_filter
def add_font
def start_group
def close_group
def __points_as_string
def __prepare_attributes
def __create_svg_header

object to be introduced

def image
def rect
def square
def circle
def line
def polyline
def ellipse
def polygon
def triangle
def rhomb
def path
def text