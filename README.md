# Logigramme

Éditeur de logigrammes dans le navigateur, avec une direction artistique sombre : blocs à bordure colorée, étiquettes en capitales monospace, losanges de décision, pastilles sur les liaisons et tracés orthogonaux.

Aucune dépendance, aucun serveur, installable comme application de bureau. Les données restent sur ton poste.

## Fonctionnalités

- **4 types d'éléments** : bloc (étiquette + titre + sous-titre), décision (losange), pastille, texte libre
- **Liaisons orthogonales** avec flèche, libellé en pastille, trait plein ou pointillé, couleur au choix
- **Poignées** pour décaler un segment (boucles de retour, embranchements)
- **Création rapide** : sélectionner un bloc puis `B` ou `D` ajoute l'étape suivante déjà reliée ; glisser un point de connexion dans le vide crée un nouveau bloc relié
- Grille magnétique, guides d'alignement, alignement et répartition multi-sélection
- Annuler / rétablir, copier / coller, dupliquer
- Sauvegarde automatique (localStorage), enregistrement et ouverture en `.json`
- Export **PNG** (x2) et **SVG**

## Utilisation

Ouvrir `index.html` dans un navigateur. Au premier lancement la page est vierge ; le bouton **Exemple** charge un logigramme de démonstration.

| Action | Raccourci |
|---|---|
| Bloc / Décision / Pastille / Texte | `B` / `D` / `P` / `T` |
| Bloc à un endroit précis | double-clic sur le fond |
| Modifier le texte | `Entrée` ou double-clic |
| Supprimer | `Suppr` |
| Dupliquer | `Ctrl+D` |
| Copier / coller | `Ctrl+C` / `Ctrl+V` |
| Annuler / rétablir | `Ctrl+Z` / `Ctrl+Y` |
| Enregistrer en .json | `Ctrl+S` |
| Déplacer finement | flèches (`Maj` pour x2) |
| Sélection rectangle | `Maj` + glisser sur le fond |
| Recadrer / grille | `F` / `G` |
| Zoom / déplacement | molette / glisser le fond |

Les titres et sous-titres acceptent plusieurs lignes (retour à la ligne dans le champ).

## Mise en ligne avec GitHub Pages

1. Créer un dépôt et y pousser `index.html` (et ce README) :
   ```bash
   git init
   git add .
   git commit -m "Logigramme"
   git branch -M main
   git remote add origin https://github.com/<utilisateur>/logigramme.git
   git push -u origin main
   ```
2. Sur GitHub : **Settings > Pages > Build and deployment**, source **Deploy from a branch**, branche `main`, dossier `/ (root)`.
3. L'app est disponible sur `https://<utilisateur>.github.io/logigramme/` après une minute.

## Installer comme application de bureau

Une fois le site en ligne sur GitHub Pages, l'app s'installe comme une application normale : elle a sa propre fenêtre, son icône dans le menu Démarrer et la barre des tâches, et elle fonctionne hors ligne.

- **Chrome / Edge** : ouvrir l'URL GitHub Pages puis cliquer sur **⬇ Installer l'app** dans la barre de l'app (ou sur l'icône d'installation dans la barre d'adresse).
- **Edge** : aussi possible via **… > Applications > Installer ce site en tant qu'application**.
- **macOS Safari** : **Fichier > Ajouter au Dock**.

Aucun droit administrateur n'est nécessaire. Pour désinstaller : clic droit sur l'icône, ou `edge://apps` / `chrome://apps`.

Les mises à jour poussées sur GitHub sont récupérées automatiquement au lancement suivant (si une connexion est disponible).

## Version bureau Windows (Rust)

Une version native, construite avec [Tauri](https://tauri.app) (Rust), est publiée dans les **Releases** du dépôt :

- `Logigramme_x.y.z_x64-setup.exe` : installeur, sans droits administrateur (installation dans le profil utilisateur)
- `Logigramme-portable.exe` : exécutable seul, rien à installer

Même interface que la version web, mais **aucune donnée dans le stockage du navigateur** :

- la session en cours est écrite par Rust dans `%LOCALAPPDATA%\io.github.onlyv.logigramme\session.json` (dossier local, non synchronisé avec un profil itinérant) ;
- **Ouvrir** / **Enregistrer** / exports PNG et SVG passent par les boîtes de dialogue natives de Windows ;
- `Ctrl+S` réenregistre dans le fichier ouvert, `Ctrl+Maj+S` enregistre sous un autre nom, `Ctrl+O` ouvre ;
- aucune connexion réseau.

L'exécutable n'est pas signé : au premier lancement, Windows SmartScreen peut afficher « Windows a protégé votre ordinateur » (**Informations complémentaires > Exécuter quand même**). Il nécessite WebView2, présent d'origine sur Windows 10 et 11.

Pour publier une nouvelle version : pousser un tag `vX.Y.Z` (le workflow `.github/workflows/windows.yml` compile et crée la release). Compilation locale : `mkdir dist && cp index.html dist/ && cd src-tauri && cargo tauri build`.

## Ouverture et sauvegarde

- Premier lancement : page vierge.
- Lancements suivants : l'app rouvre le dernier logigramme en cours d'édition, avec le même zoom.
- Pour garder plusieurs logigrammes, utiliser **Enregistrer** (fichier `.json`) et **Ouvrir**.

> La sauvegarde automatique est liée au navigateur et à l'adresse du site. Vider les données du site efface le logigramme en cours : pensez à enregistrer en `.json` ce qui compte.

## Personnaliser la DA

Tout est en haut du script dans `index.html` :

- `C` : couleurs de fond, de remplissage des blocs, des textes et des traits
- `PALETTE` : couleurs proposées pour les bordures et étiquettes
- `SANS` / `MONO` : polices (les polices système sont utilisées pour que l'export PNG soit identique à l'écran)

## Format des fichiers

```json
{
  "title": "Mon logigramme",
  "nodes": [
    { "id": "n1", "type": "box", "x": 0, "y": 0, "w": 360, "color": "#8b8cf6",
      "tag": "Déclencheur", "title": "Chaque matin à 8h", "sub": "", "dashed": false }
  ],
  "edges": [
    { "id": "e1", "from": "n1", "fromPort": "bottom", "to": "n2", "toPort": "top",
      "label": "Oui", "labelColor": "#6cc47f", "color": "#6b7280", "dashed": false }
  ]
}
```

`type` : `box`, `diamond`, `pill`, `text`. Ports : `top`, `right`, `bottom`, `left`. `x` / `y` désignent le centre de l'élément, `w: 0` signifie largeur automatique.

## Licence

MIT
