from svg_gen import Canvas, Circle
import time

canvas = Canvas('test', 200, 200, 'canvas1')
circles = []
for i in range(10000):
    radius = 10.0 + i
    x = 5.0 * i
    y = 5.0 * i
    circle = Circle('circle'+str(i), radius, x, y)
    circles.append(circle)
    canvas.add_child(circle)

stoper = time.time()
print(canvas.generate_string())
print(time.time()-stoper)

