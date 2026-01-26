// =============================================================================
// main.rs - Point d'entrée et boucle principale du programme
// =============================================================================
//
// Ce fichier contient :
// 1. La configuration de la fenêtre
// 2. La boucle de jeu principale (game loop)
// 3. La gestion des entrées utilisateur
// 4. Le changement de scènes
//
// STRUCTURE D'UN PROGRAMME MACROQUAD :
// ------------------------------------
//
// ```
// #[macroquad::main("Titre")]     ← Configure la fenêtre
// async fn main() {               ← Point d'entrée asynchrone
//     // Initialisation
//     loop {                      ← Boucle infinie (game loop)
//         // 1. Gérer les entrées
//         // 2. Mettre à jour la logique
//         // 3. Dessiner
//         next_frame().await;     ← Attend la frame suivante
//     }
// }
// ```
//
// =============================================================================

// Déclaration des modules du projet
// En Rust, chaque fichier .rs dans src/ doit être déclaré ici
mod particle;       // Types de base (Vec2, Particle, etc.)
mod spatial_hash;   // Optimisation des collisions
mod simulation;     // Toutes les simulations
mod renderer;       // Rendu graphique

// Imports depuis nos modules
use particle::Vec2;
use simulation::{
    Simulation,              // Le trait
    SceneType,               // L'enum des types de scènes
    BasicDistanceSimulation, // Les simulations concrètes
    SeparateCollisionSimulation,
    DistanceChainSimulation,
};
use renderer::Renderer;

// Imports depuis macroquad
// `prelude::*` importe les éléments les plus courants
use macroquad::prelude::*;

// =============================================================================
// Configuration de la fenêtre
// =============================================================================
//
// La fonction `window_conf()` est appelée par macroquad AVANT le démarrage
// pour configurer la fenêtre.
//
// Conf est une structure avec de nombreux champs optionnels :
// - window_title : titre de la fenêtre
// - window_width/height : dimensions initiales
// - fullscreen : plein écran
// - window_resizable : redimensionnable
// - high_dpi : support écrans haute résolution
//
// =============================================================================

fn window_conf() -> Conf {
    Conf {
        window_title: "Contraintes de Particules 2D - Portage Rust".to_owned(),
        window_width: 1024,
        window_height: 768,
        window_resizable: true,
        // `..Default::default()` remplit les autres champs avec leurs défauts
        // C'est une syntaxe pratique pour les structs avec beaucoup de champs
        ..Default::default()
    }
}

// =============================================================================
// Point d'entrée du programme
// =============================================================================
//
// L'attribut `#[macroquad::main(window_conf)]` fait plusieurs choses :
// 1. Génère la vraie fonction main() qui appelle notre fonction async
// 2. Initialise le contexte graphique (OpenGL/WebGL)
// 3. Configure la fenêtre avec window_conf()
// 4. Gère la boucle d'événements de l'OS
//
// Le `async` est nécessaire car macroquad utilise l'asynchrone pour :
// - Attendre la prochaine frame (next_frame().await)
// - Charger des ressources (images, sons, etc.)
//
// =============================================================================

#[macroquad::main(window_conf)]
async fn main() {
    // =========================================================================
    // INITIALISATION
    // =========================================================================

    // Crée le renderer
    let renderer = Renderer::new();

    // Type de scène actuelle
    let mut current_scene = SceneType::BasicDistance;

    // La simulation actuelle
    // `Box<dyn Simulation>` est un "trait object" :
    // - Box : pointeur intelligent qui alloue sur le tas (heap)
    // - dyn Simulation : type dynamique implémentant le trait Simulation
    // - Permet le polymorphisme à l'exécution
    let mut simulation: Box<dyn Simulation> = create_simulation(current_scene);

    // =========================================================================
    // BOUCLE DE JEU PRINCIPALE (GAME LOOP)
    // =========================================================================
    //
    // Cette boucle s'exécute une fois par frame (typiquement 60 fois/seconde).
    // C'est le cœur de tout programme interactif.
    //
    // Ordre classique :
    // 1. Input : lire les entrées utilisateur
    // 2. Update : mettre à jour la logique
    // 3. Render : dessiner à l'écran
    //
    // =========================================================================

    loop {
        // =====================================================================
        // 1. GESTION DES ENTRÉES (INPUT)
        // =====================================================================

        // Vérifie si des touches ont été pressées
        // is_key_pressed() retourne true UNE SEULE FOIS par appui
        // (contrairement à is_key_down() qui reste true tant qu'on appuie)

        // Touche Espace : passe à la scène suivante
        if is_key_pressed(KeyCode::Space) {
            current_scene = current_scene.next();
            simulation = create_simulation(current_scene);
        }

        // Touche 1 : scène BasicDistance
        if is_key_pressed(KeyCode::Key1) {
            current_scene = SceneType::BasicDistance;
            simulation = create_simulation(current_scene);
        }

        // Touche 2 : scène SeparateCollision
        if is_key_pressed(KeyCode::Key2) {
            current_scene = SceneType::SeparateCollision;
            simulation = create_simulation(current_scene);
        }

        // Touche 3 : scène DistanceChain
        if is_key_pressed(KeyCode::Key3) {
            current_scene = SceneType::DistanceChain;
            simulation = create_simulation(current_scene);
        }

        // Options spécifiques à DistanceChain
        // On utilise downcast_mut pour accéder au type concret
        if let Some(chain) = downcast_chain_mut(&mut *simulation) {
            // Touche F : basculer FABRIK
            if is_key_pressed(KeyCode::F) {
                chain.set_fabrik(!chain.fabrik_enabled());
            }

            // Touche C : basculer collision
            if is_key_pressed(KeyCode::C) {
                chain.set_ball_collision(!chain.ball_collision_enabled());
            }

            // Met à jour la position d'ancrage au centre de la fenêtre
            chain.set_anchor_pos(Vec2::new(
                screen_width() / 2.0,
                screen_height() / 2.0,
            ));
        }

        // Touche Escape : quitter
        if is_key_pressed(KeyCode::Escape) {
            break;  // Sort de la boucle = fin du programme
        }

        // =====================================================================
        // 2. MISE À JOUR DE LA LOGIQUE (UPDATE)
        // =====================================================================

        // Récupère la position de la souris
        // mouse_position() retourne un tuple (x, y)
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);

        // Met à jour la simulation
        // `&mut *simulation` : déréférence la Box puis prend une référence mutable
        simulation.update(mouse_pos);

        // =====================================================================
        // 3. RENDU (RENDER)
        // =====================================================================

        // Dessine la simulation
        // `&*simulation` : déréférence la Box puis prend une référence immutable
        renderer.render(&*simulation);

        // Récupère les options pour l'UI (si c'est une chaîne)
        let (fabrik, collision) = if let Some(chain) = downcast_chain(&*simulation) {
            (Some(chain.fabrik_enabled()), Some(chain.ball_collision_enabled()))
        } else {
            (None, None)
        };

        // Dessine l'interface utilisateur
        renderer.render_ui(
            current_scene.name(),
            get_fps(),
            simulation.particles().len(),
            fabrik,
            collision,
        );

        // =====================================================================
        // 4. ATTENTE DE LA PROCHAINE FRAME
        // =====================================================================
        //
        // next_frame().await fait plusieurs choses :
        // - Présente le buffer de rendu à l'écran
        // - Attend le prochain signal de synchronisation (VSync)
        // - Traite les événements de l'OS (redimensionnement, etc.)
        //
        // Le `.await` est nécessaire car c'est une opération asynchrone.
        // En gros, on "cède le contrôle" au runtime le temps d'attendre.
        //
        // =====================================================================
        next_frame().await;
    }
}

// =============================================================================
// Fonction de création de simulation
// =============================================================================
//
// Cette fonction utilise le pattern "Factory" : elle crée et retourne
// la bonne simulation en fonction du type demandé.
//
// Retourne un `Box<dyn Simulation>` qui peut contenir n'importe quelle
// simulation implémentant le trait Simulation.
//
// =============================================================================

fn create_simulation(scene_type: SceneType) -> Box<dyn Simulation> {
    // `match` est exhaustif : on DOIT gérer tous les cas
    // Le compilateur vérifie cela à la compilation
    match scene_type {
        SceneType::BasicDistance => {
            // Box::new() alloue sur le tas et retourne un Box
            Box::new(BasicDistanceSimulation::default_scene())
        }
        SceneType::SeparateCollision => {
            Box::new(SeparateCollisionSimulation::default_scene())
        }
        SceneType::DistanceChain => {
            Box::new(DistanceChainSimulation::default_scene())
        }
    }
}

// =============================================================================
// Fonctions de downcast (conversion de type dynamique)
// =============================================================================
//
// Rust n'a pas de RTTI (Runtime Type Information) automatique comme Delphi.
// Pour accéder aux méthodes spécifiques d'un type concret depuis un trait
// object, on doit "downcaster" manuellement.
//
// Ici, on utilise une approche simple basée sur le nom de la scène.
// Dans un programme plus complexe, on utiliserait le crate `downcast-rs`.
//
// =============================================================================

/// Tente de convertir une référence &dyn Simulation en &DistanceChainSimulation
fn downcast_chain(simulation: &dyn Simulation) -> Option<&DistanceChainSimulation> {
    // On utilise un trick : vérifier si should_render_main_particle() retourne false
    // car seule DistanceChainSimulation fait ça
    // C'est un hack - en production on utiliserait Any ou downcast-rs
    if !simulation.should_render_main_particle() {
        // SAFETY: On sait que c'est une DistanceChainSimulation
        // car c'est la seule à retourner false pour should_render_main_particle
        //
        // Note: Cette technique est fragile. Une vraie solution utiliserait
        // le trait Any de la bibliothèque standard.
        unsafe {
            // Convertit le pointeur de trait object en pointeur concret
            let ptr = simulation as *const dyn Simulation as *const DistanceChainSimulation;
            Some(&*ptr)
        }
    } else {
        None
    }
}

/// Version mutable du downcast
fn downcast_chain_mut(simulation: &mut dyn Simulation) -> Option<&mut DistanceChainSimulation> {
    if !simulation.should_render_main_particle() {
        unsafe {
            let ptr = simulation as *mut dyn Simulation as *mut DistanceChainSimulation;
            Some(&mut *ptr)
        }
    } else {
        None
    }
}

// =============================================================================
// Notes sur async/await en Rust
// =============================================================================
//
// Rust utilise un modèle asynchrone "zero-cost" :
// - Les fonctions async sont transformées en machines à états à la compilation
// - Pas de threads OS créés automatiquement
// - Nécessite un "runtime" pour exécuter (ici fourni par macroquad)
//
// `async fn` = fonction qui peut être "mise en pause" avec `.await`
// `.await` = attend qu'une opération asynchrone se termine
//
// Dans notre cas, `next_frame().await` :
// 1. Envoie les commandes de rendu au GPU
// 2. Attend la synchronisation verticale (VSync)
// 3. Reprend l'exécution de notre boucle
//
// C'est plus efficace qu'une boucle active car le CPU peut dormir
// pendant l'attente.
//
// =============================================================================

// =============================================================================
// Notes sur Box<dyn Trait> vs génériques
// =============================================================================
//
// Deux façons de gérer le polymorphisme en Rust :
//
// 1. GÉNÉRIQUES (monomorphisation) :
//    ```rust
//    fn process<T: Simulation>(sim: &T) { ... }
//    ```
//    - Le compilateur génère une version pour chaque type concret
//    - Très rapide (pas d'indirection)
//    - Mais : code dupliqué, tous les types doivent être connus à la compilation
//
// 2. TRAIT OBJECTS (dispatch dynamique) :
//    ```rust
//    fn process(sim: &dyn Simulation) { ... }
//    ```
//    - Une seule version du code
//    - Indirection via vtable (table de pointeurs de fonctions)
//    - Permet de stocker différents types dans la même variable
//
// On utilise Box<dyn Simulation> ici car :
// - On veut changer de simulation à l'exécution
// - Les différentes simulations ont des tailles différentes
// - La légère perte de performance est négligeable
//
// =============================================================================
