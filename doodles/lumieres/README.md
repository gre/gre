# Cirque de lumières

## License

CC BY-NC-ND 4.0

## Technical libraries

- regl library (dependency) (MIT)
- forked version of library 'mersenne-twister' (embedded)

## Inspiration and thanks

Cirque de Lumières is mainly combining two generative art technique: domain warping and 2D raymarching. This is only possible thanks to main previous work and paper on the topics.

I have discovered many years ago the incredible work of Inigo Quilez who published articles on exploring the world of shaders and raymarching. As a developer, I've also always been keen to experiment with many web technologies (canvas, webgl, webaudio,..) while keeping a foot in demoscene art and game jams. Back in ~2014 I also grew a lot with glslsandbox (before shadertoy were a thing) to learn GLSL techniques. I always has been inspired by demosceners like p01, mrdoob, mattdesl,...

I'm also very thanksful to Mikola Lysenko and the work he did in the shader and algorithmic scene. (regl, stack.gl libraries, headless-gl, ndarray,...)

---

## More about the artist

@greweb is a passionate developer currently working at Ledger. Coming from multidisciplinary scenes like demoscene and gamejam culture, he enjoyed the most hacking into WebGL shaders. His motto is to do everything with code, sharing his passion for algorithms and creative coding techniques. In a continuous search for discomfort areas to explore, he got involved this year into "pen plotting" art, using fountain pens and robotic to draw physical art. Every day, he publishes a unique physical piece with its source code. He's now exploring many ways to bond physical art and digital art in the NFT scene.

## Artistic description

Cirque de Lumières is French for “Circus of Lights”. The fusion of glowing shapes in motion explores many dualities: Organic and Mechanical, Smooth and Sharp, Minimalistic and Complex, Solid and Morphing.

Its color palette varies from hot to cold colors, sometimes dark or even pink or green.

### a programming language inception

![Screenshot 2021-10-30 at 17 49 11](https://user-images.githubusercontent.com/211411/139540177-ff92ae5b-9f94-4447-999b-2df6526676ac.png)

Cirque de Lumières generative code is a programming language inception: it compiles a few hundreds of lines of random GLSL instructions (generated code similar to what you see in the screenshot) – unique for each art. This innovation allows a great variety of outcomes, driven by a simple set of rules, primitives of shapes, and transformations. The core rendering technique used is 2D raymarching and domain warping.


---

*more detail below*

## Art mechanics

### Scene

The scene is generally composed of two parts:
- a general background that have various quality of noises.
- a shape area that is most of the time a glaring area.

## Shape scripting

The way the shape is built is using a series of "instructions". Instructions are either addition of shapes (primitives) or a transformation of the space (operators). I call them instructions because it's similar to programming language. and each mint is going to produce a unique series of instructions, that literally are GLSL code generated from JavaScript.

Example:

```glsl
p=tr(p,0.489,0.001,4.683);
s=U(s,box(p,vec2(0.079,0.113)),k);
p=pres;
p=tr(p,0.001,0.090,1.487);
s=U(s,box(p,vec2(0.053,0.099)),k);
p=pres;
p=tr(p,0.205,0.387,2.825);
s=U(s,box(p,vec2(0.026,0.126)),k);
p=pres;
p=tr(p,0.210,0.003,5.135+t2);
s=U(s,box(p,vec2(0.023,0.031)),k);
p=pres;
p=tr(p,0.088,0.001,0.376);
s=U(s,box(p,vec2(0.082,0.119)),k);
p=pres;
p=tr(p,0.272,0.149+-0.064*cos(t2),5.790);
s=U(s,box(p,vec2(0.051,0.110)),k);
p=pres;
p=tr(p,0.312,0.034,4.100);
s=U(s,box(p,vec2(0.046,0.182)),k);
p=pres;
p=tr(p,0.175+0.100*sin(t1),0.108,4.741);
s=U(s,box(p,vec2(0.023,0.077)),k);
p=pres;
p=tr(p,0.241,0.170,1.550+t1);
s=U(s,box(p,vec2(0.047,0.052)),k);
p=tr(p,0.264,0.032,2.008);
s=U(s,box(p,vec2(0.027,0.197)),k);
pmm1(p.y,0.228);
pmp(p,16.0);
p=tr(p,0.160,0.116,4.057);
s=U(s,box(p,vec2(0.100,0.106)),k);
p=pres;
p=tr(p,0.242,0.085,0.433);
s=U(s,box(p,vec2(0.051,0.101)),k);
pmp(p,14.0);
p=tr(p,0.208,0.001,2.024);
s=U(s,box(p,vec2(0.028,0.118)),k);
p=pres;
p=tr(p,0.001,0.028,3.789+t3);
s=U(s,box(p,vec2(0.043,0.061)),k);
p=tr(p,0.148,0.224,2.946+t2);
s=U(s,box(p,vec2(0.045,0.067)),k);
p=pres;
p=tr(p,0.309,0.074,0.998);
s=U(s,box(p,vec2(0.140,0.185)),k);
p=pres;
p=tr(p,0.063,0.144,2.217);
s=U(s,box(p,vec2(0.054,0.147)),k);
p=tr(p,0.008,0.077,5.270);
s=U(s,box(p,vec2(0.026,0.110)),k);
p=pres;
p=tr(p,0.010,0.351,4.003);
s=U(s,box(p,vec2(0.028,0.079)),k);
p=tr(p,0.001,0.095,3.422);
s=U(s,box(p,vec2(0.030,0.030)),k);
p=pres;
p=tr(p,0.038,0.048,5.776);
s=U(s,box(p,vec2(0.024,0.052)),k);
p=pres;
p=tr(p,0.070,0.023,0.276);
s=U(s,box(p,vec2(0.118,0.134)),k);
pmm1(p.x,1.108);
p=tr(p,0.026,0.001,0.449+t1);
s=U(s,box(p,vec2(0.029,0.110)),k);
p=tr(p,0.023,0.161,5.384);
s=U(s,box(p,vec2(0.051,0.046)),k);
p=pres;
p=tr(p,0.133,0.053,5.169+t3);
s=U(s,box(p,vec2(0.065,0.166)),k);
p=tr(p,0.032,0.000,5.514);
s=U(s,box(p,vec2(0.105,0.051)),k);
p=o;
p=tr(p,0.130,0.007,1.439);
s=U(s,sf(p,68.,0.005,0.814,1.451),k);
pmm1(p.y,0.334);
p=pres;
p=tr(p,0.355+0.047*sin(t2),0.058,0.616+t2);
s=U(s,box(p,vec2(0.025,0.074)),k);
p=pres;
p=tr(p,0.083,0.042,4.028);
s=U(s,box(p,vec2(0.024,0.059)),k);
pmp(p,3.0);
p=pres;
p=tr(p,0.001+0.035*sin(t1),0.050,5.768);
s=U(s,box(p,vec2(0.119,0.064)),k);
pmm1(p.x,0.801);
s=max(s,-1.135-s);
```


### Shapes primitives

There are only 3 main primitives that compose the "Shape":

- Blob. is a shape made of noise and that looks like a cloud. It is a rare event to only have one blob, especially combined with a low "weight".
- Rectangle. is a common shape.
- Stripe field. is a shape made of a lot of aligned lines. They are rarely visible but often will just affect other shapes.

### shape operators

To make the final shape, the primitives are blended using many operators. Here are the main ones:

- Smooth Union. will merge two shapes together with way that make the edge smooth.
- Symmetry. will mirror the area in one direction.
- Mirror Modulus. will mirror the area in a repeated way.
- Polar Modulus. will repeat the area circularly around a center.
- Translation. will move the shape.
- Rotation. will rotate the shape.
- Animation. injects TIME into one of the previous operator.

### Shapes "weight"

The weight is driving how large the shape are. A low weight makes the shape showing a "hole" in their middle. A very low weight will make the shapes only having "borders" which can make cool laser shapes (very rare).

### Color palette

There are different color palette with various rarity.

- It is very common to obtain a "hot" color palette (red)
- It is common to obtain a "cold" color palette (light blue)
- It is uncommon to obtain a "dark" color palette (black and white)
- It is rare to obtain another color palette


## Art directions

The art explores a few dualities.

### Minimalism vs Complexity

The art is made of a limited set of primitives and transformations, yet yield a huge varity. This is complexity created by simple rules like in Nature.

### Organical vs Mecanical

The art is meant to explore the frontier between very organic shapes and very mecanical shapes and motions.

### Smooth vs Hard

The art is meant to explore a balance between very smooth shapes (typically with smooth unions and with the glowing effect) and very hard shapes (mirroring and modulus operators will produce sharp areas)

### Hot vs Cold palette

I've tried to make a balance between hot and cold colors. With an extra and complementary "dark" palette.
