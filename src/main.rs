fn show_options() {
    println!(
        "[1] - Celsius para Fahrenheit\n
[2] - Fahrenheit para Celsius\n
Sua escolha:"
    );
}

fn read_input() -> f64 {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha na leitura!");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("O valor precisa ser um número!");
                continue;
            }
        };
        return input;
    }
}

fn to_fahrenheit(temperature: f64) {
    let fahrenheit: f64 = (temperature * 1.8) + 32.0;

    println!("\nResultado: {}°C = {}°F", temperature, fahrenheit);

    println!("==================================================");
}

fn to_celsius(temperature: f64) {
    let celsius: f64 = (temperature - 32.0) * 5.0 / 9.0;

    println!("\nResultado: {}°F = {}°C", temperature, celsius);

    println!("==================================================");
}

fn operation(mut choice: u32) {
    loop {
        if choice != 1 && choice != 2 {
            println!("Operação inválida!");

            show_options();

            choice = read_input() as u32;
            continue;
        } else {
            if choice == 1 {
                println!("\nQual a temperatura em Celsius? R:");
                to_fahrenheit(read_input());
                break;
            } else {
                println!("\nQual a temperatura em Fahrenheit? R:");
                to_celsius(read_input());
                break;
            }
        }
    }
}

fn ask_for_more() -> bool {
    println!(
        "\nDeseja realizar outra operação?\n
[1] - Sim\n
[2] - Não\n
Sua escolha:"
    );

    if read_input() == 1.0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("========== Conversor de Temperatura ==========\n");

    loop {
        show_options();

        let choice: u32 = read_input() as u32;

        operation(choice);

        if ask_for_more() {
            println!(
                "\nReiniciando o programa...\n
Qual operação deseja realizar agora?"
            );
            continue;
        } else {
            println!(
                "\nEncerrando o programa!\n
Obrigado por utilizar!"
            );
            break;
        }
    }
}
