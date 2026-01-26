// =============================================================================
// simulation/mod.rs - Module de simulation et trait commun
// =============================================================================
//
// Ce fichier est le point d'entrée du module `simulation`.
// En Rust, chaque dossier contenant un `mod.rs` devient un module.
//
// Structure du module :
// simulation/
// ├── mod.rs                    ← Ce fichier (déclarations + trait)
// ├── basic_distance.rs         ← Contrainte de distance simple
// ├── separate_collision.rs     ← Collision entre particules
// └── distance_chain.rs         ← Chaîne de particules liées
//
// =============================================================================

// Déclaration des sous-modules
// `pub mod` rend le module accessible depuis l'extérieur
pub mod basic_distance;
pub mod separate_collision;
pub mod distance_chain;

// Ré-exportation pour un accès plus simple
// Permet d'écrire `simulation::BasicDistanceSimulation`
// au lieu de `simulation::basic_distance::BasicDistanceSimulation`
pub use basic_distance::BasicDistanceSimulation;
pub use separate_collision::SeparateCollisionSimulation;
pub use distance_chain::DistanceChainSimulation;

use crate::particle::{Particle, Vec2};

// =============================================================================
// Trait Simulation - Interface commune pour toutes les simulations
// =============================================================================
//
// En Rust, un `trait` est similaire à une interface (Java/C#) ou une classe
// abstraite (C++/Delphi). Il définit un contrat que les types doivent respecter.
//
// Avantages des traits :
// - Polymorphisme : on peut manipuler différents types via le même trait
// - Composition : un type peut implémenter plusieurs traits
// - Généricité : on peut écrire du code générique sur des traits
//
// =============================================================================

pub trait Simulation {
    // Méthode de mise à jour appelée chaque frame
    //
    // `&mut self` : référence mutable à l'instance
    // - `&` = référence (pas de copie)
    // - `mut` = on peut modifier l'objet
    //
    // `mouse_pos` : position actuelle de la souris
    fn update(&mut self, mouse_pos: Vec2);

    // Retourne la liste des particules pour le rendu
    //
    // `&self` : référence immutable (lecture seule)
    // `&[Particle]` : slice (vue sur un tableau) de particules
    // - Plus flexible qu'un `&Vec<Particle>`
    // - Fonctionne avec Vec, arrays, ou d'autres conteneurs
    fn particles(&self) -> &[Particle];

    // Retourne la particule principale (optionnelle)
    //
    // `Option<&Particle>` : soit Some(&particule), soit None
    // - En Rust, on utilise Option au lieu de null/nil
    // - Force à gérer explicitement le cas "absent"
    fn main_particle(&self) -> Option<&Particle>;

    // Indique si la particule principale doit être rendue
    // Valeur par défaut : true
    //
    // En Rust, les traits peuvent avoir des implémentations par défaut
    fn should_render_main_particle(&self) -> bool {
        true  // Comportement par défaut
    }
}

// =============================================================================
// Énumération des types de scènes
// =============================================================================
//
// Une `enum` en Rust est beaucoup plus puissante qu'en C/Delphi :
// - Chaque variante peut contenir des données différentes
// - Le pattern matching garantit qu'on gère tous les cas
// - Pas de valeur numérique implicite (sauf si demandé)
//
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneType {
    BasicDistance,      // Contrainte de distance simple
    SeparateCollision,  // Collision entre particules
    DistanceChain,      // Chaîne de particules
}

impl SceneType {
    // Retourne la scène suivante (pour cycler avec une touche)
    pub fn next(&self) -> Self {
        // `match` est le pattern matching de Rust
        // Similaire à un switch/case mais plus puissant
        // Le compilateur vérifie qu'on gère TOUS les cas
        match self {
            SceneType::BasicDistance => SceneType::SeparateCollision,
            SceneType::SeparateCollision => SceneType::DistanceChain,
            SceneType::DistanceChain => SceneType::BasicDistance,
        }
    }

    // Retourne le nom de la scène pour l'affichage
    pub fn name(&self) -> &'static str {
        // `&'static str` : référence à une chaîne de durée de vie statique
        // Les chaînes littérales ("...") ont une durée de vie 'static
        match self {
            SceneType::BasicDistance => "Distance Basique",
            SceneType::SeparateCollision => "Collision Séparée",
            SceneType::DistanceChain => "Chaîne de Distance",
        }
    }
}

// =============================================================================
// Notes sur les traits en Rust vs classes abstraites en Delphi
// =============================================================================
//
// Delphi (POO classique) :
// ```delphi
// TConstraintParticleSimulation = class abstract
// public
//   procedure Update(const aMousePos: TPointF); virtual; abstract;
// end;
// ```
//
// Rust (traits) :
// ```rust
// pub trait Simulation {
//     fn update(&mut self, mouse_pos: Vec2);
// }
// ```
//
// Différences clés :
// 1. Pas d'héritage de classe en Rust, seulement des traits
// 2. Un type peut implémenter plusieurs traits (pas de diamant de la mort)
// 3. Les traits peuvent être implémentés pour des types existants (même std)
// 4. Le dispatch dynamique (dyn Trait) est explicite
//
// =============================================================================
