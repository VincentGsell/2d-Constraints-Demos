// =============================================================================
// distance_chain.rs - Simulation de chaîne de particules liées
// =============================================================================
//
// Cette simulation crée une chaîne de particules où chaque maillon est
// contraint à rester à distance fixe du précédent.
//
// COMPORTEMENT :
// -------------
//
//   Souris                                      Point d'ancrage
//     ↓                                              ↓
//     ●───●───●───●───●───●───●───●───●───●         ◯
//     ↑   ↑   ↑   ↑   ↑   ↑   ↑   ↑   ↑   ↑
//   Chaque maillon est relié au précédent par une contrainte de distance
//
// OPTIONS :
// ---------
// 1. Mode normal : La chaîne suit la souris librement
// 2. Mode FABRIK : La fin de la chaîne est ancrée (inverse kinematics)
// 3. Collision : Les maillons peuvent se repousser entre eux
//
// ALGORITHME FABRIK (Forward And Backward Reaching Inverse Kinematics) :
// ----------------------------------------------------------------------
// 1. Passe avant (Forward) : Du début vers la fin
//    - Chaque maillon est contraint par rapport au précédent
//
// 2. Passe arrière (Backward) : De la fin vers le début
//    - Le dernier maillon est fixé à l'ancrage
//    - Chaque maillon est contraint par rapport au suivant
//
// Cela permet à la chaîne de "chercher" à atteindre la souris tout en
// gardant son extrémité ancrée.
//
// =============================================================================

use macroquad::prelude::*;
use crate::particle::{Particle, Vec2, ConstraintResolver, random_color};
use super::Simulation;

// =============================================================================
// Structure DistanceChainSimulation
// =============================================================================

pub struct DistanceChainSimulation {
    // Particule principale (pour cohérence avec les autres simulations)
    main_particle: Particle,

    // Les maillons de la chaîne
    particles: Vec<Particle>,

    // Distance entre chaque maillon
    link_distance: f32,

    // Active l'algorithme FABRIK (ancrage du dernier maillon)
    use_fabrik: bool,

    // Active la collision entre les maillons
    ball_collision: bool,

    // Position du point d'ancrage (utilisé par FABRIK)
    anchor_pos: Vec2,
}

impl DistanceChainSimulation {
    // Constructeur
    //
    // Paramètres :
    // - count : nombre de maillons dans la chaîne
    // - ball_radius : rayon de chaque maillon
    // - link_distance : distance entre les maillons
    // - start_pos : position de départ (aussi utilisée comme ancrage)
    // - use_random_colors : couleurs aléatoires ou blanches
    pub fn new(
        count: usize,
        ball_radius: f32,
        link_distance: f32,
        start_pos: Vec2,
        use_random_colors: bool,
    ) -> Self {
        // Crée la particule principale (suit le premier maillon)
        let main_particle = Particle {
            pos: start_pos,
            radius: ball_radius,
            color: WHITE,
        };

        // Crée les maillons de la chaîne
        // Ils sont initialement alignés horizontalement
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            let pos = Vec2::new(
                start_pos.x + ((i + 1) as f32 * link_distance),
                start_pos.y,
            );

            let color = if use_random_colors {
                random_color()
            } else {
                WHITE
            };

            particles.push(Particle::new(pos, ball_radius, color));
        }

        Self {
            main_particle,
            particles,
            link_distance,
            use_fabrik: false,
            ball_collision: false,
            anchor_pos: start_pos,
        }
    }

    // Constructeur avec valeurs par défaut
    pub fn default_scene() -> Self {
        Self::new(
            20,     // 20 maillons
            15.0,   // Rayon de 15 pixels
            30.0,   // Distance de 30 pixels entre maillons
            Vec2::new(400.0, 400.0),
            true,   // Couleurs aléatoires
        )
    }

    // Active/désactive FABRIK
    pub fn set_fabrik(&mut self, enabled: bool) {
        self.use_fabrik = enabled;
    }

    // Active/désactive la collision entre maillons
    pub fn set_ball_collision(&mut self, enabled: bool) {
        self.ball_collision = enabled;
    }

    // Modifie la distance entre les maillons
    pub fn set_link_distance(&mut self, distance: f32) {
        self.link_distance = distance;
    }

    // Modifie la position de l'ancrage
    pub fn set_anchor_pos(&mut self, pos: Vec2) {
        self.anchor_pos = pos;
    }

    // Getters pour l'état actuel (utilisés par l'UI)
    pub fn fabrik_enabled(&self) -> bool {
        self.use_fabrik
    }

    pub fn ball_collision_enabled(&self) -> bool {
        self.ball_collision
    }
}

impl Simulation for DistanceChainSimulation {
    fn update(&mut self, mouse_pos: Vec2) {
        // Vérifie qu'on a au moins un maillon
        if self.particles.is_empty() {
            return;
        }

        // =================================================================
        // ÉTAPE 1 : Le premier maillon suit la souris
        // =================================================================
        self.particles[0].pos = mouse_pos;

        // =================================================================
        // ÉTAPE 2 : Passe avant (Forward pass)
        // =================================================================
        // Chaque maillon est contraint à rester à link_distance du précédent.
        //
        // Algorithme :
        //   pour i de 1 à n-1 :
        //     particles[i] = contrainte_distance(particles[i], particles[i-1], link_distance)
        //
        // Cela "tire" la chaîne derrière le premier maillon.

        for i in 1..self.particles.len() {
            // Récupère la position du maillon précédent
            let anchor = self.particles[i - 1].pos;

            // Applique la contrainte de distance
            self.particles[i].pos = ConstraintResolver::distance(
                self.particles[i].pos,
                anchor,
                self.link_distance,
            );
        }

        // =================================================================
        // ÉTAPE 3 : Passe arrière FABRIK (Backward pass) - optionnelle
        // =================================================================
        // Si FABRIK est activé, le dernier maillon est ancré et on propage
        // les contraintes vers le début.
        //
        // Cela crée un effet de "bras robotique" qui essaie d'atteindre
        // la cible tout en restant attaché.

        if self.use_fabrik {
            // Ancre le dernier maillon
            let last_idx = self.particles.len() - 1;
            self.particles[last_idx].pos = self.anchor_pos;

            // Propage vers le début (de n-1 à 1)
            for i in (1..=last_idx).rev() {
                // `rev()` inverse l'itération : n-1, n-2, ..., 1
                let anchor = self.particles[i].pos;
                self.particles[i - 1].pos = ConstraintResolver::distance(
                    self.particles[i - 1].pos,
                    anchor,
                    self.link_distance,
                );
            }
        }

        // =================================================================
        // ÉTAPE 4 : Collision entre maillons - optionnelle
        // =================================================================
        // Si activée, les maillons se repoussent pour éviter de se chevaucher.
        // Utilise un algorithme O(n²) car le nombre de maillons est faible.

        if self.ball_collision {
            // Double boucle pour vérifier toutes les paires
            for i in 0..self.particles.len() {
                for j in (i + 1)..self.particles.len() {
                    // Vecteur de i vers j
                    let to_other = self.particles[j].pos - self.particles[i].pos;

                    // Distance minimale (somme des rayons)
                    let min_dist = self.particles[i].radius + self.particles[j].radius;

                    // Si chevauchement
                    if to_other.length() <= min_dist {
                        // Même algorithme que SeparateCollision
                        let corrected = to_other.with_length(min_dist);
                        let offset = self.particles[j].pos - self.particles[i].pos - corrected;
                        let half_offset = offset / 2.0;

                        self.particles[i].pos += half_offset;
                        self.particles[j].pos -= half_offset;
                    }
                }
            }
        }

        // =================================================================
        // ÉTAPE 5 : Met à jour la particule principale
        // =================================================================
        // Pour cohérence, la particule principale suit le premier maillon.
        self.main_particle.pos = self.particles[0].pos;
    }

    fn particles(&self) -> &[Particle] {
        &self.particles
    }

    fn main_particle(&self) -> Option<&Particle> {
        Some(&self.main_particle)
    }

    // Ne pas rendre la particule principale (elle est superposée au 1er maillon)
    fn should_render_main_particle(&self) -> bool {
        false
    }
}

// =============================================================================
// Visualisation de FABRIK
// =============================================================================
//
// Sans FABRIK (chaîne libre) :
// ----------------------------
//   Souris ●
//            ↘
//             ●───●───●───●───●───●
//                                  (traîne derrière)
//
// Avec FABRIK (chaîne ancrée) :
// -----------------------------
//   Souris ●                    Ancrage ◯
//            ↘                      ↗
//             ●───●───●───●───●───●
//                    (forme un arc)
//
// L'algorithme FABRIK itère entre les deux passes (forward/backward)
// jusqu'à convergence. Ici on fait une seule itération par frame,
// ce qui suffit pour un rendu fluide.
//
// =============================================================================
