use std::collections::HashMap;
use std::io;

// Definisi struktur `Siswa` untuk menyimpan data siswa
#[derive(Debug, Clone)]
struct Siswa {
    nama: String,     // Nama siswa
    umur: u8,         // Umur siswa
    jurusan: String,  // Jurusan yang diambil oleh siswa
}

// Implementasi untuk `Jurusan` yang berfungsi untuk mengelola jurusan
#[derive(Debug)]
struct Jurusan {
    nama: String,     // Nama jurusan
    kuota: u8,        // Kuota untuk jurusan tersebut
    siswa: Vec<Siswa>, // Daftar siswa yang mendaftar di jurusan ini
}

impl Jurusan {
    // Konstruktor untuk membuat jurusan baru
    fn new(nama: &str, kuota: u8) -> Self {
        Jurusan {
            nama: nama.to_string(),
            kuota,
            siswa: Vec::new(),
        }
    }

    // Metode untuk menambahkan siswa ke dalam jurusan
    fn tambah_siswa(&mut self, siswa: Siswa) -> Result<(), String> {
        // Memeriksa apakah kuota jurusan sudah penuh
        if self.siswa.len() < self.kuota as usize {
            self.siswa.push(siswa);
            Ok(())
        } else {
            Err("Kuota jurusan penuh".to_string()) // Jika kuota penuh, gagal menambah siswa
        }
    }
}

// Struktur untuk mengelola data login admin
struct Admin {
    username: String, // Username untuk login
    password: String, // Password untuk login
}

impl Admin {
    // Konstruktor untuk membuat admin baru
    fn new(username: &str, password: &str) -> Self {
        Admin {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    // Metode untuk melakukan login
    fn login(&self, username: &str, password: &str) -> bool {
        self.username == username && self.password == password // Memeriksa kesesuaian username dan password
    }
}

// Struktur untuk Kepala Sekolah yang memiliki akses ke laporan penerimaan siswa
struct KepalaSekolah {
    nama: String, // Nama Kepala Sekolah
}

impl KepalaSekolah {
    // Konstruktor untuk membuat Kepala Sekolah
    fn new(nama: &str) -> Self {
        KepalaSekolah {
            nama: nama.to_string(),
        }
    }

    // Metode untuk melihat laporan penerimaan siswa per jurusan
    fn lihat_laporan(&self, jurusan: &HashMap<String, Jurusan>) {
        println!("Laporan Penerimaan Siswa:");
        // Menampilkan laporan siswa per jurusan
        for (nama, j) in jurusan {
            println!("Jurusan: {}", nama);
            println!("Jumlah Siswa: {}/{}", j.siswa.len(), j.kuota);
            for siswa in &j.siswa {
                println!("  - {} ({} tahun)", siswa.nama, siswa.umur);
            }
        }
    }
}

// Struktur utama `Penerimaan` untuk mengelola seluruh sistem penerimaan siswa
struct Penerimaan {
    jurusan: HashMap<String, Jurusan>, // Daftar jurusan yang tersedia
}

impl Penerimaan {
    // Konstruktor untuk membuat sistem penerimaan baru dengan beberapa jurusan
    fn new() -> Self {
        let mut jurusan = HashMap::new();
        // Menambahkan jurusan dengan kuota masing-masing
        jurusan.insert("Farmasi".to_string(), Jurusan::new("Farmasi", 10));
        jurusan.insert("Perawat".to_string(), Jurusan::new("Perawat", 15));
        jurusan.insert("TKJ".to_string(), Jurusan::new("TKJ", 20));

        Penerimaan { jurusan }
    }

    // Metode untuk mendaftar siswa ke dalam jurusan
    fn daftar_siswa(&mut self, nama: String, umur: u8, jurusan: String) -> Result<(), String> {
        // Mencari jurusan yang dipilih dan menambahkannya jika ada
        if let Some(j) = self.jurusan.get_mut(&jurusan) {
            let siswa = Siswa { nama, umur, jurusan: jurusan.clone() };
            j.tambah_siswa(siswa)
        } else {
            Err("Jurusan tidak ditemukan".to_string()) // Jika jurusan tidak ada, menampilkan pesan error
        }
    }

    // Metode untuk menampilkan daftar siswa di jurusan tertentu
    fn tampilkan_siswa(&self, jurusan: &str) {
        if let Some(j) = self.jurusan.get(jurusan) {
            println!("Daftar Siswa di Jurusan {}:", jurusan);
            for siswa in &j.siswa {
                println!("- {} ({} tahun)", siswa.nama, siswa.umur);
            }
        } else {
            println!("Jurusan tidak ditemukan");
        }
    }

    // Metode untuk login siswa berdasarkan nama
    fn login_siswa(&self, nama: &str) -> Option<&Siswa> {
        for jurusan in self.jurusan.values() {
            if let Some(siswa) = jurusan.siswa.iter().find(|s| s.nama == nama) {
                return Some(siswa); // Jika siswa ditemukan, mengembalikan data siswa
            }
        }
        None // Jika siswa tidak ditemukan, mengembalikan None
    }

    // Metode untuk menampilkan daftar jurusan yang tersedia
    fn tampilkan_jurusan(&self) {
        println!("Daftar Jurusan:");
        for (nama, jurusan) in &self.jurusan {
            println!("- {} (Kuota: {}, Terisi: {})", nama, jurusan.kuota, jurusan.siswa.len());
        }
    }
}

fn main() {
    // Membuat instance dari Admin, KepalaSekolah, dan Penerimaan
    let admin = Admin::new("admin", "password123");
    let kepala_sekolah = KepalaSekolah::new("Bapak Kepala Sekolah");
    let mut penerimaan = Penerimaan::new();

    loop {
        // Menampilkan menu utama
        println!("\n=== Sistem Penerimaan Siswa SMK Bhakti Kencana! ===");
        println!("1. Login Admin");
        println!("2. Tampilkan Laporan (Kepala Sekolah)");
        println!("3. Login Siswa");
        println!("4. Keluar");
        println!("Pilih opsi: ");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).unwrap();
        let pilihan = pilihan.trim();

        match pilihan {
            // Proses login admin
            "1" => {
                let mut username = String::new();
                let mut password = String::new();

                println!("Masukkan username: ");
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim();

                println!("Masukkan password: ");
                io::stdin().read_line(&mut password).unwrap();
                let password = password.trim();

                if admin.login(username, password) {
                    println!("Login berhasil!");

                    loop {
                        println!("\n=== Menu Admin ===");
                        println!("1. Daftar Siswa");
                        println!("2. Tampilkan Siswa per Jurusan");
                        println!("3. Kembali ke Menu Utama");
                        println!("Pilih opsi: ");

                        let mut sub_pilihan = String::new();
                        io::stdin().read_line(&mut sub_pilihan).unwrap();
                        let sub_pilihan = sub_pilihan.trim();

                        match sub_pilihan {
                            // Daftar siswa baru
                            "1" => {
                                let mut nama = String::new();
                                let mut umur = String::new();
                                let mut jurusan = String::new();

                                println!("Masukkan nama siswa: ");
                                io::stdin().read_line(&mut nama).unwrap();
                                let nama = nama.trim().to_string();

                                println!("Masukkan umur siswa: ");
                                io::stdin().read_line(&mut umur).unwrap();
                                let umur: u8 = umur.trim().parse().unwrap_or(0);

                                println!("Pilih jurusan (Farmasi/Perawat/TKJ): ");
                                io::stdin().read_line(&mut jurusan).unwrap();
                                let jurusan = jurusan.trim().to_string();

                                match penerimaan.daftar_siswa(nama, umur, jurusan) {
                                    Ok(_) => println!("Pendaftaran berhasil"),
                                    Err(e) => println!("Pendaftaran gagal: {}", e),
                                }
                            }
                            // Tampilkan daftar siswa di jurusan tertentu
                            "2" => {
                                let mut jurusan = String::new();
                                println!("Masukkan jurusan (Farmasi/Perawat/TKJ): ");
                                io::stdin().read_line(&mut jurusan).unwrap();
                                let jurusan = jurusan.trim();

                                penerimaan.tampilkan_siswa(jurusan);
                            }
                            // Kembali ke menu utama
                            "3" => break,
                            _ => println!("Pilihan tidak valid"),
                        }
                    }
                } else {
                    println!("Login gagal. Username atau password salah.");
                }
            }
            // Tampilkan laporan kepala sekolah
            "2" => {
                kepala_sekolah.lihat_laporan(&penerimaan.jurusan);
            }
            // Login siswa
            "3" => {
                let mut nama = String::new();
                println!("Masukkan nama siswa: ");
                io::stdin().read_line(&mut nama).unwrap();
                let nama = nama.trim();

                if let Some(siswa) = penerimaan.login_siswa(nama) {
                    println!("Login siswa berhasil. Selamat datang, {}!", siswa.nama);
                    loop {
                        println!("\n=== Menu Siswa ===");
                        println!("1. Lihat Profil");
                        println!("2. Lihat Daftar Jurusan");
                        println!("3. Logout");
                        println!("Pilih opsi: ");

                        let mut sub_pilihan = String::new();
                        io::stdin().read_line(&mut sub_pilihan).unwrap();
                        let sub_pilihan = sub_pilihan.trim();

                        match sub_pilihan {
                            // Lihat profil siswa
                            "1" => {
                                println!("Profil Siswa:");
                                println!("Nama: {}", siswa.nama);
                                println!("Umur: {}", siswa.umur);
                                println!("Jurusan: {}", siswa.jurusan);
                            }
                            // Lihat daftar jurusan
                            "2" => {
                                penerimaan.tampilkan_jurusan();
                            }
                            // Logout siswa
                            "3" => {
                                println!("Logout berhasil.");
                                break;
                            }
                            _ => println!("Pilihan tidak valid"),
                        }
                    }
                } else {
                    println!("Login siswa gagal. Nama tidak ditemukan.");
                }
            }
            // Keluar dari program
            "4" => {
                println!("Keluar dari sistem");
                break;
            }
            _ => println!("Pilihan tidak valid"),
        }
    }
}
