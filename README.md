
# qris-rs

**qris-rs** adalah sebuah pustaka berbasis Rust yang dirancang untuk mengubah dan memparsing data **QRIS (Quick Response Code Indonesian Standard)**. Proyek ini bertujuan untuk mempermudah pengelolaan data QRIS secara efisien, aman, dan cepat.

## Fitur

- ✅ Parsing kode QRIS menjadi struktur data yang mudah diakses.
- ✅ Validasi data QRIS sesuai standar QRIS Nasional.
- ✅ Mendukung konversi struktur data menjadi string QRIS.
- ✅ Performa tinggi dengan penggunaan Rust.

## Instalasi

Tambahkan pustaka ini ke dalam proyek Anda dengan menambahkan dependensi berikut pada `Cargo.toml`:

```toml
[dependencies]
qris = "0.1.0"
```

Lalu, jalankan perintah berikut untuk mengunduh dependensi:

```bash
cargo build
```

## Penggunaan

### Parsing QRIS

Untuk memparsing kode QRIS:

```rust
use qris::node::Nodes;

fn main(){
    let content = String::from("00020101021126710019ID.CO.CIMBNIAGA.WWW011878728356757817222102150002186871651250303UMI51450015ID.OR.QRNPG.WWW0215ID81275673266770303UMI5204599953033605802ID5914AABBCCD*6714516006KEDIRI61054423462120708123456786304097D");
    match &Nodes::from_code(&content) {
        Ok(parsed) => {
            println!("{:#?}", parsed);
        }
        Err(err) => eprint!("Fail: {}", err)
    }
}
```

### Mengubah data jumlah pada qris

contoh mengubah data jumlah yang harus dibayar pada qris

```rust
use qris::node::Nodes;

fn main(){
    let content = String::from("00020101021126710019ID.CO.CIMBNIAGA.WWW011878728356757817222102150002186871651250303UMI51450015ID.OR.QRNPG.WWW0215ID81275673266770303UMI5204599953033605802ID5914AABBCCD*6714516006KEDIRI61054423462120708123456786304097D");
    match &mut Nodes::from_code(&content) {
        Ok(parsed) => {
            parsed.set_amount(20_000);
            parsed.rewrite_crc16();
            print!("{}", parsed.dumps());
        }
        Err(err) => eprint!("Fail: {}", err)
    }
}
```



## Kontribusi

Kontribusi sangat diterima! Jika Anda ingin berkontribusi:

1. Fork repositori ini.
2. Buat branch fitur Anda (`git checkout -b fitur-anda`).
3. Commit perubahan Anda (`git commit -m 'Tambah fitur baru'`).
4. Push branch Anda (`git push origin fitur-anda`).
5. Buat Pull Request.

## Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

