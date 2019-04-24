# OS110_Rust_MediumExercism

Hallo, saya akan bahas salah satu dari beberapa _Exercism_ yang ada. Dan ini juga merupakan salah satu tugas mata kuliah _Operating System_ di Prodi Ilmu Komputer, Universitas Negeri Jakarta. Perkenalkan nama saya Ivan Adi Putra. Yuk langsung scroll

## Triangle
**Triangle** adalah salah satu _problem solving_ yang ada di _Rust Exercisme_ (https://exercism.io/my/tracks/rust). Sesuai dengan namanya, _Triangle_ merupakan _problem solving_ yang kaitannya dengan segitiga. Disini kita diberi petunjuk yaitu diperintahkan untuk menentukan/membuat agar segitiga itu merupakan segitiga sama sisi (_equilateral_), segitiga sama kaki (_isosceles_), atau segitiga sembarang (_scalene_).

## Bagaimana Cara Menyelesaikannya?
Pada saat kita mendownload file _Triangle_ tersebut, di dalamnya terdapat template fungsi, struct, dll yang memudahkan kita dalam menyelesaikan masalah.
### Langkah 1
Tentukan structnya, yaitu berupa varible input yang akan dipanggil nanti di dalam fungsi. Karna kita akan menggunakan variable input berupa angka, maka formatnya adalah _integer_. Berikut cara menulis _integer_ di dalam Rust.
```rust
pub struct Triangle {
    a: u64, 
    b: u64, 
    c: u64,
}
```
### Langkah 2
Kita akan membuat fungsi pertama yaitu memanggil variable input pada _struct_ tadi menggunakan pengindeksan _array_. Berikut ini caranya
```rust
impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let tr = Triangle {a: sides[0], b: sides[1], c: sides[2]};
        Some(tr)
}
```
a adalah indeks ke 1 dalam array, maka masukkan 0. b adalah indeks ke 2 dalam array, maka masukkan 1. Berikut dengan c.
### Langkah 3
Setelah kita tadi membuat fungsi untuk menjalankan variable penginputan maka sekarang kita lanjutkan dengan membuat fungsi dengan memasukkan konsep dari segitiga sama sisi, segitiga sama kaki, dan segitiga sembarang.
Sisi pada segitiga disini yaitu sisi a, sisi b, sisi c yang tadi kita sudah buat menggunakan array [a, b, c].

**Segitiga Sama Sisi** yaitu segitiga yang semua sisinya sama. Maka:
```rust
pub fn is_equilateral(&self) -> bool {
        return self.a == self.b && self.b == self.c
 }
 ```
 
 **Segitiga Sembarang** yaitu segitiga yang semua sisinya tidak sama. Maka:
 
 ```rust
 pub fn is_scalene(&self) -> bool {
        return self.a != self.b && self.b != self.c && self.a != self.c
 }
 ```
 
 **Segitiga Sama Kaki** yaitu segitiga yang kedua sisinya ada yang sama panjang. Maka:
 ```rust
 pub fn is_isosceles(&self) -> bool {
        return (self.a == self.b && self.a != self.c && self.b != self.c) || (self.b == self.c && self.b != self.a && self.c != self.a) || (self.c == self.a && self.c != self.b && self.a != self.b)
}
```

Nah, sampai disini kita telah menyelesaikan konsep dari ketiga segitiga tersebut. Jika kita jalankan, akan ada beberapa error karena teryata ada beberapa syarat yaitu:
- Sisi segitiga tidak boleh 0 (nol)
- Jumlah dari dua sisi tidak lebih dari satu sisi yang lain
Maka dari itu kita tambahkan saja pada fungsi pertama seperti ini:
```rust
impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if (sides[0], sides[1], sides[2]) == (0, 0, 0) || sides[2] >= sides[1] + sides[0] || sides[1] >= sides[2] + sides [0] || sides [0] >= sides[1] + sides[2] {
            return None
        }
        else {
        let tr = Triangle {a: sides[0], b: sides[1], c: sides[2]};
        Some(tr)
        }
}
```

Nah, jika kita jalankan lagi maka hasilnya tidak ada yang error lagi
Itulah review singkat dari _Triangle_ di _Rust Exercism_
