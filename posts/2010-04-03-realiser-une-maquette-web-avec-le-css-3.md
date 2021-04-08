---
title: Réaliser une maquette web avec le CSS 3
author: Gaetan
layout: post
permalink: /2010/04/realiser-une-maquette-web-avec-le-css-3/
tags:
  - css
  - sass
---

 [1]: http://compass-style.org/
 [2]: /2010/03/sass-levolution-du-css
 [3]: /images/2010/css3/exemple_border_radius.png
 [4]: /images/2010/css3/exemple_gradient.png
 [5]: /images/2010/css3/exemple_text_shadow.png
 [6]: /images/2010/css3/exemple_box_shadow.png
 [7]: /images/2010/css3/university_nostalgia_exemple.png
 [8]: http://github.com/gre/University-nostalgia
 [9]: /images/2010/css3/triangle_01.png
 [10]: /images/2010/css3/triangle_02.png
 [11]: /images/2010/css3/triangle_03.png
 [12]: /images/2010/css3/triangle_menus.png

Le CSS depuis sa version 3 constitue une **bonne évolution technologique pour palier l’utilisation abusive d’images** dans la réalisation d’une application web.

Il n’est ainsi plus nécessaire de recourir à des images pour réaliser des **ombres**, des **dégradés**, des **bordures arrondis** ou encore utiliser des **polices spécifiques**.

L’avenir du CSS3 nous promet encore plus : il est d’ores et déjà possible sur certains navigateurs d’effectuer des **animations**, des **transitions**, des **effets de reflets**,… sans devoir recourir au *javascript*, au *canvas* ou, pire encore, au *flash*.

<!--more-->

## Généralités

### Pourquoi ?

Quelques raisons pour utiliser le CSS 3…
  
#### dans une optique de maintenabilité

En utilisant des images pour réaliser des effets simples, **il faut mettre à jour l’image à chaque fois**. Si bien qu’il est nécessaire d’utiliser un logiciel de dessin et de garder les sources pour pouvoir facilement modifier les couleurs à l’avenir.  
**La modification d’un thème est alors pénible** : Pour chaque image du thème : il faut ouvrir la source de l’image avec son logiciel préféré (GIMP, Photoshop, …), modifier la couleur, exporter l’image, recharger la page,… A contrario, **il ne suffit que de modifier une ligne de code CSS pour changer sa couleur** (l’utilisation de variable avec SASS est intéréssant).

#### par simplicité

Le CSS 3 **offre plus de possibilités** donc **simplifie bien des tâches** qui étaient complexe à réaliser auparavant. Bordures arrondis, ombres, dégradés, animations, reflets,… sont maintenant facilement réalisables avec le CSS 3.

Voici un exemple frappant parmi tant d’autres :  
Auparavant, on voyait fleurir à foison des tableaux pour réaliser de simples bordures arrondis.

##### On voyait auparavant

```html
<table class="radius">
<tr>
<td class="topleft"></td>
<td class="top"></td>
<td class="topright"></td>
</tr>
<tr>
<td class="left"></td>
<td class="content">content here...</td>
<td class="right"></td>
</tr><tr>
<td class="bottomleft"></td>
<td class="bottom"></td>
<td class="bottomright"></td>
</tr>
</table>
<style>
table.radius .topleft {
background: url(topleft.png);
width: 20px;
height: 20px;
}
/* DE MEME pour les 8 autres cases ... */
</style>
```

##### Maintenant avec le CSS3

```html
<div class="radius">content here...</div>
<style>
.radius {
  border-radius: 20px;
}
</style>
```

#### dans un objectif de scalabilité des thèmes

L’utilisation des possibilités du CSS 3 au lieu d’images va permettre d’**étendre rapidement son application web à plusieurs thèmes** et de modifier facilement ces thèmes. Il suffira d’avoir un fichier pour chaque thème.

### SASS et le framework Compass

Afin de résoudre les problèmes de *cross-navigateur* abordés précédemment, je vous conseille l’utilisation du **SASS** et du framework [Compass][1].  
Pour plus de renseignements sur le SASS, n’hésitez pas à lire [cet article][2].

### Principales nouveautés du CSS 3

Dans la suite de cet article, **pour plus de simplicités, nous nous arrêtons à la compatibilité de Firefox** (préfixé par -moz) mais bien entendu, il est possible de les rendre compatibles avec tous les navigateurs récents. Comme précisé avant, l’utilisation du framework *Compass* permet entre autres de résoudre ce problème.

#### Les bordures arrondis

![][3]

```css
.exemple {  
  -moz-border-radius: 10px;  
  border: 1px solid black;  
  padding: 2px 5px;  
}  
```

#### Les dégradés

Compatibles depuis Firefox 3.6.  
![][4]

```css
.exemple {  
  -moz-border-radius: 10px;  
  border: 1px solid black;  
  padding: 2px 5px;  
  background: -moz-linear-gradient(-90deg, green, yellow);  
}  
```

#### Les ombres

##### Sous les textes

![][5]


```css
.exemple {  
  -moz-border-radius: 10px;  
  border: 1px solid black;  
  padding: 2px 5px;  
  background: -moz-linear-gradient(-90deg, green, yellow);  
  text-shadow: -1px -1px 1px yellow;  
}
```


##### Sous les éléments

![][6]

```css
.exemple {  
  -moz-border-radius: 10px;  
  border: 1px solid black;  
  padding: 2px 5px;  
  background: -moz-linear-gradient(-90deg, green, yellow);  
  text-shadow: -1px -1px 1px yellow;  
  -moz-box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.5);  
}  
```

A noter d’ailleurs que la couleur de format **rgba(r, g, b, a)** est une nouveauté du CSS 3.

### Optimiser la compatibilité avec les navigateurs plus anciens

Voici une utilisation intelligente des dégradés (avec SASS) pour permettre une meilleure **compatibilité** avec, en dernier recours, une couleur ou une image alternative :

```sass
=vertical-gradient(!from, !to, !alt=(!from/2   !to/2))  
  background = !alt  
  background = -webkit-gradient(linear, left top, left bottom, from(!from), to(!to))  
  background = -moz-linear-gradient(-90deg, !from, !to)  
  
/* exemples d'utilisation : */  
#main > header  
   vertical-gradient(#719369, #587451, url(header.png))  
  
#main > header  
   vertical-gradient(green, yellow, #80FF80)
```

## Exemple de maquette

Voici un **exemple d’utilisation du CSS3** utilisé sur un projet récent (réalisé à l’université).  

![][7]

Il n’y a aucune image (sauf l’avatar tux) et l’application est entièrement compatible avec au minimum Firefox et Chrome.  
Si l’application vous intéresse, elle est sur [github][8].


### Extrait du code SASS

```sass
@import compass.sass  
@import compass/reset.sass  
@import theme.sass  
  
=vertical-gradient(!from, !to, !alt=(!from/2   !to/2))  
  background = !alt  
  background = -webkit-gradient(linear, left top, left bottom, from(!from), to(!to))  
  background = -moz-linear-gradient(-90deg, !from, !to)  
  
!back = #e9f3e7  
!back_aside = #F8FAF5  
!back\_header\_from=#719369  
!back\_header\_to=#587451  
!back\_footer\_from=#b4c1b1  
!back\_footer\_to=#9da89a  
  
#wrapper  
   border-radius(!global\_border\_radius)  
  :border 1px solid !back-#111  
  >nav  
     border-top-radius(10px)  
    :background white  
  >header  
     vertical-gradient(!back\_header\_from, !back\_header\_to, url(/public/images/header.png))  
    :color white  
    :z-index 1  
  >footer  
     vertical-gradient(!back\_footer\_from, !back\_footer\_to)  
     border-bottom-radius(10px)  
  
  #main  
    background = !back_aside  
     box-shadow(0px, -1px, 2px, rgba(,,,0.25))  
    :z-index 2  
    >section  
      :background white
```

### Un triangle en css ???

Ce n’est pas de la magie mais **il est possible de faire des triangles en CSS** (même en CSS2) grâce à une petite astuce. Explication en images :

#### Une boite avec une couleur pour chaque bordure

![][9]

```css
.box {  
  background: black;  
  width: 40px;  
  height: 40px;  
  border-width: 15px;  
  border-color: yellow red blue green;  
  border-style: solid;  
}
```


#### width et height à 0

![][10]

#### Bordures transparentes

![][11]

```css
.box {  
  background: black;  
  width: 0;  
  height: 0;  
  border-width: 15px;  
  border-color: transparent red transparent transparent;  
  border-style: solid;  
}  
```

#### Autre utilisation des triangles

Le même procédé m’a permis de faire ceci :  
![][12]

