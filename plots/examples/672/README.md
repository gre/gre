---
date: "2022-09-25"
title: "Néons (artrush event)"
image: /images/plots/672.jpg
description: ""
tags:
  - artrush
---

> Ce weekend du 24 Septembre 2022 a eu lieu le premier [évènement "Art Rush"](https://twitter.com/ArtRushEvent) organisé par des streamers de la communauté Twitch FR Art. Ce fût l'occasion de faire de superbes rencontres et également de pouvoir approfondir ma technique et partager mon travail. Le thème était "Années 80".

J'ai réalisé cette scène tracée au stylo plume sur papier aquarelle grand format (70 par 50 centimètres), en utilisant 6 encres Diamine: _Bloody Brexit, Amber, Imperial Purple, Soft Mint, Turquoise, Flamingo Pink_.

Plusieurs variantes, petits formats, ont également été produites:

<img src="/images/plots/672-a6-plots.jpg" width="100%"/>

### Réalisé au robot traceur (plotter)

Comme tout mes travaux, un traceur a été utilisé pour dessiner tout les traits. Cela a pris approximativement 10 heures.

<video src="/images/plots/672-timelapse.mp4" width="100%" controls autoplay muted loop></video>

### L'art génératif?

Comme cet aperçu le montre, une infinité de variantes aurait pu être physiquement réalisé:

<video src="/images/plots/672-digital.mp4" width="100%" controls autoplay muted loop></video>

Cette technique s'appelle l'art génératif: j'ai en fait écris un programme qui utilise l'aléatoire pour placer les néons et construire les montagnes.

Ce programme est développé avec du "code créatif" (en Rust) et va utiliser un RNG, des algorithmes, pour constituer tout les éléments de la scène finale.

Il sauvegarde un fichier vectoriel (.SVG) qui peut ensuite être donné au robot pour tracer tout les traits au stylo plume.

<img src="/images/plots/672-curation.jpg" width="100%"/>

Le rendu digital est ainsi seulement théorique car les encres utilisés dans les contraintes du physique vont enrichir la réalisation. Théorique VS Physique:

<img src="/images/plots/672-digital.jpg" width="50%"/><img src="/images/plots/672.jpg" width="50%"/>

Vidéo résumé:

<iframe width="100%" height="400" src="https://www.youtube.com/embed/FMJcYPXj8-4?rel=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

## Making of

### Vendredi: 2 prototypes pour explorer la texture des "Néons"

Je me suis d'abord concentré sur les encres qui serait intéressant pour représenter le thème "années 80". Deux prototypes sont réalisé pour tester ces couleurs:

<img src="/images/plots/668.jpg" width="100%"/>

Ce premier prototype explore le pavage de cercles concentriques dans des zones rectangulaires différentes. La superposition de plusieurs pavages (déjà exploré dans [/plots/667](/plots/667)) offre plus de profondeur et de densité et permet de jouer avec le désordre.

Un deuxieme prototype est créé en reprenant ce même principe mais sur de tout petit cercles remplis de spirales:

<img src="/images/plots/670.jpg" width="100%"/>

Le résultat est assez surprenant car il y a un certain ordre dans ce placement aléatoire.

<video src="/images/plots/670-timelapse.mp4" width="100%" controls autoplay muted loop></video>

> Pour la petite anecdote, une partie d'une roue de mon traceur s'est cassé pendant cette réalisation!

### Samedi: réalisation du générateur

Le samedi a été crutial car c'est le jour où j'ai composé la scène finale. L'idée s'inspire du meme "Hacker Man" et principalement du premier film "TRON".

<img src="/images/plots/672-draft.jpg" width="100%"/>

> Savez vous que le bruit de perlin (Perlin Noise) a été conçu par Ken Perlin pour les textures du film TRON? L'utilisation de telle texture procédurale est un outil fondamental dans l'art génératif, et c'est ce qui me permet ici de lever mes montagnes.

Un générateur est développé (environ 3 heures investis à l'écrire), programme qui me permet de générer une infinité de variations.

### Dimanche: sélection d'un variant, réalisation physique

Le dernier jour est consacré à finaliser la scène, il y a une sélection artistique à faire pour choisir une des variations qui me semble intéressant (subjectivement), j'ai choisi le numéro 2772 qui sera donc "l'élu" présenté plus haut. (après donc avoir généré 2771 précédents variations)

Le papier que j'ai utilisé a été au début assez compliqué car il n'absorbait pas suffisamment l'encre:

<img src="/images/plots/672-begin.jpg" width="100%"/>

J'ai dû repassé une deuxième fois pour améliorer le soleil, la version finale a toujours un peu le trait visible, c'est aussi ce qui fait le charme du médium.

Pendant que le grand tracé (70x50cm) était en cours de traçage, il fût intéressant d'éprouver le générateur sur un format plus petit: voici 4 variantes sur format carte postale:

<img src="/images/plots/672-a6-plots.jpg" width="100%"/>

Il est intéressant de constater le comportement des encres. Voici quelques zooms:

<img src="/images/plots/672z-a6.jpg" width="100%"/>
<img src="/images/plots/672z1.jpg" width="100%"/>
<img src="/images/plots/672z2.jpg" width="100%"/>
<img src="/images/plots/672z3.jpg" width="100%"/>
<img src="/images/plots/672z4.jpg" width="100%"/>

### Conclusion

J'ai été très satisfait de l'organisation de "Art Rush". J'ai aimé la bonne ambiance et les échanges que j'ai pu avoir en découvrant aussi d'autres artistes aux techniques très variés. J'ai hâte de revoir les VOD car j'étais très plongé dans ma propre créa.

Je vous invite à regarder les contenus partagés par https://twitter.com/ArtRushEvent
