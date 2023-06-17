use rand::Rng;
use rand::seq::SliceRandom;

//FUNÇÃO QUE CALCULA A DISTANCIA EUCLIDIANA ENTRE DOIS PONTOS
fn calculate_distance(city_a: &[i32; 2], city_b: &[i32; 2]) -> f64 {
    let dx = (city_a[0] - city_b[0]) as f64;
    let dy = (city_a[1] - city_b[1]) as f64;
    (dx.powi(2) + dy.powi(2)).sqrt()
}

// FUNÇÃO QUE CALCULA A DISTANCIA TOTAL DA ROTA
fn calculate_total_distance(route: &[usize], city_positions: &[[i32; 2]]) -> f64 {
    let mut total_distance = 0.0;
    for i in 0..route.len() - 1 {
        let city_a = &city_positions[route[i]];
        let city_b = &city_positions[route[i + 1]];
        total_distance += calculate_distance(city_a, city_b);
    }
    total_distance + calculate_distance(&city_positions[route[route.len() - 1]], &city_positions[route[0]])
}

//FUNÇÃO DE SELEÇÃO DOS PAIS
fn selection(population: &[Vec<usize>], city_positions: &[[i32; 2]]) -> (Vec<usize>, Vec<usize>) {
    let fitness_scores: Vec<f64> = population
        .iter()
        .map(|route| 1.0 / calculate_total_distance(route, city_positions))
        .collect();

    let total_fitness: f64 = fitness_scores.iter().sum();
    let probabilities: Vec<f64> = fitness_scores.iter().map(|&fitness| fitness / total_fitness).collect();

    let mut rng = rand::thread_rng();

    let parent1_index = roulette_wheel_selection(&probabilities, &mut rng);
    let parent1 = &population[parent1_index].clone();

    let mut parent2_index = roulette_wheel_selection(&probabilities, &mut rng);
    while parent2_index == parent1_index {
        parent2_index = roulette_wheel_selection(&probabilities, &mut rng);
    }
    let parent2 = &population[parent2_index].clone();

    (parent1.clone(), parent2.clone())
}

//FUNÇÃO DE SELEÇÃO DA ROLETA
fn roulette_wheel_selection(probabilities: &[f64], rng: &mut dyn rand::RngCore) -> usize {
    let random_value = rng.gen::<f64>();
    let mut cumulative_probability = 0.0;

    for (index, &probability) in probabilities.iter().enumerate() {
        cumulative_probability += probability;
        if cumulative_probability >= random_value {
            return index;
        }
    }

    probabilities.len() - 1
}


//FUNÇÃO QUE GERA OS FILHOS DA POPULAÇÃO DADO DOIS PAIS
fn crossover(parents: (&[usize], &[usize])) -> (Vec<usize>, Vec<usize>) {
    let parent1 = parents.0;
    let parent2 = parents.1;
    let num_cities = parent1.len();

    let (gene1, gene2) = {
        let mut rng = rand::thread_rng();
        let gene1 = rng.gen_range(0..num_cities);
        let gene2 = rng.gen_range(0..num_cities);
        (gene1, gene2)
    };

    let (start_gene, end_gene) = if gene1 < gene2 {
        (gene1, gene2)
    } else {
        (gene2, gene1)
    };

    let mut child1 = vec![usize::MAX; num_cities];
    let mut child2 = vec![usize::MAX; num_cities];

    child1[start_gene..=end_gene].copy_from_slice(&parent1[start_gene..=end_gene]);
    child2[start_gene..=end_gene].copy_from_slice(&parent2[start_gene..=end_gene]);

    let mut remaining_genes_parent1: Vec<usize> = parent1
        .iter()
        .filter(|&gene| !child1[start_gene..=end_gene].contains(gene))
        .cloned()
        .collect();

    let mut remaining_genes_parent2: Vec<usize> = parent2
        .iter()
        .filter(|&gene| !child2[start_gene..=end_gene].contains(gene))
        .cloned()
        .collect();

    let mut index = (end_gene + 1) % num_cities;
    for i in 0..num_cities {
        if !child2.contains(&parent1[index]) {
            let empty_index = child2.iter().position(|&x| x == usize::MAX).unwrap();
            child2[empty_index] = parent1[index];
        }
        index = (index + 1) % num_cities;
    }

    let mut index = (end_gene + 1) % num_cities;
    for i in 0..num_cities {
        if !child1.contains(&parent2[index]) {
            let empty_index = child1.iter().position(|&x| x == usize::MAX).unwrap();
            child1[empty_index] = parent2[index];
        }
        index = (index + 1) % num_cities;
    }

    (child1, child2)
}

//FUNÇÃO DE MUTAÇÃO
fn mutation(route: &mut Vec<usize>, mutation_rate: f64) {
    let num_cities = route.len();
    let mut rng = rand::thread_rng();

    for i in 0..num_cities {
        if rng.gen::<f64>() < mutation_rate {
            let j = rng.gen_range(0..num_cities);
            route.swap(i, j);
        }
    }
}


//FUNÇÃO DO ALGORITMO GENETICO
fn genetic_algorithm(city_positions: &[[i32; 2]], population_size: usize, num_generations: usize, mutation_rate: f64) -> (Vec<usize>, f64) {
    let mut count = 0;
    let num_cities = city_positions.len();

    // CRIAÇÃO DA POPULAÇÃO INICIAL
    let mut population: Vec<Vec<usize>> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..population_size {
        let mut route: Vec<usize> = (0..num_cities).collect();
        route.shuffle(&mut rng);
        population.push(route);
    }

    for _ in 0..num_generations {
        let mut new_population: Vec<Vec<usize>> = Vec::new();

        while new_population.len() < population_size {
            let parents = selection(&population, city_positions);
            let (mut child1, mut child2) = crossover((&parents.0, &parents.1));
            mutation(&mut child1, mutation_rate);
            mutation(&mut child2, mutation_rate);
            new_population.push(child1);
            new_population.push(child2);
            count = count + 1;
        }

        // PRESERVAR O MELHOR INDIVIDUO DA POPULAÇÃO ATUAL
        let best_individual = population
        .iter()
        .min_by(|&a, &b| calculate_total_distance(a, city_positions)
        .partial_cmp(&calculate_total_distance(b, city_positions)).unwrap())
        .unwrap()
        .clone();

        new_population.push(best_individual);


        population = new_population;
    }

    let best_route = population
        .iter()
        .min_by(|&a, &b| calculate_total_distance(a, city_positions).partial_cmp(&calculate_total_distance(b, city_positions)).unwrap())
        .unwrap()
        .clone();

    let origin_city = best_route[0];
    let mut final_route = best_route.clone();
    final_route.push(origin_city);

    let best_distance = calculate_total_distance(&final_route, city_positions);

    (final_route, best_distance)
}

fn main() {
    let city_positions: [[i32; 2]; 12] = [
        [1, 5],
        [4, 6],
        [7, 5],
        [5, 4],
        [9, 4],
        [2, 3],
        [4, 2],
        [6, 2],
        [1, 1],
        [5, 1],
        [3, 0],
        [9, 0],
    ];

    //MUDE AQUI O TAMANHO DA POPULAÇÃO O NUMERO DE GERAÇÕES E A TAXA DE MUTAÇÃO
    let population_size = 100; //100
    let num_generations = 200; //200
    let mutation_rate = 0.01; //0.01

    let mut count = 0;
    let mut som = 0.0;
    let mut br: Vec<usize> = Vec::new();
    let mut bd :f64 = 0.0;
    for i in 0..40{
        let (best_route, best_distance) = genetic_algorithm(&city_positions, population_size, num_generations, mutation_rate);
        som = som + best_distance;
        count = count + 1;
        br = best_route;
        bd = best_distance;
    }
    println!("Media das Distancias = {}", som / count as f64);
    println!("Melhor rota encontrada: {:?}", br);
    println!("Distância total: {}", bd);


}
