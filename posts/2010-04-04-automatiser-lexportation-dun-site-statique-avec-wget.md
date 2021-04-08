---
title: "Automatiser l'exportation d'un site statique avec wget"
author: Gaetan
layout: post
permalink: /2010/04/automatiser-lexportation-dun-site-statique-avec-wget/
tags:
  - playframework
---

Il est préférable d’**utiliser un framework web** même si l’on veut réaliser un **site vitrine simple (statique)**, car l’on bénéficie des avantages du framework notamment de l’héritage (entre autre) des templates, de l’internationalisation, de la configuration des _routes_, du _SASS_, etc.

De plus, cela permet de rendre la maintenabilité moins longue et couteuse.

Néanmoins, son hébergeur ne permet pas toujours de faire tourner son site avec le framework utilisé (par exemple _ruby on rails_ ou _play! framework_).

Pour remédier à ce problème, il suffit d’**exporter son site statique en simples pages HTML**.

Cet article est donc destiné aux particuliers ne voulant pas investir dans un serveur dédié, fans de framework avancés qui ne sont pas supportés par les offres d’entrée de gamme des hébergeurs (qui n’offrent généralement que le support du php).

**D’ailleurs, pour ce type de site, pourquoi utiliser un framework MVC si l’on utilise que des vues ?**

## La démarche

### Le site web

**Réalisez le site** avec votre framework préféré et faites en sorte que la page principale (**/**) **permette l’accès à toutes les pages du site** par des liens (pas forcément directs).

### Exportation

Avec **wget** récupérez votre site à la ligne de commande :

```bash
wget -r -k -np "http://localhost:9000/"
```

A l’instar d’un moteur de recherche, c’est cette commande qui va s’occuper de **retracer toutes les pages web de votre site**, mais aussi toutes ses ressources (images, css, js, ..). En plus de cela, elle va **préserver le fonctionnement des liens** entre pages.

Adaptez _http://localhost:9000/_ à l’url de votre site.

Cela va créer le dossier _localhost:9000_, il ne vous restera plus qu’à l’envoyer sur votre serveur http.

## Internationalisation

Voici une solution pour exporter son internationalisation.

Elle consiste à rediriger l’utilisateur, depuis la page principale, sur la bonne page internationalisée en fonction de sa langue (indiquée par le navigateur), tout **en gardant les fichiers publics en commun** (css, images).

### Préparer ses routes

préparez les uri pour qu’elles soient de la forme **/{lang}/\*/** avec **lang** désignant la langue désirée.

#### Exemples de routes

```
/en/
/en/about/
/fr/
/fr/about/
```

### Ajouter des liens dans les templates pour changer de langue

Cette action est délicate car, comme précisé auparavant, **wget** essaye de retracer tous les liens entre pages.

Ainsi, si l’on crée des liens entre les différentes langues, **wget** créera un dossier _public_ pour chaque langue internationalisée.

Pour contourner ce problème et ainsi factoriser les fichiers communs, **une solution est de créer les liens à posteriori**.

#### Solution exemple avec play framework

```
<p>
#{if lang!='en'}
  <a ADD_HERE_ENGLISH_LINK_ATTRIBUTES>english version</a>
#{/if}
#{if lang!='fr'}
  <a ADD_HERE_FRENCH_LINK_ATTRIBUTES>french version</a>
#{/if}
</p>
```

```bash
#!/bin/bash
# script bash
wget -r -k -np "http://localhost:9000/"
sed -i s/ADD_HERE_ENGLISH_LINK_ATTRIBUTES/href=\"\\/en\\/\"/g $(find . -name "*.html")
sed -i s/ADD_HERE_FRENCH_LINK_ATTRIBUTES/href=\"\\/fr\\/\"/g $(find . -name "*.html")
```

### Préparer une page de redirection

**Placez à posteriori un script de redirection à l’url /** pour rediriger vers la page adéquat.

#### Par exemple un script php

```php
<?php
$langs = array();

if (isset($_SERVER['HTTP_ACCEPT_LANGUAGE'])) {
    preg_match_all('/([a-z]{1,8}(-[a-z]{1,8})?)\s*(;\s*q\s*=\s*(1|0\.[0-9]+))?/i', $_SERVER['HTTP_ACCEPT_LANGUAGE'], $lang_parse);
    if (count($lang_parse[1])) {
        $langs = array_combine($lang_parse[1], $lang_parse[4]);
        foreach ($langs as $lang => $val) {
            if ($val === '') $langs[$lang] = 1;
        }
        arsort($langs, SORT_NUMERIC);
    }
}

foreach ($langs as $lang => $val) {
  if (strpos($lang, 'fr') === 0) {
    header('Location: fr');
    die();
  }
  else if (strpos($lang, 'en') === 0) {
    header('Location: en');
    die();
  }
}
header('Location: en');
?>
```
