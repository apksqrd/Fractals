# Fractals

## Mandelbrot

My python mandelbrot is pretty cool and you can find it here: <https://github.com/apksqrd/Math-Challenges/blob/main/mandelbrot.py>

This is the rust-generated one:
![rust-generated-dist-sqrd-method-500-iter-4096x4096](Buddhabrot/outputs/mandelbrot/colors/test.png)

This is the best Buddhabrot.
![rust-generated-buddhabrot](Buddhabrot/outputs/buddhabrot/transposed/transposed.png)

TODO: save the raw data to make it easy to add more data iteratively. (is there an algorithm to find a new point in an area that hasn't been covered much so far, the easy way would just be to create a completely random point and after a while it will balance out but IDK)
TODO: use the fact that buddhabrot and mandelbrot are symmetrical
TODO: change the Option type thing for default settings, just make it a macro
TODO: Julia sets (combine Julia and mandelbrot)
TODO: implement more fractals from [3B1B](https://www.youtube.com/watch?v=LqbZpur38nw&t=31s)
TODO: Chaos game
TODO: L-system
TODO: Logistic map
TODO: Anti-mandelbrot and other similar things (where the c is f(point) and f can be any function. in the normal mandelbrot, f(c) = c)
