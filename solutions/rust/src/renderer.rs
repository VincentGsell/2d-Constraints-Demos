// =============================================================================
// renderer.rs - Module de rendu avec macroquad
// =============================================================================
//
// Ce module gère l'affichage des particules à l'écran.
// Il utilise macroquad, une bibliothèque graphique simple et performante.
//
// MACROQUAD EN BREF :
// -------------------
// - Inspirée de raylib (bibliothèque C populaire)
// - API procédurale simple (pas de POO complexe)
// - Gère fenêtre, entrées, audio, et rendu 2D/3D
// - Parfaite pour le prototypage et l'apprentissage
//
// ARCHITECTURE DU RENDU :
// -----------------------
// macroquad utilise un système de "immediate mode" :
// - On dessine directement chaque frame
// - Pas de scène graphique à maintenir
// - Simple mais moins optimisé que le "retained mode"
//
// =============================================================================

use macroquad::prelude::*;
use crate::particle::Particle;
use crate::simulation::Simulation;

// =============================================================================
// Renderer - Structure de rendu
// =============================================================================
//
// Cette structure est relativement simple car macroquad gère beaucoup
// de choses automatiquement. Dans un moteur plus complexe, on aurait
// des buffers GPU, des shaders, etc.
//
// =============================================================================

pub struct Renderer {
    // Couleur de fond de la fenêtre
    background_color: Color,
}

impl Renderer {
    // Crée un nouveau renderer
    pub fn new() -> Self {
        Self {
            // Gris foncé par défaut - plus agréable que le noir pur
            background_color: Color::from_rgba(40, 40, 40, 255),
        }
    }

    // Modifie la couleur de fond
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    // ==========================================================================
    // Méthode principale de rendu
    // ==========================================================================
    //
    // Cette méthode est appelée chaque frame pour dessiner la simulation.
    //
    // Paramètres :
    // - simulation : référence vers la simulation à dessiner
    //               `&dyn Simulation` = référence vers un trait object
    //               Le `dyn` indique un dispatch dynamique (polymorphisme)
    //
    // ==========================================================================

    pub fn render(&self, simulation: &dyn Simulation) {
        // 1. Efface l'écran avec la couleur de fond
        //    clear_background() est une fonction globale de macroquad
        clear_background(self.background_color);

        // 2. Dessine la particule principale si nécessaire
        if simulation.should_render_main_particle() {
            if let Some(main_particle) = simulation.main_particle() {
                // `if let Some(x) = ...` est un pattern matching conditionnel
                // C'est la façon idiomatique de gérer Option en Rust
                self.draw_particle(main_particle);
            }
        }

        // 3. Dessine toutes les autres particules
        for particle in simulation.particles() {
            self.draw_particle(particle);
        }
    }

    // ==========================================================================
    // Dessine une particule unique
    // ==========================================================================
    //
    // Une particule est simplement un cercle rempli.
    //
    // draw_circle() prend :
    // - x, y : centre du cercle (en pixels)
    // - radius : rayon (en pixels)
    // - color : couleur de remplissage
    //
    // ==========================================================================

    fn draw_particle(&self, particle: &Particle) {
        draw_circle(
            particle.pos.x,
            particle.pos.y,
            particle.radius,
            particle.color,
        );
    }

    // ==========================================================================
    // Dessine l'interface utilisateur (HUD)
    // ==========================================================================
    //
    // Affiche les informations et contrôles à l'écran.
    // Utilise draw_text() de macroquad.
    //
    // Paramètres :
    // - scene_name : nom de la scène actuelle
    // - fps : frames par seconde actuelles
    // - particle_count : nombre de particules
    // - fabrik_enabled : état de l'option FABRIK (pour la chaîne)
    // - collision_enabled : état de l'option collision (pour la chaîne)
    //
    // ==========================================================================

    pub fn render_ui(
        &self,
        scene_name: &str,
        fps: i32,
        particle_count: usize,
        fabrik_enabled: Option<bool>,
        collision_enabled: Option<bool>,
    ) {
        // Position Y de départ pour le texte
        let mut y = 25.0;
        let line_height = 20.0;  // Hauteur d'une ligne

        // Couleur du texte
        let text_color = YELLOW;

        // Taille de la police
        let font_size = 18.0;

        // Titre et FPS
        draw_text(
            &format!("Scene: {} | FPS: {}", scene_name, fps),
            10.0,
            y,
            font_size,
            text_color,
        );
        y += line_height;

        // Nombre de particules
        draw_text(
            &format!("Particules: {}", particle_count),
            10.0,
            y,
            font_size,
            text_color,
        );
        y += line_height;

        // Contrôles généraux
        draw_text(
            "Controles: [1] Distance | [2] Collision | [3] Chaine | [Espace] Suivant",
            10.0,
            y,
            font_size,
            text_color,
        );
        y += line_height;

        // Options spécifiques à la chaîne (si applicable)
        // Option<bool>.is_some() retourne true si c'est Some(...)
        if fabrik_enabled.is_some() || collision_enabled.is_some() {
            // unwrap_or(false) : retourne la valeur ou false si None
            let fabrik_str = if fabrik_enabled.unwrap_or(false) { "ON" } else { "OFF" };
            let collision_str = if collision_enabled.unwrap_or(false) { "ON" } else { "OFF" };

            draw_text(
                &format!(
                    "Options chaine: [F] FABRIK: {} | [C] Collision: {}",
                    fabrik_str,
                    collision_str
                ),
                10.0,
                y,
                font_size,
                text_color,
            );
        }
    }
}

// =============================================================================
// Implémentation de Default
// =============================================================================
//
// Le trait Default permet de créer une instance avec des valeurs par défaut.
// Permet d'utiliser Renderer::default() ou ..Default::default() dans les structs.
//
// =============================================================================

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Notes sur le rendu en Rust/macroquad vs Delphi/FMX
// =============================================================================
//
// DELPHI FMX :
// -----------
// - Événement OnPaint d'un TPaintBox
// - Canvas.FillEllipse pour dessiner
// - Système de composants visuels
//
// ```delphi
// procedure TForm.PaintBoxPaint(Sender: TObject; Canvas: TCanvas);
// begin
//   Canvas.Fill.Color := TAlphaColors.White;
//   Canvas.FillEllipse(RectF(x-r, y-r, x+r, y+r), 1);
// end;
// ```
//
// RUST/MACROQUAD :
// ---------------
// - Boucle de jeu explicite (game loop)
// - Fonctions de dessin appelées directement
// - Pas de composants visuels
//
// ```rust
// loop {
//     clear_background(GRAY);
//     draw_circle(x, y, r, WHITE);
//     next_frame().await;
// }
// ```
//
// La différence fondamentale :
// - FMX : événementiel (le système décide quand redessiner)
// - macroquad : boucle active (on décide quand redessiner)
//
// =============================================================================
