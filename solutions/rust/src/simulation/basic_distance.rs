// =============================================================================
// basic_distance.rs - Simulation de contrainte de distance basique
// =============================================================================
//
// Cette simulation est la plus simple : une particule reste à l'intérieur
// d'un cercle principal qui suit la souris.
//
// COMPORTEMENT :
// -------------
//     ┌─────────────────┐
//     │   ╭─────────╮   │   Le cercle blanc (main) suit la souris
//     │   │         │   │   Le cercle noir (particule) reste à l'intérieur
//     │   │    ●    │   │
//     │   │  (noir) │   │   Quand la souris bouge, le cercle noir
//     │   ╰─────────╯   │   est "poussé" pour rester dans le cercle blanc
//     │    (blanc)      │
//     └─────────────────┘
//
// C'est une contrainte "de contenance" : la particule ne peut pas sortir.
//
// =============================================================================

use macroquad::prelude::*;
use crate::particle::{Particle, Vec2, ConstraintResolver};
use super::Simulation;

// =============================================================================
// Structure BasicDistanceSimulation
// =============================================================================

pub struct BasicDistanceSimulation {
    // Particule principale (cercle blanc qui suit la souris)
    main_particle: Particle,

    // Liste des particules contraintes (ici, une seule)
    // Vec<T> est le tableau dynamique de Rust (similaire à TList<T> en Delphi)
    particles: Vec<Particle>,
}

impl BasicDistanceSimulation {
    // Constructeur
    //
    // Paramètres :
    // - main_radius : rayon du cercle principal
    // - ball_radius : rayon de la particule contrainte
    // - start_pos : position de départ
    // - main_color : couleur du cercle principal
    // - ball_color : couleur de la particule
    pub fn new(
        main_radius: f32,
        ball_radius: f32,
        start_pos: Vec2,
        main_color: Color,
        ball_color: Color,
    ) -> Self {
        // Crée la particule principale
        let main_particle = Particle {
            pos: start_pos,
            radius: main_radius,
            color: main_color,
        };

        // Crée la particule contrainte
        // vec![...] est une macro qui crée un Vec avec les éléments donnés
        let particles = vec![Particle {
            pos: start_pos,
            radius: ball_radius,
            color: ball_color,
        }];

        // Retourne la structure initialisée
        // En Rust, la dernière expression sans `;` est la valeur de retour
        Self {
            main_particle,
            particles,
        }
    }

    // Constructeur avec valeurs par défaut
    // Pratique pour tester rapidement
    pub fn default_scene() -> Self {
        Self::new(
            50.0,           // Rayon principal
            15.0,           // Rayon de la balle
            Vec2::new(400.0, 400.0),  // Position de départ
            WHITE,          // Couleur principale (blanc)
            BLACK,          // Couleur de la balle (noir)
        )
    }
}

// =============================================================================
// Implémentation du trait Simulation
// =============================================================================
//
// `impl Trait for Type` implémente un trait pour un type donné.
// C'est similaire à hériter d'une classe abstraite en Delphi,
// mais sans l'héritage de données (seulement le comportement).
//
// =============================================================================

impl Simulation for BasicDistanceSimulation {
    fn update(&mut self, mouse_pos: Vec2) {
        // La particule principale suit la souris
        self.main_particle.pos = mouse_pos;

        // Calcule le vecteur de la balle vers la souris
        // On accède au premier élément avec [0]
        let to_mouse = mouse_pos - self.particles[0].pos;

        // Distance maximale autorisée :
        // La balle doit rester à l'intérieur du cercle principal
        // Distance max = rayon principal - rayon balle
        let max_distance = self.main_particle.radius - self.particles[0].radius;

        // Si la balle est trop loin, on la ramène
        if to_mouse.length() > max_distance {
            // Utilise la contrainte de distance pour repositionner
            self.particles[0].pos = ConstraintResolver::distance(
                self.particles[0].pos,  // Point à contraindre
                mouse_pos,               // Point d'ancrage
                max_distance,            // Distance à maintenir
            );
        }
    }

    fn particles(&self) -> &[Particle] {
        // Retourne une slice sur le Vec
        // Le & devant self.particles convertit automatiquement Vec en slice
        &self.particles
    }

    fn main_particle(&self) -> Option<&Particle> {
        // Some() enveloppe une valeur dans Option
        Some(&self.main_particle)
    }

    // On utilise l'implémentation par défaut de should_render_main_particle()
    // qui retourne true
}

// =============================================================================
// Explication détaillée de la contrainte de distance
// =============================================================================
//
// La fonction ConstraintResolver::distance fonctionne ainsi :
//
// 1. On a un point P (la balle) et un ancrage A (la souris)
//
//        P ●────────────────● A
//          ↑                 ↑
//        point            anchor
//
// 2. On calcule la direction de A vers P : direction = P - A
//
// 3. On normalise cette direction (longueur = 1)
//
// 4. On place P à la distance voulue : P' = A + direction * distance
//
//        P'●───────● A
//          ↑       ↑
//      nouvelle  anchor
//      position
//
// Résultat : P est maintenant exactement à `distance` de A, dans la même
// direction qu'avant.
//
// =============================================================================
