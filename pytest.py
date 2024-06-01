from svg_gen import Canvas, Circle
import time
stoper = time.time()
Width = 200
Height = Width
canvas = Canvas('test.svg', Width, Height, 'canvas1')

numc = 100
d = Width/numc
x = 0
y = 0
circles = []
radius = d/2
while x<200 and y<200:
    while x<200:
        circle = Circle('circle', radius, x, y)    
        circles.append(circle)
        canvas.add_child(circle)
        x+=d
    y+=d
    x=0


canvas.complete_svg()
canvas.save_svg()
print(time.time()-stoper)
print(len(circles))

