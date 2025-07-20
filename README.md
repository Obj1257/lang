# [WIP] Interpréteur de Langage de Programmation Simple Issu De Program=Proof

Un interpréteur de langage de programmation minimal implémenté en Rust, avec inférence de types et évaluation d'expressions.

## Fonctionnalités

- **Système de Types** : Vérification statique des types avec deux types de base :
  - `TBool` - Valeurs booléennes
  - `TInt` - Valeurs entières (u16)

- **Opérations Supportées** :
  - Arithmétique : Addition (`+`)
  - Comparaison : Inférieur à (`<`)
  - Contrôle de flux : Expressions conditionnelles (`if-then-else`)
  - Littéraux : Constantes booléennes et entières

- **Inférence de Types** : Vérification automatique des types avec messages d'erreur détaillés
- **Évaluation d'Expressions** : Réduction et normalisation étape par étape des expressions

## Grammaire du Langage

```
Prog ::= Bool(bool)                           // Littéral booléen
       | Int(u16)                             // Littéral entier  
       | Add(Prog, Prog)                      // Addition
       | Lt(Prog, Prog)                       // Comparaison inférieur à
       | If(Prog, Prog, Prog)                 // Conditionnel (condition, alors, sinon)
```

## Règles de Typage

- `Add(a, b)` nécessite que `a` et `b` soient des entiers, retourne un entier
- `Lt(a, b)` nécessite que `a` et `b` soient des entiers, retourne un booléen
- `If(cond, then, else)` nécessite :
  - `cond` doit être un booléen
  - `then` et `else` doivent avoir le même type
  - Retourne le type des branches

## Utilisation

### Compilation et Exécution

```bash
# Compiler le projet
cargo build

# Exécuter le projet
cargo run

# Exécuter avec les optimisations de release
cargo run --release
```

### Exemple de Programme

L'exemple actuel dans `main.rs` représente l'expression :
```
if (1 + (2 + 3)) < 4 then 6 else 5
```

Cette expression :
1. Évalue `1 + (2 + 3)` à `6`
2. Compare `6 < 4` qui est `false`
3. Retourne la branche `else` : `5`

### Créer Vos Propres Programmes

Vous pouvez créer des programmes en utilisant l'énumération `Prog` :

```rust
// Addition simple : 2 + 3
let addition = Prog::Add(
    Box::new(Prog::Int(2)),
    Box::new(Prog::Int(3))
);

// Comparaison : 5 < 10
let comparison = Prog::Lt(
    Box::new(Prog::Int(5)),
    Box::new(Prog::Int(10))
);

// Conditionnel : if true then 42 else 0
let conditional = Prog::If(
    Box::new(Prog::Bool(true)),
    Box::new(Prog::Int(42)),
    Box::new(Prog::Int(0))
);
```

## API

### Fonctions Principales

- `infer(prog: &Prog) -> Result<Typ, String>` : Effectue l'inférence de types sur un programme
- `typable(prog: &Prog) -> bool` : Vérifie si un programme est bien typé
- `reduce(prog: &Prog) -> Option<Prog>` : Effectue une étape d'évaluation
- `normalize(prog: &Prog) -> Result<Prog, String>` : Évalue complètement un programme bien typé

## Gestion d'Erreurs

L'interpréteur fournit des messages d'erreur détaillés en français :
- `"Il y a une addition qui ne fait pas intervenir deux entiers"` - Addition avec des non-entiers
- `"Il y a une comparaison qui ne fait pas intervenir deux entiers"` - Comparaison avec des non-entiers  
- `"La condition pour un if n'est pas un bool"` - Condition non-booléenne dans une expression if
- `"Les branches d'un if n'ont pas le même type"` - Types incompatibles dans les branches if
- `"Le programme n'est pas typable"` - Le programme n'est pas bien typé

## Structure du Projet

```
src/
├── main.rs          # Implémentation principale de l'interpréteur et exemple
Cargo.toml           # Configuration du projet
README.md            # Ce fichier
```

## Prérequis

- Rust édition 2024
- Aucune dépendance externe

## Licence

Ce projet est open source. Consultez le dépôt pour les détails de la licence.

## Contribution

N'hésitez pas à contribuer en :
- Ajoutant de nouvelles fonctionnalités au langage
- Améliorant les messages d'erreur
- Ajoutant des exemples plus complets
- Optimisant le moteur d'évaluation