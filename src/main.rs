fn main() {
    struct Config {
        mommy: String,
        little: String,
        pronoun: String,
        color: String,
        positive: Vec<String>,
        negative: Vec<String>,
    }

    let config = Config {
        mommy: "mommy".to_string(),
        little: "girl".to_string(),
        pronoun: "her".to_string(),
        color: "\x1b[38;5;217m".to_string(),
        positive: [
            "*pets your head*".to_string(),
            "awe, what a good {mommys_little}~\n{mommy} knew you could do it~ ❤️".to_string(),
            "good {mommys_little}~\n{mommy}'s so proud of you~ ❤️".to_string(),
            "Keep up the good work, my love~ ❤️".to_string(),
            "{mommy} is proud of the progress you've made~ ❤️".to_string(),
            "{mommy} is so grateful to have you as {pronoun} little {mommys_little}~ ❤️".to_string(),
            "I'm so proud of you, my love~ ❤️".to_string(),
            "{mommy} is so proud of you~ ❤️".to_string(),
            "{mommy} loves seeing {pronoun} little {mommys_little} succeed~ ❤️".to_string(),
            "{mommy} thinks {pronoun} little {mommys_little} earned a big hug~ ❤️".to_string(),
            "that's a good {mommys_little}~ ❤️".to_string(),
            "you did an amazing job, my dear~ ❤️".to_string(),
            "you're such a smart cookie~ ❤️".to_string(),
        ]
        .to_vec(),
        negative: [
            "do you need {mommy}'s help~? ❤️".to_string(),
            "Don't give up, my love~ ❤️".to_string(),
            "Don't worry, {mommy} is here to help you~ ❤️".to_string(),
            "I believe in you, my sweet {mommys_little}~ ❤️".to_string(),
            "It's okay to make mistakes, my dear~ ❤️".to_string(),
            "just a little further, sweetie~ ❤️".to_string(),
            "Let's try again together, okay~? ❤️".to_string(),
            "{mommy} believes in you, and knows you can overcome this~ ❤️".to_string(),
            "{mommy} believes in you~ ❤️".to_string(),
            "{mommy} is always here for you, no matter what~ ❤️".to_string(),
            "{mommy} is here to help you through it~ ❤️".to_string(),
            "{mommy} is proud of you for trying, no matter what the outcome~ ❤️".to_string(),
            "{mommy} knows it's tough, but you can do it~ ❤️".to_string(),
            "{mommy} knows {pronoun} little {mommys_little} can do better~ ❤️".to_string(),
            "{mommy} knows you can do it, even if it's tough~ ❤️".to_string(),
            "{mommy} knows you're feeling down, but you'll get through it~ ❤️".to_string(),
            "{mommy} knows you're trying your best~ ❤️".to_string(),
            "{mommy} loves you, and is here to support you~ ❤️".to_string(),
            "{mommy} still loves you no matter what~ ❤️".to_string(),
            "You're doing your best, and that's all that matters to {mommy}~ ❤️".to_string(),
            "{mommy} is always here to encourage you~ ❤️".to_string(),
        ]
        .to_vec(),
    };

    let color_end = "\x1b[0m".to_string();
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: mommy <positive/negative>");
        return;
    }
    match args.get(1).unwrap().as_str() {
        "positive" => {
            let random = rand::random::<usize>() % config.positive.len();
            let msg = config.positive[random]
                .replace("{mommy}", &config.mommy)
                .replace("{mommys_little}", &config.little)
                .replace("{pronoun}", &config.pronoun);
            println!("{}{}{}", config.color, msg, color_end);
        }
        "negative" => {
            let random = rand::random::<usize>() % config.negative.len();
            let msg = config.negative[random]
                .replace("{mommy}", &config.mommy)
                .replace("{mommys_little}", &config.little)
                .replace("{pronoun}", &config.pronoun);
            println!("{}{}{}", config.color, msg, color_end);
        }
        _ => {
            println!("Usage: mommy <positive/negative>");
        }
    }
}
