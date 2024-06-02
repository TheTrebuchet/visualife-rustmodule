from svg_gen import Canvas, Circle, Rect, Style
import time
import random
stopper = time.time()
def rgb_to_hex(r, g, b):
    return f'#{r:02x}{g:02x}{b:02x}'

def darker(color, factor):
    r = int(color[1:3], 16)
    g = int(color[3:5], 16)
    b = int(color[5:7], 16)
    r = int(r * (1 - factor))
    g = int(g * (1 - factor))
    b = int(b * (1 - factor))
    return rgb_to_hex(r, g, b)

draw_width = 1000
n_x, n_y = 100, 100
box_width = 9.0
max_noise = 0.5
canvas = Canvas('drawing.svg', draw_width, draw_width, 'canvas')
canvas.style.fill = rgb_to_hex(255, 100, 255)


for i in range(n_x):
    max_drop = (i / 100.0 + 1.0) ** 3
    for j in range(n_y):
        rng = random.Random()        
        fill = rgb_to_hex(255, i * 255 // n_x, j * 255 // n_y)
        stroke = darker(fill, 0.3)
        opacity = rng.uniform(0.6, 1.0)
        stroke_width = rng.uniform(0.5, 2.0)
        
        noise_x = rng.uniform(-max_noise, max_noise)
        noise_y = rng.uniform(-max_noise, max_noise) + rng.uniform(max_drop / 2.0, max_drop)
        
        x = i * draw_width / n_x + noise_x
        y = j * draw_width / n_y + noise_y
        id = f'el_{i}_{j}'
        
        if rng.uniform(0.0, 1.0) < 0.1:
            circ = Circle(id, box_width, x, y)
            canvas.add_child(circ)
            circ.style.angle = rng.uniform(0.0, i * j * 60.0 / (n_x * n_y))
            circ.style.fill = fill
            circ.style.stroke = stroke
            circ.style.opacity = opacity
            circ.style.stroke_width = stroke_width
            
        else:
            rect = Rect(id, x, y, box_width, box_width)
            canvas.add_child(rect)
            rect.style.angle = rng.uniform(0.0, i * j * 60.0 / (n_x * n_y))
            rect.style.fill = fill
            rect.style.stroke = stroke
            rect.style.opacity = opacity
            rect.style.stroke_width = stroke_width
            print(rect.style.stroke_width)
print(time.time()-stopper)
stopper = time.time()
canvas.complete_svg()
canvas.save_svg()
print(time.time()-stopper)