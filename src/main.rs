// main.rs

// 1) Filtreleme bileşenini içe aktar
mod filter_component;

// Kullanmak istediğimiz tür ve fonksiyonları 'use' ile getiriyoruz.
use filter_component::{FilterCondition, custom_filter};

fn main() {
    // 2) Örnek bir koleksiyon oluştur (örneğin bir i32 vektörü)
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];

    // 3) Filtreleme için istediğin koşulu (FilterCondition) hazırla
    // Örneğin threshold = 3 olan öğeleri filtrelemek istiyorsak:
    let condition = FilterCondition::new(3);

    // 4) Koleksiyonu custom_filter ile filtrele
    let filtered_result = custom_filter(&numbers, &condition);

    // 5) Sonucu ekrana yazdır
    println!("Orijinal dizi: {:?}", numbers);
    println!("Filtered result (greater than 3): {:?}", filtered_result);
}
