# Configuration Neovim avec `vim.pack`

Cette configuration demande Neovim 0.12 ou plus récent. L’ancienne
configuration LazyVim reste intacte dans `lazy-old/` pendant la migration.

Le thème est `tokyonight-moon`, le style par défaut de LazyVim. Treesitter
assure la coloration syntaxique structurée pour les langages configurés dans
`lua/config/plugins.lua`.

## Premier démarrage

Lancer `nvim`. `vim.pack` propose d’installer les plugins au premier démarrage.
Accepter, puis redémarrer Neovim. Le fichier `nvim-pack-lock.json` créé ensuite
doit être conservé avec la configuration : il fixe les révisions des plugins.

Pour mettre les plugins à jour :

```vim
:lua vim.pack.update()
```

Examiner les changements, puis `:write` pour confirmer ou `:quit` pour annuler.
Après une mise à jour de Treesitter, mettre aussi les parseurs à jour avec
`:TSUpdate`.

## Navigation

| Raccourci | Action |
|---|---|
| `<Espace>ff` | rechercher un fichier |
| `<Espace>/` | rechercher du texte avec ripgrep |
| `<Espace>sr` | panneau de recherche/remplacement avec filtre de fichiers |
| `<Espace>fw` | rechercher le mot sous le curseur |
| `<Espace>fb` | choisir un buffer ouvert |
| `<Espace>fr` | ouvrir un fichier récent |
| `<Espace>e` | afficher/masquer l’explorateur à gauche |
| `-` | ouvrir le dossier courant avec Oil |
| `<C-s>` | mode normal, formatage, sauvegarde et lint |
| `<Espace>cd` | afficher l’erreur sous le curseur |
| `[e` / `]e` | erreur suivante / précédente dans le fichier courant |
| `<Espace>xx` | afficher/masquer tous les diagnostics connus dans Trouble |
| `<Espace>xX` | afficher/masquer les diagnostics du fichier courant dans Trouble |
| `H` / `L` | buffer précédent / suivant |
| `<Espace>bd` | fermer le buffer courant |
| `<Espace>bo` | fermer tous les autres buffers et conserver le courant |
| `<C-h/j/k/l>` | se déplacer entre les fenêtres |
| `<Espace>ss` | symboles du fichier courant |
| `:` | palette de commandes centrée avec suggestions |
| `]h` / `[h` | changement Git suivant / précédent |
| `<Espace>ghp` | aperçu du changement Git courant |
| `<Espace>gd` | aperçu du changement Git courant directement dans le fichier |
| `<Espace>ghs` | placer le changement courant dans le stage |
| `<Espace>ghr` | annuler le changement courant |
| `<Espace>ghb` | auteur et commit de la ligne courante |
| `<Espace>cp` | ouvrir/fermer l’aperçu Markdown dans le navigateur |

Les raccourcis `<Espace>` s’utilisent en mode normal : appuyer sur `<Esc>`,
puis sur les touches successivement. Par exemple, `<Espace>bd` correspond à
`Espace`, puis `b`, puis `d`, avec au maximum une seconde entre les touches.
Si le buffer contient des modifications non sauvegardées, une confirmation
permet de sauvegarder, abandonner les modifications ou annuler la fermeture.

Le curseur est animé par `smear-cursor.nvim`. La commande
`:SmearCursorToggle` permet d’activer ou désactiver temporairement l’effet.
Le défilement de la fenêtre est également animé par Snacks, notamment avec
`<C-f>`, `<C-b>`, `<C-d>`, `<C-u>` et la molette.

`<Espace>sr` ouvre le même panneau Grug Far que LazyVim. Il comporte des champs
séparés `Search`, `Replace`, `Files Filter` et `Flags`. Le filtre est prérempli
avec l'extension du fichier courant ; par exemple `*.lua`, `*.{ts,tsx}` ou
`!dist/**` permettent de limiter les fichiers parcourus par ripgrep.

Dans Telescope : `<C-n>` / `<C-p>` se déplacent, `<Entrée>` ouvre, `<Esc>`
ferme. Dans l’explorateur, `<Entrée>` ou `l` ouvre un fichier ou un dossier et
`h` remonte d’un niveau. Dans Oil, modifier les lignes puis écrire avec `:write` applique les
créations, déplacements ou suppressions de fichiers.

## Panneau des diagnostics

Trouble regroupe les erreurs et avertissements par fichier et actualise la
liste automatiquement. `<Espace>xx` ouvre ou ferme la vue globale ;
`<Espace>xX` limite la vue au fichier courant. Utiliser `<C-j>` pour rejoindre
le panneau inférieur, `<Entrée>` pour accéder au diagnostic et `q` pour fermer.

La liste contient les diagnostics remontés par les LSP et les linters.
L’analyse de tous les fichiers du projet, même jamais ouverts, dépend du
serveur ou du linter utilisé : Trouble ne lance pas cette analyse lui-même.

## Langages et LSP

Deux éléments sont distincts :

1. `nvim-lspconfig` fournit à Neovim la configuration connue d’un serveur ;
2. le serveur lui-même doit être installé sur le système et présent dans
   `$PATH`.

Les serveurs configurés, dont `clangd`, `rust-analyzer` et `dockerls`, sont
activés dans `lua/config/lsp.lua`. Pour ajouter Python, par exemple :

1. installer `pyright` afin que `pyright-langserver` soit dans `$PATH` ;
2. décommenter `"pyright"` dans la table `servers` ;
3. redémarrer Neovim et ouvrir un fichier Python ;
4. exécuter `:checkhealth vim.lsp`, puis `:LspInfo` si nécessaire.

Les raccourcis LSP principaux sont `gd` (définition), `gr` (références), `K`
(documentation), `<Espace>rn` (renommer), `<Espace>ca` (action) et
`<Espace>cf` (formater). La complétion native s’affiche automatiquement ;
utiliser `<C-n>` / `<C-p>` puis `<C-y>` pour accepter une proposition.

### Vue et Nuxt

Sous Arch Linux, installer les deux serveurs officiels :

```sh
sudo pacman -S typescript-language-server vue-language-server
```

Après redémarrage, `ts_ls` et `vue_ls` sont activés automatiquement. Dans un
fichier `.vue`, `:LspInfo` doit afficher les deux clients. Pour Nuxt, exécuter
la préparation du projet après l’installation de ses dépendances afin de
générer `.nuxt/tsconfig.json` : `npx nuxt prepare`.

### Dockerfile

Les fichiers `Dockerfile` utilisent le parseur Treesitter `dockerfile`, le
serveur `dockerls` pour la complétion et les diagnostics, ainsi que `hadolint`
à chaque sauvegarde. Mason installe automatiquement les deux exécutables au
premier démarrage.

## Lint à la sauvegarde

`<C-s>` quitte le mode insertion, formate avec le LSP puis sauvegarde le
fichier. L’événement `BufWritePost` lance ensuite
`nvim-lint`. Les linters présents sur cette machine sont configurés dans
`lua/config/lint.lua` : `clang-tidy` pour C/C++, `cargo clippy` pour Rust et
`hadolint` pour les Dockerfiles.
Le raccourci `<Espace>cl` permet aussi de relancer manuellement le lint.
Noice affiche les notifications et la progression LSP, notamment l’indexation
et les vérifications effectuées par `rust-analyzer`, sans second panneau Fidget.

Blink propose automatiquement les éléments LSP, les mots des buffers, les
chemins et les snippets pendant la saisie. `<C-Espace>` force son ouverture,
`<C-n>` / `<C-p>` sélectionnent une entrée et `<Entrée>` ou `<C-y>` l’accepte. `<C-k>` montre
la signature d’une fonction. `<Espace>cd` ouvre le détail du diagnostic sous le curseur.
En Rust, `module::` propose les éléments publics du module. Le fichier doit
être dans un projet contenant un `Cargo.toml`, et les fonctions d’un module
personnel doivent être déclarées avec `pub` pour être visibles depuis ailleurs.
