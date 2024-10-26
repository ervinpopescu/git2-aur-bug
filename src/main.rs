fn main() {
    let aur_url = "https://aur.archlinux.org/neofetch-git.git";
    match git2::Repository::clone(aur_url, "./neofetch-git") {
        Ok(_r) => println!("works"),
        Err(_err) => {}
    }
}
