fn belajar_multiple_variable(){
    // immutable, inference
    let (nama,kelas) = ("Deni Gunawan",12);
    println!("nama: {}",nama);
    println!("kelas: {}",kelas);

    // mutable, inference


    // immutable, manifest
    let(keluarga,jenis):(i8,i8) = (32,12);
    println!("keluarga: {}",keluarga);
    println!("jenis: {}",jenis);

    // variasi tipe immutable, mutable, inference dan manifest
    let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 4);
    println!("var5: {0}", var5); // hasilnya => var5: 64
    println!("var6: {0}", var6); // hasilnya => var6: 12
    var6 = 24;
    println!("var6: {0}", var6); // hasilnya => var6: 24
    println!("var7: {0}", var7); // hasilnya => var7: 4
}


fn belajar_deklarasi_variable(){
    belajar_multiple_variable();
    /*
    - let (merupakan tipe data imutable atau bersifat tetap tidak bisa di manuplasi dan bertipe inference )
    - inference artinya compiler mendeteksi nilai secara otomatis
    - let dengan deklarasi variable disebut sebagai manifest typing
    - let mut (mutable artinya variable yang kita definisikan valuenya bisa berubah ubah)
    -
    - varibale di rust didefinisikan sebagai index. jika variable telah di eksekusi maka ketika kita perlu memanggil
    - kembali index tersebut maka dia tidak akan dipanggil karena variable yang kita call sudah di paka
    */

    // mutable,inference
    let mut message_number = 1;
    // inmutable,inference
    let message_string = "ini string message";
    println!("message number {1} : {0}",message_number,message_string);

    // mutable,inference
    message_number = 2;
    let message_2 = "world";
    println!("message number {}: {}", message_number, message_2);

    message_number = 3;
    // imutable,manifest
    let message_2:i8 = 24;
    println!("message number {}: {}", message_number, message_2);


    // caramenulis variable dengan tipe manifest bisa seperti ini
    let belajar:i8;
    belajar = 10;
    println!(" tampilkan data : {}",belajar);
}


fn komentar_belajar() {
    // ini single commentar
    println!("Keep learning Rust, you're doing great!");
    /*
    multiple komentar
    */
}


fn main() {
    belajar_deklarasi_variable();
    println!("Hello, world!");
    println!("===========");
    println!("Testing");
    komentar_belajar();
}
