# Zyou_htmltopdf

<p align="center">
  <img src=".github/Zyou_htmltopdf.png" height="150" alt="Zyou_htmltopdf Logo">
</p>

<h1 align="center">👨‍💻 Zyou_htmltopdf 📄</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" title="Go to Rust homepage"><img src="https://img.shields.io/badge/Rust-1-blue?logo=rust&logoColor=white" alt="Made with Rust"></a>
  <a href="https://www.rust-lang.org/" title="Go to Rust homepage"><img src="https://img.shields.io/badge/Crate-Zyou_htmltopdf-green?logo=crate&logoColor=black" alt="Made with Rust"></a>
  <a href="https://github.com/zyouax/Zyou_htmltopdf/actions"><img src="https://img.shields.io/github/workflow/status/zyouax/Zyou_htmltopdf/CI?label=Tests&style=flat-square" alt="Tests"></a>
  <a href="https://github.com/zyouax/Zyou_htmltopdf/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License"></a>
  <a href="https://github.com/zyouax/Zyou_htmltopdf"><img src="https://img.shields.io/github/stars/zyouax/Zyou_htmltopdf?style=flat-square" alt="GitHub Stars"></a>
</p>

<p align="center">
  <strong>Une bibliothèque Rust pour convertir vos fichiers HTML en PDF avec simplicité !</strong>.
</p>

---
<h1 align="center">⚠️ en Développement ⚠️</h1>

---

### Annexes

- [Installation](#installation)
- [Utilisation](#utilisation)
- [Configurations supportés](#configurations-supportés)
- [Configuration](#configuration)
- [Licence](#licence)
- [Contributeurs](#contributeurs)

---

## Fonctionnalités
- ✅ Parseur HTML minimaliste et rapide
- ✅ Support des styles inline et `<style>`
- ✅ Moteur de rendu layout + boîte de style
- ✅ Génération de PDF native (sans `wkhtmltopdf`, ni WebView)
- ✅ Prise en charge des balises HTML courantes : `div`, `p`, `img`, `ul`, `h1-h6`, `form`, etc.
- ✅ Couleurs, tailles, marges, polices, fonds, bordures
- ✅ Images embarquées (`.png`, `.jpg`, etc.)
- ✅ Liens cliquables (`<a href="...">`)
- ✅ Tests unitaires pour le DOM et CSS
- 🧱 Pas de dépendance externe lourde (libre et offline)

## Installation
Ajoutez à votre `Cargo.toml` :

```toml
[dependencies]
zyou_htmltopdf = "0.1.0"
```

ou clonez le projet :

```bash
git clone https://github.com/zyouax/zyou_htmltopdf.git
cd zyou_htmltopdf
cargo build --release
```

## Utilisation

exemple de fichier `input.html` :

```html
<!DOCTYPE html>
<html>
<head>
  <style>
    h1 { color: #b1ff33; font-family: Times; }
  </style>
</head>
<body>
  <h1 class="title">Démo</h1>
  <p>Hello world!</p>
  <img style="height: 180px;" src="images.png" alt="test"/>
</body>
</html>
```

exemple de fichier `main.rs` :

```rust
use zyou_htmltopdf::{parse_html, collect_stylesheets, compute_layout, write_pdf};

fn main() {
    let html = std::fs::read_to_string("input.html").unwrap();
    let dom = parse_html(&html);
    let css = collect_stylesheets(&dom.borrow());
    let layout = compute_layout(&dom.borrow(), 595.0, 842.0, Some(&css));
    let pdf = write_pdf(&layout);
    std::fs::create_dir_all("output").unwrap();
    std::fs::write("output/output.pdf", pdf).unwrap();
}
```

Résultat : `output/output.pdf` généré automatiquement 🎉

## Configurations supportés
- <b>🦀 Rust 1.76</b> ou supérieur (<b>edition 2024</b>)
- <b>OS supportés</b> : Linux, macOS, Windows
- <b>PDF</b> : format A4, 595x842 pts

## Configuration

Le layout engine utilise un modèle de boîte (`box_model`) avec héritage CSS partiel :

- Styles supportés : `font-size`, `font-family`, `color`, `background`, `margin`, `padding`, `display`, `width`, `height`, `border-width`

- Sélecteurs supportés : `tag`, .`class`, `#id`, `parent > child`

- Balises HTML ignorées automatiquement : `script`, `style` (contenu traité), `meta`, `head`, etc.

Pagination, flexbox et tableaux sont prévus pour la version `1.0`.

## Licence
Code sous licence <b>MIT</b> – libre pour usage personnel et commercial.
Voir le fichier `LICENSE` pour plus de détails.

## Contributeurs
[Zyouax](https://github.com/zyouax) – Créateur, développeur principal

Ouvert aux PR et suggestions ! Propose une idée via [issues](https://github.com/zyouax/zyou_htmltopdf/issues/1)

## 🤝 Contributions bienvenues

Tu veux aider à :

- améliorer le moteur CSS ?

- ajouter `display: flex` ou `grid` ?

- gérer la pagination dynamique ?

- exporter vers des formats multiples ?

N'hésite pas à forker, proposer une PR ou discuter d'une RFC.
C’est un projet <b>communautaire</b>. 🧠✨

<p align="center"> Merci pour ta visite et bon code ! ⚡ </p>