// =============================================================================
// separate_collision.rs - Simulation de collision entre particules
// =============================================================================
//
// Cette simulation gère des milliers de particules qui :
// 1. Sont repoussées par le cercle principal (souris)
// 2. Se repoussent entre elles (pas de chevauchement)
//
// COMPORTEMENT :
// -------------
//     ┌──────────────────────────────┐
//     │    ●  ●    ●  ●  ●    ●      │
//     │  ●    ●  ●      ●  ●    ●    │
//     │    ●      ╭───────╮    ●     │   Cercle principal
//     │  ●    ●   │       │  ●    ●  │   repousse les particules
//     │    ●      │   ◯   │    ●     │
//     │  ●    ●   │       │  ●    ●  │
//     │    ●      ╰───────╯    ●     │
//     │  ●    ●  ●      ●  ●    ●    │
//     │    ●  ●    ●  ●  ●    ●      │
//     └──────────────────────────────┘
//
// OPTIMISATION :
// -------------
// Avec N particules, la détection de collision naïve est O(n²).
// On utilise un Spatial Hash pour réduire à O(n) en moyenne.
//
// =============================================================================

use macroquad::prelude::*;
use crate::particle::{Particle, Vec2, random_color};
use crate::spatial_hash::SpatialHash;
use super::Simulation;

// =============================================================================
// Structure SeparateCollisionSimulation
// =============================================================================

pub struct SeparateCollisionSimulation {
    // Particule principale (suit la souris, repousse les autres)
    main_particle: Particle,

    // Toutes les petites particules
    particles: Vec<Particle>,

    // Nombre d'itérations de résolution de collision par frame
    // Plus d'itérations = meilleure séparation, mais plus lent
    iterations: usize,

    // Rayon des particules (toutes identiques pour simplifier)
    ball_radius: f32,
}

impl SeparateCollisionSimulation {
    // Constructeur
    //
    // Paramètres :
    // - count : nombre de particules à créer
    // - ball_radius : rayon de chaque particule
    // - main_radius : rayon du cercle principal
    // - start_pos : position de départ
    // - main_color : couleur du cercle principal
    // - use_random_colors : true = couleurs aléatoires, false = blanc
    pub fn new(
        count: usize,
        ball_radius: f32,
        main_radius: f32,
        start_pos: Vec2,
        main_color: Color,
        use_random_colors: bool,
    ) -> Self {
        let main_particle = Particle {
            pos: start_pos,
            radius: main_radius,
            color: main_color,
        };

        // Crée les particules en grille
        // On les dispose initialement en grille pour éviter trop de chevauchement
        let mut particles = Vec::with_capacity(count);

        // Variables pour la disposition en grille
        let mut grid_x = 0;  // Colonne actuelle
        let mut grid_y = 0;  // Ligne actuelle
        let cols = 50;       // Nombre de colonnes

        for i in 0..count {
            // Passe à la ligne suivante tous les `cols` éléments
            if i > 0 && i % cols == 0 {
                grid_x = 0;
                grid_y += 1;
            }

            // Calcule la position
            // On décale de 100 pixels à droite de start_pos
            let pos = Vec2::new(
                start_pos.x + 100.0 + (grid_x as f32 * ball_radius),
                start_pos.y + (grid_y as f32 * ball_radius),
            );

            // Choisit la couleur
            let color = if use_random_colors {
                random_color()
            } else {
                WHITE
            };

            particles.push(Particle::new(pos, ball_radius, color));
            grid_x += 1;
        }

        Self {
            main_particle,
            particles,
            iterations: 1,
            ball_radius,
        }
    }

    // Constructeur avec valeurs par défaut
    pub fn default_scene() -> Self {
        Self::new(
            4000,   // 4000 particules
            5.0,    // Rayon de 5 pixels
            50.0,   // Rayon principal de 50
            Vec2::new(400.0, 400.0),
            WHITE,
            true,   // Couleurs aléatoires
        )
    }

    // Modifie le nombre d'itérations
    pub fn set_iterations(&mut self, iterations: usize) {
        self.iterations = iterations;
    }
}

impl Simulation for SeparateCollisionSimulation {
    fn update(&mut self, mouse_pos: Vec2) {
        // 1. Met à jour la position de la particule principale
        self.main_particle.pos = mouse_pos;

        // 2. Repousse les particules hors du cercle principal
        // ---------------------------------------------------
        // Pour chaque particule, on vérifie si elle est trop proche
        // du cercle principal et on la repousse si nécessaire.

        for particle in &mut self.particles {
            // Vecteur du cercle principal vers la particule
            let to_particle = self.main_particle.pos - particle.pos;

            // Distance minimale = somme des rayons (pas de chevauchement)
            let min_distance = self.main_particle.radius + particle.radius;

            // Si trop proche, on repousse
            if to_particle.length() < min_distance {
                // Calcule le nouveau vecteur avec la bonne longueur
                let corrected = to_particle.with_length(min_distance);

                // Calcule le décalage nécessaire
                let offset = self.main_particle.pos - particle.pos - corrected;

                // Applique le décalage
                particle.pos += offset;
            }
        }

        // 3. Résout les collisions entre particules
        // -----------------------------------------
        // C'est ici qu'on utilise le Spatial Hash pour l'optimisation.
        // On fait plusieurs itérations pour améliorer la séparation.

        // Crée le spatial hash avec cellules de taille 2*rayon
        let mut hash = SpatialHash::new(self.ball_radius * 2.0);

        // Buffer réutilisable pour les indices des voisins
        let mut nearby = Vec::new();

        for _ in 0..self.iterations {
            // Vide et remplit le hash avec les positions actuelles
            hash.clear();
            for (i, particle) in self.particles.iter().enumerate() {
                hash.insert(i, &particle.pos);
            }

            // Pour chaque particule, vérifie les collisions avec ses voisins
            for i in 0..self.particles.len() {
                // Récupère les voisins potentiels
                nearby.clear();
                hash.get_nearby(&self.particles[i].pos, &mut nearby);

                // Vérifie chaque voisin
                for &j in &nearby {
                    // Évite de vérifier deux fois la même paire (i, j) et (j, i)
                    // et de vérifier une particule avec elle-même
                    if j <= i {
                        continue;
                    }

                    // Vecteur de i vers j
                    let to_other = self.particles[j].pos - self.particles[i].pos;

                    // Distance minimale (somme des rayons)
                    let min_dist = self.particles[i].radius + self.particles[j].radius;

                    // Si chevauchement détecté
                    if to_other.length() <= min_dist {
                        // Calcule la correction
                        let corrected = to_other.with_length(min_dist);
                        let offset = self.particles[j].pos - self.particles[i].pos - corrected;

                        // Divise par 2 car on déplace les deux particules
                        // Chacune bouge de la moitié de la distance
                        let half_offset = offset / 2.0;

                        // Applique la correction aux deux particules
                        self.particles[i].pos += half_offset;
                        self.particles[j].pos -= half_offset;
                    }
                }
            }
        }
    }

    fn particles(&self) -> &[Particle] {
        &self.particles
    }

    fn main_particle(&self) -> Option<&Particle> {
        Some(&self.main_particle)
    }
}

// =============================================================================
// Explication de l'algorithme de collision
// =============================================================================
//
// PROBLÈME : Deux particules se chevauchent
//
//      ●──────●
//      A      B
//      ↑──────↑
//     Ces deux particules se chevauchent
//
// SOLUTION : Les séparer de manière égale
//
// 1. Calcule le vecteur de A vers B : to_other = B - A
//
// 2. Calcule la distance minimale : min_dist = rayon_A + rayon_B
//
// 3. Si |to_other| < min_dist, il y a chevauchement
//
// 4. Calcule la correction :
//    - On veut que |to_other| = min_dist
//    - corrected = normalize(to_other) * min_dist
//    - offset = B - A - corrected (la différence à corriger)
//
// 5. Chaque particule se déplace de offset/2 :
//    - A se déplace de +offset/2 (vers B)
//    - B se déplace de -offset/2 (vers A)
//
// Résultat :
//      ●────────────●
//      A            B
//      ↑            ↑
//     Maintenant séparées correctement
//
// =============================================================================
