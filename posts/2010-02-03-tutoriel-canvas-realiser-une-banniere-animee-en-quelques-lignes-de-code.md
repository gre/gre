---
title: 'Tutoriel Canvas : Réaliser une bannière animée en quelques lignes de code'
thumbnail: /images/2010/animated_banner.png
description: Ce tutoriel vise à présenter canvas comme une librairie très simple d’utilisation et haut niveau grâce au langage javascript.
author: Gaetan
layout: post
permalink: /2010/02/tutoriel-canvas-realiser-une-banniere-animee-en-quelques-lignes-de-code
tags:
  - animation
  - canvas
  - javascript
  - bezier
---

 [1]: /2011/03/html-canvas-pour-les-neophytes
 [2]: /demo/animate-banner/canvas-cartesian.png
 [3]: /demo/animate-banner/step1.html
 [4]: /demo/animate-banner/bezier-schema.png
 [5]: /demo/animate-banner/step2.html
 [6]: /demo/animate-banner/bezier-exemple.png
 [7]: http://paulirish.com/2011/requestanimationframe-for-smart-animating/
 [8]: /demo/animate-banner/step3.html
 [9]: /demo/animate-banner/step4.html
 [10]: /demo/animate-banner/step5.html
 [11]: /demo/animate-banner/step6.html
 [12]: /demo/animate-banner/final.html

![](/images/2010/animated_banner.png)

**Pré-requis conseillé: [Voir la vidéo : HTML Canvas pour les néophytes][1]**

Le **HTML 5** intègre de nouvelles technologies comme le **canvas**, une véritable API graphique destinée à remplacer le flash dans les années à venir.

Ce tutoriel vise à présenter **canvas** comme une **librairie très simple d’utilisation et haut niveau** grâce au langage **javascript**.

Il vous apprendra à réaliser une animation similaire à la bannière <del>actuelle</del> (ancienne maintenant) de mon blog en **quelques lignes de code**.


Il est volontairement ordonnancé de manière didactique, si vous maitriser les concepts, n’hésitez pas à avancer.

<!--more-->

## Code de base (skeleton template)

Nous allons travailler avec ce code **html** de base :

```html
<!DOCTYPE html>
<html>
  <head><title></title></head>
  <body>
    <canvas id="tuto" width="500" height="100" style="border: 1px solid;"></canvas>
    <script language="javascript">
      var canvas = document.getElementById('tuto');
      var ctx = canvas.getContext('2d');

      // Le code javascript ira ici

    </script>
  </body>
</html>
```

Ce code nous servira dans tous le reste du tutoriel. 

### Deux choses notables :

* Nous avons créé un élément **canvas** et indiqué ses dimensions **(500×100)**. Pour mieux pouvoir repérer ses dimensions, nous lui avons ajouté une bordure.
* En **javascript**, nous avons récupéré l’élément DOM puis le contexte 2d qui nous servira pas la suite.

## Quelques notions de base du canvas

### La surface du canvas

Le **canvas** occupe une surface dont la dimension est définie par les paramètres *width* et *height*.

Pour ceux qui ne serait pas familier avec les **bibliothèques graphiques**, cette surface peut être vu comme un quadrillage de pixel sur **deux dimensions** : **x** variant de *0 à width* et **y** variant de *0 à height*.

Le point d’origine de ce repère orthonormé est situé **dans le coin haut gauche du canvas**:

![][2]

### Le concept de contexte

Le contexte 2d récupéré dans la variable **ctx** est en fait l’**interface de programmation** (API) de la bibliothèque graphique **canvas**.

C’est en quelque sorte l’**intermédiaire entre le programmeur et la bibliothèque graphique**.

Ainsi par exemple, si l’on veux dessiner un rectangle de dimension **20×10** à la position **(5,6)**, il suffit simplement d’écrire: 

```javascript
ctx.fillRect(5,6,20,10)
```

### Les paramètres globaux du canvas

Pour dessiner, toute librairie graphique a besoin de connaitre un certain nombre de paramètres tels que la *couleur du trait, la taille de la brosse, etc*.

Plutôt que de devoir passer en paramètre ces informations aux fonctions du contexte, **canvas** met directement à disposition ses paramètres afin de pouvoir les modifier facilement.

Ainsi, nous pourrons définir la couleur de remplissage dans **ctx.fillStyle** et la couleur de trait dans **ctx.strokeStyle**.

## Commençons à coder

Avant d’attaquer l’animation, nous allons commencer à manier les **outils de tracage**.

### Chemins et traits simples

Nous allons commencer par tracer un trait simple qui va traverser tous le canvas.

Essayez le code suivant:

```javascript
ctx.beginPath();              // commence à tracer un chemin  
ctx.moveTo(, 20);             // défini le premier point de tracage à la position (0, 20)  
ctx.lineTo(canvas.width, 30); // Tracer une ligne jusqu'à la position (canvas.width, 30). canvas.width désigne la largeur du canvas (500 dans notre exemple).  
ctx.stroke();                 // Indique au canvas de dessiner le chemin tracé depuis le beginPath
```

[Voir le résultat][3]

### Chemins et courbes de bézier

#### La notion de courbe de bézier

Une courbe de bézier est définie par 4 points :

*   Deux points désignant le début et la fin du trait.
*   Deux points appelés **poignées** permettant de contrôler la courbe.

![][4]

#### Application en canvas

Essayez le code suivant :

```javascript
ctx.beginPath();  
ctx.moveTo(, 20);  
ctx.bezierCurveTo(canvas.width/3, canvas.height, 2*canvas.width/3, , canvas.width, 20);  
ctx.stroke();
```

[Voir le résultat][5]

**De la même façon, la procédure consiste à:**

* commencer un chemin,
* se placer à une certaine position,
* effectuer un traçage (courbe de bézier),
* terminer le traçage (**ctx.stroke()**).

Intéressons nous plus particulièrement au traçage de la courbe de bézier avec l’appel de **ctx.bezierCurveTo**.

##### bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y)

Cette fonction prends en argument les coordonnées du premier point de contrôle **(cp1x, cp1y)**, du deuxième point de contrôle **(cp2x, cp2y)** et du point final **(x, y)**. A noter que le point de début est le point sur lequel on s’est placé au moyen de **moveTo**.

**Dans notre exemple, les deux points de contrôle sont placés ainsi :**

![][6]

### Ajout de l’animation

Pour animer notre courbe de bézier, nous allons faire varier les 4 points de notre courbe en fonction du temps.

En javascript, il y a deux moyen d’effectuer une animation :

* <del>Au moyen de **setInterval** permettant d’appeler une fonction par interval de temps régulier.</del>
* <del>Au moyen de **setTimeout** permettant d’appeler une fonction après un temps donné.</del>
* **[UPDATE 2011]** Il est maintenant recommandé d’utiliser **requestAnimationFrame** qui est l’équivalent de setTimeout mais destiné à l’animation donc plus performant. [Plus d’informations içi (en)][7]

La première approche est parfaite pour une animation **“statique”, avec peu d’intéraction**.

La seconde approche est intéressante lorsque l’animation **doit être contrôlée** (intéraction). En effet, cette approche consiste à appeler setTimeout à chaque cycle d’animation.

Nous choisirons d’utiliser **setInterval**, plus simple et plus adaptée à notre tutoriel.

#### Approche linéaire

Commençons simplement par une **évolution linéaire des points**.

Essayez le code suivant :

```javascript
var i = 0; // variable fonction du temps
var cycle = function() {
  ctx.clearRect(0,0,canvas.width,canvas.height); // clean the canvas
  var y = Math.abs(canvas.height-i%(2*canvas.height)); // y évolue par rebond entre 0 et canvas.height au cours du temps (linéarité)
  ctx.beginPath();
  ctx.moveTo(0, y);
  ctx.bezierCurveTo(canvas.width/3, canvas.height/2, 2*canvas.width/3, canvas.height/2, canvas.width, y);
  ctx.stroke();
  i++;
};
setInterval(cycle, 30); // lance le cycle chaque 30 millisecondes
```

[Voir le résultat][8]

Pour l’instant les poignées sont fixes et l’évolution linéaire de **y** donne un effet de rebond peu intéréssant.

C’est pour cela que nous abandonnons l’idée d’une évolution linéaire des positions au cours du temps pour l’approche sinusoïdale.

#### Approche sinusoïdale

Comme nous l’avons vu, l’évolution linéaire n’est pas adaptée pour ce genre d’animation (effet de rebond). Il faudrait rendre l’animation plus fluide.

Pour cela, nous allons utiliser **une évolution sinusoïdale des positions au cours du temps**.

Essayez le code suivant :

```javascript
var i = 0; // variable fonction du temps
var cycle = function() {
  ctx.clearRect(0,0,canvas.width,canvas.height);
  var offset = i/20;
  var y = (Math.sin(offset)+1)*canvas.height/2; // y varie de 0 à canvas.height
  var cpy1 = (Math.cos(offset)+0.5)*canvas.height; // les poignées évoluent également de façon sinusoïdale
  var cpy2 = canvas.height - cpy1;
  ctx.beginPath();
  ctx.moveTo(0, y);
  ctx.bezierCurveTo(canvas.width/3, cpy1, 2*canvas.width/3, cpy2, canvas.width, y);
  ctx.stroke();
  i++;
};
setInterval(cycle, 30);
```

[Voir le résultat][9]

### Peaufinage

#### Amélioration du style du trait

Essayez le code suivant :

```javascript
ctx.strokeStyle = 'rgba(80,150,240,0.5)'; // couleur bleu avec opacité de 50%
ctx.lineWidth = 5; // épaisseur de trait de 5 pixels
var i = 0;
var cycle = function() {
  ctx.clearRect(0,0,canvas.width,canvas.height);
  var offset = i/20;
  var y = (Math.sin(offset)+1)*canvas.height/2;
  var cpy1 = (Math.cos(offset)+0.5)*canvas.height;
  var cpy2 = canvas.height - cpy1;
  ctx.beginPath();
  ctx.moveTo(0, y);
  ctx.bezierCurveTo(canvas.width/3, cpy1, 2*canvas.width/3, cpy2, canvas.width, y);
  ctx.stroke();
  i++;
};
setInterval(cycle, 30);
```

[Voir le résultat][10]

#### Ajout de plusieurs courbes

Pour avoir un effet plus accrochant, nous allons ajouter plusieurs courbes de bézier avec un décalage temporel entre elles.

Nous allons également attribuer plusieurs styles aux différentes courbes.

```javascript
var numberOfLines = 5;
var i = 0;
var cycle = function() {
  ctx.clearRect(0,0,canvas.width,canvas.height);
  for(var j=0; j<numberOfLines; ++j) {
    var offset = (i+j*10)/20;
    ctx.lineWidth = 1+2*(numberOfLines-j); // épaisseur variable en fonction de la ligne
    ctx.strokeStyle = 'rgba(80,150,240,'+(j/5+0.1)+')'; // opacité variable en fonction de la ligne
    var y = (Math.sin(offset)+1)*canvas.height/2;
    var cpy1 = (Math.cos(offset)+0.5)*canvas.height;
    var cpy2 = canvas.height - cpy1;
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.bezierCurveTo(canvas.width/3, cpy1, 2*canvas.width/3, cpy2, canvas.width, y);
    ctx.stroke();
  }
  i++;
};
setInterval(cycle, 30);
```

[Voir le résultat][11]

### Aller plus loin

Il est possible de continuer encore plus loin en ajoutant l’**évolution de plusieurs paramètres en fonction du temps**.
Pour conclure, voici la démonstration finale:

```javascript
var i = 0;
var cycle = function() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  for(var j=0; j<numberOfLines; ++j) {
    ctx.lineWidth = 1+2*(numberOfLines-j);
    ctx.strokeStyle = 'rgba(100,200,'+Math.floor(Math.abs(Math.cos(i/80)*256))+','+(j/5+0.1)+')';
    var offset = (i+j*10*Math.abs(Math.cos(i/100)))/20;
    var y = (Math.sin(offset)+1)*canvas.height/2;
    var cpy1 = (Math.cos(offset)+0.5)*canvas.height;
    var cpy2 = canvas.height - cpy1;
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.bezierCurveTo(canvas.width/3, cpy1, 2*canvas.width/3, cpy2, canvas.width, y);
    ctx.stroke();
  }
  i++;
};
setInterval(cycle, 30);
```
[Voir le résultat][12]

Nous avons ajouté:

* L’évolution de la **couleur** au cours du temps.
* L’évolution du **décalage entre les courbes** au cours du temps.
