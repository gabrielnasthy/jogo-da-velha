// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Jogo da Velha
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021

use std::io;

fn main() {
    // Inicializa o tabuleiro com espaços vazios
    let mut tabuleiro = [[' '; 3]; 3];
    let mut jogador;
    let mut ganhou = false;

    imprime_tabuleiro(&tabuleiro);

    // Loop para fazer jogadas enquanto ninguém ganha e o tabuleiro não está cheio
    for rodada in 1..=9 {
        // Define o jogador da rodada
        if rodada % 2 == 1 {
            jogador = 'X';
        } else {
            jogador = 'O';
        }

        // Le a jogada de um jogador
        let (linha, coluna) = le_jogada(jogador);

        // Verifica se a posição está vazia
        if tabuleiro[linha][coluna] == ' ' {
            tabuleiro[linha][coluna] = jogador;
            imprime_tabuleiro(&tabuleiro);
        } else {
            println!("Posição já ocupada, tente novamente.");
            continue;
        }

        // Verifica se alguém ganhou
        if verifica_ganhador(&tabuleiro, jogador) {
            println!("Jogador {} ganhou!", jogador);
            ganhou = true;
            break;
        }
    }

    if !ganhou {
        println!("Empate!");
    }
}

fn imprime_tabuleiro(tabuleiro: &[[char; 3]; 3]) {
    println!(" {} | {} | {}", tabuleiro[0][0], tabuleiro[0][1], tabuleiro[0][2]);
    println!("---+---+---");
    println!(" {} | {} | {}", tabuleiro[1][0], tabuleiro[1][1], tabuleiro[1][2]);
    println!("---+---+---");
    println!(" {} | {} | {}", tabuleiro[2][0], tabuleiro[2][1], tabuleiro[2][2]);
}

fn le_jogada(jogador: char) -> (usize, usize) {
    loop {
        println!("Jogador {}, escolha sua jogada (linha e coluna): ", jogador);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
        let cordenada: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if cordenada.len() == 2 {

            let linha = cordenada[0];
            let coluna = cordenada[1];

            // Verifica se a linha e a coluna etsão dentro dos limites
            if linha < 3 && coluna < 3 {
                return (linha, coluna);
            }
        }
        println!("Entrada inválida, tente novamente.");
    }
}

fn verifica_ganhador(tabuleiro: &[[char; 3]; 3], jogador: char) -> bool {
    // Verifica linhas, colunas e diagonais
    for i in 0..3 {
        if (tabuleiro[i][0] == jogador && tabuleiro[i][1] == jogador && tabuleiro[i][2] == jogador) ||
            (tabuleiro[0][i] == jogador && tabuleiro[1][i] == jogador && tabuleiro[2][i] == jogador) {
            return true;
        }
    }
    if (tabuleiro[0][0] == jogador && tabuleiro[1][1] == jogador && tabuleiro[2][2] == jogador) ||
        (tabuleiro[0][2] == jogador && tabuleiro[1][1] == jogador && tabuleiro[2][0] == jogador) {
        return true;
    }
    false
}