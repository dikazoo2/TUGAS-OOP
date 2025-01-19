use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, Label, Box, Orientation};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Siswa {
    nama: String,
    umur: u8,
    jurusan: String,
}

#[derive(Debug)]
struct Jurusan {
    nama: String,
    kuota: u8,
    siswa: Vec<Siswa>,
}

impl Jurusan {
    fn new(nama: &str, kuota: u8) -> Self {
        Jurusan {
            nama: nama.to_string(),
            kuota,
            siswa: Vec::new(),
        }
    }

    fn tambah_siswa(&mut self, siswa: Siswa) -> Result<(), String> {
        if self.siswa.len() < self.kuota as usize {
            self.siswa.push(siswa);
            Ok(())
        } else {
            Err("Kuota jurusan penuh".to_string())
        }
    }
}

struct Penerimaan {
    jurusan: HashMap<String, Jurusan>,
}

impl Penerimaan {
    fn new() -> Self {
        let mut jurusan = HashMap::new();
        jurusan.insert("Farmasi".to_string(), Jurusan::new("Farmasi", 10));
        jurusan.insert("Perawat".to_string(), Jurusan::new("Perawat", 15));
        jurusan.insert("TKJ".to_string(), Jurusan::new("TKJ", 20));

        Penerimaan { jurusan }
    }

    fn daftar_siswa(&mut self, nama: String, umur: u8, jurusan: String) -> Result<(), String> {
        if let Some(j) = self.jurusan.get_mut(&jurusan) {
            let siswa = Siswa { nama, umur, jurusan: jurusan.clone() };
            j.tambah_siswa(siswa)
        } else {
            Err("Jurusan tidak ditemukan".to_string())
        }
    }
}

fn main() {
    let application = Application::new(Some("com.example.siswa"), Default::default());

    application.connect_activate(|app| {
        // Membuat window utama
        let window = ApplicationWindow::new(app);
        window.set_title("Sistem Penerimaan Siswa");
        window.set_default_size(400, 300);

        // Membuat layout vertical
        let vbox = Box::new(Orientation::Vertical, 10);

        // Membuat tombol login
        let login_button = Button::with_label("Login Admin");
        vbox.append(&login_button);

        // Menambahkan event pada tombol login
        login_button.connect_clicked(move |_| {
            // Membuat window login admin
            let login_window = ApplicationWindow::new(app);
            login_window.set_title("Login Admin");
            login_window.set_default_size(300, 200);

            let vbox = Box::new(Orientation::Vertical, 10);

            let username_label = Label::new(Some("Username:"));
            let username_entry = Entry::new();
            let password_label = Label::new(Some("Password:"));
            let password_entry = Entry::new();
            password_entry.set_visibility(false);

            let login_button = Button::with_label("Login");

            vbox.append(&username_label);
            vbox.append(&username_entry);
            vbox.append(&password_label);
            vbox.append(&password_entry);
            vbox.append(&login_button);

            login_window.set_child(Some(&vbox));
            login_window.show();

            // Event ketika tombol login diklik
            login_button.connect_clicked(move |_| {
                let username = username_entry.text().to_string();
                let password = password_entry.text().to_string();

                if username == "admin" && password == "password123" {
                    println!("Login berhasil!");
                    // Masuk ke menu admin
                } else {
                    println!("Login gagal!");
                }
            });
        });

        window.set_child(Some(&vbox));
        window.show();
    });

    application.run();
}
