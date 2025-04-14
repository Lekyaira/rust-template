use std::{ 
    fs::{ File, OpenOptions },
    io::Write,
    error::Error, 
    path::Path, 
    io::{ self, BufRead },
    fmt,
};
use chrono::Local;
use indicatif::{ ProgressBar, ProgressStyle };

#[derive(Debug, Clone, PartialEq)]
enum TrainingError {
    NotEnoughValues(isize),
    AlphaTooLarge,
}
impl Error for TrainingError {}
impl fmt::Display for TrainingError {
    fn fmt(&self, f: &mut fmt::Formatter)
        -> fmt::Result
    {
        match self {
            Self::NotEnoughValues(line_no) => write!(f, "Not enough input parameters on line {line_no}"),
            Self::AlphaTooLarge => write!(f, "Alpha too large"),
        }
    }
}

fn read_file<P>(filename: P) 
    -> Result<Vec<(f64, f64)>, Box<dyn Error>>
    where P: AsRef<Path>
{
    // Open the file.
    let file = File::open(filename)?;
    // Get an iterator for each line in the file.
    let lines = io::BufReader::new(file).lines();

    // Create an empty Vec.
    let mut output: Vec<(f64, f64)> = vec![];

    // Iterate over the lines.
    let mut line_no = 1;
    for line in lines.map_while(Result::ok) {
        // Split the line by comma.
        let s: Vec<&str> = line
            .as_str()
            .split(',')
            .collect();
        if s.len() < 2 {
            return Err(Box::new(TrainingError::NotEnoughValues(line_no)));
        }
        // Add the values to the output Vec.
        let input_value = s[0].trim().parse()?;
        let output_value = s[1].trim().parse()?;
        output.push((input_value, output_value));
        line_no += 1;
    }

    // Return the Vec.
    Ok(output)
}

fn log(line: String) 
    -> Result<(), Box<dyn Error>>
{
    // Open log file for write.
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?;
    // Prepend newline.
    let line = format!("\n{line}");
    // Write log entry to the file.
    file.write(line.as_str().as_bytes())?;

    Ok(())
}

fn cost(values: Vec<(f64, f64)>, w: f64, b: f64) 
    -> f64 
{
    // Calculate the cost for w,b.
    let m = values.len() as f64;
    let mut cost = 0.0;

    for (x, y) in values.iter() {
        let f_wb = w * x + b;                   // f_w,b(x) = wx+b
        let fxmy2 = (f_wb - y) * (f_wb - y);    // (f_wb(x) - y)²
        cost = cost + fxmy2;                    // ∑(f_wb(x) - y)²
    }
    let total_cost = 1.0 / (2.0 * m) * cost;    // (1/2m)∑(f_wb(x) - y)²

    // Return the total cost.
    total_cost
}

fn gradient(values: Vec<(f64, f64)>, w: f64, b: f64)
    -> (f64, f64)
{
    // Calculate the gradient for w,b.
    let mut dj_dw = 0.0;
    let mut dj_db = 0.0;
    let m = values.len() as f64;

    for (x, y) in values.iter() {
        let f_wb = w * x + b;           // f_w,b(x)
        let dj_dw_i = (f_wb - y) * x;   // (f_w,b(x) - y)x
        let dj_db_i = f_wb - y;         // f_w,b(x) - y
        dj_dw += dj_dw_i;               // ∑(f_w,b(x) - y)x
        dj_db += dj_db_i;               // ∑(f_w,b(x) - y)
    }
    // Average the results.
    dj_dw = dj_dw / m;                  // ∂J(w,b)/∂w = (1/m)∑(f_w,b(x) - y)x
    dj_db = dj_db / m;                  // ∂J(w,b)/∂b = (1/m)∑(f_w,b(x) - y)

    // Return the gradient.
    (dj_dw, dj_db)
}

fn gradient_descent(values: Vec<(f64, f64)>, alpha: f64, w: f64, b: f64)
    -> Result<(f64, f64), Box<dyn Error>>
{
    // Set up the progress spinner.
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Training...");
    spinner.set_style(ProgressStyle::with_template("{msg} {spinner}").unwrap());

    // Calculate gradient descent.
    let mut w = w;
    let mut b = b;
    let mut cost_prev = 0.0;
    let mut i = 0;
    loop {
        // Calculate the gradient.
        let (dj_dw, dj_db) = gradient(values.clone(), w, b);
        // Update the parameters.
        b = b - alpha * dj_db;
        w = w - alpha * dj_dw;
        // Calculate the cost.
        let cost = cost(values.clone(), w, b);
        // Print the cost at intervals 10 times or as many iterations if < 10.
        if i % 1000 == 0 {
            log(format!("\tIteration: {i}\n\t\tCost: {cost}\n\t\tdj_dw: {dj_dw}, dj_db: {dj_db}\n\t\tw: {w}, b: {b}"))?;
            //println!("Iteration: {i}\n\tCost: {cost}\n\tdj_dw: {dj_dw}, dj_db: {dj_db}\n\tw: {w}, b: {b}");
        }
        // If the cost doesn't change, we're done.
        if cost == cost_prev { break; }
        // If the cost is getting higher the alpha is too large.
        if cost > cost_prev + 1000.0 {
            return Err(Box::new(TrainingError::AlphaTooLarge));
        }
        cost_prev = cost;
        i += 1;
        // Spin the spinner.
        spinner.tick();
    }
    // Clean up the spinner.
    spinner.finish();

    // Return the calculates w,b.
    Ok((w, b))
}

fn main() {
    // Start log entry.
    let now = Local::now();
    log(format!("Training run started on {now}"))
        .unwrap_or_else(|e| println!("Log error: {e}"));
    // Read the file.
    let values = read_file("./training_values")
        .expect("Invalid training data!");
    // Calculate the gradient descent.
    //let alpha = 1.0e-5;
    let mut alpha = 1.0;
    let w_init = 0.0;
    let b_init = 0.0;
    // Fit alpha. Start at 1.0 and work down in 0.1 increments.
    loop {
        match gradient_descent(values.clone(), alpha, w_init, b_init) {
            Err(e) => {
                if let Some(training_error) = e.downcast_ref::<TrainingError>() {
                    if *training_error != TrainingError::AlphaTooLarge {
                        println!("Error: {e}");
                        break;
                    }
                } else {
                    println!("Error: {e}");
                    break;
                }
            },
            Ok((w, b)) => { // Success!
                println!("\nFinal w,b: ({w}, {b}).\nFit Equation: f(x) = {w}x+{b}");
                break;
            },
        }
        // Failed. Try again with a smaller alpha.
        log("\tAlpha too large. Trying again with smaller alpha...".into())
            .unwrap_or_else(|e| println!("Log error: {e}"));
        //println!("Alpha too large. Trying again with smaller alpha...");
        alpha *= 0.1;
    }
}
