use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .about-wrapper {
    max-width: 760px;
    margin: 0 auto;
    padding: 60px 24px 80px;
  }

  .about-header {
    margin-bottom: 48px;
  }

  .about-header .label {
    margin-bottom: 20px;
  }

  .about-header h1 {
    margin-bottom: 16px;
  }

  .about-header p {
    font-size: 1.05rem;
    line-height: 1.8;
    max-width: 600px;
  }

  .about-section {
    margin-bottom: 32px;
  }

  .about-section-card {
    padding: 32px;
  }

  .about-section-card h3 {
    margin-bottom: 12px;
    color: var(--text-primary);
  }

  .about-section-card p {
    font-size: 14px;
    line-height: 1.8;
    margin-bottom: 0;
  }

  .about-section-card p + p {
    margin-top: 12px;
  }

  .skill-list {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 16px;
  }

  .skill-tag {
    padding: 5px 14px;
    border-radius: 100px;
    background: var(--accent-soft);
    border: 1px solid var(--accent-border);
    color: var(--accent);
    font-size: 12px;
    font-weight: 500;
  }

  .timeline {
    display: flex;
    flex-direction: column;
    gap: 20px;
    margin-top: 20px;
  }

  .timeline-item {
    display: flex;
    gap: 16px;
  }

  .timeline-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--accent);
    margin-top: 6px;
    flex-shrink: 0;
    box-shadow: 0 0 10px var(--accent);
  }

  .timeline-content h4 {
    margin-bottom: 4px;
  }

  .timeline-content p {
    font-size: 13px;
  }
</style>

<div class="about-wrapper">
  <div class="about-header">
    <div class="label">Tentang</div>
    <h1 style="margin-top:16px;">Halo, Saya sedang<br>Belajar</h1>
    <p style="margin-top:16px;">
      Halaman ini adalah dokumentasi perjalanan belajar saya di bidang ilmu komputer
      — khususnya algoritma, struktur data, dan pemrograman. Saya percaya bahwa
      proses belajar yang terdokumentasi dengan baik adalah investasi jangka panjang.
    </p>
  </div>

  <div class="about-section">
    <div class="about-section-card glass">
      <h3>Latar Belakang</h3>
      <p>
        Saya sedang mendalami ilmu komputer dengan fokus pada pemahaman konsep fundamental.
        Website ini dibuat sebagai catatan pribadi sekaligus portofolio pembelajaran yang dapat
        diakses kapan saja.
      </p>
      <p>
        Setiap topik yang saya pelajari akan didokumentasikan lengkap dengan implementasi kode,
        penjelasan konsep, dan simulasi interaktif agar pemahaman lebih mendalam.
      </p>
    </div>
  </div>

  <div class="about-section">
    <div class="about-section-card glass">
      <h3>Topik yang Dipelajari</h3>
      <div class="skill-list">
        <span class="skill-tag">Rust</span>
        <span class="skill-tag">Insertion Sort</span>
        <span class="skill-tag">Bubble Sort</span>
        <span class="skill-tag">Algoritma</span>
        <span class="skill-tag">Struktur Data</span>
        <span class="skill-tag">Web Development</span>
      </div>
    </div>
  </div>

  <div class="about-section">
    <div class="about-section-card glass">
      <h3>Proses Pembelajaran</h3>
      <div class="timeline">
        <div class="timeline-item">
          <div class="timeline-dot"></div>
          <div class="timeline-content">
            <h4>Insertion Sort</h4>
            <p>Mempelajari algoritma insertion sort dan membuat simulasi visualisasi animasi perubahan data secara step-by-step.</p>
          </div>
        </div>
        <div class="timeline-item">
          <div class="timeline-dot"></div>
          <div class="timeline-content">
            <h4>Bubble Sort</h4>
            <p>Mempelajari algoritma bubble sort dan membuat simulasi visualisasi animasi perbandingan dan pertukaran elemen.</p>
          </div>
        </div>
        <div class="timeline-item">
          <div class="timeline-dot" style="background:var(--text-muted);box-shadow:none;opacity:0.4;"></div>
          <div class="timeline-content">
            <h4 style="color:var(--text-muted);">Topik selanjutnya</h4>
            <p>Topik berikutnya akan ditambahkan seiring proses belajar berlanjut.</p>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="about-section">
    <div class="about-section-card glass">
      <h3>Tujuan Website Ini</h3>
      <p>
        Website ini dibangun menggunakan <strong style="color:var(--text-primary)">Rust</strong> dengan framework Axum
        sebagai bagian dari proses belajar web development. Setiap halaman dirancang untuk
        menampilkan materi pembelajaran dengan simulasi yang dapat dioperasikan secara langsung.
      </p>
    </div>
  </div>
</div>
"#;
    Html(layout("About", "about", body))
}
