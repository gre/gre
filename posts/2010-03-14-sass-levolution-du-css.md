---
title: "SASS : l'évolution du CSS pour Play, Rails ou autres"
author: Gaetan
layout: post
permalink: /2010/03/sass-levolution-du-css/
tags:
  - css
  - sass
---

[1]: http://sass-lang.com/
[2]: http://www.playframework.org/
[3]: http://compass-style.org/
[4]: http://sass-lang.com/docs/yardoc/file.SASS_REFERENCE.html

**SASS, Syntactically Awesome Stylesheets**, est un langage de feuille de style évolué qui permet de factoriser beaucoup de code css et de rendre son écriture et sa maintenance **rapide et moins contraignante**. Il est compilé en css.

<!--more-->

## Pourquoi utiliser SASS ?

Son utilisation a de nombreux avantages par rapport au CSS :

- sa **simplicité** (pas de crochets, pas de point virgule mais juste de l’indentation)
- l’**imbrication** des sélecteurs css (appliquant l’idée DRY : don’t repeat yourself)
- l’utilisation de **variables**
- l’utilisation d’**opérations élémentaires** (sur les pixels, les couleurs, …)
- la **factorisation** du code (au lieu de faire des copier-coller, on peux factoriser le code à travers les “mixins”).
- La **réduction** css et la **clarté** du code
- La **compression du code** compilé avec la possibilité de tout mettre dans un fichier (via l’héritage) et de minimifier le code css.

Ce langage n’est pas difficile à apprendre, cela ressemble au css, avec de nombreuses fonctionnalités intéressantes en plus.

## La syntaxe du langage

La syntaxe du sass est **compatible avec celle du css à quelques exceptions près** :

- Ne plus mettre de point virgule **;**
- Ne plus mettre de crochets **{ }**
- Respecter les conventions traditionnelles (**attribut: valeur** un espace après le deux points mais pas avant)
- Respecter l’indentation : Il faut choisir une indentation et s’y tenir dans un même fichier. Au choix : une tabulation, 2 espaces, 4 espaces, … Les lignes _propriétés: valeurs_ d’un sélecteur css doivent dépasser d’une indentation ce sélecteur.

En respectant ces points, vous pouvez déjà **coder en SASS comme en CSS**.

Mais cela ne serait pas intéressant sans les nouveautés suivantes :

### La factorisation des sélecteurs en plusieurs niveaux

Au lieu d’avoir ce type d’arborescence à un niveau :

```sass
.main .head
  color: red
.main .body
  color: blue
```

Nous pouvons factoriser le sélecteur “_.main_” et se ramener à deux niveaux :

```sass
.main
  .head
    color: red
  .body
    color: blue
```

Ce procédé de factorisation basé sur l’esprit **DRY** (Don’t Repeat Yourself) est aussi applicable sur les attributs eux-mêmes :

```sass
a
  font:
    family: serif
    weight: bold
    size: 1.2em
```

sera compilé en css par :

```css
a {
  font-family: serif;
  font-weight: bold;
  font-size: 1.2em;
}
```

### Les variables

La possibilité d’utiliser des variables est un gros apport au css. Elle permet **une meilleure maintenance du code et une meilleure scalabilité d’une application** (en utilisant par exemple des fichiers sass de thèmes définissant toutes les couleurs, images, polices, …).  
Il existe plusieurs **types de variables** (nombre réel, pixels, couleurs, chaines de caractères …) et il est possible d’utiliser des **opérations élémentaires**.

Lorsqu’on écrit une ligne **attribut / value** avec l’utilisation de variables (dynamique),  
on utilise le caractère ‘**=**‘ au lieu de ‘**:**‘ pour l’affectation.

#### Les couleurs

```sass
!link_color = red
a
  color = !link_color
  &:hover
    color = !link_color #222
```

A noter que le symbole **&** remplace le sélecteur parent.

Ce qui donne le code compilé suivant :

```css
a {
  color: red;
}
a:hover {
  color: #ff2222;
}
```

#### Les pixels

```sass
!margin = 16px
.border
  padding = !margin / 2
  margin = !margin / 2
```

donne le code compilé :

```css
.border {
  padding: 8px;
  margin: 8px;
}
```

### Les “mixins”

Les mixins sont des procédures qui contiennent plusieurs lignes de sass.  
Il est possible d’utiliser des arguments sur ces mixins.

```sass
=border-radius(!radius = 5px)
  border-radius= !radius
  -moz-border-radius= !radius
  -webkit-border-radius= !radius
```

```sass
#wrapper
   border-radius(10px)
  > footer
     border-radius()
```

Cet exemple est typiquement intéressant car il permet d’utiliser **border-radius** de façon **cross-browser** et avec une ligne de code.

A noter qu’il est possible d’affecter des valeurs par défaut aux mixins.

Le résultat css compilé est le suivant :

```css
#wrapper {
  border-radius: 10px;
  -moz-border-radius: 10px;
  -webkit-border-radius: 10px;
}
#wrapper > footer {
  border-radius: 5px;
  -moz-border-radius: 5px;
  -webkit-border-radius: 5px;
}
```

### Exemple complet

Voici un exemple complet de l’utilisation du SASS

```sass
/* This is just an example */

/* variables */
!main_width = 900px
!aside_width = 300px
!section_width = 520px

!link_color = red

!font_title = "Liberation","Georgia","serif"

/* mixins */
=border-radius(!radius = 5px)
  border-radius= !radius
  -moz-border-radius= !radius
  -webkit-border-radius= !radius

=block()
  display: block
  overflow: auto

/* colors */
a
  color = !link_color
  &:hover
    color = !link_color #222


/* layout */

#wrapper
  margin:  auto
  position: relative
  width = !main_width
   border-radius(10px)
  >nav
     block()
    padding: 2px
      top: 5px
    font-size: 1.2em
    font-family = !font_title
    a
      font-weight: bold
      &:hover
        color: white

  >header
     block()
    clear: both
    height: 48px
    font-family = !font_title

  >footer
     block()
    padding: 5px
    text-align: center
    clear: both

  #main
    position: relative
    >section
       block()
      width = !section_width
      padding: 20px

    >aside
       block()
      float: right
      width = !aside_width
      padding: 20px
```

et le résultat du fichier CSS compilé

```css
/* This is just an example */
/* variables */
/* mixins */
/* colors */
a {
  color: red;
}
  a:hover {
    color: #ff2222;
}

/* layout */
#wrapper {
  margin:  auto;
  position: relative;
  width: 900px;
  border-radius: 10px;
  -moz-border-radius: 10px;
  -webkit-border-radius: 10px;
}
  #wrapper >nav {
    display: block;
    overflow: auto;
    padding: 2px;
    padding-top: 5px;
    font-size: 1.2em;
    font-family: Liberation, Georgia, serif;
}
    #wrapper >nav a {
      font-weight: bold;
}
      #wrapper >nav a:hover {
        color: white;
}
  #wrapper >header {
    display: block;
    overflow: auto;
    clear: both;
    height: 48px;
    font-family: Liberation, Georgia, serif;
}
  #wrapper >footer {
    display: block;
    overflow: auto;
    padding: 5px;
    text-align: center;
    clear: both;
}
  #wrapper #main {
    position: relative;
}
    #wrapper #main >section {
      display: block;
      overflow: auto;
      width: 520px;
      padding: 20px;
}
    #wrapper #main >aside {
      display: block;
      overflow: auto;
      float: right;
      width: 300px;
      padding: 20px;
}
```

Le SASS offre **encore plus de possibilités**, notamment l’interpolation, les conditions, les boucles, …  
Vous trouverez plus d’informations sur la _documentation SASS_.

## Utilisation

### Pré-requis

**Note**: Ce qui suit ne s’applique pas pour le plugin sass du framework Play! .

Pour utiliser SASS, sous linux, installez les packets **ruby** et **rubygems** puis installez **haml** avec la commande :

```bash
gem install haml
```

### Avec le framework java Play!

Grâce au module sass de play framework, le SASS est **compilé à la volée** au moment du chargement d’une page (en mode développement) ou au chargement de l’application (en mode production).

#### Installation

Depuis play 1.1, il suffit de lancer la commande

```bash
play install sass
```

Ensuite il faut activer le module dans la configuration de l’application (fichier _conf/application.conf_).

### Avec le framework Ruby on Rails

#### Installation

Pour activer le plugin SASS sur une application Rails, lancez :

```bash
haml --rails path/to/rails/app
```

### Autrement

Vous pouvez toujours utiliser SASS en compilant vos fichier sass en css à chaque modification (voir _Commandes pratiques_).  
Vous inclurez ensuite le fichier css compilé dans votre html.

## Convertir ses anciens CSS en SASS

Si vous ne voulez pas repartir de zéro dans le design d’un projet, vous pouvez tout à fait repartir avec les anciens CSS en les exportant en SASS.

## Commandes pratiques

- Pour convertir vos fichier CSS en SASS il vous suffit d’utiliser : `css2sass`

- Pour compiler vos fichier SASS en CSS, utilisez: `sass`

## Liens

- [Site du langage SASS][1]
- [Site du framework Play!][2]

### Aller plus loin

- [Compass : framework SASS][3]
- [Documentation SASS][4]
