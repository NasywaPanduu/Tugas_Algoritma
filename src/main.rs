use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};

#[derive(Clone)] // Menambahkan trait Clone untuk struct Passenger
struct Passenger {
    nama: String,
}

struct DaftarPenumpang {
    passengers: Vec<Passenger>,
    passenger_stack: VecDeque<Passenger>,
}

fn main() {
    let mut daftar = DaftarPenumpang {
        passengers: Vec::new(),
        passenger_stack: VecDeque::new(),
    };

    loop {
        println!("Pilihan:");
        println!("1. Lihat daftar penumpang");
        println!("2. Tambah penumpang");
        println!("3. Edit penumpang");
        println!("4. Hapus penumpang");
        println!("5. Tambah ke Stack");
        println!("6. Keluar");

        let pilihan: u32 = input("Masukkan pilihan Anda: ").trim().parse().expect("Masukkan angka!");

        match pilihan {
            1 => tampilkan_daftar(&daftar.passengers),
            2 => tambah_penumpang(&mut daftar),
            3 => edit_penumpang(&mut daftar.passengers),
            4 => hapus_penumpang(&mut daftar.passengers),
            5 => tambah_ke_stack(&mut daftar),
            6 => break,
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn input(pesan: &str) -> String {
    print!("{}", pesan);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input!");
    
    input.trim().to_string()
}

fn tampilkan_daftar(daftar: &Vec<Passenger>) {
    println!("Daftar Penumpang:");
    for (index, penumpang) in daftar.iter().enumerate() {
        println!("{}. {}", index + 1, penumpang.nama);
    }
}

fn tambah_penumpang(daftar: &mut DaftarPenumpang) {
    let nama = input("Masukkan nama penumpang: ");
    let penumpang = Passenger { nama };
    daftar.passengers.push(penumpang.clone());
    println!("Penumpang berhasil ditambahkan!");
}

fn edit_penumpang(daftar: &mut Vec<Passenger>) {
    tampilkan_daftar(daftar);
    let index: usize = input("Masukkan nomor penumpang yang ingin diubah: ").trim().parse().expect("Masukkan angka!");
    if index > 0 && index <= daftar.len() {
        let new_nama = input("Masukkan nama baru: ");
        daftar[index - 1].nama = new_nama;
        println!("Penumpang berhasil diubah!");
    } else {
        println!("Nomor penumpang tidak valid!");
    }
}

fn hapus_penumpang(daftar: &mut Vec<Passenger>) {
    tampilkan_daftar(daftar);
    let index: usize = input("Masukkan nomor penumpang yang ingin dihapus: ").trim().parse().expect("Masukkan angka!");
    if index > 0 && index <= daftar.len() {
        daftar.remove(index - 1);
        println!("Penumpang berhasil dihapus!");
    } else {
        println!("Nomor penumpang tidak valid!");
    }
}

fn tambah_ke_stack(daftar: &mut DaftarPenumpang) {
    if let Some(penumpang) = daftar.passengers.pop() {
        daftar.passenger_stack.push_front(penumpang.clone());
        println!("Penumpang berhasil ditambahkan ke Stack!");
    } else {
        println!("Daftar penumpang kosong!");
    }
}
