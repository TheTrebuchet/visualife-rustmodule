from svg_gen import Canvas, Circle
import time

canvas = Canvas()
circles = []
for i in range(10000):
    radius = 10.0 + i
    x = 5.0 * i
    y = 5.0 * i
    circle = Circle('circle'+str(i), radius, x, y)
    circles.append(circle)
    canvas.add_child(circle)

stoper = time.time()
for c in circles: c.cx = 0
result = canvas.generate_string()
print(result)
print(time.time()-stoper)

