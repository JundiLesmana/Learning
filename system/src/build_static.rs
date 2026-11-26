use learning_notes::pages::{home, about, mylearn, insert_sorting, bubble_sorting, pointer, statistics, tree, network, eigen};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("docs");
    fs::create_dir_all(out_dir)?;

    let routes = vec![
        ("index.html", home::handler().await.0),
        ("about/index.html", about::handler().await.0),
        ("mylearn/index.html", mylearn::handler().await.0),
        ("Insert-Sorting/index.html", insert_sorting::handler().await.0),
        ("Bubble-Sorting/index.html", bubble_sorting::handler().await.0),
        ("Pointer/index.html", pointer::handler().await.0),
        ("Distribusi-Normal/index.html", statistics::handler().await.0),
        ("Pohon/index.html", tree::handler().await.0),
        ("Jaringan-Komputer/index.html", network::handler().await.0),
        ("Nilai-Eigen/index.html", eigen::handler().await.0),
    ];

    for (path, html) in routes {
        let full_path = out_dir.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, html)?;
        println!("Generated: {}", full_path.display());
    }

    // 404 fallback untuk SPA routing di GitHub Pages subpath
    let not_found = home::handler().await.0;
    fs::write(out_dir.join("404.html"), not_found)?;
    println!("Generated: 404.html");

    let nojekyll = out_dir.join(".nojekyll");
    fs::write(nojekyll, "")?;
    println!("Generated: .nojekyll");

    Ok(())
}