---
date: "2023-01-15"
title: "3D sine waves"
image: /images/plots/889.jpg
tweet: https://twitter.com/greweb/status/1614873537084362753
---

Made for Genuary 2023 #15 "Sine Waves".

Made 3D objects, printed with a Creatily Ender 3. The generator yields various low-poly tree variants (.STL format) that can be printed.

This is made in #rustlang using isosurface lib for the marching cubes algorithm and with a resolution of 16^3 to stay in low poly realm.

It uses the "Signed Distance Function" paradigm which aim to give, for any point in space, the distance to the closest object. This paradigm can usually be used to do shader raymarching, but here also perfectly works with marching cubes.

This was relatively trivial to implement but what took me the most time was figuring out how to make my shape contains in the `[0,1]` domain and not touching edges. I used the intersection with a rounded cube for this. `op_intersection(box, pyramid)`
