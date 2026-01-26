// =============================================================================
// spatial_hash.rs - Partitionnement spatial pour optimiser les collisions
// =============================================================================
//
// PROBLÈME RÉSOLU :
// -----------------
// Détecter les collisions entre N particules de manière naïve nécessite de
// comparer chaque particule avec toutes les autres : O(n²) comparaisons.
// Avec 4000 particules, cela fait 16 millions de comparaisons par frame !
//
// SOLUTION : SPATIAL HASH (Table de hachage spatiale)
// ---------------------------------------------------
// L'espace est divisé en cellules de taille fixe (grille virtuelle).
// Chaque particule est placée dans la cellule correspondant à sa position.
// Pour trouver les voisins d'une particule, on ne regarde que les cellules
// adjacentes (9 cellules en 2D : la cellule courante + 8 voisines).
//
// Complexité : O(n) en moyenne au lieu de O(n²) !
//
// FONCTIONNEMENT :
// ---------------
//           Cellule (0,0)  Cellule (1,0)  Cellule (2,0)
//          ┌─────────────┬─────────────┬─────────────┐
//          │  ●          │      ●      │             │
//          │      ●      │             │    ●        │
//          ├─────────────┼─────────────┼─────────────┤
//          │             │   ● ← Cette │             │
//          │  ●          │   particule │      ●      │
//          ├─────────────┼─────────────┼─────────────┤
//          │      ●      │      ●      │             │
//          │             │             │    ●        │
//          └─────────────┴─────────────┴─────────────┘
//
// Pour la particule centrale, on ne vérifie que les 9 cellules autour d'elle.
// =============================================================================

use std::collections::HashMap;
use crate::particle::Vec2;

// =============================================================================
// SpatialHash - Structure principale
// =============================================================================

pub struct SpatialHash {
    // Taille d'une cellule de la grille
    // Idéalement = diamètre des particules (2 * rayon)
    cell_size: f32,

    // Table de hachage : clé = identifiant de cellule, valeur = liste d'indices
    // HashMap est la table de hachage standard de Rust
    // i64 : identifiant unique de cellule (combinaison de x et y)
    // Vec<usize> : liste des indices des particules dans cette cellule
    cells: HashMap<i64, Vec<usize>>,
}

impl SpatialHash {
    // Crée une nouvelle table de hachage spatiale
    //
    // Paramètre cell_size : taille des cellules
    // - Trop petit = beaucoup de cellules, overhead mémoire
    // - Trop grand = trop de particules par cellule, perd l'avantage
    // - Idéal = environ 2x le rayon des particules
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            // HashMap::new() crée une table vide
            // La capacité s'ajuste automatiquement
            cells: HashMap::new(),
        }
    }

    // Calcule l'identifiant unique d'une cellule à partir de ses coordonnées
    //
    // Technique : on combine x et y en un seul i64
    // - Les 32 bits de poids fort contiennent x
    // - Les 32 bits de poids faible contiennent y
    //
    // Exemple : cellule (3, 5) → 0x0000000300000005
    #[inline]  // Indique au compilateur d'inliner cette fonction (optimisation)
    fn get_cell_key(&self, cell_x: i32, cell_y: i32) -> i64 {
        // `as` effectue une conversion de type
        // `<<` décale les bits vers la gauche (multiplication par 2^32)
        // `|` combine avec un OR binaire
        // `& 0xFFFFFFFF` masque pour ne garder que 32 bits
        ((cell_x as i64) << 32) | ((cell_y as i64) & 0xFFFFFFFF)
    }

    // Calcule les coordonnées de cellule pour une position donnée
    #[inline]
    fn get_cell_coords(&self, pos: &Vec2) -> (i32, i32) {
        // Division entière : on tronque vers zéro
        // floor() arrondit vers -∞ (important pour les coordonnées négatives)
        let cell_x = (pos.x / self.cell_size).floor() as i32;
        let cell_y = (pos.y / self.cell_size).floor() as i32;
        (cell_x, cell_y)
    }

    // Vide toutes les cellules sans libérer la mémoire allouée
    // Plus efficace que de recréer la structure à chaque frame
    pub fn clear(&mut self) {
        // `&mut self` : on modifie la structure
        // iter_mut() : itérateur mutable sur les entrées
        for list in self.cells.values_mut() {
            list.clear();  // Vide le Vec sans désallouer
        }
    }

    // Insère une particule dans la grille
    //
    // Paramètres :
    // - index : l'indice de la particule dans le tableau principal
    // - pos : la position de la particule
    pub fn insert(&mut self, index: usize, pos: &Vec2) {
        let (cell_x, cell_y) = self.get_cell_coords(pos);
        let key = self.get_cell_key(cell_x, cell_y);

        // entry() donne accès à une entrée de la HashMap
        // or_insert_with() insère une valeur si la clé n'existe pas
        // Le || Vec::new() est une closure (fonction anonyme) appelée si nécessaire
        self.cells
            .entry(key)
            .or_insert_with(|| Vec::new())
            .push(index);
    }

    // Récupère tous les indices des particules potentiellement proches
    //
    // On regarde les 9 cellules autour de la position :
    //   ┌───┬───┬───┐
    //   │NW │ N │NE │   N = Nord, S = Sud
    //   ├───┼───┼───┤   E = Est, W = Ouest
    //   │ W │ C │ E │   C = Centre (cellule courante)
    //   ├───┼───┼───┤
    //   │SW │ S │SE │
    //   └───┴───┴───┘
    pub fn get_nearby(&self, pos: &Vec2, result: &mut Vec<usize>) {
        let (cell_x, cell_y) = self.get_cell_coords(pos);

        // Parcourt les 9 cellules (3x3 autour de la position)
        // -1..=1 est un range inclusif : -1, 0, 1
        for dx in -1..=1 {
            for dy in -1..=1 {
                let key = self.get_cell_key(cell_x + dx, cell_y + dy);

                // Si la cellule existe, ajoute tous ses indices au résultat
                // if let Some(x) = ... est un pattern matching conditionnel
                if let Some(indices) = self.cells.get(&key) {
                    // extend() ajoute tous les éléments d'un itérateur
                    // iter().copied() crée un itérateur qui copie les valeurs
                    result.extend(indices.iter().copied());
                }
            }
        }
    }
}

// =============================================================================
// Notes sur les choix d'implémentation
// =============================================================================
//
// 1. Pourquoi HashMap et pas un tableau 2D ?
//    - L'espace de jeu peut être grand ou infini
//    - Seules les cellules occupées utilisent de la mémoire
//    - Fonctionne avec des coordonnées négatives
//
// 2. Pourquoi ne pas utiliser un BTreeMap ?
//    - HashMap a une complexité O(1) en moyenne pour get/insert
//    - BTreeMap a O(log n) mais garde les clés triées (inutile ici)
//
// 3. Pourquoi vider plutôt que recréer ?
//    - clear() conserve la mémoire allouée
//    - Évite des allocations répétées chaque frame
//    - Les mêmes cellules seront probablement réutilisées
//
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_retrieve() {
        let mut hash = SpatialHash::new(10.0);

        // Insère 3 particules
        hash.insert(0, &Vec2::new(5.0, 5.0));   // Cellule (0, 0)
        hash.insert(1, &Vec2::new(15.0, 5.0));  // Cellule (1, 0)
        hash.insert(2, &Vec2::new(5.0, 15.0));  // Cellule (0, 1)

        // Cherche les voisins de (5, 5)
        let mut nearby = Vec::new();
        hash.get_nearby(&Vec2::new(5.0, 5.0), &mut nearby);

        // Devrait trouver les 3 particules (toutes dans les cellules adjacentes)
        assert_eq!(nearby.len(), 3);
    }

    #[test]
    fn test_far_particles_not_found() {
        let mut hash = SpatialHash::new(10.0);

        // Insère une particule loin
        hash.insert(0, &Vec2::new(100.0, 100.0));

        // Cherche autour de l'origine
        let mut nearby = Vec::new();
        hash.get_nearby(&Vec2::new(0.0, 0.0), &mut nearby);

        // Ne devrait rien trouver
        assert!(nearby.is_empty());
    }
}
