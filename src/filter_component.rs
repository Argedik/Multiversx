/// Bu struct, filtreleme için gerekli koşulu saklar.
// Örnek olarak bir "threshold" (eşik değeri) alıyoruz.
pub struct FilterCondition {
  threshold: i32,
}

impl FilterCondition {
  /// Yeni bir FilterCondition oluşturmak için yardımcı fonksiyon
  pub fn new(threshold: i32) -> Self {
      FilterCondition { threshold }
  }

  /// Parametre olarak aldığı öğenin (i32) belirli bir koşulu
  /// sağlayıp sağlamadığına karar veren fonksiyon
  pub fn is_match(&self, item: &i32) -> bool {
      *item > self.threshold
  }
}

/// Gönderilen koleksiyondaki öğeleri tek tek inceleyerek, 
/// FilterCondition ile sağlanan koşula uyanları yeni bir 
/// vektörde döndüren fonksiyon
pub fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
  let mut filtered = Vec::new();
  for item in collection {
      if condition.is_match(item) {
          filtered.push(*item);
      }
  }
  filtered
}
