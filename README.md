# Robotika
LINK TUGAS UTS : https://youtu.be/NerNGfWnnrg?si=X_A7VUck1dRilW10

LINK TUGAS WEEK 9 : https://youtu.be/B614RmNyouE

==========UAS==================

LINK UAS : 

Deksripsi :

## Chapter yang Telah Diselesaikan
### Chapter 2: Komunikasi Antar Node di ROS
- Mengimplementasikan komunikasi antara klien dan server menggunakan **ROS ActionLib**.
- Simulasi mencakup pengiriman goal dari klien ke server dengan batas waktu tertentu.
- Server memproses goal, memberikan umpan balik secara berkala, dan mengirimkan hasil setelah proses selesai atau jika terjadi preemption.
- Menggunakan `SimpleActionClient` dan `SimpleActionServer` untuk komunikasi yang efisien antara komponen.

**Hasil:**
- Berhasil memodelkan komunikasi klien-server menggunakan ROS.
- Memperoleh pemahaman mengenai pengelolaan proses jangka panjang dan penanganan interupsi.

---

### Chapter 3: Visualisasi Robot URDF
- Mengimpor dan mengonfigurasi model **URDF (Unified Robot Description Format)** ke dalam RViz.
- Memvisualisasikan struktur robot, termasuk joint dan link, dengan slider interaktif untuk mengontrol pergerakan joint.
- Menggunakan grid dan fixed frame sebagai referensi untuk menjaga akurasi posisi selama simulasi.

**Hasil:**
- Memahami cara memodelkan robot menggunakan URDF.
- Menguji kinematika dan mekanisme robot di lingkungan simulasi.

---

### Chapter 4: Simulasi Robot Diferensial
- Melakukan simulasi robot beroda diferensial di **Gazebo**.
- Mengintegrasikan kontrol manual melalui keyboard untuk navigasi robot.
- Menggunakan RViz untuk menampilkan data tambahan seperti peta, laser scan, dan grid.

**Hasil:**
- Menguji algoritma gerak dan kontrol robot secara interaktif.
- Mengidentifikasi tantangan pada pemetaan dan sinkronisasi frame di RViz.

---

### Chapter 6: Pemetaan dan Lokalisasi
- Mengimplementasikan pemetaan dan lokalisasi menggunakan **GMapping** di RViz dan Gazebo.
- Memvisualisasikan kemampuan robot untuk memetakan lingkungan sekitarnya dan bernavigasi secara otonom.
- Menampilkan hasil peta dengan detail objek di lingkungan simulasi.

**Hasil:**
- Berhasil membangun sistem pemetaan yang berfungsi.
- Menemukan beberapa area yang perlu ditingkatkan, seperti sinkronisasi frame dan penanganan data redundan.

---

## Tantangan yang Dihadapi
1. **Peringatan TF_REPEATED_DATA:**
   - Mengatasi masalah data transformasi waktu yang redundan di RViz.
2. **Sinkronisasi Frame:**
   - Menyelaraskan frame map untuk memastikan akurasi dalam navigasi dan pemetaan.

---

## Insight yang Didapatkan
- Memperoleh pengalaman praktis dalam menggunakan alat dan paket ROS.
- Memahami pentingnya integrasi sistem dan proses debugging dalam pengembangan robot.
- Mengembangkan kemampuan pemodelan, simulasi, dan navigasi yang penting untuk aplikasi robotika.

---

## Signifikansi untuk Portofolio
Proyek ini menunjukkan kemampuan dalam:
- Membangun dan mengelola simulasi menggunakan **ROS**.
- Memodelkan robot dengan **URDF**.
- Menggunakan **Gazebo** untuk interaksi lingkungan yang realistis.
- Mengintegrasikan **RViz** untuk visualisasi dan analisis performa robot.
- Melakukan debugging dan optimasi sistem robotik.

