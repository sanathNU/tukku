use tukku_rust_macro::kannada;

fn main() {
     kannada! {
        ಸಾರ್ವಜನಿಕ ಕಾರ್ಯ ಮುಖ್ಯ() {
            ಮಾಡು x = 42;
            ಯದಿ x > 0 {
                println!("ಧನಾತ್ಮಕ");
            }
        }
    }
}