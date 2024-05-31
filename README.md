# visualife-rustmodule
a python module that could be later integrated into visualife, that would would be able to speed up svg generation significantly


## the main strategy I can see is:
Each object can hold an infinite number of references to other objects, hence we have a tree
After or during creation of the new object, we give the parent its reference.
When we finally create the drawing we call only the master canvas.
Each (and only) canvas will have a method for creating the so-called tree and this will be a problematic operation, this will nicely utilize the rust performance.
Any kind of style can be easly cloned from the parent.
This solves most of the objectives we have.

currently this strategy seems to be working

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