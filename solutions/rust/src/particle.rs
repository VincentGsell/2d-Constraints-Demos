// =============================================================================
// particle.rs - Types de base pour les particules et les contraintes
// =============================================================================
// Ce module définit les structures fondamentales du système de particules :
// - Vec2 : Vecteur 2D avec opérations mathématiques
// - Particle : Une particule avec position, rayon et couleur
// - ConstraintResolver : Fonctions pour résoudre les contraintes
// =============================================================================

use macroquad::prelude::*;

// =============================================================================
// Vec2 - Vecteur 2D
// =============================================================================
// En Rust, on utilise souvent des structs pour représenter des données.
// #[derive(...)] génère automatiquement des implémentations de traits :
// - Clone : permet de dupliquer la valeur avec .clone()
// - Copy : permet la copie implicite (comme les types primitifs)
// - Debug : permet l'affichage avec {:?} pour le débogage
// - PartialEq : permet la comparaison avec ==
// - Default : fournit une valeur par défaut (0.0, 0.0)
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,  // Coordonnée X (f32 = nombre flottant 32 bits)
    pub y: f32,  // Coordonnée Y
}

// Bloc d'implémentation pour Vec2
// En Rust, les méthodes sont définies dans des blocs `impl`
impl Vec2 {
    // Constructeur - crée un nouveau Vec2
    // `pub` signifie que cette fonction est publique (accessible depuis d'autres modules)
    // `const` signifie que cette fonction peut être évaluée à la compilation
    pub const fn new(x: f32, y: f32) -> Self {
        // `Self` est un alias pour le type actuel (Vec2)
        Self { x, y }
    }

    // Vecteur nul (0, 0) - utile comme valeur par défaut
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    // Calcule la longueur (magnitude) du vecteur
    // Formule : sqrt(x² + y²) - théorème de Pythagore
    pub fn length(&self) -> f32 {
        // `self` est une référence à l'instance actuelle
        // En Rust, on utilise `self` au lieu de `this` (autres langages)
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Calcule la longueur au carré (évite la racine carrée coûteuse)
    // Utile pour comparer des distances sans calcul de racine
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    // Normalise le vecteur (le rend de longueur 1)
    // Un vecteur normalisé est appelé "vecteur unitaire"
    pub fn normalize(&self) -> Self {
        let len = self.length();
        // On vérifie que la longueur n'est pas nulle pour éviter la division par zéro
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            // Si le vecteur est nul, on retourne un vecteur nul
            Self::ZERO
        }
    }

    // Modifie la longueur du vecteur tout en gardant sa direction
    // Exemple : set_length(5.0) sur un vecteur (3, 4) donne (3, 4) normalisé * 5
    pub fn with_length(&self, new_length: f32) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len * new_length,
                y: self.y / len * new_length,
            }
        } else {
            Self::ZERO
        }
    }

    // Calcule la distance entre deux points
    // Utilise la soustraction de vecteurs puis calcule la longueur
    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    // Convertit en Vec2 de macroquad pour le rendu
    // macroquad utilise son propre type Vec2, on doit convertir
    pub fn to_macroquad(&self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2::new(self.x, self.y)
    }

    // Crée un Vec2 depuis le type macroquad
    pub fn from_macroquad(v: macroquad::math::Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

// =============================================================================
// Implémentation des opérateurs pour Vec2
// =============================================================================
// En Rust, on surcharge les opérateurs en implémentant des traits du module std::ops
// Cela permet d'écrire du code naturel comme : vec_a + vec_b, vec * 2.0, etc.
// =============================================================================

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

// Opérateur + : addition de deux vecteurs
impl Add for Vec2 {
    type Output = Self;  // Le type du résultat de l'addition

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Opérateur - : soustraction de deux vecteurs
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Opérateur * : multiplication par un scalaire (nombre)
impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Opérateur / : division par un scalaire
impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

// Opérateur += : addition en place
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        // `&mut self` signifie qu'on modifie l'instance en place
        self.x += other.x;
        self.y += other.y;
    }
}

// Opérateur -= : soustraction en place
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// =============================================================================
// Particle - Structure représentant une particule
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub pos: Vec2,       // Position de la particule
    pub radius: f32,     // Rayon de la particule
    pub color: Color,    // Couleur (type de macroquad)
}

impl Particle {
    // Crée une nouvelle particule
    pub fn new(pos: Vec2, radius: f32, color: Color) -> Self {
        Self { pos, radius, color }
    }
}

// Implémentation de Default pour Particle
// Permet d'utiliser Particle::default() ou ..Default::default() dans les initialisations
impl Default for Particle {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            radius: 10.0,
            color: WHITE,  // WHITE est une constante de macroquad
        }
    }
}

// =============================================================================
// ConstraintResolver - Fonctions de résolution de contraintes
// =============================================================================
// Les contraintes sont des règles que les particules doivent respecter.
// La contrainte de distance maintient une distance fixe entre deux points.
// =============================================================================

pub struct ConstraintResolver;

impl ConstraintResolver {
    // Contrainte de distance : positionne `point` à une distance fixe de `anchor`
    //
    // Principe :
    // 1. Calcule la direction de anchor vers point (vecteur normalisé)
    // 2. Place point à la distance spécifiée dans cette direction
    //
    // Paramètres :
    // - point : le point à contraindre
    // - anchor : le point d'ancrage (ne bouge pas)
    // - distance : la distance à maintenir
    //
    // Retourne : la nouvelle position du point
    pub fn distance(point: Vec2, anchor: Vec2, distance: f32) -> Vec2 {
        // Calcule le vecteur de anchor vers point
        let direction = point - anchor;

        // Normalise (longueur 1) puis multiplie par la distance désirée
        // Ajoute anchor pour obtenir la position absolue
        direction.normalize() * distance + anchor
    }
}

// =============================================================================
// Fonctions utilitaires
// =============================================================================

// Génère une couleur aléatoire
// Utilise le crate `rand` pour les nombres aléatoires
// Note: on utilise `::rand` car macroquad expose aussi un module `rand`
pub fn random_color() -> Color {
    use ::rand::Rng;  // `::rand` force l'utilisation du crate externe
    // thread_rng() donne un générateur aléatoire local au thread
    let mut rng = ::rand::thread_rng();
    Color::new(
        rng.gen::<f32>(),  // Rouge : 0.0 à 1.0
        rng.gen::<f32>(),  // Vert : 0.0 à 1.0
        rng.gen::<f32>(),  // Bleu : 0.0 à 1.0
        1.0,               // Alpha (opacité) : toujours 1.0 (opaque)
    )
}

// =============================================================================
// Tests unitaires
// =============================================================================
// En Rust, les tests sont intégrés au code source dans un module #[cfg(test)]
// Ce module n'est compilé que lors de l'exécution de `cargo test`
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;  // Importe tout du module parent

    #[test]
    fn test_vec2_length() {
        // Vecteur (3, 4) a une longueur de 5 (triangle 3-4-5)
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalize();
        // Un vecteur normalisé a une longueur de 1
        assert!((n.length() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_constraint_distance() {
        let point = Vec2::new(10.0, 0.0);
        let anchor = Vec2::ZERO;
        let result = ConstraintResolver::distance(point, anchor, 5.0);
        // Le point devrait être à distance 5 de l'ancre
        assert!((result.length() - 5.0).abs() < 0.0001);
    }
}
