---
title: HTML Canvas pour les néophytes
description: Cette vidéo de 20 minutes présente les possibilités du Canvas à travers quelques démos et l’implémentation pas à pas d’un exemple basique.
author: Gaetan
layout: post
permalink: /2011/03/html-canvas-pour-les-neophytes/
tags:
  - canvas
  - html
  - javascript
---

 [1]: http://whatwg.org/html
 [2]: http://www.whatwg.org/specs/web-apps/current-work/multipage/the-canvas-element.html
 [3]: http://www.mrspeaker.net/dev/parcycle/
 [4]: http://en.inforapid.org/
 [5]: http://gre.github.io/same-game-gravity
 [6]: http://fizz.bloom.io/ 
 [7]: http://easeljs.com/
 [8]: http://processingjs.org/ 

Cette vidéo de 20 minutes présente les possibilités du Canvas à travers quelques démos et l’implémentation pas à pas d’un exemple basique.  
Elle est destinée à des développeurs débutant dans l’utilisation de Canvas.

<iframe src="http://player.vimeo.com/video/20957255?portrait=0" width="550" height="410" frameborder="0" webkitAllowFullScreen mozallowfullscreen allowFullScreen></iframe>

*Dans la suite de l’article : les liens et codes de la vidéo …*

<!--more-->

## Liens

### Specs

*   [whatwg.org/html][1]
*   [Canvas][2]

### Exemples

*   [parcycle][3]
*   [Visualisation des relations entre sujets (wikipédia)][4]
*   [Same Game][5]
*   [fizz : visualisation des tweets][6]

### Bibliothèques graphiques

*   [EaselJS][7]
*   [Processing JS][8]


## Exemple de l’implémentation

```html
<html>
  <head>
    <style>
      body {
        background: #ddd;
      }
      canvas {
        background: #fff;
      }
    </style>
  </head>
  <body>
    <canvas id="sketch" width="300" height="300"></canvas>
    <img id="image" src="http://www.whatwg.org/images/logo" style="display: none;" />
    
    <script type="text/javascript">
    (function(){
      
      var canvas = document.getElementById('sketch');
      var ctx = canvas.getContext('2d');
      var img = document.getElementById('image');
      
      var i = 0;
      setInterval(function() {
        ctx.clearRect(0, 0, 300, 300);
        
        ctx.drawImage(img, 0, 0, 300, 300);
        
        ctx.fillStyle = 'rgba(255, 0, '+Math.floor(Math.sin(i/50)*255)+', 0.8)';
        ctx.fillRect(100, i % 300, 100, 100);
        
        /*
        
        ctx.strokeStyle = '#09F';
        ctx.lineWidth = 5;
        
        ctx.beginPath();               // commence à tracer un chemin
        ctx.moveTo(0, 20);             // défini le premier point de tracage à la position (0, 20)
        ctx.lineTo(canvas.width-100, 30);  // Tracer une ligne jusqu'à la position (canvas.width, 30). canvas.width désigne la largeur du canvas (500 dans notre exemple).
        ctx.bezierCurveTo(100, 200, 0, 100, 300, 300);
        ctx.stroke();                  // Indique au canvas de dessiner le chemin tracé depuis le beginPath
        
        */
        
        ++ i;
      }, 30);
      
    }());
    </script>
  </body>
</html>
```
